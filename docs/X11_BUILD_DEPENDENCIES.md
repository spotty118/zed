# X11 Build Dependencies

When building Zed with X11 support on Linux, the following system dependencies are required:

## Required Packages (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    libx11-dev \
    libxcb1-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-dev \
    pkg-config
```

## Required Packages (Fedora/RHEL/CentOS)

```bash
sudo dnf install -y \
    libX11-devel \
    libxcb-devel \
    libxkbcommon-devel \
    libxkbcommon-x11-devel \
    pkgconfig
```

## Required Packages (Arch Linux)

```bash
sudo pacman -S \
    libx11 \
    libxcb \
    libxkbcommon \
    libxkbcommon-x11 \
    pkgconf
```

## Verification

You can verify that the dependencies are properly installed by running:

```bash
pkg-config --exists x11 && echo "X11 found"
pkg-config --exists xcb && echo "XCB found"
pkg-config --exists xkbcommon && echo "xkbcommon found"
```

## Build Command

Once dependencies are installed, build with X11 support:

```bash
cargo build --features "gpui/x11"
```

## Common Error

If you see an error like:
```
The system library `x11` required by crate `x11` was not found.
The file `x11.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
```

This indicates that the X11 development headers are not installed. Install the packages listed above to resolve this issue.