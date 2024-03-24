#!/usr/bin/env bash

# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install wasm-pack
cargo install wasm-pack

# Add Rust WASM target
rustup target add wasm32-unknown-unknown

# Install and add Clyppy
cargo install clippy
rustup component add clippy
