import { onMounted, onUnmounted, ref } from 'vue'

export const API_BASE = '/api/v1/quote'
const FALLBACK_BASE = 'https://quote.cyrene.hk/api/v1/quote'

async function fetchApi(path) {
  try {
    const response = await fetch(`${API_BASE}${path}`, { cache: 'no-store' })
    if (!response.ok) throw new Error(String(response.status))
    return await response.json()
  } catch (error) {
    const response = await fetch(`${FALLBACK_BASE}${path}`, { cache: 'no-store' })
    if (!response.ok) throw error
    return await response.json()
  }
}

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
      quote.value = await fetchApi(`?format=json${suffix}`)
      quoteState.value = 'ready'
    } catch {
      if (!quote.value) quoteState.value = 'error'
    }
  }

  async function loadStats() {
    try {
      const data = await fetchApi('/count')
      stats.value = data
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
    rotationTimer = window.setInterval(refreshQuote, 180000)
  })

  onUnmounted(() => window.clearInterval(rotationTimer))

  return { quote, quoteState, stats, activeCategory, refreshQuote, loadStats, selectCategory, clearCategory }
}
