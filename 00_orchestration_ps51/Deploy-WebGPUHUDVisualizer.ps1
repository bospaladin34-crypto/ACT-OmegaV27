<#
===================================================================================
 Launch ACT-Omega v27.0 WebGPU Visualizer & Telemetry HUD (Port 8090)
 Invariants: Zero-Python | PS5.1 Safe | Deno Native Serve
===================================================================================
#>
$vizDir = "C:\sovereign_manifold_v27\00_orchestration_ps51\visualizer"
$port = 8090

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LAUNCHING HUD VISUALIZER (PORT $port)" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Start Background Deno Server
$serverProc = Start-Process -FilePath "deno" -ArgumentList "run --allow-net --allow-read `"$vizDir\server.ts`"" -PassThru -WindowStyle Hidden
Write-Host "  [RUNNING]: Local HUD Server listening at http://localhost:$port (PID: $($serverProc.Id))" -ForegroundColor Green

# 2. Launch Default Web Browser to Visualizer URL
Start-Sleep -Milliseconds 800
Start-Process "http://localhost:$port"
Write-Host "  [OPENED]: Browser connected to 3D Topology Canvas." -ForegroundColor Green

Write-Host "`n[PHASE 9 COMPLETE]: WebGPU Visualizer & Telemetry HUD Live." -ForegroundColor Green