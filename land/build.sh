#!/bin/bash
targets=(
    "aarch64-unknown-linux-gnu"
    "x86_64-unknown-linux-gnu"
    "aarch64-apple-darwin"
    "x86_64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

for target in "${targets[@]}"; do
    echo "Building for $target..."
    cargo zigbuild --release --target "$target" --verbose
done

