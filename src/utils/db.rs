// src/utils/db.rs
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

pub struct DbContext {
    pub mysql_pool: MySqlPool,
}

impl DbContext {
    pub async fn init() -> Self {

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
        let max_connections = env::var("DATABASE_MAX_CONNECTION")
            .expect("DATABASE_MAX_CONNECTION missing")
            .parse::<u32>()
            .expect("Invalid DATABASE_MAX_CONNECTION value");

        let mysql_pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_url)
            .await
            .expect("Failed to connect to MySQL");

        Self {
            mysql_pool,
        }
    }
}