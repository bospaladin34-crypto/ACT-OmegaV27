<#
===================================================================================
 Deploy-Pixel10SensorHAL.ps1: Push Sensor HAL Pipeline to Pixel 10 via ADB
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$termuxDir = "$rootPath\06_android_edge_ndk\termux"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: DEPLOYING SENSOR HAL PIPELINE TO PIXEL 10" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# Push files to /data/local/tmp/
& adb.exe push "$termuxDir\setup_termux_sensors.sh" /data/local/tmp/
& adb.exe push "$termuxDir\run_pixel10_edge_node.ts" /data/local/tmp/

Write-Host "  [ADB PUSH]: Pushed setup_termux_sensors.sh & run_pixel10_edge_node.ts to /data/local/tmp/" -ForegroundColor Green
Write-Host "`n=================================================================" -ForegroundColor Cyan
Write-Host " [INSTRUCTIONS FOR PIXEL 10 (TERMUX)]:" -ForegroundColor Yellow
Write-Host " 1. (Optional) Install Termux sensor package:" -ForegroundColor Yellow
Write-Host "    pkg install -y termux-api" -ForegroundColor Green
Write-Host " 2. Run the hardware sensor HAL engine:" -ForegroundColor Yellow
Write-Host "    deno run --allow-net --allow-read --allow-write --allow-run /data/local/tmp/run_pixel10_edge_node.ts 127.0.0.1" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan