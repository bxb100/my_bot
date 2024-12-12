use crate::types::MyResult;
use chrono::{DateTime, Utc};
use cron::Schedule;
use log::info;
use sqlx::{query, query_as, PgPool};

#[derive(Debug)]
pub struct JobSchedule {
    pub name: &'static str,
    pub schedule: Schedule,
    pub metadata: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Job {
    pub id: i64,
    pub name: String,
    pub scheduled_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

pub async fn insert(
    pool: &PgPool,
    name: &str,
    scheduled_at: &DateTime<Utc>,
    metadata: &serde_json::Value,
) -> MyResult<Job> {
    let job = query_as!(
        Job,
        // language=postgresql
        r#"
        INSERT INTO jobs (name, scheduled_at, metadata)
        VALUES ($1, $2, $3)
        returning *
        "#,
        name,
        scheduled_at,
        metadata
    )
    .fetch_one(pool)
    .await?;

    Ok(job)
}

pub async fn get_by_name_and_scheduled_at(
    pool: &PgPool,
    name: &str,
    schedule_at: &DateTime<Utc>,
) -> MyResult<Job> {
    info!(
        "get_by_name_and_scheduled_at(name = {}, schedule_at = {})",
        name, schedule_at
    );

    let job = query_as!(
        Job,
        // language=postgresql
        r#"
        SELECT *
        FROM jobs
        WHERE name = $1 AND scheduled_at = $2
        "#,
        name,
        schedule_at
    )
    .fetch_one(pool)
    .await?;

    Ok(job)
}

pub async fn get_jobs_to_execute(pool: &PgPool) -> MyResult<Vec<Job>> {
    let jobs = query_as!(
        Job,
        // language=postgresql
        r#"
        SELECT *
        FROM jobs
        WHERE scheduled_at <= now() AND error_message IS NULL
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(jobs)
}

pub async fn update_job_executed_at(pool: &PgPool, id: i64) -> MyResult<()> {
    query!(
        // language=postgresql
        r#"
        UPDATE jobs
        SET executed_at = now()
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_job(pool: &PgPool, id: i64) -> MyResult<()> {
    query!(
        // language=postgresql
        r#"
        DELETE FROM jobs
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_job_error_message(pool: &PgPool, id: i64, message: &String) -> MyResult<()> {
    query!(
        // language=postgresql
        r#"
        UPDATE jobs
        SET error_message = $2
        WHERE id = $1
        "#,
        id,
        message
    )
    .execute(pool)
    .await?;

    Ok(())
}
