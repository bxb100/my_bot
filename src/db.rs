use crate::config::BOT_CONFIG;
use log::info;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

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
