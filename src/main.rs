use axum::{
    Router, http::Method,
};
use dotenvy::dotenv;
use repository::url_repo;
use tower_http::cors::{Any, CorsLayer};
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use service::{cache_service::CacheService, url_service::UrlService};

mod utils;
mod repository;
mod service;
mod routes;
mod models;
mod dtos;
mod constants;

pub struct AppState {
    pub url_service: UrlService,
}

// Cấu hình OpenAPI
#[derive(OpenApi)]
#[openapi(
    info(title = "URL Shortener API", version = "1.0.0"),
    tags((name = "System", description = "Các API hệ thống"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // 1. Load biến môi trường
    dotenv().ok();
    // tracing_subscriber::fmt::init(); // Khởi tạo logging

    // 2. Khởi tạo kết nối DB và Redis (từ module utils)
    let db_context = utils::db::DbContext::init().await;

    let cache_ttl = env::var("REDIS_CACHE_TTL_SECONDS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3600); // Mặc định TTL là 1 giờ nếu không có biến môi trường

    let cache_service = CacheService::new(db_context.redis_pool, cache_ttl); // TTL mặc định 1 giờ
    //khởi tạo repository và service
    //Sử dụng Arc để chia sẻ instance giữa các route handler mà không cần clone toàn bộ dữ liệu bên trong
    let url_repo = Arc::new(url_repo::UrlRepository::new(db_context.mysql_pool));
    let url_service = UrlService::new(url_repo, Arc::new(cache_service));


    // Tạo AppState và chia sẻ nó qua Arc để có thể sử dụng trong các route handler
    let app_state = Arc::new(AppState {
        url_service,
    });

    // Tạo tài liệu OpenAPI bằng cách merge các tài liệu từ các route khác nhau
    let mut openapi = ApiDoc::openapi();
    openapi.merge(routes::url_routes::UrlDoc::openapi());

    let cors = CorsLayer::new()
    // Cho phép tất cả các nguồn (chỉ dùng khi phát triển)
    .allow_origin(Any) 
    // Cho phép các phương thức bạn cần
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    // Cho phép header Content-Type để gửi JSON
    .allow_headers([axum::http::header::CONTENT_TYPE]);

    // 3. Cấu hình Router
    let app = Router::new()
        // Tích hợp giao diện Scalar cho tài liệu OpenAPI tại đường dẫn /scalar
        .merge(Scalar::with_url("/scalar", openapi))
        .merge(routes::url_routes::create_route())
        .layer(cors)
        .with_state(app_state);

    // 4. Chạy Server
    let host = env::var("SERVER_HOST").expect("SERVER_HOST not found");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT not found");
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server đang chạy tại: http://{}", addr);
    println!("📄 Tài liệu API (Scalar): http://{}/scalar", addr);

    axum::serve(listener, app).with_graceful_shutdown(async {
        // Khi bám ctrl+c, sẽ thực hiện shutdown server một cách nhẹ nhàng đợi db đóng lại đầy đủ
        tokio::signal::ctrl_c()
            .await
            .expect("Không thể lắng nghe tín hiệu Ctrl+C");
        println!("\n[INFO] Đang đóng các kết nối và tắt server...");
    }).await.unwrap();
}
