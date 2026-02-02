use std::{ env, sync::Arc};

use axum::{ Json, Router, extract::{Path, State}, http::{StatusCode}, response::{IntoResponse, Redirect}, routing::{delete, get, post, put}};
use utoipa::OpenApi;
use validator::Validate;
use crate::{AppState, dtos::url_dto::{UrlRequest, UrlResponse}};

#[derive(OpenApi)]
#[openapi(
    paths(get_all_url,redirect_url),
    components(
        schemas(UrlRequest, UrlResponse)
    ),
    tags((name = "URL Management", description = "Các API thao tác với mã rút gọn"))
)]
pub struct UrlDoc; //api docs struc for url routes


#[utoipa::path(
    get,
    path = "/api/get_all",
    tag = "URL Management",
    responses((status = 200, description = "Danh sách URL"))
)]
pub async fn get_all_url(State(state): State<Arc<AppState>>) -> impl IntoResponse{
    let url_result =  state.url_service.get_all_url().await;
    match url_result {
        Ok(urls) => axum::Json(urls).into_response(),
        Err(e) => {
            eprintln!("Error fetching URLs: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi khi lấy danh sách URL").into_response()
        }
        
    } 
}

#[utoipa::path(
    get,
    path = "/{short_code}",
    tag = "URL Management",
    params(("short_code" = String, Path, description = "Mã rút gọn 10 ký tự")),
    responses(
        (status = 302, description = "Redirect thành công"),
        (status = 404, description = "Không tìm thấy link")
    )
)]
pub async fn redirect_url(Path(short_code): Path<String>,State(state): State<Arc<AppState>>) -> impl IntoResponse{
    match state.url_service.get_url_by_code(&short_code).await {
        Ok(urls) =>  match urls {
                Some(url_model) => {
                    return Redirect::temporary(&url_model.long_url).into_response();},
                None => StatusCode::NOT_FOUND.into_response()
        },
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("{}",err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
 }
}

#[utoipa::path(
    post,
    path = "/api/create",
    tag = "URL Management",
    request_body = UrlRequest,
    responses(
        (status = 201, description = "Tạo mã rút gọn thành công", body = UrlResponse),
        (status = 400, description = "URL không hợp lệ"),
        (status = 500, description = "Lỗi hệ thống")
    )
)]
pub async fn create_short_url(State(state): State<Arc<AppState>>,
    Json(payload): Json<UrlRequest>) -> impl IntoResponse {

    //get base_url from env
    let base_url = match env::var("BASE_URL") {
    Ok(val) => val,
    Err(_) => {
        eprintln!("Error: BASE_URL not found in .env file");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    };

    //check body validate
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, format!("Validation error: {:?}", err.to_string())).into_response();
    }

    match state.url_service
    .create_short_url(&payload.long_url)
    .await {
        Ok(code) => {
        Json(
            UrlResponse {
            long_url: payload.long_url, 
            short_code: format!("{}/{}",base_url,code), 
            created_at: chrono::Utc::now()
            }
        ).into_response() 
        }
        Err(err) => {
            eprintln!("some thing went wrong when create short_url: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
    .route("/api/get_all", get(get_all_url))
    .route("/:short_code", get(redirect_url))
} 