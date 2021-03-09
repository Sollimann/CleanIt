fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .format(true)
        .compile(&["protos/roomba_service.proto"], &["protos"])?;
    Ok(())
}
