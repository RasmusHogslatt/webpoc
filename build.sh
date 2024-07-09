#!/bin/bash
set -e

echo "Current directory:"
pwd

echo "Contents of current directory:"
ls -la

echo "Building shared.."
cd shared || exit
echo "Contents of shared directory:"
ls -la
cargo build --release
cd ..

echo "Building frontend..."
cd frontend || exit
echo "Contents of frontend directory:"
ls -la

trunk build --release
cd ..

echo "Building backend..."
echo "Contents of backend directory before build:"
ls -la backend
echo "Contents of backend/src directory:"
ls -la backend/src
echo "Contents of backend Cargo.toml:"
cat backend/Cargo.toml

# Ensure target directory exists
mkdir -p backend/target/release

# Run cargo build from the root of the workspace
cd backend || exit
cargo build --release
cd ..

echo "Contents of backend/target directory after build:"
ls -la backend/target
echo "Contents of backend/target/release directory after build:"
ls -la backend/target/release

# Check for the presence of the binary
echo "Searching for the backend binary..."
BINARY=$(find backend/target/release -maxdepth 1 -type f -executable)
if [ -z "$BINARY" ]; then
    echo "ERROR: Backend binary does not exist!"
    echo "Contents of backend/target/release:"
    ls -la backend/target/release
    exit 1
else
    echo "Backend binary found: $BINARY"
    echo "Details of the backend binary:"
    ls -l $BINARY
    file $BINARY
    du -h $BINARY
fi

echo "Final directory structure:"
find . -type d
