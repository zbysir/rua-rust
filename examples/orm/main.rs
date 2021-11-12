#[macro_use]
extern crate diesel;
extern crate postgres;

use std::io::{stderr, Write};
use std::process::exit;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

mod schema;

// or use mod from dir
//
// mod schema_lib;
// use schema_lib::*;

#[derive(Queryable, Debug)]
pub struct People {
    pub id: i32,
    pub name: String,
}

fn main() {
    let db = match PgConnection::establish("postgres://postgres:123456@localhost") {
        Ok(x) => { x }
        Err(e) => {
            // TODO 输入的标准输出
            println!("连接 DB 错误：{}", e);
            exit(1)
        }
    };
    let chocolate = schema::person::dsl::person.limit(5).load::<People>(&db).unwrap();

    println!("Found person {:?}", chocolate);
}