<script lang="ts">
  import { page } from '$lib/utils/stores';
  import { goto } from '$lib/utils/navigation';
  import { authService } from '$services/auth.service';
  import { authStore } from '$stores/auth';
  import { onMount } from 'svelte';

  // Form state
  let username = '';
  let password = '';
  let loading = false;
  let error = '';

  // Get return URL from query parameters
  let returnUrl = '/';
  
  // Subscribe to page store for URL params and check authentication state
  $: {
    returnUrl = $page.url.searchParams.get('returnUrl') || '/';
  }
  
  // Check if already authenticated on mount
  onMount(() => {
    // Check if already logged in
    let isAuthenticated = false;
    const unsubscribe = authStore.subscribe(state => {
      isAuthenticated = state.isAuthenticated;
    });
    
    // If authenticated, redirect to returnUrl
    if (isAuthenticated) {
      goto(returnUrl);
    }
    
    unsubscribe();
  });

  // Handle login form submission
  async function handleLogin() {
    // Reset error state
    error = '';
    
    // Validate form
    if (!username || !password) {
      error = 'Please enter both username and password';
      return;
    }

    try {
      loading = true;
      
      // Attempt login
      const success = await authService.login({ username, password });
      
      if (success) {
        // Ensure the redirect happens with a fresh page load
        window.location.href = returnUrl;
      } else {
        error = 'Invalid username or password';
      }
    } catch (err) {
      error = err instanceof Error ? err.message : 'An error occurred during login';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Login - RCP Desk</title>
</svelte:head>

<div class="bg-white shadow-md rounded-lg w-full max-w-lg p-8">
  <div class="text-center mb-8">
    <img src="/rust-icon.png" alt="RCP Logo" class="h-16 w-auto mx-auto mb-2">
    <h1 class="text-2xl font-bold text-gray-800">RCP Desk Login</h1>
  </div>
  
  <form on:submit|preventDefault={handleLogin}>
    {#if error}
      <div class="bg-red-50 border-l-4 border-red-500 p-4 mb-6 rounded">
        <p class="text-red-700">{error}</p>
      </div>
    {/if}
    
    <div class="mb-6">
      <label for="username" class="block text-sm font-medium text-gray-700 mb-1">Username</label>
      <input
        type="text"
        id="username"
        bind:value={username}
        autocomplete="username"
        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        placeholder="Enter your username"
        disabled={loading}
      />
    </div>
    
    <div class="mb-6">
      <label for="password" class="block text-sm font-medium text-gray-700 mb-1">Password</label>
      <input
        type="password"
        id="password"
        bind:value={password}
        autocomplete="current-password"
        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        placeholder="Enter your password"
        disabled={loading}
      />
    </div>
    
    <button
      type="submit"
      class="w-full bg-primary-600 text-white py-2 px-4 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition duration-200 font-medium"
      disabled={loading}
    >
      {loading ? 'Logging in...' : 'Log In'}
    </button>
  </form>
</div>