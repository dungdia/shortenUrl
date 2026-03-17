use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UrlRequest {
    #[validate(url(message = "Định dạng URL không hợp lệ"))]
    pub long_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UrlResponse {
    pub short_code: String,
    pub long_url: String,
    
    #[schema(value_type = String, format = "date-time", example = "2026-03-14T13:00:00Z")]
    pub created_at: DateTime<Utc>,
}