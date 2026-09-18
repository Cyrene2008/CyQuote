param(
  [string]$Binary,
  [string]$TaskName = "CyQuote"
)

$ErrorActionPreference = "Stop"
if (-not $Binary) {
  $candidates = @("$PSScriptRoot\..\cyquote.exe", "$PSScriptRoot\..\bin\windows\cyquote.exe")
  $Binary = ($candidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1)
}
if (-not $Binary) { throw "未找到 cyquote.exe，请使用 -Binary 指定路径" }

$exe = (Resolve-Path -LiteralPath $Binary).Path
$workdir = Split-Path -Parent $exe

$action = New-ScheduledTaskAction -Execute $exe -WorkingDirectory $workdir
$trigger = New-ScheduledTaskTrigger -AtLogOn
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -ExecutionTimeLimit ([TimeSpan]::Zero)

try {
  $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Highest
  Register-ScheduledTask -TaskName $TaskName -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
  Write-Host "已注册计划任务（最高权限）"
} catch {
  $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Limited
  Register-ScheduledTask -TaskName $TaskName -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
  Write-Host "已注册计划任务（当前用户权限，未提权）"
}

Start-ScheduledTask -TaskName $TaskName
Write-Host "CyQuote 已启动，日志见 $workdir\cyquote.log"
