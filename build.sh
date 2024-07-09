#!/bin/bash
set -e

echo "Current directory:"
pwd

echo "Contents of current directory:"
ls -la

echo "Building frontend..."
cd frontend || exit
trunk build --release
cd ..

echo "Building backend..."

# Run cargo build from the root for backend
cargo build --release --package backend

# Show target strucutre
echo "Target directory structure:"
cd target || exit
find . -type d
cd ..

echo "Final directory structure:"
find . -type d
