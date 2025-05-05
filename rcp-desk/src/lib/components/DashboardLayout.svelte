<script lang="ts">
  import Sidebar from '$components/Sidebar.svelte';
  import Gravatar from '$components/Gravatar.svelte';
  import { page } from '$utils/stores';
  import { goto } from '$utils/navigation';
  import { onMount } from 'svelte';
  
  // Import the auth store and sidebar store
  import { authStore } from '$stores/auth';
  import { sidebarStore, toggleSidebar } from '$stores/sidebar';
  import { authService } from '$services/auth.service';
  
  // Define the sidebar state type
  interface SidebarState {
    isCollapsed: boolean;
  }
  
  // Subscribe to the sidebar store to get current state
  let isCollapsed: boolean = false;
  sidebarStore.subscribe((state: SidebarState): void => {
    isCollapsed = state.isCollapsed;
  });
  
  // User menu state
  let isUserMenuOpen = false;
  let userMenuRef: HTMLDivElement;
  
  // Handle user menu toggle
  function toggleUserMenu(): void {
    isUserMenuOpen = !isUserMenuOpen;
  }
  
  // Navigate to profile page
  function goToProfile(): void {
    isUserMenuOpen = false;
    goto('/profile');
  }
  
  // Handle user logout
  async function handleLogout(): Promise<void> {
    isUserMenuOpen = false;
    await authService.logout();
    goto('/login');
  }
  
  // Click outside handler to close user menu
  function handleClickOutside(event: MouseEvent): void {
    if (userMenuRef && !userMenuRef.contains(event.target as Node) && isUserMenuOpen) {
      isUserMenuOpen = false;
    }
  }
  
  // Add click outside event listener
  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<div class="dashboard-layout">
  <Sidebar />
  
  <div class="flex-1 flex flex-col overflow-hidden">
    <header class="proxmox-header flex items-center">
      <button 
        class="sidebar-toggle text-gray-600 hover:text-secondary-500"
        on:click={toggleSidebar}
        aria-label={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
      >
        {#if isCollapsed}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        {/if}
      </button>
      
      <div class="flex-1"></div>
      
      <div class="flex items-center">
        <span class="text-sm text-gray-700 mr-2">{$authStore?.user?.name || 'Administrator'}</span>
        <div class="relative" bind:this={userMenuRef}>
          <button 
            class="flex items-center text-gray-600 hover:text-secondary-500 p-1 rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            aria-label="User menu"
            aria-expanded={isUserMenuOpen}
            aria-haspopup="true"
            on:click={toggleUserMenu}
          >
            <Gravatar 
              email={$authStore?.user?.email || ''} 
              size={32} 
              defaultImage="identicon" 
              className="rounded-full"
            />
          </button>
          
          {#if isUserMenuOpen}
            <div class="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 divide-y divide-gray-100 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button">
              <div class="py-1" role="none">
                <button 
                  class="flex items-center w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" 
                  role="menuitem" 
                  on:click={goToProfile}
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                  </svg>
                  Profile
                </button>
              </div>
              <div class="py-1" role="none">
                <button 
                  class="flex items-center w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-100" 
                  role="menuitem" 
                  on:click={handleLogout}
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                  </svg>
                  Logout
                </button>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </header>
    
    <div class="flex-1 overflow-auto bg-gray-100">
      <div class="container mx-auto p-6">
        <slot />
      </div>
    </div>
  </div>
</div>

<style>
  .dashboard-layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }
  
  .proxmox-header {
    background-color: #fff;
    border-bottom: 1px solid #e2e8f0;
    height: 60px;
  }
  
  .sidebar-toggle {
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    transition: all 150ms ease-in-out;
  }
  
  .sidebar-toggle:hover {
    background-color: rgba(0, 0, 0, 0.05);
  }
  
  .sidebar-toggle:focus {
    outline: none;
    box-shadow: 0 0 0 2px rgba(66, 153, 225, 0.5);
  }
</style>