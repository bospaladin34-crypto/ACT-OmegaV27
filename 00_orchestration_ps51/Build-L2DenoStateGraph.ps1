<#
===================================================================================
 Build & Test L2 Deno TypeScript StateGraph & Chat Continuity Engine
 Invariants: Zero-Python | PS5.1 Safe | Deno Strict Type Checking | Deno.openKv
===================================================================================
#>
$ErrorActionPreference = "Stop"
$l2Dir = "C:\sovereign_manifold_v27\03_l2_deno_stategraph"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: TESTING L2 DENO STATEGRAPH & CONTINUITY" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $l2Dir
try {
    # 1. Type check all TypeScript source files
    Write-Host "[STEP 1]: Running Deno Type Checking (deno check)..." -ForegroundColor Cyan
    & deno check src/ffi_bridge.ts src/continuity_checkpointer.ts src/stategraph.ts src/hydro_bus_drop.ts tests/stategraph_test.ts
    if ($LASTEXITCODE -ne 0) { throw "[TYPE_CHECK_FAIL]: Deno type checking failed with code $LASTEXITCODE" }
    Write-Host "  [PASS]: All TypeScript modules type-checked cleanly." -ForegroundColor Green

    # 2. Execute Native Deno Test Suite
    Write-Host "`n[STEP 2]: Executing Native Deno Tests..." -ForegroundColor Cyan
    & deno test --allow-ffi --allow-read --allow-write --allow-env --unstable-kv tests/stategraph_test.ts
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: Deno tests failed with code $LASTEXITCODE" }

    Write-Host "`n=================================================================" -ForegroundColor Cyan
    Write-Host " [PHASE 4 COMPLETE]: L2 Deno StateGraph & Continuity Verified." -ForegroundColor Green
    Write-Host "   - Deno FFI Bridge to vesper_cabi.dll: PASSED" -ForegroundColor Green
    Write-Host "   - Pregel 15.965 Hz Super-Step Loop: PASSED" -ForegroundColor Green
    Write-Host "   - Transactional Deno.openKv() Checkpointer: PASSED" -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Cyan
}
finally {
    Pop-Location
}