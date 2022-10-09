import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      $src: resolve('./src'),
      $utils: resolve('./src/utils'),
      $bootstrap: resolve('./node_modules/bootstrap/dist'),
      $bootstrap_icons: resolve('./node_modules/bootstrap-icons/font')
    }
  }
})
