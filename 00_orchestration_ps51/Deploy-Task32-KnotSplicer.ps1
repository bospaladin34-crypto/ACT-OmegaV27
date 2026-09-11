Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 32 KNOT SPLICER TEST SUITE      " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\01_l0_rust_compute"
& rustc --edition 2021 tests\test_knot_splicer.rs -o target\test_knot_standalone.exe
& .\target\test_knot_standalone.exe
Pop-Location