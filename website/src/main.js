import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import VueFluentWidgets from 'vue-fluent-widgets'
import 'vue-fluent-widgets/style.css'
import App from './App.vue'
import HomeView from './views/HomeView.vue'
import DocsView from './views/DocsView.vue'
import DownloadView from './views/DownloadView.vue'
import DeployView from './views/DeployView.vue'
import AboutView from './views/AboutView.vue'
import { theme } from './theme'
import './styles.css'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: HomeView },
    { path: '/docs', component: DocsView },
    { path: '/download', component: DownloadView },
    { path: '/deploy', component: DeployView },
    { path: '/about', component: AboutView },
    { path: '/:pathMatch(.*)*', redirect: '/' }
  ],
  scrollBehavior: () => ({ top: 0 })
})

const app = createApp(App)
app.use(router)
app.use(VueFluentWidgets)
app.provide('theme', theme)
app.mount('#app')
