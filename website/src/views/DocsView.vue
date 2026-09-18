<script setup>
import { computed, onMounted, ref } from 'vue'
import { API_BASE } from '../composables/useQuote'

const endpoints = [
  { method: 'GET', path: '/', description: '从全部（或指定分类）中随机返回一条语录' },
  { method: 'GET', path: '/?category=崩铁', description: '指定单个分类随机' },
  { method: 'GET', path: '/?category=崩铁,人民日报', description: '逗号分隔多个分类随机' },
  { method: 'GET', path: '/categories', description: '返回全部分类列表' },
  { method: 'GET', path: '/count', description: '返回总条数与各分类条数' },
  { method: 'GET', path: '/count?category=崩铁', description: '返回单个分类条数（多分类会返回 400）' },
  { method: 'GET', path: '/health', description: '健康检查，用于探活' }
]

const params = [
  { name: 'format', required: '否', description: 'API 始终返回 JSON，保留该参数用于兼容' },
  { name: 'category', required: '否', description: '分类名，多个用英文逗号分隔；不存在的分类会被忽略' }
]

const fields = [
  { name: 'value', description: '语录正文' },
  { name: 'author', description: '作者，可能为空字符串' },
  { name: 'from', description: '出处 / 作品，可能为空字符串' },
  { name: 'category', description: '本次抽取涉及的分类数组' },
  { name: 'source', description: '固定为 "CyQuote"' }
]

const errors = [
  { code: '400', description: '参数错误，例如 /count 指定了多个分类' },
  { code: '404', description: '分类不存在，或没有可用语录' },
  { code: '405', description: '使用了非 GET 方法' }
]

const trial = ref({ path: '', status: '', body: '' })
const counts = ref({})
const trialCategory = ref('')
const categoryOptions = computed(() => [
  { value: '', label: '不选择（全部分类）' },
  ...Object.entries(counts.value).map(([name, count]) => ({ value: name, label: `${name}（${count} 条）` }))
])
const trialPath = computed(() => {
  const suffix = trialCategory.value ? `&category=${encodeURIComponent(trialCategory.value)}` : ''
  return `/?format=json${suffix}`
})

async function run(path) {
  trial.value = { path, status: '请求中…', body: '' }
  try {
    const response = await fetch(`${API_BASE}${path}`, { cache: 'no-store' })
    const text = await response.text()
    let pretty = text
    try { pretty = JSON.stringify(JSON.parse(text), null, 2) } catch {}
    trial.value = { path, status: `${response.status} ${response.statusText}`, body: pretty }
  } catch (error) {
    trial.value = { path, status: '请求失败', body: error.message }
  }
}

async function loadCounts() {
  try {
    const response = await fetch(`${API_BASE}/count`, { cache: 'no-store' })
    if (!response.ok) throw new Error(String(response.status))
    const data = await response.json()
    counts.value = data.categories || {}
  } catch {
    counts.value = {}
  }
}

onMounted(loadCounts)
</script>

<template>
  <main>
    <section class="page-head">
      <span class="hero-kicker">接口文档</span>
      <h1>简单、直白、够用</h1>
      <p class="hero-lead">所有接口均为 GET，返回 UTF-8 JSON，附带 <code>Access-Control-Allow-Origin</code>。</p>
      <div class="base-url"><span>线上地址</span><code>https://quote.cyrene.hk/api/v1/quote</code></div>
    </section>

    <section class="section">
      <h2>端点一览</h2>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>方法</th><th>路径</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in endpoints" :key="item.path">
              <td><span class="method">{{ item.method }}</span></td>
              <td><code>{{ item.path }}</code></td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
    </section>

    <section class="section">
      <h2>在线调试</h2>
      <p class="section-lead">下面会直接请求当前站点的 API，返回结果实时展示。</p>
      <FluentCard class="trial-card">
        <div class="trial-params">
          <div class="param-info"><code>category</code><span>分类过滤；不选则从全部分类随机，多分类在 URL 里用逗号分隔。</span></div>
          <FluentSelect v-model="trialCategory" :options="categoryOptions" aria-label="选择分类" />
        </div>
        <div class="trial-actions">
          <button type="button" class="chip chip-primary" @click="run(trialPath)">发送请求</button>
          <button type="button" class="chip" @click="run('/categories')">分类列表</button>
          <button type="button" class="chip" @click="run('/count')">全部条数</button>
          <button type="button" class="chip" @click="run('/health')">健康检查</button>
        </div>
        <div v-if="trial.path" class="trial-result">
          <div class="trial-meta"><code>{{ trial.path }}</code><span>{{ trial.status }}</span></div>
          <pre>{{ trial.body }}</pre>
        </div>
      </FluentCard>
    </section>

    <section class="section">
      <h2>请求参数</h2>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>参数</th><th>必填</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in params" :key="item.name">
              <td><code>{{ item.name }}</code></td>
              <td>{{ item.required }}</td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
    </section>

    <section class="section">
      <h2>响应字段</h2>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>字段</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in fields" :key="item.name">
              <td><code>{{ item.name }}</code></td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
      <pre class="code-block">{
  "value": "因为世界对我温柔，我就长成温柔的模样。",
  "author": "德谬歌",
  "from": "HSR",
  "category": ["崩铁"],
  "source": "CyQuote"
}</pre>
    </section>

    <section class="section">
      <h2>错误码</h2>
      <FluentCard class="table-card">
        <table class="doc-table">
          <thead><tr><th>状态码</th><th>说明</th></tr></thead>
          <tbody>
            <tr v-for="item in errors" :key="item.code">
              <td><span class="method">{{ item.code }}</span></td>
              <td>{{ item.description }}</td>
            </tr>
          </tbody>
        </table>
      </FluentCard>
    </section>

    <section class="section">
      <h2>调用示例</h2>
      <pre class="code-block">curl "https://quote.cyrene.hk/api/v1/quote?format=json"
curl "https://quote.cyrene.hk/api/v1/quote?format=json&category=崩铁"
curl "https://quote.cyrene.hk/api/v1/quote/count"</pre>
      <p class="section-lead" style="margin-top: 18px">Nginx 反代示例（把线上路径映射到本机服务）：</p>
      <pre class="code-block">location /api/v1/quote/ {
    proxy_pass http://127.0.0.1:9093/;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}</pre>
      <p class="section-lead" style="margin-top: 18px">前端跨域调用示例：</p>
      <pre class="code-block">const response = await fetch('https://quote.cyrene.hk/api/v1/quote?format=json')
const quote = await response.json()
console.log(quote.value)</pre>
    </section>
  </main>
</template>
