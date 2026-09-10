param (
    [string]$Message = "[ACT-Ω v27.0] Automated Sync | Epoch $(Get-Date -Format 'yyyyMMdd_HHmmss') | Parity: 1.000000"
)

$ErrorActionPreference = 'Stop'
$root = "C:\sovereign_manifold_v27"
Push-Location $root

Write-Host "-> Checking for modified files..." -ForegroundColor Cyan
$status = & git status --porcelain
if (-not $status) {
    Write-Host "-> No changes detected. Repository clean." -ForegroundColor Green
    Pop-Location
    return
}

Write-Host "-> Changes detected. Staging files..." -ForegroundColor Yellow
& git add .

Write-Host "-> Committing transaction: $Message" -ForegroundColor Yellow
& git commit -m "$Message"

Write-Host "-> Pushing to origin vesper_production..." -ForegroundColor Cyan
& git push origin vesper_production

Write-Host "-> Sync complete!" -ForegroundColor Green
Pop-Location