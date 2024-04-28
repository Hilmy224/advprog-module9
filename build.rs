fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"], //path to your proto file
            &["proto"],                //directory where the proto file is located
        )?;
    Ok(())
}