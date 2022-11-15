#!/bin/sh
set -e

echo "Linting common..."
cd common
cargo clippy
cd ..

echo "\nLinting server..."
cd server
cargo clippy
cd ..

echo "\nLinting client..."
cd client
cargo clippy --target=wasm32-unknown-unknown
cd ..
