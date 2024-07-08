#!/bin/bash
set -e

# Build the WebAssembly binary
cargo build --release --target wasm32-wasi

# Use wasm-bindgen to generate JavaScript bindings
wasm-bindgen --out-dir ./dist --target web target/wasm32-wasi/release/your_app_name.wasm

# Copy your index.html and any other static assets to the dist folder
# cp index.html dist/
# cp assets/* dist/