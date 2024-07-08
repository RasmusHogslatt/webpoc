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
trunk build --release
cd ..

echo "Building backend..."
cd backend || exit
echo "Contents of backend directory:"
ls -la
cargo build --release
cd ..

echo "Build complete. Contents of dist directory:"
ls -la dist