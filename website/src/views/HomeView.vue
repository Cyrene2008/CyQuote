<script setup>
import QuoteShowcase from '../components/QuoteShowcase.vue'
import { RouterLink } from 'vue-router'

const features = [
  { icon: 'window-20-regular', title: '单文件运行', text: 'Go / Rust 两套等价实现，编译为单个二进制，内存占用低，随启随用。' },
  { icon: 'shield-20-regular', title: '内容安全', text: '语录逐条筛选，适合课堂与自习场景；支持分类权重与随机抽取。' },
  { icon: 'cloud-20-regular', title: '轻松自部署', text: '默认只监听本机，配合 Nginx 反代即可对外服务；附带系统服务脚本。' },
  { icon: 'code-20-regular', title: '开放接口', text: '随机语录、分类列表、条数统计三个端点，UTF-8 JSON、跨域友好。' }
]
</script>

<template>
  <main>
    <section class="hero">
      <span class="hero-kicker">CyQuote · 昔言</span>
      <h1>内容安全的语录库</h1>
      <p class="hero-lead">由 Cyrene2008 维护，逐条筛选；轻量、可自行部署，为课堂与自习时光提供值得被认真对待的文字。</p>
      <div class="hero-actions">
        <RouterLink class="hero-primary" to="/docs"><FluentIcon icon="code-20-regular" :width="16" /><span>查看接口</span></RouterLink>
        <RouterLink class="ghost-button" to="/download">下载部署</RouterLink>
      </div>
    </section>

    <section class="section">
      <QuoteShowcase />
    </section>

    <section class="section">
      <h2>为什么是 CyQuote</h2>
      <p class="section-lead">一个安静、可靠、可自己掌控的语录服务。</p>
      <div class="feature-grid">
        <FluentCard v-for="feature in features" :key="feature.title" class="feature-card">
          <span class="feature-icon"><FluentIcon :icon="feature.icon" :width="20" /></span>
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.text }}</p>
        </FluentCard>
      </div>
    </section>

    <section class="section">
      <FluentCard class="cta-card">
        <div>
          <h2>三分钟部署属于你的语录 API</h2>
          <p>下载一个压缩包，运行、改配置、挂反代，完成。</p>
        </div>
        <div class="cta-actions">
          <RouterLink class="hero-primary" to="/download">前往下载</RouterLink>
          <RouterLink class="ghost-button" to="/deploy">部署指南</RouterLink>
        </div>
      </FluentCard>
    </section>
  </main>
</template>
