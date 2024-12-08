use crate::db::Database;
use crate::types::MyResult;
use sqlx::{query, query_as};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Users {
    pub id: i64,
    pub name: Option<String>,
    pub points: i64,
    pub daily_reward: i64,
}

#[derive(Debug, Default)]
pub struct UpsertUsers {
    pub id: i64,
    pub name: Option<String>,
    pub points: i64,
    pub daily_reward: i64,
}

pub struct UserDao {
    pub database: Database,
}

impl UserDao {
    pub async fn get_by_id(&self, id: i64) -> MyResult<Option<Users>> {
        let data = query_as!(
            Users,
            // language=sqlite
            r#"SELECT * FROM user WHERE id = ?1"#,
            id
        )
        .fetch_optional(self.database.pool)
        .await?;

        Ok(data)
    }

    pub async fn upsert(&self, user: UpsertUsers) -> MyResult<Users> {
        let old = self.get_by_id(user.id).await?;

        if old.is_some() {
            query!(
                // language=sqlite
                r#"UPDATE user SET name = ?1, points = ?2, daily_reward = ?3 WHERE id = ?4"#,
                user.name,
                user.points,
                user.daily_reward,
                user.id
            )
            .execute(self.database.pool)
            .await?;

            return Ok(Users {
                id: user.id,
                name: user.name,
                points: user.points,
                daily_reward: user.daily_reward,
            });
        }

        let inserted = query_as!(
            Users,
            // language=sqlite
            r#"INSERT INTO user (id, name, points, daily_reward) VALUES (?1, ?2, ?3, ?4) RETURNING *"#,
            user.id,
            user.name,
            user.points,
            user.daily_reward
        )
            .fetch_one(self.database.pool)
            .await?;

        Ok(inserted)
    }
}
