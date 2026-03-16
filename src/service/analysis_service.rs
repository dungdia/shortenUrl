use std::sync::Arc;
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
}