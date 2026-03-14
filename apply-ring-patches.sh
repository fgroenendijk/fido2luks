#!/bin/bash
set -e

echo "Applying ring compilation patches..."

# Check if we're in the right location
if [ ! -f "patches/ring.patch" ]; then
    echo "Error: patches/ring.patch not found"
    echo "Please run this script from the project root directory"
    exit 1
fi

# Check if ring crate needs to be set up
if [ ! -d "patches/ring/src" ]; then
    echo "Setting up original ring crate..."
    
    SYSTEM_RING="${CARGO_HOME:-$HOME/.cargo}/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.13.5"
    if [ -d "$SYSTEM_RING" ]; then
        echo "Using system ring crate from cargo cache"
        cp -r "$SYSTEM_RING"/* patches/ring/
    else
        echo "Error: Could not find system ring crate at $SYSTEM_RING"
        echo "Please run 'cargo build' first to download dependencies"
        exit 1
    fi
else
    echo "Using existing ring crate in patches/ring"
fi

# Apply the patch using -d flag (no directory change needed)
    echo "Applying patch file..."
patch -p1 -d patches/ring < patches/ring.patch

echo "✅ Ring crate patched successfully!"
