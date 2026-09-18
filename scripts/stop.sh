#!/usr/bin/env bash
# 停止 CyQuote：优先使用 systemd，否则结束进程。
set -euo pipefail

if systemctl list-unit-files 2>/dev/null | grep -q '^cyquote.service'; then
  sudo systemctl stop cyquote
  echo "已通过 systemd 停止"
  exit 0
fi

pkill -f '/cyquote( |$)' && echo "CyQuote 已停止" || echo "CyQuote 未在运行"
