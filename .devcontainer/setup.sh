#!/usr/bin/env bash

# Install setup deps
sudo dnf in curl @development-tools -y

# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. $HOME/.cargo/env

# Install wasm-pack
cargo install wasm-pack

# Add Rust WASM target
rustup target add wasm32-unknown-unknown
