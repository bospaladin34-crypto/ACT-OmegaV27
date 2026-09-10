<#
===================================================================================
 Expand-HydroBusCapacity.ps1: Dynamic Multi-Chunk Hydro-Bus Expansion
 Invariants: Zero-Python | PS5.1 Safe | Compiles C++23 Chunk Allocator & Tests
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$l1Dir = "$rootPath\02_l1_cpp23_cabi"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING & TESTING MULTI-CHUNK ALLOCATOR" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

Push-Location $l1Dir
try {
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # 1. Recompile CABI DLL with multi-chunk endpoints
    Write-Host "[STEP 1]: Compiling C++23 Release CABI DLL with Multi-Chunk Engine..." -ForegroundColor Cyan
    & cl.exe /nologo /std:c++latest /O2 /arch:AVX2 /LD /I include src/shared_memory_ring.cpp src/cabi_exports.cpp /Fe:vesper_cabi.dll
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_FAIL]: Failed to compile vesper_cabi.dll" }

    # 2. Compile & Run Multi-Chunk Native Tests
    Write-Host "`n[STEP 2]: Compiling C++23 Multi-Chunk Test Binary..." -ForegroundColor Cyan
    & cl.exe /nologo /std:c++latest /O2 /arch:AVX2 /EHsc /I include tests/test_chunk_allocator.cpp /Fe:tests/test_chunk_allocator.exe
    if ($LASTEXITCODE -ne 0) { throw "[BUILD_FAIL]: Failed to compile test_chunk_allocator.cpp" }

    Write-Host "`n[STEP 3]: Executing Native Multi-Chunk Invariant Tests..." -ForegroundColor Cyan
    & ".\tests\test_chunk_allocator.exe"
    if ($LASTEXITCODE -ne 0) { throw "[TEST_FAIL]: test_chunk_allocator.exe failed with code $LASTEXITCODE" }

    $ErrorActionPreference = $prevEAP

    Write-Host "`n=================================================================" -ForegroundColor Cyan
    Write-Host " [TASK 10 COMPLETE]: Dynamic Multi-Chunk Allocator Verified." -ForegroundColor Green
    Write-Host "   - 4 KB Virtual Page Alignment: PASSED" -ForegroundColor Green
    Write-Host "   - Inductive Direct Limit Expansion (64 -> 128 slots): PASSED" -ForegroundColor Green
    Write-Host "   - O(1) Zero Relocation Pointer Offsets: PASSED" -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Cyan
}
finally {
    Pop-Location
}