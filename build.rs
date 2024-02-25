fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/jobworker.proto", "proto/engine.proto"], &["proto"])?;

    Ok(())
}
