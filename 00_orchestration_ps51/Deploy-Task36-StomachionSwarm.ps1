Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 36 STOMACHION SWARM TEST        " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\03_l2_deno_stategraph"
deno test --allow-all tests/subagent_mesh_test.ts
Pop-Location