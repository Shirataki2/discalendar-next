import vuetify, {transformAssetUrls} from 'vite-plugin-vuetify'
import path from 'path';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: true,
  devtools: {
    enabled: true,

    timeline: {
      enabled: true
    }
  },
  build: {
    transpile: ['vuetify'],
  },
  modules: [
    (_opt, nuxt) => {
      nuxt.hooks.hook('vite:extendConfig', (cfg) => {
        cfg.plugins?.push(
          vuetify({ autoImport: true }),
        )
      })
    },
    '@nuxtjs/eslint-module',
    '@pinia/nuxt',
    '@pinia-plugin-persistedstate/nuxt',
    '@vueuse/nuxt',
    '@nuxt/content',
    '@nuxthq/studio',
  ],
  vite: {
    ssr: {
      noExternal: ['vuetify'],
    },
    vue: {
      template: {
        transformAssetUrls,
      },
    }
  },
  routeRules: {
    '/api/v1/**': {
      proxy: 'http://localhost:5000/**',
    },
  },
  eslint: {
    cache: false,
    overrideConfigFile: path.resolve(__dirname, '.eslintrc.yml'),
  },
  pinia: {
    storesDirs: ['./stores/**']
  },
  studio: {
    enabled: true,
  }
})
