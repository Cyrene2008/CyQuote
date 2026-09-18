<script setup>
const platforms = [
  {
    id: 'windows',
    title: 'Windows',
    icon: 'window-20-regular',
    binary: 'cyquote.exe',
    note: '解压后直接运行；附带 PowerShell 计划任务脚本（静默、开机自启）。',
    packages: [
      { label: 'Go 版', detail: '约 2 MB · 无依赖', href: 'https://github.com/Cyrene2008/CyQuote/releases/latest' },
      { label: 'Rust 版', detail: '约 160 KB · 体积更小', href: 'https://github.com/Cyrene2008/CyQuote/releases/latest' }
    ]
  },
  {
    id: 'linux',
    title: 'Linux',
    icon: 'window-20-regular',
    binary: './cyquote',
    note: '解压后运行；附带 systemd 服务脚本与 nohup 兜底方案。',
    packages: [
      { label: 'Go 版', detail: '约 2 MB · 静态编译', href: 'https://github.com/Cyrene2008/CyQuote/releases/latest' },
      { label: 'Rust 版', detail: '约 240 KB · 体积更小', href: 'https://github.com/Cyrene2008/CyQuote/releases/latest' }
    ]
  }
]

const contents = [
  { name: 'cyquote（或 cyquote.exe）', description: '服务端二进制，Go / Rust 二选一' },
  { name: 'config.example.json', description: '配置模板：监听地址、端口、语录文件路径' },
  { name: 'data/quotes.jsonc', description: '示例语录数据（首次运行也会自动生成）' },
  { name: 'scripts/', description: '安装/启动/停止脚本（systemd、计划任务等）' }
]
</script>

<template>
  <main>
    <section class="page-head">
      <span class="hero-kicker">下载</span>
      <h1>选一个压缩包，跑起来</h1>
      <p class="hero-lead">Go 与 Rust 两套等价实现，功能完全一致，按喜好任选其一即可。</p>
    </section>

    <section class="section">
      <div class="download-grid">
        <FluentCard v-for="platform in platforms" :key="platform.id" class="download-card">
          <div class="download-head"><FluentIcon :icon="platform.icon" :width="20" /><h3>{{ platform.title }}</h3></div>
          <p>{{ platform.note }}</p>
          <p class="inline-note">运行命令：<code>{{ platform.binary }}</code></p>
          <div class="download-actions">
            <FluentHyperlinkButton v-for="item in platform.packages" :key="item.label" :href="item.href" target="_blank">{{ item.label }}（{{ item.detail }}）</FluentHyperlinkButton>
          </div>
        </FluentCard>
      </div>
      <p class="section-lead" style="margin-top: 16px">所有版本都在 <a class="inline-link" href="https://github.com/Cyrene2008/CyQuote/releases" target="_blank" rel="noreferrer">GitHub Releases</a> 发布，包含 Windows / Linux 的 Go 与 Rust 共四个压缩包。</p>
    </section>

    <section class="section">
      <h2>压缩包内容</h2>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>文件</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in contents" :key="item.name">
              <td><code>{{ item.name }}</code></td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
    </section>

    <section class="section">
      <FluentCard class="cta-card">
        <div>
          <h2>下载完下一步？</h2>
          <p>看部署指南：三分钟完成运行、配置与反代。</p>
        </div>
        <div class="cta-actions">
          <RouterLink class="hero-primary" to="/deploy">部署指南</RouterLink>
          <RouterLink class="ghost-button" to="/docs">接口文档</RouterLink>
        </div>
      </FluentCard>
    </section>
  </main>
</template>
