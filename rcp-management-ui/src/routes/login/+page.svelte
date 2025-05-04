<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import apiService from '$lib/api/api';
  
  let username = '';
  let password = '';
  let errorMessage = '';
  let isLoading = false;
  
  onMount(() => {
    // Check if already authenticated
    const token = localStorage.getItem('auth_token');
    if (token) {
      goto('/');
    }
  });
  
  async function handleLogin() {
    if (!username || !password) {
      errorMessage = 'Please enter both username and password';
      return;
    }
    
    try {
      isLoading = true;
      errorMessage = '';
      
      await apiService.login(username, password);
      goto('/');
    } catch (error) {
      console.error('Login error:', error);
      errorMessage = error.response?.data?.message || 'Invalid username or password. Please try again.';
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="login-container">
  <div class="login-card">
    <h1 class="text-center mb-4">RCP Management</h1>
    
    {#if errorMessage}
      <div class="alert alert-danger" role="alert">
        {errorMessage}
      </div>
    {/if}
    
    <form on:submit|preventDefault={handleLogin}>
      <div class="mb-3">
        <label for="username" class="form-label">Username</label>
        <input
          type="text"
          class="form-control"
          id="username"
          bind:value={username}
          disabled={isLoading}
          placeholder="Enter username"
          autocomplete="username"
          required
        />
      </div>
      
      <div class="mb-3">
        <label for="password" class="form-label">Password</label>
        <input
          type="password"
          class="form-control"
          id="password"
          bind:value={password}
          disabled={isLoading}
          placeholder="Enter password"
          autocomplete="current-password"
          required
        />
      </div>
      
      <button 
        type="submit" 
        class="btn btn-primary w-100" 
        disabled={isLoading}
      >
        {#if isLoading}
          <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
          Signing in...
        {:else}
          Sign in
        {/if}
      </button>
    </form>
  </div>
</div>

<style>
  .login-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 80vh;
  }
  
  .login-card {
    background-color: #fff;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    padding: 2rem;
    width: 100%;
    max-width: 400px;
  }
</style>