#!/usr/bin/env bash
# 启动 CyQuote：优先使用 systemd，否则以 nohup 方式后台运行。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${1:-}"
if [ -z "$BIN" ]; then
  for candidate in "$ROOT/cyquote" "$ROOT/bin/linux/cyquote" "$ROOT/cyquote-rust" "$ROOT/bin/linux/cyquote-rust"; do
    if [ -x "$candidate" ]; then BIN="$candidate"; break; fi
  done
fi
if [ -z "$BIN" ]; then
  echo "未找到 cyquote 可执行文件，请作为第一个参数传入路径" >&2
  exit 1
fi
BIN="$(readlink -f "$BIN")"

if systemctl list-unit-files 2>/dev/null | grep -q '^cyquote.service'; then
  sudo systemctl start cyquote
  echo "已通过 systemd 启动"
  exit 0
fi

cd "$(dirname "$BIN")"
nohup "$BIN" >>cyquote.log 2>&1 &
echo "CyQuote 已在后台启动（PID $!）"
