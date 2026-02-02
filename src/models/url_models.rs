use chrono::{DateTime, Utc};


#[derive(sqlx::FromRow, serde::Serialize,serde::Deserialize)]
pub struct UrlModel {
    pub id: i32,
    pub long_url: String,
    pub short_code: String,
    pub created_at: DateTime<Utc>,
}