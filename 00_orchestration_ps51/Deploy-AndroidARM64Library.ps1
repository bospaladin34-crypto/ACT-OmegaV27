<#
===================================================================================
 Deploy-AndroidARM64Library.ps1: Push ARM64 Native Source to Pixel 10 via ADB
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$ndkDir = "$rootPath\06_android_edge_ndk"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: PUSHING ARM64 NATIVE SOURCE TO PIXEL 10" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# Push files to /data/local/tmp/
& adb.exe push "$ndkDir\cpp\vesper_arm64.cpp" /data/local/tmp/
& adb.exe push "$ndkDir\termux\build_native_arm64_lib.sh" /data/local/tmp/
& adb.exe push "$ndkDir\termux\test_native_arm64_ffi.ts" /data/local/tmp/

Write-Host "  [ADB PUSH]: Pushed vesper_arm64.cpp, build script, and test to /data/local/tmp/" -ForegroundColor Green
Write-Host "`n=================================================================" -ForegroundColor Cyan
Write-Host " [INSTRUCTIONS FOR PIXEL 10 (TERMUX)]:" -ForegroundColor Yellow
Write-Host " 1. Open Termux and navigate to your working folder:" -ForegroundColor Yellow
Write-Host "    cd ~ && cp /data/local/tmp/vesper_arm64.cpp /data/local/tmp/build_native_arm64_lib.sh /data/local/tmp/test_native_arm64_ffi.ts ./" -ForegroundColor Green
Write-Host " 2. Compile the shared library:" -ForegroundColor Yellow
Write-Host "    bash build_native_arm64_lib.sh" -ForegroundColor Green
Write-Host " 3. Run the native ARM64 FFI benchmark test:" -ForegroundColor Yellow
Write-Host "    deno run --allow-ffi --allow-read --allow-write test_native_arm64_ffi.ts" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan