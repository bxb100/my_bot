use crate::types::MyResult;
use sqlx::{query_as, SqlitePool};

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct Gamble {
    pub id: i64,
    pub serial_id: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub action: String,
    pub amount: Option<i64>,
}

pub async fn get_by_serial_id(pool: &SqlitePool, serial_id: &String) -> MyResult<Vec<Gamble>> {
    let data = query_as!(
        Gamble,
        // language=sqlite
        r#"SELECT * from gambles where serial_id = ?1"#,
        serial_id
    )
    .fetch_all(pool)
    .await?;

    Ok(data)
}

pub async fn insert(pool: &SqlitePool, gambles: Gamble) -> MyResult<()> {
    sqlx::query!(
            // language=sqlite
            r#"INSERT INTO gambles (serial_id, user_id, user_name, action, amount) VALUES (?1, ?2, ?3, ?4, ?5)"#,
            gambles.serial_id,
            gambles.user_id,
            gambles.user_name,
            gambles.action,
            gambles.amount
        )
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_by_user_id_and_empty_amount(pool: &SqlitePool, user_id: i64) -> MyResult<Gamble> {
    let data = query_as!(
        Gamble,
        // language=sqlite
        r#"SELECT * from gambles where user_id = ?1 and amount is null order by serial_id desc"#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(data)
}

pub async fn get_by_user_id_and_serial_id(
    pool: &SqlitePool,
    user_id: i64,
    serial_id: &str,
) -> MyResult<Option<Gamble>> {
    let data = query_as!(
        Gamble,
        // language=sqlite
        r#"SELECT * FROM gambles where user_id = ?1 and serial_id = ?2"#,
        user_id,
        serial_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(data)
}

pub async fn update_amount(pool: &SqlitePool, id: i64, amount: i64) -> MyResult<()> {
    sqlx::query!(
        // language=sqlite
        r#"UPDATE gambles SET amount = ?2 where id = ?1"#,
        id,
        amount
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_by_serial_id(pool: &SqlitePool, serial_id: &String) -> MyResult<()> {
    sqlx::query!(
        // language=sqlite
        r#"DELETE FROM gambles where serial_id = ?1"#,
        serial_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
