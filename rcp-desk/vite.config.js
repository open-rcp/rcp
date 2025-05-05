import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import Icons from 'unplugin-icons/vite';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    sveltekit(),
    Icons({
      compiler: 'svelte',
      autoInstall: true,
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    proxy: {
      // Proxy API requests to the backend during development
      '/api/v1': {
        target: 'http://localhost:8080', // Default API server address
        changeOrigin: true,
        secure: false,
        // Rewrite request path if needed
        // rewrite: (path) => path.replace(/^\/api\/v1/, '')
      },
    },
  },
}));
