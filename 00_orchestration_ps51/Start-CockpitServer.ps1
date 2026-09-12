Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LAUNCHING COCKPIT SERVER & ADB BRIDGE        " -ForegroundColor Cyan
Write-Host " Serving: http://localhost:8098/hud & /mobile                    " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\03_l2_deno_stategraph"
deno run --allow-all src/server.ts
Pop-Location