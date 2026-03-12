use sqlx::MySqlPool;

pub struct AnalyticsRepository {
    pool: MySqlPool,
}

impl AnalyticsRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool: pool }
    }

    pub async fn insert_log(&self, url_id: i32, ip: Option<String>, ua: Option<String>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO access_logs (url_id, visitor_ip, user_agent) VALUES (?, ?, ?)",
            url_id, ip, ua
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn count_clicks(&self, url_id: i32) -> Result<i64, sqlx::Error> {
        let row = sqlx::query!("SELECT COUNT(*) as count FROM access_logs WHERE url_id = ?", url_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.count as i64)
    }

    pub async fn get_clicks_in_range(
        &self, 
        url_id: i32, 
        start: chrono::NaiveDateTime, 
        end: chrono::NaiveDateTime
    ) -> Result<i64, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM access_logs 
             WHERE url_id = ? AND accessed_at BETWEEN ? AND ?",
            url_id, start, end
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count as i64)
    }
}