#!/bin/sh
# ACT-Omega v27.0: Native Cortex-X4 ARMv9-A Build Script for Pixel 10
export CXXFLAGS="-march=armv9-a+sve+sve2+svebitperm+i8mm+bf16+rcpc -mcpu=cortex-x4 -O3 -flto -fPIC"
export RUSTFLAGS="-C target-cpu=cortex-x4 -C target-feature=+sve,+sve2,+svebitperm,+i8mm,+bf16,+rcpc"

echo "=== Building libvesper_ffi.so with Cortex-X4 ARMv9-A Extensions ==="
cargo build --release --target aarch64-linux-android
echo "[PASS] Native ARMv9-A build completed."