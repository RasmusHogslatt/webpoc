#!/bin/bash
set -e

echo "Current directory:"
pwd

echo "Contents of current directory:"
ls -la

echo "Building shared.."
echo "Contents of shared directory:"
ls -la
cargo build --release --package shared

echo "Building frontend..."
cd frontend || exit
echo "Contents of frontend directory:"
ls -la

trunk build --release
cd ..

echo "Building backend..."
# Ensure target directory exists
mkdir -p backend/target/release

# Run cargo build from the root of the workspace
cargo build --release --package backend

echo "Final directory structure:"
find . -type d
