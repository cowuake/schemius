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

# Install additional utilities
sudo dnf install -y cloc

# Add useful aliases to shell
echo alias build-schemius-web="wasm-pack build --release --target web /workspaces/schemius/schemius-web/" > .bashrc
