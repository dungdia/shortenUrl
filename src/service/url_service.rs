use std::sync::Arc;

use nanoid::nanoid;

use crate::repository::url_repo::UrlRepository;
use crate::models::url_models::UrlModel;
use crate::utils::custom_error::CustomError;

use super::cache_service::CacheService;

pub struct UrlService {
    pub repo: Arc<UrlRepository>,
    pub cache_service: Arc<CacheService>
}

impl UrlService {
    pub fn new(repo: Arc<UrlRepository>,cache_service: Arc<CacheService>) -> Self {
        Self { repo, cache_service }
    }

    pub async fn get_all_url(&self) -> Result<Vec<UrlModel>, CustomError> {
        let result = self.repo.get_all_url().await?;

        Ok(result)
    }

    pub async fn get_url_by_code(&self, short_code: &str) -> Result<Option<UrlModel>, CustomError> {
        let result = self.repo.get_url_by_code(short_code).await?;

        Ok(result)
    }
    
    fn generate_short_code(&self) -> String{
        nanoid!(10) //generate new short code with length = 10 using nanoid
    }

    pub async fn create_short_url(&self, long_urls: &str) -> Result<String,CustomError> {
        let mut short_code = self.generate_short_code(); //get new short_code

        //check if code already exist if already generate new one
        while self.repo.get_url_by_code(&short_code).await?.is_some()  {
            short_code = self.generate_short_code();
        }

        self.repo.create_short_url(&short_code, long_urls).await?;

        Ok(short_code)
    }

    pub async fn update_long_url(&self,short_code: &str, long_urls: &str) -> Result<bool,CustomError>{
        if !self.repo.get_url_by_code(short_code).await?.is_some() {
           
            return Err(CustomError::NotFound(Some("Url not found".to_string())));
        };
    
        self.repo.update_long_url(short_code, long_urls).await?;
    
        Ok(true)
    }

    pub async fn delete_url(&self,short_code: &str) -> Result<bool, CustomError>{
        if !self.repo.get_url_by_code(short_code).await?.is_some() {
           
            return Err(CustomError::NotFound(Some("Url not found".to_string())));
        };

        self.repo.delete_url(short_code).await?;

        Ok(true)
    }
}