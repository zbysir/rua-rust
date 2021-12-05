use serde_derive::Deserialize;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Default)]
#[derive(Deserialize)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize,Default)]
struct Opt {
    database: Option<Database>,
    debug: Option<bool>,
    micro_registry_address:Option<bool>,
}

fn main() -> Result<(), ConfigError> {
    let mut settings = Config::default();

    // read from file
    settings.merge(File::with_name("config.toml"))?;

    // read from env
    // test it with: DATABASE_URL=44 cargo run
    settings.merge(Environment::new().separator("_"))?;

    //  { database: Database { url: "44" }, debug: Some(false) }
    println!("\n{:?}", settings.try_into::<Opt>().unwrap());

    Ok(())
}
