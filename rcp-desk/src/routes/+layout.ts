// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

// Handle client-side auth in load function
export async function load({ url }: { url: URL }) {
  // Public routes that don't need authentication
  const publicRoutes = ['/login'];
  
  // Return early for public routes
  if (publicRoutes.includes(url.pathname)) {
    return {
      isPublicRoute: true,
    };
  }
  
  // For protected routes, auth check will happen in the +layout.svelte
  // using browser-only APIs
  return {
    isPublicRoute: false,
    currentPath: url.pathname,
  };
}
