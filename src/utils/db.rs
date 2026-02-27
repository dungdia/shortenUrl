use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use deadpool_redis::{Config, Runtime};
use std::env;
pub struct DbContext {
    pub mysql_pool: MySqlPool,
    pub redis_pool: deadpool_redis::Pool,
}

impl DbContext {
    pub async fn init() -> Self {

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL missing");
        let max_connections = env::var("DATABASE_MAX_CONNECTION")
            .expect("DATABASE_MAX_CONNECTION missing")
            .parse::<u32>()
            .expect("Invalid DATABASE_MAX_CONNECTION value");

        let mysql_pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_url)
            .await
            .expect("Failed to connect to MySQL");

        let mysql_pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_url)
            .await
            .expect("Failed to connect to MySQL");

        let cfg = Config::from_url(redis_url);
        let redis_pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis pool");

        Self {
            mysql_pool,
            redis_pool,
        }
    }
}