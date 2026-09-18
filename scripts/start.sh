#!/usr/bin/env bash
# 启动 CyQuote：优先使用 systemd，否则以 nohup 方式后台运行。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${1:-$ROOT/bin/linux/cyquote}"
BIN="$(readlink -f "$BIN")"

if systemctl list-unit-files 2>/dev/null | grep -q '^cyquote.service'; then
  sudo systemctl start cyquote
  echo "已通过 systemd 启动"
  exit 0
fi

cd "$(dirname "$BIN")"
nohup "$BIN" >>cyquote.log 2>&1 &
echo "CyQuote 已在后台启动（PID $!）"
