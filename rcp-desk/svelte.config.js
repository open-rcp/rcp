// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { resolve } from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    alias: {
      // $lib is already configured by default, but we explicitly define it for clarity
      '$lib': resolve('./src/lib'),
      
      // Add your custom path aliases here
      '$components': resolve('./src/lib/components'),
      '$stores': resolve('./src/lib/stores'),
      '$services': resolve('./src/lib/services'),
      '$utils': resolve('./src/lib/utils')
    }
  },
};

export default config;
