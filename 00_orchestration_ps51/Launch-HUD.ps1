$hudFile = "C:\sovereign_manifold_v27\00_orchestration_ps51\visualizer\act_omega_unified_hud.html"
$profileDir = "C:\sovereign_manifold_v27\data\hud_profile"

if (-not (Test-Path $profileDir)) { New-Item -ItemType Directory -Path $profileDir -Force | Out-Null }

Start-Process "msedge.exe" -ArgumentList @(
    "--app=file:///$hudFile",
    "--window-size=1920,1080",
    "--user-data-dir=`"$profileDir`"",
    "--no-first-run",
    "--disable-default-apps"
)