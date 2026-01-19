use axum::{Router, http::Method};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

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

    let mut openapi = ApiDoc::openapi();

    

    let app: axum::Router = Router::new()
    .merge(Scalar::with_url("/scalar", openapi))
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
