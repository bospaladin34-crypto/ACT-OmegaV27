Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 30 VIRTUALIZED SENTINEL TEST    " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\03_l2_deno_stategraph"
deno test --allow-all tests/spatial_sentinel_test.ts
Pop-Location