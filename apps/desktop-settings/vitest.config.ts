import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vitest/config'

const packagesRoot = resolve(__dirname, '../../packages')

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@main': resolve(__dirname, 'src/main'),
      '@renderer': resolve(__dirname, 'src/renderer'),
      '@shared': resolve(__dirname, '../../packages/shared-types/src'),
      '@shurufa/shared-types': resolve(__dirname, '../../packages/shared-types/src'),
      '@shurufa/shared-ui': resolve(__dirname, '../../packages/shared-ui/src')
    }
  },
  server: {
    fs: {
      allow: [__dirname, packagesRoot]
    }
  },
  test: {
    environment: 'happy-dom',
    globals: true,
    setupFiles: [resolve(__dirname, 'src/test/setup.ts')],
    include: [
      'src/**/*.test.ts',
      '../../packages/shared-ui/src/**/*.test.ts'
    ]
  }
})
