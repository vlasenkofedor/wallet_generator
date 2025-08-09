#!/bin/bash

cargo build --release

BINARY_NAME=$(grep '^name =' Cargo.toml | head -n 1 | cut -d '"' -f 2)
BINARY_PATH="target/release/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

strip "$BINARY_PATH"
upx --best "$BINARY_PATH"

echo "Build completed: $BINARY_PATH"