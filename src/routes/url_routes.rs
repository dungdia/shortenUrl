use std::{ env, sync::Arc};

use axum::{ Json, Router, extract::{Path, State}, http::{StatusCode}, response::{IntoResponse, Redirect}, routing::{delete, get, post, put}};
use utoipa::OpenApi;
use crate::{AppState};

#[derive(OpenApi)]
#[openapi(
    paths(get_all_url,redirect_url),
    components(
        schemas()
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


pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
    .route("/api/get_all", get(get_all_url))
    .route("/:short_code", get(redirect_url))
} 