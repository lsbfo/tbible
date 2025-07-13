#!/bin/bash
cargo build --release
cp ./target/release/bible ./bible
echo "✅ Ready: ./bible"
