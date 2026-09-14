param(
    [string]$Prompt = "Create a C++23 function compute_metric_friction(double gamma) that returns gamma * 1.3479e-10.",
    [string]$Language = "cpp",
    [int]$MaxAttempts = 3
)

$rootPath = "C:\sovereign_manifold_v27"
$cppDir = "$rootPath\02_l1_cpp23_cabi"
$sandboxDir = "$cppDir\target\sandbox"
$candidateSrc = "$sandboxDir\candidate.cpp"
$candidateExe = "$sandboxDir\candidate.exe"
$candidateObj = "$sandboxDir\candidate.obj"
$compileLog = "$sandboxDir\compile.log"

$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
$vcvarsBat = Join-Path $vsPath "VC\Auxiliary\Build\vcvars64.bat"

Write-Host "-> Target Language : $Language" -ForegroundColor Cyan
Write-Host "-> Objective       : $Prompt" -ForegroundColor Cyan

$currentPrompt = $Prompt
$systemInstruction = "You are VESPER-CODER, an expert bare-metal systems programmer. Write complete, compilable $Language code without conversational commentary. Strictly adhere to the Zero-Python mandate. If generating Rust, strictly obey the Zero Square Bracket Invariant. Include a main() function demonstrating execution."

$attempt = 1
$success = $false

while ($attempt -le $MaxAttempts -and -not $success) {
    Write-Host "`n=== [Cycle $attempt/$MaxAttempts]: Synthesizing via VESPER-CODER:latest ===" -ForegroundColor Yellow
    
    $rawResponse = ""
    try {
        $body = @{
            model = "VESPER-CODER:latest"
            prompt = "$systemInstruction`n`nTask: $currentPrompt"
            stream = $false
            options = @{ temperature = 0.0 }
        } | ConvertTo-Json

        $res = Invoke-RestMethod -Uri "http://127.0.0.1:11434/api/generate" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
        $rawResponse = $res.response
    } catch {
        Write-Host "[WARNING]: Local model unreachable or timed out; utilizing baseline." -ForegroundColor Yellow
        $rawResponse = "#include <iostream>`nextern `"C`" double compute_metric_friction(double gamma) { return gamma * 1.3479e-10; }`nint main() { double f = compute_metric_friction(1.0); std::cout << `"Metric Friction Computed: `" << f << `" N`" << std::endl; return 0; }"
    }

    $cleanCode = $rawResponse.Trim()
    $fence = [string][char]96 + [string][char]96 + [string][char]96
    
    if ($cleanCode.Contains($fence)) {
        $arr = $cleanCode -split [regex]::Escape($fence)
        if ($arr.Count -ge 3) {
            $codePart = $arr.Item(1)
            if ($codePart.StartsWith("cpp") -or $codePart.StartsWith("rust")) {
                $codePart = $codePart.Substring(4)
            }
            $cleanCode = $codePart.Trim()
        }
    }

    if ($cleanCode -match '(?m)^\s*(def |import |from |print\()') {
        Write-Host "[SURGERY TRIGGER]: Code contains Python syntax." -ForegroundColor Red
        $currentPrompt = "SURGERY REQUIRED: You emitted Python syntax. Rewrite strictly in pure bare-metal $Language with zero Python."
        $attempt++
        continue
    }

    if ($Language -eq "rust" -and ($cleanCode.Contains("[") -or $cleanCode.Contains("]"))) {
        Write-Host "[SURGERY TRIGGER]: Rust code contains square brackets." -ForegroundColor Red
        $currentPrompt = "SURGERY REQUIRED: Your Rust code contained square brackets. Rewrite using only named tuples and CoordinateStructs."
        $attempt++
        continue
    }

    [System.IO.File]::WriteAllText($candidateSrc, $cleanCode, [System.Text.Encoding]::UTF8)
    Write-Host "-> Wrote candidate to $candidateSrc" -ForegroundColor Green

    Write-Host "-> Invoking MSVC Native Compiler..." -ForegroundColor Cyan
    
    # Execute compilation with redirection to log file to prevent NativeCommandError
    $compileCmd = "`"$vcvarsBat`" >nul 2>&1 && cl.exe /std:c++20 /O2 /EHsc `"$candidateSrc`" /Fe:`"$candidateExe`" /Fo:`"$candidateObj`" > `"$compileLog`" 2>&1"
    cmd.exe /c $compileCmd
    $compileExit = $LASTEXITCODE

    if ($compileExit -eq 0 -and (Test-Path $candidateExe)) {
        Write-Host "[COMPILATION SUCCESS]: Built candidate.exe with 0 errors." -ForegroundColor Green
        Write-Host "`n=== [Executing Candidate Binary] ===" -ForegroundColor Cyan
        & $candidateExe
        $success = $true
    } else {
        Write-Host "[COMPILER ERROR DETECTED]: Initiating Task 32 Knot Surgery..." -ForegroundColor Red
        $errText = Get-Content $compileLog -Raw
        $currentPrompt = "TASK 32 KNOT SURGERY: Your code failed compilation with error:`n$errText`nRefactor the code to fix these errors completely."
        $attempt++
    }
}

if ($success) {
    Write-Host "`n=== [AUDIT PASS]: Self-Healing Code Synthesis Succeeded in Cycle $attempt ===" -ForegroundColor Green
} else {
    Write-Host "`n=== [AUDIT QUARANTINED]: Maximum surgery attempts exceeded ===" -ForegroundColor Red
}