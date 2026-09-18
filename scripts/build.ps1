# 本地构建 Go 与 Rust 的 Windows/Linux 二进制到 bin/。
# 说明：Rust 的 Linux 目标需要本地已配置交叉编译工具链，常规发布由 GitHub Actions 完成。
$ErrorActionPreference = "Stop"
$root = (Resolve-Path "$PSScriptRoot\..").Path
$bin = Join-Path $root "bin"
New-Item -ItemType Directory -Force -Path (Join-Path $bin "windows"), (Join-Path $bin "linux") | Out-Null

Push-Location (Join-Path $root "server-go")
$env:CGO_ENABLED = "0"
if (Get-Command go -ErrorAction SilentlyContinue) {
  $env:GOOS = "windows"; $env:GOARCH = "amd64"
  go build -trimpath -ldflags "-s -w -H=windowsgui" -o (Join-Path $bin "windows/cyquote.exe") .
  $env:GOOS = "linux"
  go build -trimpath -ldflags "-s -w" -o (Join-Path $bin "linux/cyquote") .
  Remove-Item Env:GOOS
  Write-Host "Go 构建完成"
} else {
  Write-Warning "未检测到 Go，跳过 Go 构建"
}
Pop-Location

Push-Location (Join-Path $root "server-rust")
cargo build --release
$rustBin = Join-Path (Get-Location) "target/release"
if (Test-Path (Join-Path $rustBin "cyquote.exe")) { Copy-Item (Join-Path $rustBin "cyquote.exe") (Join-Path $bin "windows/cyquote-rust.exe") -Force }
if (Test-Path (Join-Path $rustBin "cyquote")) { Copy-Item (Join-Path $rustBin "cyquote") (Join-Path $bin "linux/cyquote-rust") -Force }
Pop-Location
Write-Host "Rust 构建完成，输出目录：$bin"
