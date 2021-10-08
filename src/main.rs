use std::io::Error;

#[macro_use]
extern crate serde_derive;
// extern crate serde_json;

fn main() -> Result<(), Error> {
    return Ok(())
}

// 序列化
#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

// 字符串拼接
fn add(a: &mut String) {
    a.push_str("!");
}

fn add2(mut a: String) -> String {
    a.push_str("!");
    a
}

// 写测试
#[cfg(test)]
mod test{
    use std::fs::File;
    use std::io::Error;
    use crate::{add, add2, Person};

    #[test]
    fn append_str(){
        let mut s = "Hello, world".to_string();
        add(&mut s);
        let b = add2(s.clone());
        assert_eq!(s,"Hello, world!");
        assert_eq!(b,"Hello, world!!");
    }

    // 返回 Result 枚举
    #[test]
    fn json() -> Result<(), Error>{
        let f = match File::open("src/sample.json") {
            Ok(f) =>  f,
            Err(e) => {
                return Err(e);
            }
        };
        let v: Person = serde_json::from_reader(f)?;
        println!("{:?}", v);
        return Ok(())
    }
}