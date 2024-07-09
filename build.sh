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
echo "Contents of backend/target directory:"
ls -la target
echo "Contents of backend/target/release directory:"
ls -la target/release
echo "Size of backend binary:"
du -h target/release/backend
cd ..

echo "Final directory structure:"
find . -type d