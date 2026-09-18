$ErrorActionPreference = "SilentlyContinue"
$processes = Get-Process -Name cyquote
if (-not $processes) { Write-Host "CyQuote 未在运行"; exit 0 }
$processes | Stop-Process -Force
Write-Host "CyQuote 已停止"
