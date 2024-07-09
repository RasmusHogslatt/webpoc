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
echo "Contents of backend directory before build:"
ls -la
echo "Contents of backend/src directory:"
ls -la src
echo "Contents of backend Cargo.toml:"
cat Cargo.toml

# Ensure target directory exists
mkdir -p target/release

cargo build --release

echo "Contents of backend/target directory after build:"
ls -la target
echo "Contents of backend/target/release directory after build:"
ls -la target/release
echo "Size of backend binary:"
du -h target/release/backend
cd ..

echo "Checking for backend binary:"
if [ -f "backend/target/release/backend" ]; then
    echo "Backend binary exists and its details are:"
    ls -l backend/target/release/backend
    file backend/target/release/backend
else
    echo "ERROR: Backend binary does not exist!"
    echo "Contents of backend/target/release:"
    ls -la backend/target/release
    exit 1
fi

echo "Final directory structure:"
find . -type d
