<#
===================================================================================
 Harvest-Pixel10Sensors.ps1: Live Physics Telemetry Stream Monitor for Pixel 10
 Invariants: Zero-Python | PS5.1 Safe | Real Magnetometer dB & b2 Record Monitor
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$telemetryLog = "$rootPath\09_telemetry_and_specs\ledgers\memory_ring_telemetry.jsonl"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LIVE PIXEL 10 PHYSICS & b2 VOID MONITOR" -ForegroundColor Cyan
Write-Host " [PRESS CTRL+C TO STOP MONITORING]" -ForegroundColor Yellow
Write-Host "=================================================================" -ForegroundColor Cyan

Get-Content $telemetryLog -Wait -Tail 10 | ForEach-Object {
    try {
        $data = $_ | ConvertFrom-Json
        if ($data.device -match "Pixel-10") {
            $b2Text = if ($data.isB2Void) { "[b2 VOID DETECTED]" } else { "[STABLE FLUX]" }
            Write-Host " [PHYSICS SYNC]: Epoch $($data.epoch) | dB: $($data.deltaMagUt.ToString('F4')) uT $b2Text | b2: $($data.b2RateRecSec.ToString('F2')) rec/s ($($data.entropyNatsSec.ToString('F2')) nats/s) | Z: $($data.gravityZ.ToString('F4')) m/s2 | P: $($data.pressureHpa.ToString('F2')) hPa" -ForegroundColor Green
        }
    } catch {}
}