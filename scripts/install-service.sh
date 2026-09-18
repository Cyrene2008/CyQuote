#!/usr/bin/env bash
# 安装 systemd 服务并开机自启（需要 sudo）。
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
WORKDIR="$(dirname "$BIN")"

sudo tee /etc/systemd/system/cyquote.service >/dev/null <<EOF
[Unit]
Description=CyQuote API server
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
WorkingDirectory=$WORKDIR
ExecStart=$BIN
Restart=on-failure
RestartSec=3
User=$(id -un)
Group=$(id -gn)

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now cyquote
echo "CyQuote 服务已安装并启动，可用 systemctl status cyquote 查看状态"
