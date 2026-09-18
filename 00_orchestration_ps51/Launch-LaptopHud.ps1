[CmdletBinding()]
param(
    [string]$TargetUrl = "http://127.0.0.1:8098/hud",
    [string]$ProfileDir = "C:\sovereign_manifold_v27\data\hud_profile",
    [Parameter(ValueFromRemainingArguments = $true)]
    $RemainingArgs
)

Write-Host "[LAUNCH] Starting ACT-Omega Detached Workstation HUD..." -ForegroundColor Cyan
$EdgePath   = "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"
$ChromePath = "C:\Program Files\Google\Chrome\Application\chrome.exe"
$AppArgs = @(
    "--app=$TargetUrl",
    "--user-data-dir=$ProfileDir",
    "--window-size=1150,760",
    "--class=ActOmegaHud",
    "--no-first-run",
    "--no-default-browser-check"
)

if (Test-Path $EdgePath) {
    Write-Host "[BROWSER] Spawning Microsoft Edge App-Mode..." -ForegroundColor Green
    Start-Process -FilePath $EdgePath -ArgumentList $AppArgs
} elseif (Test-Path $ChromePath) {
    Write-Host "[BROWSER] Spawning Google Chrome App-Mode..." -ForegroundColor Green
    Start-Process -FilePath $ChromePath -ArgumentList $AppArgs
} else {
    Start-Process $TargetUrl
}
