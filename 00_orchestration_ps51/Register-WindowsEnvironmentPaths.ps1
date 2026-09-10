<#
===================================================================================
 Register Windows Environment Variables for ACT-Omega v27.0
===================================================================================
#>
Write-Host "[ENVIRONMENT]: Registering Global Manifold Invariants..." -ForegroundColor Cyan

[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_ROOT", "C:\sovereign_manifold_v27\", "User")
[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_SHM_NAME", "Global\ACT_OMEGA_E8_HYPER_MANIFOLD", "User")
[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_CARRIER_HZ", "15.965", "User")
[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_PORT_HUD", "8090", "User")
[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_PORT_DERP", "8098", "User")
[System.Environment]::SetEnvironmentVariable("ACT_OMEGA_INVARIANTS_STRICT", "1", "User")

# Set current session environment
$env:ACT_OMEGA_ROOT = "C:\sovereign_manifold_v27\"
$env:ACT_OMEGA_SHM_NAME = "Global\ACT_OMEGA_E8_HYPER_MANIFOLD"
$env:ACT_OMEGA_CARRIER_HZ = "15.965"
$env:ACT_OMEGA_PORT_HUD = "8090"
$env:ACT_OMEGA_PORT_DERP = "8098"
$env:ACT_OMEGA_INVARIANTS_STRICT = "1"

Write-Host "  [OK]: Windows Environment Variables Registered." -ForegroundColor Green