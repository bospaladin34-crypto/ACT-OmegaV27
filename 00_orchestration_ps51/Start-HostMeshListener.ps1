<#
===================================================================================
 Start-HostMeshListener.ps1: Launch Sovereign P2P Host Listener (Port 8098)
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$vizDir = "$rootPath\00_orchestration_ps51\visualizer"
$port = 8098

# Get Local IPv4 Address
$localIp = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notmatch "Loopback|vEthernet|Virtual" -and $_.IPAddress -match "^192\.168\.|^10\." }).IPAddress | Select-Object -First 1
if (-not $localIp) { $localIp = "127.0.0.1" }

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: HOST P2P MESH LISTENER ACTIVE" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "  Workstation Local LAN IP : $localIp" -ForegroundColor Green
Write-Host "  WebSocket DERP Relay Port : $port" -ForegroundColor Green
Write-Host "  Direct QUIC Mesh UDP Port : 41234" -ForegroundColor Green
Write-Host "  Assigned Slot for Pixel 10 : Slot 50" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "`nTo connect your Google Pixel 10 from Termux, run:" -ForegroundColor Yellow
Write-Host "  deno run --allow-net run_pixel10_edge_node.ts $localIp" -ForegroundColor Yellow
Write-Host "=================================================================" -ForegroundColor Cyan