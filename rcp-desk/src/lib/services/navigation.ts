/**
 * Navigation service for Tauri app that doesn't rely on SvelteKit's $app/navigation
 */

/**
 * Navigate to a URL - replacement for SvelteKit's goto function
 */
export function navigate(url: string): void {
  window.location.href = url;
}

/**
 * Check if code is running in browser
 */
export const isBrowser = typeof window !== 'undefined';