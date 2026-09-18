# CyQuote 昔言

> 内容安全的语录库 —— 可自行部署的轻量 API 服务。

CyQuote 是 [CyTime 昔时时钟](https://time.cyrene.hk) 的语录服务端，提供 Go 与 Rust 两种等价实现，单个二进制即可运行：

- 首次运行自动生成配置文件与示例语录文件（`data/quotes.jsonc`）；
- 默认监听 `127.0.0.1:9093`，配合 Nginx / Caddy 反向代理即可对外服务；
- 支持按分类随机、分类列表与条数统计；
- 提供 Windows / Linux 的系统服务与守护脚本，静默运行、开机自启。

## 快速开始

1. 从 [Releases](../../releases) 下载对应平台的二进制（Go 或 Rust 任选其一）。
2. 放入任意目录，直接运行：

```bash
./cyquote            # Windows: cyquote.exe
```

首次启动会在同目录生成：

```
config.json         # 监听地址/端口、语录文件路径
data/quotes.jsonc   # 语录数据（示例）
cyquote.log         # 运行日志
```

3. 编辑 `config.json` 与 `data/quotes.jsonc` 后重启服务即可。

## 配置

```json
{
  "listen": "127.0.0.1",
  "port": 9093,
  "quotesFile": "data/quotes.jsonc",
  "allowOrigin": "*"
}
```

| 字段 | 说明 |
| --- | --- |
| `listen` | 监听地址，默认仅本机；如需直连可改为 `0.0.0.0` |
| `port` | 监听端口，默认 `9093` |
| `quotesFile` | 语录文件路径（相对二进制所在目录） |
| `allowOrigin` | CORS `Access-Control-Allow-Origin` 的值 |

## API

配合 Nginx 反代时，可将 `quote.example.com/api/v1/quote/` 内部重写为 `/`：

```nginx
location /api/v1/quote/ {
    proxy_pass http://127.0.0.1:9093/;
}
```

| 端点 | 说明 |
| --- | --- |
| `GET /` 或 `GET /quote` | 随机返回一条语录 |
| `GET /quote?category=崩铁` | 指定单个分类 |
| `GET /quote?category=崩铁,人民日报` | 逗号分隔多个分类 |
| `GET /categories` | 返回全部分类 |
| `GET /count` | 返回总条数与各分类条数 |
| `GET /count?category=崩铁` | 返回单个分类条数 |
| `GET /health` | 健康检查 |

响应示例：

```json
{
  "value": "因为世界对我温柔，我就长成温柔的模样。",
  "author": "德谬歌",
  "from": "HSR",
  "category": ["崩铁"],
  "source": "CyQuote"
}
```

## 服务与守护脚本

脚本位于 `scripts/`：

- Windows（PowerShell）
  - `install-service.ps1`：注册登录自启计划任务并立即启动（优先最高权限，失败自动降级当前用户）
  - `uninstall-service.ps1`：停止并移除计划任务
  - `start.ps1` / `stop.ps1`：手动启动/停止（静默，无控制台窗口）
- Linux（Bash）
  - `install-service.sh`：安装 systemd 服务并开机自启（需要 sudo）
  - `uninstall-service.sh`：停用并移除服务
  - `start.sh` / `stop.sh`：手动启动/停止

## 本地构建

```bash
# Go
cd server-go
go build -trimpath -ldflags "-s -w" -o ../bin/cyquote

# Rust
cd server-rust
cargo build --release
```

跨平台与静默参数由 `.github/workflows/release.yml` 在 CI 中处理（Windows 使用 `-H=windowsgui` / `windows_subsystem`，不弹控制台）。

## 许可

GNU General Public License v3.0，详见 [LICENSE](LICENSE)。

Copyright © 2025-2026 Cyrene2008
