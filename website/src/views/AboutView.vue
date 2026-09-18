<script setup>
const milestones = [
  { version: 'v0.1.1', items: ['修复 Rust 版 JSONC 解析导致的中文双重编码', '发布产物按 Go / Rust 分开打包', 'PowerShell 脚本改为 UTF-8 BOM，兼容 Windows PowerShell 5.1'] },
  { version: 'v0.1.0', items: ['首个版本：随机语录、分类列表、条数统计、健康检查', 'Go 与 Rust 两套单文件实现', 'Windows / Linux 服务脚本与发布工作流'] }
]
</script>

<template>
  <main>
    <section class="page-head">
      <span class="hero-kicker">关于</span>
      <h1>CyQuote 昔言</h1>
      <p class="hero-lead">内容安全的语录库 —— 为 CyTime 昔时时钟而生的语录服务，也欢迎任何人自部署使用。</p>
    </section>

    <section class="section">
      <h2>项目简介</h2>
      <p class="section-lead">CyQuote 提供「随机语录 / 分类列表 / 条数统计」三个基础能力，Go 与 Rust 两套等价实现均为单文件、低占用。默认只监听本机，由使用者自行通过 Nginx 等反向代理对外服务，数据完全由自己掌控。</p>
      <div class="feature-grid">
        <FluentCard class="feature-card"><span class="feature-icon"><FluentIcon icon="shield-20-regular" :width="20" /></span><h3>内容安全</h3><p>语录逐条筛选，适合课堂与自习场景。</p></FluentCard>
        <FluentCard class="feature-card"><span class="feature-icon"><FluentIcon icon="code-20-regular" :width="20" /></span><h3>开源许可</h3><p>GNU GPL v3.0，源码与构建流程全部公开。</p></FluentCard>
        <FluentCard class="feature-card"><span class="feature-icon"><FluentIcon icon="cloud-20-regular" :width="20" /></span><h3>自部署友好</h3><p>单文件 + 配置文件 + 语录文件，无数据库依赖。</p></FluentCard>
      </div>
    </section>

    <section class="section">
      <h2>更新记录</h2>
      <div class="timeline">
        <div v-for="milestone in milestones" :key="milestone.version" class="timeline-item">
          <span class="timeline-version">{{ milestone.version }}</span>
          <ul>
            <li v-for="item in milestone.items" :key="item">{{ item }}</li>
          </ul>
        </div>
      </div>
    </section>

    <section class="section">
      <FluentCard class="cta-card">
        <div>
          <h2>开源与联系</h2>
          <p>CyQuote 以 GNU GPL v3.0 开源；使用中遇到问题欢迎在仓库反馈。</p>
        </div>
        <div class="cta-actions">
          <a class="hero-primary" href="https://github.com/Cyrene2008/CyQuote" target="_blank" rel="noreferrer">GitHub 仓库</a>
          <a class="ghost-button" href="https://time.cyrene.hk" target="_blank" rel="noreferrer">CyTime 昔时时钟</a>
        </div>
      </FluentCard>
      <p class="section-lead" style="margin-top: 16px">Copyright © 2025-2026 Cyrene2008 · <a class="inline-link" href="https://icp.gov.moe/?keyword=20260093" target="_blank" rel="noreferrer">萌ICP备20260093号</a></p>
    </section>
  </main>
</template>
