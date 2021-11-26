fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("helloworld.HelloRequest", "#[derive(serde_derive::Deserialize, serde_derive::Serialize)]")
        .type_attribute("helloworld.HelloReply", "#[derive(serde_derive::Deserialize, serde_derive::Serialize)]")
        .compile(&["proto/helloworld.proto"],&["proto"])?;
    Ok(())
}