param([string]$TaskName = "CyQuote")

$ErrorActionPreference = "SilentlyContinue"
Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
Get-Process -Name cyquote -ErrorAction SilentlyContinue | Stop-Process -Force
Write-Host "已停止并移除 CyQuote 计划任务"
