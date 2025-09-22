fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = std::env::var("CARGO_PKG_VERSION")?;
    let out_dir = std::env::var("OUT_DIR")?;

    let mut parts = version.split(|c: char| !c.is_ascii_digit());
    
    let major = parts.next()
        .ok_or("Missing major version")?
        .parse::<u16>()
        .map_err(|e| format!("Failed to parse major version: {}", e))?
        .to_be_bytes();
    
    let minor = parts.next()
        .ok_or("Missing minor version")?
        .parse::<u16>()
        .map_err(|e| format!("Failed to parse minor version: {}", e))?
        .to_be_bytes();
    
    let patch = parts.next()
        .ok_or("Missing patch version")?
        .parse::<u16>()
        .map_err(|e| format!("Failed to parse patch version: {}", e))?
        .to_be_bytes();

    std::fs::write(
        std::path::Path::new(&out_dir).join("version_bytes"),
        [major[0], major[1], minor[0], minor[1], patch[0], patch[1]],
    )?;
    
    Ok(())
}
