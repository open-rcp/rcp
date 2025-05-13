import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  // Tauri expects a fixed port, specify it below
  server: {
    port: 5173,
    strictPort: true,
  },
  // to make use of `TAURI_PLATFORM`, `TAURI_ARCH`, etc.
  // https://tauri.app/v2/api/config/#buildconfig.beforebuildcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    rollupOptions: {
      // Externalize the Tauri API
      external: ['@tauri-apps/api/tauri', '@tauri-apps/api/event'],
    }
  }
});
