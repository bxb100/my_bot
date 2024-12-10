use crate::types::MyResult;
use chrono::{DateTime, Utc};
use cron::Schedule;
use log::info;
use sqlx::{query, query_as, SqlitePool};

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
    pub metadata: Option<String>,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

pub async fn insert(
    pool: &SqlitePool,
    name: &str,
    scheduled_at: &DateTime<Utc>,
    metadata: &serde_json::Value,
) -> MyResult<Job> {
    let job = query_as!(
        Job,
        // language=sqlite
        r#"
        INSERT INTO jobs (name, scheduled_at, metadata)
        VALUES ($1, $2, $3)
        returning id, name, scheduled_at as "scheduled_at: _", metadata, executed_at as "executed_at: _", error_message
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
    pool: &SqlitePool,
    name: &str,
    schedule_at: &DateTime<Utc>,
) -> MyResult<Job> {
    info!(
        "get_by_name_and_scheduled_at(name = {}, schedule_at = {})",
        name, schedule_at
    );

    let job = query_as!(
        Job,
        // language=sqlite
        r#"
        SELECT id, name, scheduled_at as "scheduled_at: _", metadata, executed_at as "executed_at: _", error_message
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

pub async fn get_jobs_to_execute(pool: &SqlitePool) -> MyResult<Vec<Job>> {
    let jobs = query_as!(
        Job,
        // language=sqlite
        r#"
        SELECT id, name, scheduled_at as "scheduled_at: _", metadata, executed_at as "executed_at: _", error_message
        FROM jobs
        WHERE datetime(scheduled_at) <= current_timestamp AND error_message IS NULL
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(jobs)
}

pub async fn update_job_executed_at(pool: &SqlitePool, id: i64) -> MyResult<()> {
    query!(
        // language=sqlite
        r#"
        UPDATE jobs
        SET executed_at = current_timestamp
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_job(pool: &SqlitePool, id: i64) -> MyResult<()> {
    query!(
        // language=sqlite
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

pub async fn update_job_error_message(
    pool: &SqlitePool,
    id: i64,
    message: &String,
) -> MyResult<()> {
    query!(
        // language=sqlite
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
