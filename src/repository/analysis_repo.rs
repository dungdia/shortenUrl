use sqlx::MySqlPool;

use crate::models::analysis_models::UrlClickCount;

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
    pub async fn get_click_stats_by_url(&self, short_code: String) -> Result<Option<UrlClickCount>, sqlx::Error> {
        sqlx::query_as::<sqlx::MySql, UrlClickCount>(
            r#"
            SELECT 
                u.id as url_id, 
                u.long_url, 
                u.short_code, 
                COUNT(a.id) as click_count
            FROM urls u
            LEFT JOIN access_logs a ON u.id = a.url_id
            WHERE u.short_code = ?
            GROUP BY u.id
            "#
        )
        .bind(short_code)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_all_click_stats(&self) -> Result<Vec<UrlClickCount>, sqlx::Error> {
        sqlx::query_as::<sqlx::MySql, UrlClickCount>(
            r#"
            SELECT 
                u.id as url_id, 
                u.long_url, 
                u.short_code, 
                COUNT(a.id) as click_count
            FROM urls u
            LEFT JOIN access_logs a ON u.id = a.url_id
            GROUP BY u.id
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

}