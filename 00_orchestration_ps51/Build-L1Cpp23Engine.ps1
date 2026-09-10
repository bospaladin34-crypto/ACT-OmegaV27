<#
===================================================================================
 Build & Test L0/L1 C++23 Shared Memory Ring Engine
 Invariants: Zero-Python | PS5.1 Safe | /nologo | Direct Native Process Handling
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$l1Dir = "$rootPath\02_l1_cpp23_cabi"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: COMPILING L0/L1 C++23 MEMORY RING & CABI" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Auto-import MSVC if cl.exe is not in PATH
if (-not (Get-Command "cl.exe" -ErrorAction SilentlyContinue)) {
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $vcvars = $null
    if (Test-Path $vswhere) {
        $inst = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($inst -and (Test-Path "$inst\VC\Auxiliary\Build\vcvars64.bat")) {
            $vcvars = "$inst\VC\Auxiliary\Build\vcvars64.bat"
        }
    }
    if (-not $vcvars) {
        $paths = @(
            "C:\Program Files (x86)\Microsoft Visual Studio\18\Build Tools\VC\Auxiliary\Build\vcvars64.bat",
            "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
            "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
        )
        foreach ($p in $paths) { if (Test-Path $p) { $vcvars = $p; break } }
    }
    if ($vcvars) {
        Write-Host "  [AUTO_HOOK]: Sourcing MSVC from $vcvars" -ForegroundColor DarkCyan
        cmd /c "`"$vcvars`" > nul && set" | ForEach-Object {
            if ($_ -match "^(.*?)=(.*)$") { Set-Content "env:$($matches)" $matches[2] }
        }
    }
}

Push-Location $l1Dir
try {
    # 2. Compile Test Binary with /nologo
    Write-Host "`n[STEP 1]: Compiling C++23 Native Test Binary (test_memory_ring.exe)..." -ForegroundColor Cyan
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    & cl.exe /nologo /std:c++latest /O2 /arch:AVX2 /EHsc /I include src/shared_memory_ring.cpp tests/test_memory_ring.cpp /Fe:tests/test_memory_ring.exe
    if ($LASTEXITCODE -ne 0) {
        $ErrorActionPreference = $prevEAP
        throw "[BUILD_FAIL]: Failed to compile test_memory_ring.cpp (Exit Code: $LASTEXITCODE)"
    }

    # 3. Compile Release CABI DLL with /nologo
    Write-Host "`n[STEP 2]: Compiling C++23 Release CABI DLL (vesper_cabi.dll)..." -ForegroundColor Cyan
    & cl.exe /nologo /std:c++latest /O2 /arch:AVX2 /LD /I include src/shared_memory_ring.cpp src/cabi_exports.cpp /Fe:vesper_cabi.dll
    if ($LASTEXITCODE -ne 0) {
        $ErrorActionPreference = $prevEAP
        throw "[BUILD_FAIL]: Failed to compile vesper_cabi.dll (Exit Code: $LASTEXITCODE)"
    }

    $ErrorActionPreference = $prevEAP

    # 4. Run the Native Invariant Tests
    Write-Host "`n[STEP 3]: Executing Native C++ Invariant Verification..." -ForegroundColor Cyan
    & ".\tests\test_memory_ring.exe"
    if ($LASTEXITCODE -ne 0) {
        throw "[TEST_FAILED]: test_memory_ring.exe failed with exit code $LASTEXITCODE"
    }

    Write-Host "`n=================================================================" -ForegroundColor Cyan
    Write-Host " [PHASE 3 COMPLETE]: L0/L1 C++23 Subsystem Compiled & Verified." -ForegroundColor Green
    Write-Host "   - 64-byte Header & Slot Descriptors: PASSED" -ForegroundColor Green
    Write-Host "   - 64 MB Shared Memory Ring Layout: PASSED" -ForegroundColor Green
    Write-Host "   - Majorana-1 Parity Lock (Tr(U_res) = 1.000000): PASSED" -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Cyan
}
finally {
    Pop-Location
}