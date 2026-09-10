Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 24 E-J-A COPROCESSOR TEST SUITE " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\01_l0_rust_compute"
cargo test --test test_eja --quiet
Pop-Location
Write-Host "[PASSED]: E-J-A Abductive Coprocessor verified on host." -ForegroundColor Green