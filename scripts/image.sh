#!/usr/bin/env bash
cargo build --example heightmap
./target/debug/examples/heightmap "../heightmap.png"
if [ $? -ne 0 ]; then
    echo "Error running Rust program."
    exit 1
fi
 
display "./heightmap.png"
cargo clean
