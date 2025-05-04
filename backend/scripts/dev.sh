#!/bin/bash

# Install sqlx-cli if not already installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli
fi

# Create database if it doesn't exist
echo "Creating database if it doesn't exist..."
sqlx database create || true

# Run migrations
echo "Running migrations..."
sqlx migrate run

# Build the application
echo "Building the application..."
cargo build

echo "Development setup complete!" 