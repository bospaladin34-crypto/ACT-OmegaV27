Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: RUNNING TASK 25 THERMODYNAMIC GOVERNOR TEST  " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Push-Location "C:\sovereign_manifold_v27\02_l1_cpp23_cabi\tests"
if (Test-Path ".\test_governor.exe") {
    .\test_governor.exe
} else {
    Write-Host "test_governor.exe not built yet. Compile with MSVC or Clang." -ForegroundColor Yellow
}
Pop-Location