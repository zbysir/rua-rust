// extern crate redis;
extern crate r2d2_redis;

use std::ops::DerefMut;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use r2d2_redis::redis::Commands;

use std::error::Error;

pub struct RedisCli {
    pub pool: r2d2::Pool<RedisConnectionManager>,
}

// 使用连接池的 Cli
impl RedisCli {
    pub fn open(add: &str) -> Result<RedisCli, Box<dyn Error>> {
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = r2d2::Pool::builder()
            .build(manager)
            .unwrap();

        Ok(RedisCli { pool })
    }
    pub async fn get<T: for<'a> serde::Deserialize<'a>>(&self, key: &str) -> Result<T, Box<dyn Error>> {
        let mut conn = self.pool.get().unwrap();
        let x = redis::cmd("GET").arg(key).query::<String>(conn.deref_mut()).unwrap();
        let k: T = serde_json::from_str(&x).unwrap();

        Ok(k)
    }
}
