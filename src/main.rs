use axum::{
    Router, routing::get,
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

mod utils;

// Cấu hình OpenAPI
#[derive(OpenApi)]
#[openapi(
    info(title = "URL Shortener API", version = "1.0.0"),
    tags((name = "System", description = "Các API hệ thống"))
)]
struct ApiDoc;


async fn hello_world() -> &'static str {
    "Hello, World!"
}



#[tokio::main]
async fn main() {
    // 1. Load biến môi trường
    dotenv().ok();

    let db_context = utils::db::DbContext::init().await;

    db_context.check_database_connection().await.expect("Database connection failed");

    let openapi = ApiDoc::openapi();    

    let app: axum::Router = Router::new()
    .merge(Scalar::with_url("/scalar", openapi))
    .route("/", get(hello_world))
    .with_state(());
    
    let host = env::var("SERVER_HOST").expect("SERVER_HOST not found");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT not found");
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server đang chạy tại: http://{}", addr);
    println!("📄 Tài liệu API (Scalar): http://{}/scalar", addr);

    axum::serve(listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).with_graceful_shutdown(async {
        // Khi bám ctrl+c, sẽ thực hiện shutdown server một cách nhẹ nhàng đợi db đóng lại đầy đủ
        tokio::signal::ctrl_c()
            .await
            .expect("Không thể lắng nghe tín hiệu Ctrl+C");
        println!("\n[INFO] Đang đóng các kết nối và tắt server...");
    }).await.unwrap();
}
