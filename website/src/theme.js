import { computed, ref } from 'vue'

const params = new URLSearchParams(window.location.search)
export const isLight = ref(document.documentElement.classList.contains('light') || params.get('theme') === 'light')
export const theme = computed(() => (isLight.value ? 'light' : 'dark'))

if (isLight.value) document.documentElement.classList.add('light')
else document.documentElement.classList.remove('light')

export function toggleTheme() {
  isLight.value = !isLight.value
  document.documentElement.classList.toggle('light', isLight.value)
}
