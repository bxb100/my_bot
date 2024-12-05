use crate::db::Database;
use crate::types::MyResult;
use sqlx::query_as;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Wager {
    pub id: i64,
    pub time_id: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub action: String,
    pub amount: Option<i64>,
}

pub struct UpsertWager {
    pub time_id: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub action: String,
    pub amount: Option<i64>,
}

pub struct WagerDao {
    pub database: Database,
}

impl WagerDao {
    pub async fn get_by_time_id(&self, time_id: String) -> MyResult<Vec<Wager>> {
        let data = query_as!(
            Wager,
            // language=sqlite
            r#"SELECT id, time_id, user_id, user_name, action, amount from wager where time_id = ?1"#,
            time_id
        )
            .fetch_all(self.database.pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(&self, wager: UpsertWager) {
        sqlx::query!(
            // language=sqlite
            r#"INSERT INTO wager (time_id, user_id, user_name, action, amount) VALUES (?1, ?2, ?3, ?4, ?5)"#,
            wager.time_id,
            wager.user_id,
            wager.user_name,
            wager.action,
            wager.amount
        )
            .execute(self.database.pool)
            .await
            .expect("Error inserting wager");
    }

    pub async fn get_by_user_id_and_empty_amount(&self, user_id: i64) -> MyResult<Wager> {
        let data = query_as!(
            Wager,
            // language=sqlite
            r#"SELECT id, time_id, user_id, user_name, action, amount from wager where user_id = ?1 and amount is null order by time_id desc"#,
            user_id
        )
            .fetch_one(self.database.pool)
            .await?;

        Ok(data)
    }

    pub async fn get_by_user_id_and_time_id(
        &self,
        user_id: i64,
        time_id: &str,
    ) -> MyResult<Option<Wager>> {
        let data = query_as!(
            Wager,
            // language=sqlite
            r#"SELECT * FROM wager where user_id = ?1 and time_id = ?2"#,
            user_id,
            time_id
        )
        .fetch_optional(self.database.pool)
        .await?;

        Ok(data)
    }

    pub async fn update(&self, wager: UpsertWager, id: i64) -> MyResult<()> {
        sqlx::query!(
            // language=sqlite
            r#"UPDATE wager SET user_name = ?2, action = ?3, amount = ?4, time_id = ?5 where id = ?1"#,
            id,
            wager.user_name,
            wager.action,
            wager.amount,
            wager.time_id,
        )
            .execute(self.database.pool)
            .await?;

        Ok(())
    }
}
