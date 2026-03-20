use std::sync::Arc;
use crate::models::generic_model::PaginatedResponse;
use crate::repository::analysis_repo::AnalyticsRepository;
use crate::models::analysis_models::UrlClickCount;
use crate::utils::custom_error::CustomError;

pub struct AnalyticsService {
    repo: Arc<AnalyticsRepository>,
}

impl AnalyticsService {
    pub fn new(repo: Arc<AnalyticsRepository>) -> Self {
        Self { repo }
    }

    pub async fn log_visit(&self, url_id: i32, ip: Option<String>, ua: Option<String>) {
        if let Err(e) = self.repo.insert_log(url_id, ip, ua).await {
            eprintln!("Analytics Error: {}", e);
        }
    }

    pub async fn get_click_stats_by_url(&self, short_code: String) -> Result<Option<UrlClickCount>, CustomError> {
        let sqlx_result = self.repo.get_click_stats_by_url(short_code).await?;

        Ok(sqlx_result)
    }

    pub async fn get_all_click_stats(&self) -> Result<Vec<UrlClickCount>, CustomError> {
        let sqlx_result = self.repo.get_all_click_stats().await?;

        Ok(sqlx_result)
    }

    
    pub async fn get_list_pagination(
        &self, 
        search: Option<String>, 
        page: i64, 
        per_page: i64
    ) -> Result<PaginatedResponse<UrlClickCount>, CustomError> {
        let offset = (page - 1) * per_page;
    
        // 1. Gọi Repo để lấy dữ liệu đã lọc và phân trang
        let items_future = self.repo.get_paginated_urls(search.clone(), per_page, offset);
        let total_future = self.repo.count_total_urls(search);
    
        // 2. Chạy song song cả 2 và đợi kết quả của cả hai trả về
        // tokio::join! sẽ trả về một tuple chứa kết quả của từng Future
        let (items_res, total_res) = tokio::join!(items_future, total_future);
    
        // 3. Xử lý lỗi của từng kết quả bằng dấu ?
        let items = items_res?;
        let total = total_res?;
        
        // 3. Tính toán tổng số trang
        let last_page = if total == 0 {
            1
        } else {
            (total as f64 / per_page as f64).ceil() as i64
        };
    
        // 4. Trả về kết quả dưới dạng PaginatedResponse
        let response = PaginatedResponse {
            items,
            total,
            page,
            per_page,
            last_page
        };

        Ok(response)
    }
   
}