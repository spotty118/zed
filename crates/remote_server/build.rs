use std::process::Command;

const ZED_MANIFEST: &str = include_str!("../zed/Cargo.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zed_cargo_toml: cargo_toml::Manifest =
        toml::from_str(ZED_MANIFEST)
            .map_err(|e| format!("failed to parse zed Cargo.toml: {}", e))?;
    
    let package = zed_cargo_toml.package
        .ok_or("zed Cargo.toml missing package section")?;
    let version = package.version
        .ok_or("zed Cargo.toml missing version")?;
    
    println!("cargo:rustc-env=ZED_PKG_VERSION={}", version);
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET")?
    );

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    if let Some(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");
    }
    
    Ok(())
}
