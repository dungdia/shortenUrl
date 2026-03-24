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

    pub async fn get_paginated_urls(
        &self, 
        search: Option<String>,
        limit: i64, 
        offset: i64
    ) -> Result<Vec<UrlClickCount>, sqlx::Error> {
        // Tạo pattern tìm kiếm cho SQL LIKE
        let search_term = search
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

        // 2. Sử dụng Query Builder hoặc logic IF trong SQL
        // Ở đây ta dùng cách đơn giản: nếu search_term là Some thì thêm % %, không thì dùng NULL
        let pattern = search_term.as_ref().map(|s| format!("%{}%", s));

        sqlx::query_as::<sqlx::MySql, UrlClickCount>(
            r#"
            SELECT u.id as url_id, u.long_url, u.short_code, COUNT(a.id) as click_count
            FROM urls u
            LEFT JOIN access_logs a ON u.id = a.url_id
            WHERE (? IS NULL OR u.short_code LIKE ? OR u.long_url LIKE ?)
            GROUP BY u.id
            ORDER BY u.id DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(&pattern) // Bind cho short_code
        .bind(&pattern) // Bind cho long_url
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn count_total_urls(&self, search: Option<String>) -> Result<i64, sqlx::Error> {
        // 1. Làm sạch input: loại bỏ khoảng trắng và kiểm tra chuỗi rỗng
        let search_term = search
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
    
        // 2. Chỉ tạo pattern nếu thực sự có từ khóa tìm kiếm
        let pattern = search_term.as_ref().map(|s| format!("%{}%", s));
    
        // 3. Sử dụng logic (? IS NULL) để MySQL có thể tối ưu hóa query
        // Nếu pattern là None, MySQL sẽ bỏ qua các điều kiện LIKE phía sau
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) 
            FROM urls 
            WHERE (? IS NULL OR short_code LIKE ? OR long_url LIKE ?)
            "#
        )
        .bind(&pattern) // Bind cho kiểm tra NULL
        .bind(&pattern) // Bind cho short_code
        .bind(&pattern) // Bind cho long_url
        .fetch_one(&self.pool)
        .await?;
    
        Ok(row.0)
    }


}