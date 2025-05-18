// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@pinia/nuxt',
    'pinia-plugin-persistedstate/nuxt',
    '@element-plus/nuxt'
  ],
  // pinia-plugin-persistedstate配置
  piniaPluginPersistedstate: {
    storage: 'localStorage',
    debug: process.env.NODE_ENV !== 'production'
  },
  // element-plus配置
  elementPlus: {
    importStyle: 'scss',
    themes: ['dark'],
  },
  typescript: {
    strict: true
  },
  app: {
    head: {
      title: 'HotSpot',
      meta: [
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ]
    }
  },
  runtimeConfig: {
    public: {
      apiBaseUrl: process.env.NUXT_PUBLIC_API_BASE_URL || 'http://localhost:3000/api/v1',
      wsBaseUrl: process.env.NUXT_PUBLIC_WS_BASE_URL || 'ws://localhost:3000/ws',
      // 高德地图新版安全加载配置
      amapApiKey: process.env.NUXT_PUBLIC_AMAP_API_KEY || '',
      amapServiceSecurityCode: process.env.NUXT_PUBLIC_AMAP_SERVICE_SECURITY_CODE || '',
      amapServiceHost: process.env.NUXT_PUBLIC_AMAP_SERVICE_HOST || '',
    }
  },
  compatibilityDate: '2025-05-06',
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          // 禁用 legacy-js-api 的弃用警告
          silenceDeprecations: ['legacy-js-api'],
        },
      },
    },
  }
})
