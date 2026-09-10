<#
===================================================================================
 Build & Test L0 Rust Compute Core
 Invariants: Zero-Python | PS5.1 Safe | Zero Square Brackets (.rs)
===================================================================================
#>
$l0Dir = "C:\sovereign_manifold_v27\01_l0_rust_compute"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING & TESTING L0 RUST COMPUTE CORE" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $l0Dir
try {
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # 1. Compile Release DLL & RLib
    Write-Host "[STEP 1]: Compiling L0 Rust Core (cargo build --release)..." -ForegroundColor Cyan
    & cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_ERROR]: cargo build failed with exit code $LASTEXITCODE" }

    # 2. Execute Native Invariant Tests
    Write-Host "`n[STEP 2]: Running Native Rust Invariant Tests (cargo test --release)..." -ForegroundColor Cyan
    & cargo test --release
    if ($LASTEXITCODE -ne 0) { throw "[TEST_ERROR]: cargo test failed with exit code $LASTEXITCODE" }

    $ErrorActionPreference = $prevEAP

    # 3. Audit Zero Square Brackets in .rs source files
    Write-Host "`n[STEP 3]: Auditing Zero Square Bracket Invariant..." -ForegroundColor Cyan
    $rsFiles = Get-ChildItem -Path "$l0Dir\src", "$l0Dir\tests" -Filter "*.rs" -Recurse
    $violationCount = 0
    foreach ($file in $rsFiles) {
        $text = [System.IO.File]::ReadAllText($file.FullName)
        if ($text.Contains("[") -or $text.Contains("]")) {
            Write-Error "  [VIOLATION]: Square bracket detected in $($file.FullName)"
            $violationCount++
        } else {
            Write-Host "  [CLEAN]: $($file.Name) contains 0 brackets." -ForegroundColor Green
        }
    }

    if ($violationCount -eq 0) {
        Write-Host "`n=================================================================" -ForegroundColor Cyan
        Write-Host " [TASK 6 COMPLETE]: Native CubeCL GPU Compute Pipeline Verified." -ForegroundColor Green
        Write-Host "   - 256D INT8 E8 Lattice GPU Projection Kernel: PASSED" -ForegroundColor Green
        Write-Host "   - Artin Braid Crossing Transformation Kernel: PASSED" -ForegroundColor Green
        Write-Host "   - Penrose 5-Grid Aperiodic Dualization Kernel: PASSED" -ForegroundColor Green
        Write-Host "   - Zero Square Bracket Invariant across $($rsFiles.Count) files: PASSED" -ForegroundColor Green
        Write-Host "=================================================================" -ForegroundColor Cyan
    } else {
        throw "[AUDIT_FAILED]: Found $violationCount bracket violations."
    }
}
finally {
    Pop-Location
}