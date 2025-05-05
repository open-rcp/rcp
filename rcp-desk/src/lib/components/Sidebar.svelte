<script lang="ts">
  import { page } from '$utils/stores';
  import { authStore } from '$stores/auth';
  import { sidebarStore } from '$stores/sidebar';
  import { onMount } from 'svelte';
  
  // Define the sidebar state type
  interface SidebarState {
    isCollapsed: boolean;
  }
  
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
  
  // Properly type the isCollapsed variable
  let isCollapsed: boolean = false;
  
  // Subscribe to the sidebar store with proper types
  const unsubscribe = sidebarStore.subscribe((state: SidebarState): void => {
    isCollapsed = state.isCollapsed;
  });
  
  // Check screen size on component mount
  onMount(() => {
    // Check if screen is below medium breakpoint (768px)
    const checkScreenSize = (): void => {
      sidebarStore.update((state: SidebarState): SidebarState => ({ 
        ...state, 
        isCollapsed: window.innerWidth < 768 
      }));
    };
    
    // Set initial state
    checkScreenSize();
    
    // Add resize listener
    window.addEventListener('resize', checkScreenSize);
    
    // Cleanup
    return () => {
      window.removeEventListener('resize', checkScreenSize);
      unsubscribe();
    };
  });
  
  function isActive(href: string): boolean {
    return $page.url.pathname === href || 
           ($page.url.pathname !== '/' && $page.url.pathname.startsWith(href) && href !== '/');
  }
  
  // Close sidebar on mobile when navigating
  function handleNavigate(): void {
    if (window.innerWidth < 768) {
      sidebarStore.update((state: SidebarState): SidebarState => ({ 
        ...state, 
        isCollapsed: true 
      }));
    }
  }
</script>

<aside class={`bg-primary-800 text-white transition-all duration-300 ${isCollapsed ? 'sidebar-collapsed' : 'sidebar-expanded'}`}>
  <div class="p-4 flex items-center">
    <div class='flex items-center justify-center w-full'>
      <img src="tauri.svg" alt="RCP Logo" class="h-8 w-auto object-contain" />
      {#if !isCollapsed}
        <span class="ml-3 text-lg font-bold text-white">RCP Desk</span>
      {/if}
    </div>
  </div>
  
  <div class="mt-3">
    <nav class="space-y-0 px-2">
      {#each navItems as item}
        <a 
          href={item.href}
          class={`flex items-center py-2.5 transition duration-150 ease-in-out border-l-4 
                 ${isCollapsed ? 'px-2' : 'px-4'} 
                 ${isActive(item.href) ? 'bg-primary-900 border-secondary-500 text-white' : 'border-transparent text-gray-300 hover:bg-primary-700 hover:text-white'}`}
          onclick={handleNavigate}
          aria-current={isActive(item.href) ? 'page' : undefined}
        >
          <div class={`flex-shrink-0 nav-icon ${isCollapsed ? 'mx-auto' : ''}`}>
            {@html item.icon}
          </div>
          {#if !isCollapsed}
            <span class="ml-3 text-sm font-medium">{item.name}</span>
          {/if}
        </a>
      {/each}
    </nav>
  </div>
  
  <div class="mt-auto p-4">
    {#if !isCollapsed}
      <div class="text-sm text-gray-400">
        <div class="inline-block">Logged in as <strong>{$authStore?.user?.name || 'User'}</strong></div>
      </div>
    {/if}
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
    min-width: 72px;  /* Increased from 64px to provide more space */
    width: 72px;
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