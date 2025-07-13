#!/bin/bash
cargo build --release
cp ./target/release/terminal-bible ./bible
echo "✅ Ready: ./bible"
