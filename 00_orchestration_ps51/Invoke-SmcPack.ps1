param(
    [string]$OutputPath = "C:\sovereign_manifold_v27\03_smc_capsule\manifold_v27.smc"
)
Write-Host "=== Packaging ACT-Ω v27.0 Sovereign Manifold Capsule ===" -ForegroundColor Cyan
& "C:\sovereign_manifold_v27\02_l1_cpp23_cabi\target\test_smc_pack.exe"