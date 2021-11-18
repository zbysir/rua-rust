extern crate redis;

use redis::Commands;
use std::error::Error;

pub struct RedisCli {
    pub cli: redis::Client,
}

impl RedisCli {
    pub fn open(add: &str) -> Result<RedisCli, Box<dyn Error>> {
        // Ok(RedisCli{conn: redis::Client::open("redis://127.0.0.1/")?});
        match redis::Client::open("redis://127.0.0.1/") {
            Ok(cli) => {
                Ok(RedisCli { cli })
            }
            Err(e) => Err(Box::from(e))
        }
    }
    pub fn get<T: for<'a> serde::Deserialize<'a>>(&self, key :&str) -> Result<T, Box<dyn Error>>{
        let mut con = self.cli.get_connection()?;

        let x = con.get::<&str, String>(key).unwrap();
        let k: T = serde_json::from_str(&x).unwrap();

        Ok(k)
    }
}
