use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "env")]
struct Opt {
    // Use `env` to enable specifying the option with an environment
    // variable. Command line arguments take precedence over env.
    /// URL for the API server
    #[structopt(long, env = "API_URL")]
    api_url: String,

    // The default value is used if neither argument nor environment
    // variable is specified.
    /// Number of retries
    #[structopt(long, env = "RETRIES")]
    retries: Option<u32>,
}

// RETRIES=1 cargo run -- --api-url=123
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
