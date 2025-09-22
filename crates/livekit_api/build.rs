fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .type_attribute("SendDataResponse", "#[allow(clippy::empty_docs)]")
        .compile_protos(
            &["vendored/protocol/livekit_room.proto"],
            &["vendored/protocol"],
        )?;
    Ok(())
}
