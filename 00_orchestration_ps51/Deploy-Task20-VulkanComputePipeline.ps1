<#
===================================================================================
 Deploy-Task20-VulkanComputePipeline.ps1: Push 512-Tensor NEON Pipeline to Pixel 10
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$ndkDir = "$rootPath\06_android_edge_ndk"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: PUSHING 512-TENSOR NEON PIPELINE TO PIXEL 10" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# Push updated C++ and Deno source to /data/local/tmp/
& adb.exe push "$ndkDir\cpp\vesper_arm64.cpp" /data/local/tmp/
& adb.exe push "$ndkDir\termux\run_pixel10_edge_node.ts" /data/local/tmp/

Write-Host "  [ADB PUSH]: Pushed vesper_arm64.cpp & run_pixel10_edge_node.ts to /data/local/tmp/" -ForegroundColor Green
Write-Host "`n=================================================================" -ForegroundColor Cyan
Write-Host " [RECOMPILE & RUN ON PIXEL 10 (VIA SSH OR TERMUX)]:" -ForegroundColor Yellow
Write-Host " In your SSH / Termux session:" -ForegroundColor Yellow
Write-Host "   clang++ -std=c++23 -O3 -fPIC -shared /data/local/tmp/vesper_arm64.cpp -o libvesper_ffi.so" -ForegroundColor Green
Write-Host "   deno run --allow-net --allow-read --allow-write --allow-ffi /data/local/tmp/run_pixel10_edge_node.ts 127.0.0.1" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan