// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

// Handle client-side auth in load function
export async function load({ url }: { url: URL }) {
  // Public routes that don't need authentication
  const publicRoutes = ['/login'];
  
  // Check if the current path is in public routes
  const isPublicRoute = publicRoutes.includes(url.pathname);
  
  // Return data with route information
  return {
    isPublicRoute,
    currentPath: url.pathname
  };
}
