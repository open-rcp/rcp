<script>
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  
  // Authentication store - will enhance this with proper API later
  export const authStore = writable({
    authenticated: true, // Default to true for now during development
    user: {
      name: 'Admin User',
      role: 'admin'
    },
    token: null
  });
  
  // Sidebar state
  export const isSidebarOpen = writable(true);
  
  onMount(() => {
    // Check for stored auth token on load
    const storedToken = localStorage.getItem('rcp_auth_token');
    if (storedToken) {
      try {
        const tokenData = JSON.parse(storedToken);
        if (new Date(tokenData.expires) > new Date()) {
          authStore.set({
            authenticated: true,
            user: tokenData.user,
            token: tokenData.token
          });
        } else {
          // Token expired, remove it
          localStorage.removeItem('rcp_auth_token');
        }
      } catch (error) {
        console.error('Failed to parse auth token', error);
      }
    }
  });
  
  // Toggle sidebar
  function toggleSidebar() {
    isSidebarOpen.update(value => !value);
  }
</script>

<div class="dashboard-layout">
  {#if $authStore.authenticated}
    <aside class="sidebar" class:hidden={!$isSidebarOpen}>
      <div class="p-4 border-b">
        <h2 class="text-xl font-semibold text-primary-600">RCP Desk</h2>
      </div>
      <nav class="p-2">
        <ul class="space-y-1">
          <li>
            <a href="/" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
              </svg>
              Dashboard
            </a>
          </li>
          <li>
            <a href="/servers" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M2 5a2 2 0 012-2h12a2 2 0 012 2v2a2 2 0 01-2 2H4a2 2 0 01-2-2V5zm14 1a1 1 0 11-2 0 1 1 0 012 0zM2 13a2 2 0 012-2h12a2 2 0 012 2v2a2 2 0 01-2 2H4a2 2 0 01-2-2v-2zm14 1a1 1 0 11-2 0 1 1 0 012 0z" clip-rule="evenodd" />
              </svg>
              Servers
            </a>
          </li>
          <li>
            <a href="/sessions" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v1h8v-1zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-1a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v1h-3zM4.75 12.094A5.973 5.973 0 004 15v1H1v-1a3 3 0 013.75-2.906z" />
              </svg>
              Sessions
            </a>
          </li>
          <li>
            <a href="/users" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path d="M9 6a3 3 0 11-6 0 3 3 0 016 0zM17 6a3 3 0 11-6 0 3 3 0 016 0zM12.93 17c.046-.327.07-.66.07-1a6.97 6.97 0 00-1.5-4.33A5 5 0 0119 16v1h-6.07zM6 11a5 5 0 015 5v1H1v-1a5 5 0 015-5z" />
              </svg>
              Users
            </a>
          </li>
          <li>
            <a href="/config" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
              </svg>
              Configuration
            </a>
          </li>
          <li>
            <a href="/logs" class="flex items-center p-2 rounded-md hover:bg-gray-100">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-gray-500" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
              </svg>
              Logs
            </a>
          </li>
        </ul>
      </nav>
    </aside>
    
    <main class="main-content">
      <header class="bg-white p-4 shadow-sm flex justify-between items-center">
        <button class="p-2" on:click={toggleSidebar} aria-label="Toggle sidebar">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        
        <div class="flex items-center gap-4">
          <div class="flex items-center">
            <span class="mr-2 text-sm font-medium">{$authStore.user?.name || 'User'}</span>
            <button 
              class="p-1 rounded-full bg-gray-100"
              aria-label="Log out"
              on:click={() => {
                authStore.set({ 
                  authenticated: false, 
                  user: { name: '', role: '' },
                  token: null 
                });
                localStorage.removeItem('rcp_auth_token');
                window.location.href = '/login';
              }}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 001 1h12a1 1 0 001-1V4a1 1 0 00-1-1H3zm7 4a1 1 0 10-2 0v4a1 1 0 102 0V7zm1 4a1 1 0 102 0V7a1 1 0 10-2 0v4z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>
        </div>
      </header>
      
      <div class="p-6">
        <slot />
      </div>
    </main>
  {:else}
    <!-- Login layout -->
    <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div class="max-w-md w-full space-y-8">
        <slot />
      </div>
    </div>
  {/if}
</div>