<#
===================================================================================
 Watch-Pixel10LiveStream.ps1: Live Terminal Stream Monitor for Pixel 10 on Slot 50
 Invariants: Zero-Python | PS5.1 Safe | Real-Time 15.965 Hz Stream Monitor
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$computeLog = "$rootPath\09_telemetry_and_specs\ledgers\device_compute_accounting.jsonl"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LIVE PIXEL 10 (SLOT 50) REAL-TIME MONITOR" -ForegroundColor Cyan
Write-Host " [PRESS CTRL+C TO STOP MONITORING]" -ForegroundColor Yellow
Write-Host "=================================================================" -ForegroundColor Cyan

if (-not (Test-Path $computeLog)) {
    New-Item -ItemType File -Force -Path $computeLog | Out-Null
}

Get-Content $computeLog -Wait -Tail 10 | ForEach-Object {
    try {
        $data = $_ | ConvertFrom-Json
        Write-Host " [1:1 MESH LOCK]: Epoch $($data.epoch) | Slot $($data.slot) ($($data.device)) | Parity: $($data.parityTrace) | Phase: $($data.phaseDelta) | Status: $($data.status)" -ForegroundColor Green
    } catch {
        Write-Host $_ -ForegroundColor DarkGray
    }
}