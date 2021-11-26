extern crate test;
extern crate redis;
extern crate r2d2_redis;

use std::ops::DerefMut;
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

use std::error::Error;

pub struct RedisCliPool {
    pub pool: r2d2::Pool<RedisConnectionManager>,
}

// 使用连接池的 Cli
impl RedisCliPool {
    pub fn open(add: &str) -> Result<RedisCliPool, Box<dyn Error>> {
        let manager = RedisConnectionManager::new(add).unwrap();
        let pool = r2d2::Pool::builder()
            .build(manager)?;

        Ok(RedisCliPool { pool })
    }
    pub async fn get<T: for<'a> serde::Deserialize<'a>>(&self, key: &str) -> Result<T, Box<dyn Error>> {
        let mut conn = self.pool.get().unwrap();
        let x = r2d2_redis::redis::cmd("GET").arg(key).query::<String>(conn.deref_mut()).unwrap();
        let k: T = serde_json::from_str(&x).unwrap();

        Ok(k)
    }
    pub async fn get_str(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let mut conn = self.pool.get()?;
        let x = r2d2_redis::redis::cmd("GET").arg(key).query(conn.deref_mut()).unwrap();

        Ok(x)
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[bench]
    fn bench_pool(b: &mut test::Bencher) {
        let cli = RedisCliPool::open("redis://localhost").unwrap();

        b.iter(|| {
            let f = cli.get_str("abc");
            println!("{:?}", block_on(f).unwrap())
        });
    }

}