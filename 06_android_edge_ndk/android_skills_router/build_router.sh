#!/bin/sh
export RUSTFLAGS="-C target-cpu=cortex-x4 -C target-feature=+sve,+sve2,+svebitperm,+i8mm,+bf16,+rcpc"
cargo build --release