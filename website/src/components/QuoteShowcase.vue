<script setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { useQuote } from '../composables/useQuote'
import { useTypewriter } from '../composables/useTypewriter'

const { quote, quoteState, stats, activeCategory, refreshQuote, selectCategory, clearCategory } = useQuote()
const { displayed, isTyping } = useTypewriter(() => quote.value?.value || '', { typeSpeed: () => 'standard' })
const bodyRef = ref(null)
const innerRef = ref(null)
let heightObserver

function syncHeight() {
  if (bodyRef.value && innerRef.value) bodyRef.value.style.height = `${innerRef.value.offsetHeight}px`
}

onMounted(() => {
  if (typeof ResizeObserver !== 'undefined' && innerRef.value) {
    heightObserver = new ResizeObserver(syncHeight)
    heightObserver.observe(innerRef.value)
  }
  syncHeight()
})

onUnmounted(() => heightObserver?.disconnect())
</script>

<template>
  <div class="quote-showcase">
    <FluentCard class="quote-card" role="button" tabindex="0" @click="refreshQuote" @keydown.enter="refreshQuote">
      <div ref="bodyRef" class="quote-body">
        <div ref="innerRef" class="quote-inner">
          <template v-if="quoteState === 'ready' && quote">
            <p class="quote-text" :class="{ 'is-typing': isTyping }">{{ displayed }}</p>
            <div class="quote-meta">
              <span>{{ [quote.author, quote.from].filter(Boolean).join(' · ') || 'CyQuote' }}</span>
              <span class="quote-category">{{ (quote.category || []).join(' / ') }}</span>
            </div>
          </template>
          <p v-else-if="quoteState === 'error'" class="quote-text muted">暂时无法获取语录，请稍后再试。</p>
          <p v-else class="quote-text muted">正在获取语录…</p>
          <span class="quote-hint">♪ 点击换一句 · 每 15 秒自动刷新</span>
        </div>
      </div>
    </FluentCard>

    <div v-if="stats" class="category-chips">
      <button type="button" class="chip" :class="{ active: !activeCategory }" @click="clearCategory">全部 · {{ stats.total }}</button>
      <button v-for="(count, name) in stats.categories" :key="name" type="button" class="chip" :class="{ active: activeCategory === name }" @click="selectCategory(name)">{{ name }} · {{ count }}</button>
    </div>
  </div>
</template>
