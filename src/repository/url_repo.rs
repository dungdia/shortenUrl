use sqlx::{MySqlPool};

use crate::models::url_models::UrlModel;

pub struct UrlRepository {
    pub mysql_pool: MySqlPool,
    pub redis_pool: deadpool_redis::Pool,
}

impl UrlRepository {
    //Associate function to create new repository accept mysql pool and redis client as parameters 
    pub fn new(mysql_pool: MySqlPool, redis_pool: deadpool_redis::Pool) -> Self {
        Self {
            mysql_pool,
            redis_pool,
        }

    }

    pub async fn get_short_code_by_url(&self,long_urls: &str) -> Result<Option<UrlModel>, sqlx::Error> {
        sqlx::query_as::<sqlx::MySql,UrlModel>(
            "SELECT id, short_code, long_url, created_at FROM urls WHERE long_url = ?"
        )
        .bind(long_urls)
        .fetch_optional(&self.mysql_pool)
        .await
    }

    pub async fn get_url_by_code(&self, short_code: &str) -> Result<Option<UrlModel>, sqlx::Error> {
        sqlx::query_as::<sqlx::MySql,UrlModel>(
            "SELECT id, short_code, long_url, created_at FROM urls WHERE short_code = ?"
        )
        .bind(short_code)
        .fetch_optional(&self.mysql_pool)
        .await
    }

    pub async fn get_all_url(&self) -> Result<Vec<UrlModel>,sqlx::Error> {
       sqlx::query_as::<sqlx::MySql,UrlModel>("SELECT id, short_code, long_url, created_at FROM urls")
        .fetch_all(&self.mysql_pool)
        .await
    }

    pub async fn create_short_url(&self,short_code: &str ,long_urls: &str) -> Result<bool,sqlx::Error> {
        let result = sqlx::query("INSERT INTO urls (short_code, long_url) VALUES (?, ?)")
        .bind(short_code)
        .bind(long_urls)
        .execute(&self.mysql_pool)
        .await?;

        if result.rows_affected() > 0 {
           return Ok(true);
        }
        Ok(false)
    }

    pub async fn update_long_url(&self,short_urls: &str, new_long_urls: &str) -> Result<bool,sqlx::Error> {
        let result = sqlx::query("UPDATE urls SET long_url = ? WHERE short_code = ?")
        .bind(new_long_urls)
        .bind(short_urls)
        .execute(&self.mysql_pool)
        .await?;

        Ok(result.rows_affected() > 0)
        }

     pub async fn update_short_url(&self,new_short_urls: &str, long_urls: &str) -> Result<bool,sqlx::Error> {
        let result = sqlx::query("UPDATE urls SET short_code = ? WHERE long_url = ?")
        .bind(new_short_urls)
        .bind(long_urls)
        .execute(&self.mysql_pool)
        .await?;

        
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_url(&self, short_urls: &str) -> Result<bool,sqlx::Error>{
        let result = sqlx::query("DELETE FROM urls WHERE short_code = ?")
        .bind(short_urls)
        .execute(&self.mysql_pool)
        .await?;

      Ok(result.rows_affected() > 0)
    }



}