<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import apiService from '$lib/api/api';
  
  let isAuthenticated = false;
  let isLoading = true;
  
  onMount(() => {
    const token = localStorage.getItem('auth_token');
    isAuthenticated = !!token;
    isLoading = false;
    
    // Redirect to login if not authenticated
    if (!isAuthenticated && $page.url.pathname !== '/login') {
      goto('/login');
    }
  });
  
  function handleLogout() {
    apiService.logout();
    isAuthenticated = false;
    goto('/login');
  }
</script>

<svelte:head>
  <title>RCP Management Dashboard</title>
  <meta name="description" content="Remote Control Protocol Management Dashboard" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
</svelte:head>

<div class="app">
  {#if isLoading}
    <div class="loading">
      <div class="spinner-border" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    </div>
  {:else}
    {#if isAuthenticated && $page.url.pathname !== '/login'}
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
          <a class="navbar-brand" href="/">RCP Management</a>
          <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
            <span class="navbar-toggler-icon"></span>
          </button>
          <div class="collapse navbar-collapse" id="navbarNav">
            <ul class="navbar-nav me-auto">
              <li class="nav-item">
                <a class="nav-link" class:active={$page.url.pathname === '/'} href="/">Dashboard</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" class:active={$page.url.pathname === '/users'} href="/users">Users</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" class:active={$page.url.pathname === '/sessions'} href="/sessions">Sessions</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" class:active={$page.url.pathname === '/config'} href="/config">Configuration</a>
              </li>
              <li class="nav-item">
                <a class="nav-link" class:active={$page.url.pathname === '/logs'} href="/logs">Logs</a>
              </li>
            </ul>
            <div class="d-flex">
              <button class="btn btn-outline-light" on:click={handleLogout}>Logout</button>
            </div>
          </div>
        </div>
      </nav>
    {/if}
    
    <main class="container mt-4">
      <slot />
    </main>
    
    <footer class="footer mt-auto py-3 bg-light">
      <div class="container text-center">
        <span class="text-muted">RCP Management Dashboard © 2025</span>
      </div>
    </footer>
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  
  main {
    flex: 1;
  }
  
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }
</style>