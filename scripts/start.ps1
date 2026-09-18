param([string]$Binary = "$PSScriptRoot\..\bin\windows\cyquote.exe")

$ErrorActionPreference = "Stop"
$exe = (Resolve-Path -LiteralPath $Binary).Path
Start-Process -FilePath $exe -WorkingDirectory (Split-Path -Parent $exe) -WindowStyle Hidden
Write-Host "CyQuote 已在后台启动（$exe）"
