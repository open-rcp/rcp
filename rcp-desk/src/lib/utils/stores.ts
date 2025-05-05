/**
 * Store utilities for the application
 * This replaces the $app/stores imports
 */

import { writable, derived } from 'svelte/store';

/**
 * Creates a simple page store with URL information
 */
export const page = (() => {
  // Get the current URL
  const getUrl = () => new URL(window.location.href);
  
  // Create URL store
  const urlStore = writable<URL>(getUrl());
  
  // Update URL on navigation
  if (typeof window !== 'undefined') {
    window.addEventListener('popstate', () => {
      urlStore.set(getUrl());
    });
  }
  
  return derived(urlStore, $url => ({
    url: $url,
    path: $url.pathname,
    params: {}, // Add params logic if needed
  }));
})();