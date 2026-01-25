use std::sync::Arc;

use crate::repository::url_repo::UrlRepository;
use crate::models::url_models::UrlModel;

pub struct UrlService {
    pub repo: Arc<UrlRepository>,
}

impl UrlService {
    pub fn new(repo: Arc<UrlRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_url(&self) -> Result<Vec<UrlModel>, sqlx::Error> {
        let result = self.repo.get_all_url().await?;

        Ok(result)
    }
    
}