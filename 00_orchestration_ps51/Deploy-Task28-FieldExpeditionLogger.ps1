Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RE-DEPLOYING TASK 28 FIELD EXPEDITION LOGGER " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
& adb.exe reverse tcp:8098 tcp:8098
& adb.exe push "C:\sovereign_manifold_v27\06_android_edge_ndk\termux\run_pixel10_edge_node.ts" /data/local/tmp/
Write-Host "[DEPLOYED]: Re-pushed Field Logger to /data/local/tmp/ on Pixel 10." -ForegroundColor Green