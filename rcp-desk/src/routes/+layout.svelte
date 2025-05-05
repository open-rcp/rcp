<script lang="ts">
  import '../app.css';
  import DashboardLayout from '$components/DashboardLayout.svelte';
  import { onMount } from 'svelte';
  import { authGuard } from '$lib/guards/auth.guard';
  import { authService } from '$services/auth.service';
  import { browser } from '$lib/utils/environment';
  
  // Get properties from the layout data
  export let data: {
    isPublicRoute?: boolean;
    currentPath?: string;
  };
  
  // Authentication state
  let isLoading = true;
  let authenticated = false;

  // Handle authentication on mount
  onMount(async () => {
    if (browser) {
      console.log('Layout mounted, path:', data.currentPath, 'isPublic:', data.isPublicRoute);
      
      // Don't check authentication for public routes
      if (data.isPublicRoute) {
        isLoading = false;
        return;
      }

      // For protected routes, check authentication
      try {
        // First check if already authenticated
        if (authService.isAuthenticated()) {
          authenticated = true;
          isLoading = false;
          return;
        }

        // Try to restore auth from refresh token
        const success = await authService.initAuth();
        authenticated = success;
      } catch (error) {
        console.error('Authentication error:', error);
        authenticated = false;
      } finally {
        isLoading = false;
      }
      
      // If not authenticated after all checks, redirect to login
      if (!authenticated && data.currentPath) {
        authGuard(data.currentPath);
      }
    }
  });
</script>

{#if isLoading}
  <!-- Loading state -->
  <div class="min-h-screen flex items-center justify-center bg-gray-100">
    <div class="text-center">
      <div class="spinner mb-3"></div>
      <p class="text-gray-600">Loading...</p>
    </div>
  </div>
{:else if data.isPublicRoute}
  <!-- Public route (login) -->
  <slot />
{:else if authenticated}
  <!-- Protected route with dashboard layout -->
  <DashboardLayout>
    <slot />
  </DashboardLayout>
{:else}
  <!-- Fallback while redirecting to login -->
  <div class="min-h-screen flex items-center justify-center bg-gray-100">
    <div class="text-center">
      <div class="spinner mb-3"></div>
      <p class="text-gray-600">Redirecting to login...</p>
    </div>
  </div>
{/if}

<style>
  .spinner {
    width: 40px;
    height: 40px;
    margin: 0 auto;
    border: 3px solid rgba(0, 0, 0, 0.1);
    border-radius: 50%;
    border-top-color: #3498db;
    animation: spin 1s ease-in-out infinite;
  }
  
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>