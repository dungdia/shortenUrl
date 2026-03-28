use deadpool_redis::{redis::AsyncCommands};

pub struct CacheService {
    pool: deadpool_redis::Pool,
    default_cache_ttl: u64,
}

impl CacheService {

    pub fn new(pool: deadpool_redis::Pool,default_cache_ttl: u64) -> Self {
        Self { pool, default_cache_ttl }
    }

    pub async fn get_key(&self, key: &str) -> Result<Option<String>, deadpool_redis::PoolError> {
        let mut conn = self.pool.get().await?;
        let value: Option<String> = conn.get(key).await?;
        if value.is_some() {
            let _: () = conn.expire(key, self.default_cache_ttl as i64).await?;
        }
        Ok(value)
    }

    pub async fn delete_key(&self, key: &str) -> Result<(), deadpool_redis::PoolError> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(key).await?;
        Ok(())
    }

    pub async fn set_key_string(&self,key: &str,value: &str) -> Result<(), deadpool_redis::PoolError> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set_ex(key, value, self.default_cache_ttl).await?;
        Ok(())
    }
    
}