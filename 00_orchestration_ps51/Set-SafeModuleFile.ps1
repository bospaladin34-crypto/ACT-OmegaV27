param (
    [Parameter(Mandatory=$true)][string]$TargetPath,
    [Parameter(Mandatory=$true)][AllowEmptyString()][string]$Content
)
$dir = [System.IO.Path]::GetDirectoryName($TargetPath)
if (-not [System.IO.Directory]::Exists($dir)) {
    [System.IO.Directory]::CreateDirectory($dir) | Out-Null
}
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($TargetPath, $Content, $utf8NoBom)
Write-Host "[PS51_SAFE_WRITE]: Written $TargetPath (UTF-8 No-BOM)" -ForegroundColor Cyan