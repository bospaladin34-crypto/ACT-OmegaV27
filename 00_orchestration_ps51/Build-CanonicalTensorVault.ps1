<#
===================================================================================
 Build & Test 5-Tier Canonical Tensor Vault Subsystem
 Invariants: Zero-Python | PS5.1 Safe | Zero Square Brackets (.rs)
===================================================================================
#>
$vaultDir = "C:\sovereign_manifold_v27\04_canonical_tensor_vault"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING & TESTING CANONICAL TENSOR VAULT" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $vaultDir
try {
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # 1. Compile Vault Crate
    Write-Host "[STEP 1]: Compiling Canonical Tensor Vault Crate..." -ForegroundColor Cyan
    & cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_FAIL]: cargo build failed with code $LASTEXITCODE" }

    # 2. Run Native Invariant Tests
    Write-Host "`n[STEP 2]: Running Native Vault Tests..." -ForegroundColor Cyan
    & cargo test --release
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: cargo test failed with code $LASTEXITCODE" }

    $ErrorActionPreference = $prevEAP

    # 3. Audit Zero Square Brackets
    Write-Host "`n[STEP 3]: Auditing Zero Square Bracket Invariant..." -ForegroundColor Cyan
    $rsFiles = Get-ChildItem -Path "$vaultDir\src", "$vaultDir\tests" -Filter "*.rs" -Recurse
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
        Write-Host " [PHASE 5 COMPLETE]: 5-Tier Canonical Tensor Vault Verified." -ForegroundColor Green
        Write-Host "   - Algebraic Triplets (beta, lambda_E8, Q): PASSED" -ForegroundColor Green
        Write-Host "   - Zero Square Bracket Invariant (.rs): PASSED" -ForegroundColor Green
        Write-Host "   - Majorana-1 Parity Conservation: PASSED" -ForegroundColor Green
        Write-Host "=================================================================" -ForegroundColor Cyan
    } else {
        throw "[AUDIT_FAILED]: Found $violationCount bracket violations."
    }
}
finally {
    Pop-Location
}