use sqlx::{MySqlPool};

use crate::models::url_models::UrlModel;

pub struct UrlRepository {
    pub mysql_pool: MySqlPool,
}

impl UrlRepository {
    //Associate function to create new repository accept mysql pool and redis client as parameters 
    pub fn new(mysql_pool: MySqlPool) -> Self {
        Self {
            mysql_pool
        }

    }

    pub async fn get_all_url(&self) -> Result<Vec<UrlModel>,sqlx::Error> {
       sqlx::query_as::<sqlx::MySql,UrlModel>("SELECT id, short_code, long_url, created_at FROM urls")
        .fetch_all(&self.mysql_pool)
        .await
    }
}