import { onMounted, onUnmounted, ref } from 'vue'

export const API_BASE = '/api/v1/quote'

export function useQuote() {
  const quote = ref(null)
  const quoteState = ref('loading')
  const stats = ref(null)
  const activeCategory = ref('')
  let rotationTimer

  async function refreshQuote() {
    quoteState.value = 'loading'
    try {
      const suffix = activeCategory.value ? `&category=${encodeURIComponent(activeCategory.value)}` : ''
      const response = await fetch(`${API_BASE}?format=json${suffix}`, { cache: 'no-store' })
      if (!response.ok) throw new Error(String(response.status))
      quote.value = await response.json()
      quoteState.value = 'ready'
    } catch {
      if (!quote.value) quoteState.value = 'error'
    }
  }

  async function loadStats() {
    try {
      const response = await fetch(`${API_BASE}/count`, { cache: 'no-store' })
      if (!response.ok) throw new Error(String(response.status))
      stats.value = await response.json()
    } catch {
      stats.value = null
    }
  }

  function selectCategory(name) {
    activeCategory.value = activeCategory.value === name ? '' : name
    refreshQuote()
  }

  function clearCategory() {
    activeCategory.value = ''
    refreshQuote()
  }

  onMounted(() => {
    refreshQuote()
    loadStats()
    rotationTimer = window.setInterval(refreshQuote, 15000)
  })

  onUnmounted(() => window.clearInterval(rotationTimer))

  return { quote, quoteState, stats, activeCategory, refreshQuote, loadStats, selectCategory, clearCategory }
}
