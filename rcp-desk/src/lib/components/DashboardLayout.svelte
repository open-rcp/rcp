<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import { page } from '$app/stores';
  
  // Import the auth store
  import { authStore } from '$lib/stores/auth';
</script>

<div class="dashboard-layout">
  <Sidebar />
  
  <div class="flex-1 flex flex-col overflow-hidden">
    <header class="proxmox-header flex items-center justify-end">
      <div class="flex items-center mr-4">
        <span class="text-sm text-gray-700 mr-4">{$authStore?.user?.name || 'Administrator'}</span>
        <button 
          class="text-gray-600 hover:text-secondary-500"
          aria-label="User menu"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.121 17.804A13.937 13.937 0 0112 16c2.5 0 4.847.655 6.879 1.804M15 10a3 3 0 11-6 0 3 3 0 016 0zm6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
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
</style>