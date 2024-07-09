#!/bin/bash
set -e

echo "Current directory:"
pwd

echo "Contents of current directory:"
ls -la

# echo "Building shared.."
# echo "Contents of shared directory:"
# ls -la
# cargo build --release --package shared

echo "Building frontend..."
cd frontend || exit
echo "Contents of frontend directory:"
ls -la

trunk build --release
cd ..

echo "Building backend..."
echo "Contents of backend directory:"
cd backend || exit
ls -la
cd ..

# Run cargo build from the root for backend
cargo build --release --package backend

# Show target strucutre
echo "Target directory structure:"
cd target || exit
find . -type d
cd ..

echo "Final directory structure:"
find . -type d
