#![feature(test)]

#[macro_use]
extern crate serde_derive;

extern crate redis;

use redis::Commands;
use redis::AsyncCommands;

use serde_derive::Serialize;

// 序列化
#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Deserialize)]
struct AddressFull {
    street: String,
    city: String,
    #[serde(default)]
    num: String,
}

mod redis_cli;

#[tokio::main]
async fn main() {
    let mut c = conn("redis://127.0.0.1/");

    let a = Address {
        street: "1".to_string(),
        city: "2".to_string(),
    };

    println!("fetch_an_integer: {:?}", fetch_an_integer());
    println!("fetch_with_multiplexed_connect: {:?}", fetch_with_multiplexed_connect().await);

    println!("set_into: {:?}", set_into(&mut c, a));
    let b: AddressFull = get_into().unwrap();
    println!("get_into: {:?}", b);

    println!("c.get_str: {:?}", get_str(&mut c));

    let cli = redis_cli::RedisCliPool::open("redis://127.0.0.1/").unwrap();
    let c: Address = cli.get("my_key").await.unwrap();
    println!("cliPool.get: {:?}", c);
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}

async fn fetch_with_multiplexed_connect() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let mut cmd = redis::Cmd::new();

    let _: () = cmd.arg("SET").arg("my_key").arg(42).query_async(&mut conn).await.unwrap();

    conn.get("my_key").await
}

fn set_into<T: serde::ser::Serialize>(con: &mut redis::Connection, x: T) -> redis::RedisResult<()>
{
    let str = serde_json::to_string(&x).unwrap();
    let _: () = con.set("my_key", str)?;
    Ok(())
}

fn conn(url: &str) -> redis::Connection {
    let client = redis::Client::open(url).unwrap();
    client.get_connection().unwrap()
}

fn get_str(con: &mut redis::Connection) -> String {
    let x = con.get::<&str, String>("my_key").unwrap();
    x
}

// https://serde.rs/lifetimes.html
pub trait DeserializeOwned {}

impl<T> DeserializeOwned for T where T: for<'de> serde::de::Deserialize<'de> {}

fn get_into<T: serde::de::DeserializeOwned>() -> redis::RedisResult<T>
{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let x = con.get::<&str, String>("my_key").unwrap();
    let k: T = serde_json::from_str(&x).unwrap();

    Ok(k)
}

fn get_into2<T: for<'a> serde::Deserialize<'a>>() -> redis::RedisResult<T>
{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let x = con.get::<&str, String>("my_key").unwrap();
    let k: T = serde_json::from_str(&x).unwrap();

    Ok(k)
}
