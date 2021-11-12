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

use schema::person;

#[derive(Queryable, Debug)]
pub struct People {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "person"]
pub struct NewPeople {
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
    let new_people = NewPeople {
        name: "13".to_string(),
    };

    diesel::insert_into(schema::person::table)
        .values(&new_people)
        .execute(&db)
        .expect("Error saving new post");

    let chocolate = schema::person::dsl::person.load::<People>(&db).unwrap();

    println!("Found person {:?}", chocolate);
}