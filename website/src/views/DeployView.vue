<script setup>
const config = [
  { name: 'listen', default: '127.0.0.1', description: '监听地址；默认仅本机，如需直连可改为 0.0.0.0' },
  { name: 'port', default: '9093', description: '监听端口' },
  { name: 'quotesFile', default: 'data/quotes.jsonc', description: '语录文件路径（相对二进制所在目录）' },
  { name: 'allowOrigin', default: '*', description: 'CORS 响应头；可改为具体域名进行收紧' }
]
</script>

<template>
  <main>
    <section class="page-head">
      <span class="hero-kicker">部署指南</span>
      <h1>三分钟，属于你的语录 API</h1>
      <p class="hero-lead">下载 → 运行 → 改配置 → 反代，全过程不依赖数据库与运行时。</p>
    </section>

    <section class="section">
      <h2>第一步 · 运行服务</h2>
      <p class="section-lead">首次启动会自动生成配置、示例语录与运行日志。</p>
      <pre class="code-block"># Windows
cyquote.exe

# Linux
./cyquote

# 启动后验证
curl http://127.0.0.1:9093/health</pre>
    </section>

    <section class="section">
      <h2>第二步 · 修改配置</h2>
      <p class="section-lead">编辑程序同目录生成的 <code>config.json</code>，重启服务生效。</p>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>字段</th><th>默认值</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in config" :key="item.name">
              <td><code>{{ item.name }}</code></td>
              <td><code>{{ item.default }}</code></td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
      <p class="section-lead" style="margin-top: 14px">语录文件为 JSONC（支持注释与尾逗号），顶层键即分类，可直接用 CyTime 的语录文件替换：</p>
      <pre class="code-block">{
  "励志": [
    { "value": "路虽远，行则将至；事虽难，做则必成。" },
    { "value": "不积跬步，无以至千里。", "author": "荀子", "from": "《劝学》" }
  ]
}</pre>
    </section>

    <section class="section">
      <h2>第三步 · 反向代理</h2>
      <p class="section-lead">把域名下的 <code>/api/v1/quote/</code> 指到本机服务即可：</p>
      <pre class="code-block">location = /api/v1/quote {
    proxy_pass http://127.0.0.1:9093/;
}
location /api/v1/quote/ {
    proxy_pass http://127.0.0.1:9093/;
}</pre>
      <p class="section-lead" style="margin-top: 14px">验证：</p>
      <pre class="code-block">curl "https://你的域名/api/v1/quote/?format=json"</pre>
    </section>

    <section class="section">
      <h2>开机自启</h2>
      <div class="deploy-grid">
        <FluentCard class="deploy-card">
          <h3><FluentIcon icon="window-20-regular" :width="18" />Windows</h3>
          <p>压缩包内 <code>scripts/install-service.ps1</code> 会注册登录自启的计划任务（优先最高权限，失败自动降级当前用户）。</p>
          <pre class="small">.\scripts\install-service.ps1</pre>
        </FluentCard>
        <FluentCard class="deploy-card">
          <h3><FluentIcon icon="window-20-regular" :width="18" />Linux · systemd</h3>
          <p>压缩包内 <code>scripts/install-service.sh</code> 会安装并启动 systemd 服务。</p>
          <pre class="small">sudo ./scripts/install-service.sh
systemctl status cyquote</pre>
        </FluentCard>
        <FluentCard class="deploy-card">
          <h3><FluentIcon icon="window-20-regular" :width="18" />Linux · 宝塔 / 无 systemd</h3>
          <p>用计划任务做守护，或面板的 Supervisor 管理器：每分钟检测进程，不在就拉起。</p>
          <pre class="small">pgrep -x cyquote &gt;/dev/null || (cd /你的路径 &amp;&amp; setsid nohup ./cyquote &gt;&gt; cyquote.log 2&gt;&amp;1 &amp;)</pre>
        </FluentCard>
      </div>
      <p class="section-lead" style="margin-top: 16px">也支持 Docker 部署，把二进制挂载进任意 Linux 基础镜像并设置 <code>--restart unless-stopped</code> 即可。</p>
    </section>

    <section class="section">
      <FluentCard class="cta-card">
        <div>
          <h2>接口怎么调？</h2>
          <p>端点、参数、错误码与在线调试都在文档里。</p>
        </div>
        <div class="cta-actions">
          <RouterLink class="hero-primary" to="/docs">接口文档</RouterLink>
        </div>
      </FluentCard>
    </section>
  </main>
</template>
