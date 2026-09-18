param([string]$Binary)

$ErrorActionPreference = "Stop"
if (-not $Binary) {
  $candidates = @("$PSScriptRoot\..\cyquote.exe", "$PSScriptRoot\..\bin\windows\cyquote.exe")
  $Binary = ($candidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1)
}
if (-not $Binary) { throw "未找到 cyquote.exe，请使用 -Binary 指定路径" }

$exe = (Resolve-Path -LiteralPath $Binary).Path
Start-Process -FilePath $exe -WorkingDirectory (Split-Path -Parent $exe) -WindowStyle Hidden
Write-Host "CyQuote 已在后台启动（$exe）"
