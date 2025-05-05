<script lang="ts">
  import '../app.css';
  import DashboardLayout from '$components/DashboardLayout.svelte';
  import AuthLayout from '$components/AuthLayout.svelte';
  import { onMount } from 'svelte';
  import { authGuard } from '$lib/guards/auth.guard';
  import { authService } from '$services/auth.service';
  import { authStore } from '$stores/auth';
  import { browser } from '$lib/utils/environment';
  import { goto } from '$lib/utils/navigation';
  
  // Get properties from the layout data
  export let data: {
    isPublicRoute?: boolean;
    currentPath?: string;
  };
  
  // Authentication state
  let isLoading = true;
  let authenticated = false;
  
  // Subscribe to auth store changes
  authStore.subscribe(state => {
    authenticated = state.isAuthenticated;
  });

  // Handle authentication on mount
  onMount(async () => {
    if (browser) {
      // If already on login page and authenticated, redirect to dashboard
      if (data.isPublicRoute && authenticated) {
        goto('/');
        return;
      }
      
      // If on login page, no need for further checks
      if (data.isPublicRoute) {
        isLoading = false;
        return;
      }

      // For protected routes, check authentication
      try {
        // First check if already authenticated from store
        if (authenticated) {
          isLoading = false;
          return;
        }

        // Try to restore auth from refresh token if not authenticated
        const success = await authService.initAuth();
        if (success) {
          authenticated = true;
          isLoading = false;
          return;
        }
        
        // If we reach this point, we're not authenticated
        // Redirect to login page
        if (data.currentPath && data.currentPath !== '/login') {
          goto(`/login?returnUrl=${encodeURIComponent(data.currentPath)}`);
        } else {
          goto('/login');
        }
      } catch (error) {
        console.error('Authentication error:', error);
      } finally {
        isLoading = false;
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
{:else if data.isPublicRoute && !authenticated}
  <!-- Public route (login) with AuthLayout -->
  <AuthLayout>
    <slot />
  </AuthLayout>
{:else if authenticated}
  <!-- Authenticated route with dashboard layout -->
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