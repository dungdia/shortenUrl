use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError{
    #[error("Không tìm thấy dữ liệu")]
    NotFound(Option<String>),

    // Lỗi từ MySQL (sqlx)
    #[error("Lỗi cơ sở dữ liệu: {0}")]
    DatabaseError(sqlx::Error),

    // Lỗi khi không lấy được kết nối từ Pool (Deadpool)
    #[error("Lỗi kết nối Redis Pool: {0}")]
    RedisPoolError(#[from] deadpool_redis::PoolError),

    // Lỗi khi thực thi lệnh Redis
    #[error("Lỗi thực thi Redis: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Lỗi định dạng dữ liệu: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Lỗi nội bộ: {0}")]
    Internal(String),
}

impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => CustomError::NotFound(None),
            _ => CustomError::DatabaseError(err),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        match self {
            CustomError::NotFound(msg) => (StatusCode::NOT_FOUND, msg
                .unwrap_or_else(|| -> String { "Không tìm thấy dữ liệu".to_string() })
            ).into_response(),
            CustomError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            _ => {
                eprintln!("Internal error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi hệ thống").into_response()
            }
        }
    }
}