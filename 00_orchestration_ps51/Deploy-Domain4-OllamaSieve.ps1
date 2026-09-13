Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING DOMAIN 4/6 OLLAMA SIEVE TEST         " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\03_l2_deno_stategraph"
deno test --allow-all tests/ollama_sieve_test.ts
Pop-Location