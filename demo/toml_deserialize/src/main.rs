use std::fs::File;
use std::io::Read;
use serde_derive::Deserialize;

#[derive(Debug)]
#[derive(Deserialize)]
struct Opt {
    api_url: String,

    retries: Option<u32>,
}

fn main() {
    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: Opt = toml::from_str(&str_val).unwrap();
    println!("{:#?}", config);
}
