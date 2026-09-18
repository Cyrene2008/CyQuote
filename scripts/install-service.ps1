param(
  [string]$Binary = "$PSScriptRoot\..\bin\windows\cyquote.exe",
  [string]$TaskName = "CyQuote"
)

$ErrorActionPreference = "Stop"
$exe = (Resolve-Path -LiteralPath $Binary).Path
$workdir = Split-Path -Parent $exe
if (-not (Test-Path -LiteralPath $exe)) { throw "未找到二进制：$Binary" }

$action = New-ScheduledTaskAction -Execute $exe -WorkingDirectory $workdir
$trigger = New-ScheduledTaskTrigger -AtLogOn
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -ExecutionTimeLimit ([TimeSpan]::Zero)

$registered = $false
try {
  $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Highest
  Register-ScheduledTask -TaskName $TaskName -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
  $registered = $true
  Write-Host "已注册计划任务（最高权限）"
} catch {
  $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Limited
  Register-ScheduledTask -TaskName $TaskName -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
  $registered = $true
  Write-Host "已注册计划任务（当前用户权限，未提权）"
}

if ($registered) {
  Start-ScheduledTask -TaskName $TaskName
  Write-Host "CyQuote 已启动，日志见 $workdir\cyquote.log"
}
