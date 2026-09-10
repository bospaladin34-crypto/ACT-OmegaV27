<#
===================================================================================
 Sync-Pixel10ContinuityVault.ps1: Audit Active Session Anchor & Vault Persistence
===================================================================================
#>
$rootPath = "C:\sovereign_manifold_v27"
$anchorFile = "$rootPath\08_continuity_checkpoints\active_session_anchor.json"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " [ACT-OMEGA V27.0]: LIVE CHAT CONTINUITY & VAULT ANCHOR AUDIT" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

if (Test-Path $anchorFile) {
    Write-Host "`n[CURRENT ACTIVE SESSION ANCHOR]:" -ForegroundColor Yellow
    Get-Content $anchorFile | Write-Host -ForegroundColor Green
} else {
    Write-Warning "[WARN]: active_session_anchor.json not found."
}
Write-Host "=================================================================" -ForegroundColor Cyan