#!/bin/bash

cleanup() {
    echo "Shutting down..."
    pkill -P $$
    exit
}

trap cleanup SIGINT SIGTSTP SIGTERM

echo "Starting the Web Api..."
(cd backend && RUST_LOG=info cargo run) &

echo "Starting the CanBan Board..."
(cd frontend && npm start) &

wait

echo "All processes have been terminated, successfully..."
