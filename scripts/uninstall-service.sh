#!/usr/bin/env bash
# 停用并移除 systemd 服务（需要 sudo）。
set -euo pipefail

sudo systemctl disable --now cyquote 2>/dev/null || true
sudo rm -f /etc/systemd/system/cyquote.service
sudo systemctl daemon-reload
echo "CyQuote 服务已移除"
