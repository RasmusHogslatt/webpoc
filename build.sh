#!/bin/bash
set -e

echo "Current directory:"
pwd

echo "Contents of current directory:"
ls -la

echo "Building frontend..."
cd frontend || exit
echo "Contents of frontend directory:"
ls -la

echo "Renaming cargo.toml to Cargo.toml if necessary"
[ -f cargo.toml ] && mv cargo.toml Cargo.toml

echo "Contents of frontend directory after renaming:"
ls -la

trunk build --release
cd ..

echo "Building backend..."
cd backend || exit
echo "Contents of backend directory:"
ls -la
echo "Contents of backend/src directory:"
ls -la src
echo "Contents of backend Cargo.toml:"
cat Cargo.toml
cargo build --release
cd ..

echo "Build complete. Contents of dist directory:"
ls -la dist