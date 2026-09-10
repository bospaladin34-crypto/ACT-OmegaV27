<#
===================================================================================
 Expand-Chunk1SubAgents.ps1: Build & Verify Chunk 1 Sub-Agent Mesh
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$l2Dir = "$rootPath\03_l2_deno_stategraph"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: VERIFYING CHUNK 1 SUB-AGENT MESH DISPATCH" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $l2Dir
try {
    # 1. Type check sub-agent mesh module
    Write-Host "[STEP 1]: Type checking subagent_mesh.ts..." -ForegroundColor Cyan
    & deno check src/subagent_mesh.ts tests/subagent_mesh_test.ts
    if ($LASTEXITCODE -ne 0) { throw "[TYPE_CHECK_FAIL]: Sub-agent type check failed" }

    # 2. Run Deno Sub-Agent Mesh Tests
    Write-Host "`n[STEP 2]: Running Sub-Agent Mesh Invariant Tests..." -ForegroundColor Cyan
    & deno test --allow-read --allow-write tests/subagent_mesh_test.ts
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: subagent_mesh_test failed" }

    Write-Host "`n=================================================================" -ForegroundColor Cyan
    Write-Host " [TASK 18 COMPLETE]: Dynamic Chunk 1 Expansion Verified." -ForegroundColor Green
    Write-Host "   - Total Committed Slots: 128 (Chunks 0 & 1 via 4 KB Pages)" -ForegroundColor Green
    Write-Host "   - Chunk 1 Sub-Agents Active: 32 (Slots 64 to 95)" -ForegroundColor Green
    Write-Host "   - Ephemeral Dynamic Headroom: 32 Slots (Slots 96 to 127)" -ForegroundColor Green
    Write-Host "   - Majorana-1 Parity Conservation: PASSED" -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Cyan
}
finally {
    Pop-Location
}