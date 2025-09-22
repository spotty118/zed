fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let mut path = std::path::PathBuf::from(&cargo_manifest_dir);

    if path.file_name().as_ref().and_then(|name| name.to_str()) != Some("inspector_ui") {
        return Err(format!(
            "expected CARGO_MANIFEST_DIR to end with crates/inspector_ui, but got {}",
            cargo_manifest_dir
        ).into());
    }
    path.pop();

    if path.file_name().as_ref().and_then(|name| name.to_str()) != Some("crates") {
        return Err(format!(
            "expected CARGO_MANIFEST_DIR to end with crates/inspector_ui, but got {}",
            cargo_manifest_dir
        ).into());
    }
    path.pop();

    println!("cargo:rustc-env=ZED_REPO_DIR={}", path.display());
    Ok(())
}
