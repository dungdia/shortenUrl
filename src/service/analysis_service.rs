use std::sync::Arc;
use crate::repository::analysis_repo::AnalyticsRepository;

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
}