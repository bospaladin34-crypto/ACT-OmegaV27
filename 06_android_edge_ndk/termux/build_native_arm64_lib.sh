#!/bin/bash
# ===================================================================================
# Compile libvesper_ffi.so natively on Android 17 (Pixel 10 Tensor G5 / ARM64 NEON)
# ===================================================================================

echo "================================================================="
echo " [ACT-OMEGA V27.0]: COMPILING LIBVESPER_FFI.SO ON ANDROID 17"
echo "================================================================="

# Compile shared library with ARM64 optimizations
clang++ -std=c++23 -O3 -fPIC -shared -march=armv8.2-a+fp16+rcpc+dotprod \
    vesper_arm64.cpp -o libvesper_ffi.so

if [ $? -eq 0 ]; then
    echo "  [SUCCESS]: libvesper_ffi.so compiled successfully!"
    file libvesper_ffi.so
    ls -lh libvesper_ffi.so
else
    echo "  [FALLBACK]: Compiling with standard ARM64 flags..."
    clang++ -std=c++23 -O3 -fPIC -shared vesper_arm64.cpp -o libvesper_ffi.so
    if [ $? -eq 0 ]; then
        echo "  [SUCCESS]: libvesper_ffi.so compiled successfully!"
        ls -lh libvesper_ffi.so
    else
        echo "  [FAIL]: Compilation failed. Ensure clang++ is installed via 'pkg install clang'."
        exit 1
    fi
fi
echo "================================================================="