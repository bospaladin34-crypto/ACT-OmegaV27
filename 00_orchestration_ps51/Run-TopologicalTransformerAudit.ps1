<#
===================================================================================
 Run-TopologicalTransformerAudit.ps1: Build & Verify Topological Transformer
 Invariants: Zero-Python | PS5.1 Safe | Zero Square Brackets (.rs) | UTF-8 No-BOM
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$l0Dir = "$rootPath\01_l0_rust_compute"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING & TESTING TOPOLOGICAL TRANSFORMER" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $l0Dir
try {
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # 1. Recompile L0 Core with Topological Transformer
    Write-Host "[STEP 1]: Compiling L0 Core with Topological Transformer..." -ForegroundColor Cyan
    & cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_FAIL]: cargo build failed with code $LASTEXITCODE" }

    # 2. Run Native Transformer Tests
    Write-Host "`n[STEP 2]: Executing Native Topological Attention Tests..." -ForegroundColor Cyan
    & cargo test --release --test test_transformer
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: test_transformer failed with code $LASTEXITCODE" }

    $ErrorActionPreference = $prevEAP

    # 3. Audit Zero Square Brackets
    Write-Host "`n[STEP 3]: Auditing Zero Square Bracket Invariant..." -ForegroundColor Cyan
    $rsFiles = Get-ChildItem -Path "$l0Dir\src", "$l0Dir\tests" -Filter "*.rs" -Recurse
    $violationCount = 0
    foreach ($file in $rsFiles) {
        $text = [System.IO.File]::ReadAllText($file.FullName)
        if ($text.Contains("[") -or $text.Contains("]")) {
            Write-Error "  [VIOLATION]: Square bracket in $($file.FullName)"
            $violationCount++
        } else {
            Write-Host "  [CLEAN]: $($file.Name) contains 0 brackets." -ForegroundColor Green
        }
    }

    if ($violationCount -eq 0) {
        Write-Host "`n=================================================================" -ForegroundColor Cyan
        Write-Host " [TASK 9 COMPLETE]: Custom Topological Transformer Verified." -ForegroundColor Green
        Write-Host "   - Non-Commutative Artin Braid Self-Attention: PASSED" -ForegroundColor Green
        Write-Host "   - Reidemeister Cancellation on Identity Loops: PASSED" -ForegroundColor Green
        Write-Host "   - 256D INT8 Conway-Sloane E8 Output Projections: PASSED" -ForegroundColor Green
        Write-Host "   - Zero Square Bracket Invariant across $($rsFiles.Count) files: PASSED" -ForegroundColor Green
        Write-Host "=================================================================" -ForegroundColor Cyan
    } else {
        throw "[AUDIT_FAILED]: Found $violationCount bracket violations."
    }
}
finally {
    Pop-Location
}