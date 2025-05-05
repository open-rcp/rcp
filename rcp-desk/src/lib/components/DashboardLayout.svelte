<script lang="ts">
  import Sidebar from '$components/Sidebar.svelte';
  import { page } from '$utils/stores';
  import { goto } from '$utils/navigation';
  
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
  
  // Handle user logout
  async function handleLogout(): Promise<void> {
    await authService.logout();
    goto('/login');
  }
</script>

<div class="dashboard-layout">
  <Sidebar />
  
  <div class="flex-1 flex flex-col overflow-hidden">
    <header class="proxmox-header flex items-center">
      <button 
        class="sidebar-toggle ml-4 text-gray-600 hover:text-secondary-500"
        on:click={toggleSidebar}
        aria-label={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
      >
        {#if isCollapsed}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="13 17 18 12 13 7"></polyline>
            <polyline points="6 17 11 12 6 7"></polyline>
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        {/if}
      </button>
      
      <div class="flex-1"></div>
      
      <div class="flex items-center mr-4">
        <span class="text-sm text-gray-700 mr-4">{$authStore?.user?.name || 'Administrator'}</span>
        <div class="relative">
          <button 
            class="text-gray-600 hover:text-secondary-500 p-1 rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            aria-label="User menu"
            on:click={handleLogout}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
          </button>
        </div>
      </div>
    </header>
    
    <div class="flex-1 overflow-auto bg-gray-100 p-4">
      <slot />
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