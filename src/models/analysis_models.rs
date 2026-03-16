
#[derive(sqlx::FromRow, serde::Serialize,serde::Deserialize)]
pub struct UrlClickCount {
    pub url_id: i32,
    pub long_url: String,
    pub short_code: String,
    pub click_count: i64,
}