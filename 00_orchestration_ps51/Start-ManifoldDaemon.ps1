Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LAUNCHING INFINITE AUTOPOIETIC DAEMON        " -ForegroundColor Cyan
Write-Host " 15.965 Hz Carrier Loop & Win32 Memory Ring Supervisory Node     " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\03_l2_deno_stategraph"
deno run --allow-all src/autopoietic_daemon.ts
Pop-Location