#!/bin/bash
   set -e

   echo "Building frontend..."
   cd frontend
   trunk build --release
   cd ..

   echo "Building backend..."
   cd backend
   cargo build --release
   cd ..