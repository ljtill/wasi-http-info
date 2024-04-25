#!/usr/bin/env bash

set -e

# Update local package index
echo "Updating package cache..."
sudo apt-get update

# Install dependency packages
echo "Installing host dependencies..."
sudo apt-get install -y pkg-config

# Install compilation target
echo "Adding compilation target..."
rustup target add wasm32-wasi

# Install dependency crates
echo "Installing crate dependencies..."
cargo install cargo-component
