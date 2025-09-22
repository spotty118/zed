# macOS-Only Build Configuration

This repository has been configured for macOS-only builds. The following scripts are disabled:

## Disabled Scripts

- `bundle-linux` - Linux bundle creation
- `bundle-windows.ps1` - Windows bundle creation  
- `linux` - Linux dependency installation
- `install-linux` - Linux installation script
- Any other Linux/Windows specific scripts

## Supported Scripts

- `bundle-mac` - macOS bundle creation (fully supported)
- All other macOS-compatible scripts

## Note

To restore multi-platform support, you would need to:

1. Re-enable the disabled CI jobs in `.github/workflows/ci.yml`
2. Uncomment Windows dependencies in `Cargo.toml`
3. Re-enable Linux and Windows bundle scripts
4. Update documentation to reflect multi-platform support

For the official multi-platform version, please visit: https://github.com/zed-industries/zed