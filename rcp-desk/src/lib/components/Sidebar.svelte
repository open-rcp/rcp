<script lang="ts">
  import { page } from '$app/stores';
  import { authStore } from '$lib/stores/auth';
  
  // Navigation items
  const navItems = [
    {
      name: 'Dashboard',
      href: '/',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>`
    },
    {
      name: 'Servers',
      href: '/servers',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
            </svg>`
    },
    {
      name: 'Sessions',
      href: '/sessions',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>`
    },
    {
      name: 'Users',
      href: '/users',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>`
    },
    {
      name: 'Configuration',
      href: '/config',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>`
    },
    {
      name: 'Logs',
      href: '/logs',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>`
    }
  ];
  
  let isCollapsed = $state(false);
  
  function toggleSidebar() {
    isCollapsed = !isCollapsed;
  }
  
  function isActive(href: string): boolean {
    return $page.url.pathname === href || 
           ($page.url.pathname !== '/' && $page.url.pathname.startsWith(href) && href !== '/');
  }
  
  // Close sidebar on mobile when navigating
  function handleNavigate() {
    if (window.innerWidth < 768) {
      isCollapsed = true;
    }
  }
</script>

<aside class={`bg-primary-800 text-white transition-all duration-300 ${isCollapsed ? 'sidebar-collapsed' : 'sidebar-expanded'}`}>
  <div class="p-4 flex items-center justify-between">
    <div class="flex items-center">
      <img src="/logo.svg" alt="RCP Logo" class="h-8 w-8" />
      {#if !isCollapsed}
        <span class="ml-3 text-lg font-bold text-white">RCP Desk</span>
      {/if}
    </div>
    <button 
      class="sidebar-toggle"
      onclick={toggleSidebar}
      aria-label={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
    >
      {#if isCollapsed}
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
      {:else}
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
      {/if}
    </button>
  </div>
  
  <div class="mt-3">
    <nav class="space-y-0 px-2">
      {#each navItems as item}
        <a 
          href={item.href}
          class={`flex items-center px-4 py-2.5 text-sm font-medium transition duration-150 ease-in-out border-l-4 
                 ${isActive(item.href) ? 'bg-primary-900 border-secondary-500 text-white' : 'border-transparent text-gray-300 hover:bg-primary-700 hover:text-white'}`}
          onclick={handleNavigate}
          aria-current={isActive(item.href) ? 'page' : undefined}
        >
          <div class="flex-shrink-0 nav-icon">
            {@html item.icon}
          </div>
          {#if !isCollapsed}
            <span class="ml-3">{item.name}</span>
          {/if}
        </a>
      {/each}
    </nav>
  </div>
  
  <div class="mt-auto p-4">
    {#if !isCollapsed}
      <div class="text-sm text-gray-400">
        <div>Logged in as</div>
        <div class="font-semibold">{$authStore?.user?.name || 'User'}</div>
      </div>
    {/if}
    <a 
      href="/logout" 
      class="mt-2 flex items-center px-4 py-2 text-sm font-medium rounded-md text-gray-300 hover:bg-primary-700 hover:text-white transition duration-150 ease-in-out"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
      </svg>
      {#if !isCollapsed}
        <span class="ml-3">Logout</span>
      {/if}
    </a>
  </div>
</aside>

<style>
  aside {
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: sticky;
    top: 0;
    overflow-y: auto;
    box-shadow: 0 0 10px rgba(0,0,0,0.1);
  }
  
  .sidebar-expanded {
    min-width: 240px;
    width: 240px;
  }
  
  .sidebar-collapsed {
    min-width: 64px;
    width: 64px;
  }
  
  .sidebar-toggle {
    color: rgba(255, 255, 255, 0.7);
    border-radius: 0.125rem;
    padding: 0.25rem;
  }
  
  .sidebar-toggle:hover {
    color: rgba(255, 255, 255, 1);
    background-color: rgba(255, 255, 255, 0.1);
  }
  
  .sidebar-toggle:focus {
    outline: none;
    box-shadow: 0 0 0 2px rgba(246, 128, 44, 0.5);
  }
  
  .nav-icon {
    flex-shrink: 0;
  }
  
  @media (max-width: 768px) {
    .sidebar-expanded {
      position: fixed;
      z-index: 50;
    }
    
    .sidebar-collapsed {
      width: 0;
      min-width: 0;
      overflow: hidden;
      position: fixed;
    }
  }
</style>