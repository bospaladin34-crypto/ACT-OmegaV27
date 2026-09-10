Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING REVISION OMEGA.1 TOKENIZER TEST      " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\01_l0_rust_compute"
cargo test --test test_tokenizer_loom
Pop-Location