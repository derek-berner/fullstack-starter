#!/bin/bash

# Wait for the backend to be ready
echo "Waiting for backend to be ready..."
until curl -s http://localhost:8000/health > /dev/null; do
    echo "Backend not ready yet, waiting..."
    sleep 1
done

echo "Testing health endpoint..."
curl -v http://localhost:8000/health

echo -e "\nTesting versioned health endpoint..."
curl -v http://localhost:8000/api/v1/health

echo -e "\nAll tests completed!" 