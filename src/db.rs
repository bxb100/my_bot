use crate::config::BOT_CONFIG;
use crate::dao::jobs::{
    delete_job, get_by_name_and_scheduled_at, get_jobs_to_execute, insert,
    update_job_error_message, update_job_executed_at, JobSchedule,
};
use crate::jobs::jobs;
use crate::types::{Context, MyResult};
use chrono::Utc;
use log::{error, info, trace};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: &'static SqlitePool,
}

impl Database {
    pub async fn new() -> Self {
        Database {
            pool: Self::get_connection_pool().await,
        }
    }

    pub async fn get_connection_pool() -> &'static SqlitePool {
        static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

        POOL.get_or_init(|| async {
            info!("Init SQLite");
            SqlitePoolOptions::new()
                .connect(&BOT_CONFIG.database_url)
                .await
                .expect("Failed to connect SQLite")
        })
        .await
    }
}

pub async fn schedule_jobs(db: &Database, jobs: Vec<JobSchedule>) -> anyhow::Result<()> {
    for job in jobs {
        let mut upcoming = job.schedule.upcoming(Utc).take(1);

        if let Some(scheduled_at) = upcoming.next() {
            schedule_job(db, job.name, job.metadata, scheduled_at).await?;
        }
    }

    Ok(())
}

pub async fn schedule_job(
    db: &Database,
    job_name: &str,
    job_metadata: serde_json::Value,
    when: chrono::DateTime<Utc>,
) -> anyhow::Result<()> {
    let all_jobs = jobs();
    if !all_jobs.iter().any(|j| j.name() == job_name) {
        anyhow::bail!("Job {} does not exist in the current job list.", job_name);
    }

    if get_by_name_and_scheduled_at(db.pool, job_name, &when)
        .await
        .is_err()
    {
        insert(db.pool, job_name, &when, &job_metadata).await?;
    }

    Ok(())
}

pub async fn run_scheduled_jobs(ctx: &Context, db: &Database) -> anyhow::Result<()> {
    let jobs = get_jobs_to_execute(db.pool).await?;
    trace!("jobs to execute: {:#?}", jobs);

    for job in jobs.iter() {
        update_job_executed_at(db.pool, job.id).await?;

        match handle_job(ctx, &job.id, &job.name, job.metadata.as_ref()).await {
            Ok(_) => {
                trace!("job successfully executed (id={})", job.id);
                delete_job(db.pool, job.id).await?;
            }
            Err(e) => {
                error!("job failed on execution (id={:?}, error={:?})", job.id, e);
                update_job_error_message(db.pool, job.id, &e.to_string()).await?;
            }
        }
    }

    Ok(())
}

async fn handle_job(
    ctx: &Context,
    id: &i64,
    name: &String,
    metadata: Option<&String>,
) -> MyResult<()> {
    for job in jobs() {
        if job.name() == name {
            return job.run(id, ctx, metadata).await;
        }
    }
    trace!(
        "handle_job fell into default case: (name={:?}, metadata={:?})",
        name,
        metadata
    );

    Ok(())
}
