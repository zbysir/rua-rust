#[macro_use]
extern crate serde_derive;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}
// 序列化
#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

// run test by cmd:
//  cargo test json -- --nocapture

// 写测试
#[cfg(test)]
mod test{
    use std::fs::File;
    use std::io::Error;
    use crate::{ Person, Address};

    // 返回 Result 枚举
    #[test]
    fn json() -> Result<(), Error>{
        let f = match File::open("./src/sample.json") {
            Ok(f) =>  f,
            Err(e) => {
                return Err(e);
            }
        };
        let v: Person = serde_json::from_reader(f)?;
        println!("{:?}", v);
        return Ok(())
    }
    #[test]
    fn to_json() -> Result<(), Error>{
        let p =  Address{
            street:"1".to_string(),
            city:"1".to_string(),
        };
        let v = serde_json::to_string(&p)?;
        println!("{:?}", v);
        return Ok(())
    }
}