Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 34 BRAID ATTENTION TEST SUITE   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\01_l0_rust_compute"
& rustc --edition 2021 tests\test_braid_attention.rs -o target\test_braid_standalone.exe
& .\target\test_braid_standalone.exe
Pop-Location