#!/bin/bash

# Start the Rust API in the background
cargo run &

# Capture the Rust API's process ID
SERVER_PID=$!

# Allow some time for the server to start
sleep 5

# Run the Postman tests
postman collection run api_test.postman_collection.json -n 1000

# Stop the Rust server
kill $SERVER_PID
