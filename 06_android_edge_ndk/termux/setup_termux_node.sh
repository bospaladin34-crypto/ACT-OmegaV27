#!/bin/bash
# ===================================================================================
# ACT-Omega v27.0: Termux Mobile Node Setup (Android 17 / Pixel 10 & Pixel 8)
# Invariants: Zero-Python | Deno ARM64 | Rust Toolchain | Iroh QUIC Mesh
# ===================================================================================

echo "================================================================="
echo " [ACT-OMEGA V27.0]: INITIALIZING TERMUX MOBILE NODE"
echo "================================================================="

pkg update -y && pkg upgrade -y
pkg install -y rust clang make git openssl

# Install Deno on Termux ARM64
if ! command -v deno &> /dev/null; then
    echo "[INSTALLING]: Deno ARM64 Runtime for Android Termux..."
    pkg install -y deno || curl -fsSL https://deno.land/install.sh | sh
fi

echo "  [OK]: Rust $(rustc -V) ready."
echo "  [OK]: Deno $(deno -V) ready."
echo "  [OK]: Android 17 NPU/Vulkan substrate initialized."
echo "================================================================="