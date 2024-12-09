use crate::types::MyResult;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, SqlitePool};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Users {
    pub id: i64,
    pub name: Option<String>,
    pub points: i64,
    pub daily_reward: Option<DateTime<Utc>>,
}

pub async fn get_by_id(pool: &SqlitePool, id: i64) -> MyResult<Option<Users>> {
    // https://github.com/launchbadge/sqlx/issues/598
    let data = query_as!(
        Users,
        // language=sqlite
        r#"SELECT id, name, points, daily_reward as "daily_reward: _" FROM users WHERE id = ?1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(data)
}

pub async fn insert(pool: &SqlitePool, users: Users) -> MyResult<()> {
    query!(
        // language=sqlite
        r#"INSERT INTO users (id, name, points, daily_reward) VALUES (?1, ?2, ?3, ?4)"#,
        users.id,
        users.name,
        users.points,
        users.daily_reward
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_amount(pool: &SqlitePool, id: i64, amount: i64) -> MyResult<()> {
    query!(
        // language=sqlite
        r#"UPDATE users SET points = ?1 WHERE id = ?2"#,
        amount,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
