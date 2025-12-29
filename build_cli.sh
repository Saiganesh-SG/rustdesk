#!/bin/bash
# Build script for RustDesk CLI

set -e

echo "Building RustDesk CLI..."
echo "========================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo (Rust) is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check for build dependencies
echo "Checking build dependencies..."

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" = "Darwin" ]; then
    # macOS
    echo "Detected macOS ($ARCH)"
    
    if [ -z "$VCPKG_ROOT" ]; then
        echo ""
        echo "WARNING: VCPKG_ROOT environment variable is not set."
        echo ""
        echo "RustDesk requires the following C/C++ libraries to build:"
        echo "  - opus (for audio codec)"
        echo "  - libyuv (for video processing)"
        echo "  - libvpx (for video codec)"
        echo "  - aom (for AV1 codec)"
        echo ""
        echo "You have two options to install these dependencies:"
        echo ""
        echo "Option 1: Use vcpkg (Recommended)"
        echo "  1. Install vcpkg:"
        echo "     git clone https://github.com/microsoft/vcpkg"
        echo "     cd vcpkg && ./bootstrap-vcpkg.sh"
        echo "  2. Set VCPKG_ROOT environment variable:"
        echo "     export VCPKG_ROOT=\$HOME/vcpkg"
        echo "  3. Install dependencies:"
        echo "     \$VCPKG_ROOT/vcpkg install libvpx libyuv opus aom"
        echo ""
        echo "Option 2: Use Homebrew"
        echo "  1. Install Homebrew from https://brew.sh if not installed"
        echo "  2. Install required packages:"
        echo "     brew install opus libyuv libvpx aom"
        echo ""
        
        # Check if Homebrew packages are available
        if command -v brew &> /dev/null; then
            HOMEBREW_PREFIX="$(brew --prefix)"
            MISSING_PACKAGES=""
            
            for pkg in opus libyuv; do
                if [ ! -d "$HOMEBREW_PREFIX/Cellar/$pkg" ]; then
                    MISSING_PACKAGES="$MISSING_PACKAGES $pkg"
                fi
            done
            
            if [ -n "$MISSING_PACKAGES" ]; then
                echo "❌ Missing Homebrew packages:$MISSING_PACKAGES"
                echo ""
                echo "Install them with: brew install$MISSING_PACKAGES"
                exit 1
            else
                echo "✓ Found Homebrew packages: opus, libyuv"
            fi
        else
            echo "❌ Neither VCPKG_ROOT is set nor Homebrew is installed."
            echo "Please choose one of the options above and try again."
            exit 1
        fi
    else
        echo "✓ VCPKG_ROOT is set to: $VCPKG_ROOT"
        
        # Check if vcpkg packages are installed
        if [ -d "$VCPKG_ROOT/installed" ]; then
            echo "✓ vcpkg packages directory found"
        else
            echo "❌ vcpkg packages not found in $VCPKG_ROOT/installed"
            echo "Please install dependencies with:"
            echo "  \$VCPKG_ROOT/vcpkg install libvpx libyuv opus aom"
            exit 1
        fi
    fi
elif [ "$OS" = "Linux" ]; then
    echo "Detected Linux"
    
    if [ -z "$VCPKG_ROOT" ]; then
        echo ""
        echo "WARNING: VCPKG_ROOT environment variable is not set."
        echo ""
        echo "For Linux, you can either:"
        echo "  1. Set up vcpkg (see README.md for instructions)"
        echo "  2. Use system package manager to install development libraries"
        echo "     (opus-dev, libyuv-dev, libvpx-dev, libaom-dev)"
        echo ""
        echo "The build will attempt to use pkg-config to find system libraries."
        echo ""
    else
        echo "✓ VCPKG_ROOT is set to: $VCPKG_ROOT"
    fi
else
    echo "Detected OS: $OS"
fi

echo ""

# Initialize git submodules
echo "Initializing git submodules..."
git submodule update --init --recursive

# Build the CLI binary
echo ""
echo "Building rustdeskcli binary..."
cargo build --bin rustdeskcli --release

# Check if build was successful
if [ -f "target/release/rustdeskcli" ]; then
    echo ""
    echo "✓ Build successful!"
    echo ""
    echo "The CLI binary is located at: target/release/rustdeskcli"
    echo ""
    echo "Usage example:"
    echo "  ./target/release/rustdeskcli --remoteId <ID> --idServer <SERVER> --key <KEY>"
    echo ""
    echo "For more information, see CLI_README.md"
else
    echo ""
    echo "✗ Build failed!"
    echo "Please check the error messages above."
    exit 1
fi
