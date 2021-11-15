extern crate postgres;

use std::process::exit;
use postgres::{Client, NoTls};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let mut conn = match Client::connect("postgres://postgres:123456@localhost", NoTls) {
        Ok(x) => { x }
        Err(e) => {
            // TODO 输入的标准输出
            println!("连接 DB 错误：{}", e);
            exit(1)
        }
    };

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).err();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {:?}", person);
    }
}