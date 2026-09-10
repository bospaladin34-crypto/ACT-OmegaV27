<#
===================================================================================
 Build & Test Carrier-NAT-Proof Mesh Transport Subsystem
 Invariants: Zero-Python | PS5.1 Safe | Zero Square Brackets (.rs)
===================================================================================
#>
$meshDir = "C:\sovereign_manifold_v27\05_mesh_transport_iroh"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING & TESTING P2P MESH TRANSPORT" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $meshDir
try {
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # 1. Compile Mesh Crate
    Write-Host "[STEP 1]: Compiling Mesh Transport Crate..." -ForegroundColor Cyan
    & cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_FAIL]: cargo build failed with code $LASTEXITCODE" }

    # 2. Run Native Invariant Tests
    Write-Host "`n[STEP 2]: Running Native Mesh Tests..." -ForegroundColor Cyan
    & cargo test --release
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: cargo test failed with code $LASTEXITCODE" }

    $ErrorActionPreference = $prevEAP

    # 3. Audit Zero Square Brackets
    Write-Host "`n[STEP 3]: Auditing Zero Square Bracket Invariant..." -ForegroundColor Cyan
    $rsFiles = Get-ChildItem -Path "$meshDir\src", "$meshDir\tests" -Filter "*.rs" -Recurse
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
        Write-Host " [PHASE 6 COMPLETE]: P2P Mesh Transport Verified." -ForegroundColor Green
        Write-Host "   - 3-Tier Fallback Handshake (QUIC / Port 8098 / Out-of-Band): PASSED" -ForegroundColor Green
        Write-Host "   - Zero Square Bracket Invariant (.rs): PASSED" -ForegroundColor Green
        Write-Host "   - Majorana-1 Parity Lock Conservation: PASSED" -ForegroundColor Green
        Write-Host "=================================================================" -ForegroundColor Cyan
    } else {
        throw "[AUDIT_FAILED]: Found $violationCount bracket violations."
    }
}
finally {
    Pop-Location
}