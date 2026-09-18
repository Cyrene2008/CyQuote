<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <div class="site">
    <header class="nav">
      <div class="nav-inner">
        <RouterLink class="brand" to="/">
          <span class="brand-mark"><FluentIcon icon="book-open-20-regular" :width="20" /></span>
          <span class="brand-copy"><strong>CyQuote 昔言</strong><small>内容安全的语录库</small></span>
        </RouterLink>
        <nav class="nav-links" aria-label="页面导航">
          <RouterLink to="/">首页</RouterLink>
          <RouterLink to="/docs">接口文档</RouterLink>
          <RouterLink to="/download">下载</RouterLink>
          <RouterLink to="/deploy">部署</RouterLink>
          <RouterLink to="/about">关于</RouterLink>
        </nav>
        <div class="nav-actions">
          <FluentHyperlinkButton class="nav-github" href="https://github.com/Cyrene2008/CyQuote" target="_blank"><FluentIcon icon="code-20-regular" :width="16" /><span>GitHub</span></FluentHyperlinkButton>
        </div>
      </div>
    </header>

    <RouterView />

    <footer class="footer">
      <div class="footer-inner">
        <div>CyQuote 昔言 · v0.1.1 · Copyright © 2025-2026 Cyrene2008</div>
        <div class="footer-links">
          <a href="https://github.com/Cyrene2008/CyQuote" target="_blank" rel="noreferrer">CyQuote 仓库</a>
          <a href="https://time.cyrene.hk" target="_blank" rel="noreferrer">CyTime 昔时时钟</a>
          <a href="https://cyrene.hk" target="_blank" rel="noreferrer">作者首页</a>
          <a href="https://icp.gov.moe/?keyword=20260093" target="_blank" rel="noreferrer">萌ICP备20260093号</a>
        </div>
      </div>
    </footer>
  </div>
</template>
