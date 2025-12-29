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

# Initialize git submodules
echo "Initializing git submodules..."
git submodule update --init --recursive

# Build the CLI binary
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
