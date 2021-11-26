use hello_world::job_client::JobClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = JobClient::connect("http://localhost:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.trigger_create_rebate(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}