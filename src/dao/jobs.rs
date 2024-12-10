use crate::types::MyResult;
use chrono::{DateTime, Utc};
use log::info;
use sqlx::{query_as, SqlitePool};

#[derive(Debug)]
pub struct Job {
    id: i64,
    name: String,
    scheduled_at: DateTime<Utc>,
    metadata: serde_json::Value,
    executed_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
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
