<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  // Mock data for initial development
  let serverStatus = $state({
    status: "running",
    uptime: 3600, // in seconds
    connections: {
      total: 15,
      active: 3
    },
    resources: {
      cpu_usage: 2.5,
      memory_usage: 128.4 // in MB
    }
  });

  let sessions = $state([
    {
      id: "52023038-41a6-41ff-bb87-e18b24163e92",
      client_name: "UserWorkstation",
      client_address: "192.168.1.50",
      started_at: "2025-05-04T12:15:30Z",
      idle: false
    },
    {
      id: "8fd9429f-0ae4-4360-9b42-27b7d5f09d63",
      client_name: "MobileApp",
      client_address: "192.168.1.100", 
      started_at: "2025-05-04T12:30:45Z",
      idle: true
    },
    {
      id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      client_name: "TabletDevice",
      client_address: "192.168.1.75",
      started_at: "2025-05-04T13:10:20Z",
      idle: false
    }
  ]);

  // Format relative time (e.g., "2 hours ago")
  function timeAgo(dateString: string): string {
    const date = new Date(dateString);
    const now = new Date();
    const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);
    
    let interval = Math.floor(seconds / 31536000);
    if (interval > 1) return interval + " years ago";
    
    interval = Math.floor(seconds / 2592000);
    if (interval > 1) return interval + " months ago";
    
    interval = Math.floor(seconds / 86400);
    if (interval > 1) return interval + " days ago";
    
    interval = Math.floor(seconds / 3600);
    if (interval > 1) return interval + " hours ago";
    
    interval = Math.floor(seconds / 60);
    if (interval > 1) return interval + " minutes ago";
    
    return Math.floor(seconds) + " seconds ago";
  }

  // Format uptime
  function formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    if (days > 0) {
      return `${days}d ${hours}h ${minutes}m`;
    } else if (hours > 0) {
      return `${hours}h ${minutes}m`;
    } else {
      return `${minutes}m`;
    }
  }

  // In the future, we will fetch real data from the RCP API
  // For now we're using mock data for UI development
  async function fetchData() {
    try {
      // This would be replaced with actual API calls
      // const response = await api.get('/server/status');
      // serverStatus = response.data;
      
      // We'll use invoke to call Tauri commands when we have the backend ready
      // For now, just simulate a delay
      await new Promise(resolve => setTimeout(resolve, 500));
      
    } catch (error) {
      console.error("Failed to fetch dashboard data:", error);
    }
  }

  onMount(() => {
    fetchData();
    
    // Set up periodic data refresh
    const interval = setInterval(fetchData, 10000); // Refresh every 10 seconds
    
    return () => clearInterval(interval);
  });
</script>

<svelte:head>
  <title>Dashboard | RCP Desk</title>
</svelte:head>

<main class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold text-center mb-8">Welcome to RCP Desk</h1>

  <div class="flex justify-center items-center space-x-4 mb-6">
    <a href="https://vitejs.dev" target="_blank" class="hover:opacity-80 transition-opacity">
      <img src="/vite.svg" class="h-12" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank" class="hover:opacity-80 transition-opacity">
      <img src="/tauri.svg" class="h-12" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank" class="hover:opacity-80 transition-opacity">
      <img src="/svelte.svg" class="h-12" alt="SvelteKit Logo" />
    </a>
  </div>
  
  <p class="text-center text-gray-600 mb-8">Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <div class="card bg-white shadow-md rounded-lg p-6 max-w-md mx-auto">
    <form class="flex space-x-2" onsubmit={greet}>
      <input 
        id="greet-input"
        placeholder="Enter a name..." 
        bind:value={name}
        class="form-input flex-1 px-4 py-2 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
      />
      <button 
        type="submit"
        class="btn-primary"
      >
        Greet
      </button>
    </form>
    
    {#if greetMsg}
      <p class="mt-4 text-center font-medium">{greetMsg}</p>
    {/if}
  </div>
  
  <div class="space-y-6 mt-12">
    <div class="flex justify-between items-center">
      <h1 class="text-2xl font-bold text-gray-800">Dashboard</h1>
      <div>
        <button class="btn-primary flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
          </svg>
          Refresh
        </button>
      </div>
    </div>
    
    <!-- Status Overview -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <div class="card">
        <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Server Status</h2>
        <div class="flex items-center">
          <span class="status-active">Active</span>
          <span class="ml-2 text-sm text-gray-500">Uptime: {formatUptime(serverStatus.uptime)}</span>
        </div>
      </div>
      
      <div class="card">
        <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Active Sessions</h2>
        <div class="text-2xl font-bold text-primary-600">{serverStatus.connections.active}</div>
        <p class="text-sm text-gray-500">Out of {serverStatus.connections.total} total connections</p>
      </div>
      
      <div class="card">
        <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">CPU Usage</h2>
        <div class="text-2xl font-bold text-primary-600">{serverStatus.resources.cpu_usage}%</div>
        <div class="w-full bg-gray-200 rounded-full h-2.5 mt-2">
          <div class="bg-primary-600 h-2.5 rounded-full" style="width: {serverStatus.resources.cpu_usage}%"></div>
        </div>
      </div>
      
      <div class="card">
        <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Memory Usage</h2>
        <div class="text-2xl font-bold text-primary-600">{serverStatus.resources.memory_usage} MB</div>
        <div class="w-full bg-gray-200 rounded-full h-2.5 mt-2">
          <div class="bg-primary-600 h-2.5 rounded-full" style="width: {(serverStatus.resources.memory_usage / 1024) * 100}%"></div>
        </div>
      </div>
    </div>
    
    <!-- Recent Sessions -->
    <div class="card overflow-hidden">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold">Recent Sessions</h2>
        <a href="/sessions" class="text-sm text-primary-600 hover:text-primary-800">View all</a>
      </div>
      
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead>
            <tr>
              <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Client</th>
              <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Address</th>
              <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Started</th>
              <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th class="px-6 py-3 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each sessions as session, i}
              <tr>
                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{session.client_name}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{session.client_address}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{timeAgo(session.started_at)}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  {#if session.idle}
                    <span class="status-inactive">Idle</span>
                  {:else}
                    <span class="status-active">Active</span>
                  {/if}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <a href="/sessions/{session.id}" class="text-primary-600 hover:text-primary-900 mr-3">View</a>
                  <button class="text-red-600 hover:text-red-900">Terminate</button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
    
    <!-- Quick Actions -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="card">
        <h3 class="text-lg font-medium mb-4">Quick Actions</h3>
        <div class="space-y-2">
          <button class="w-full btn-outline flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
              <path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM14 11a1 1 0 011 1v1h1a1 1 0 110 2h-1v1a1 1 0 11-2 0v-1h-1a1 1 0 110-2h1v-1a1 1 0 011-1z" />
            </svg>
            Start New Server
          </button>
          <button class="w-full btn-outline flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
            </svg>
            View System Logs
          </button>
          <button class="w-full btn-outline flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
            </svg>
            Configure System
          </button>
        </div>
      </div>
      
      <div class="card">
        <h3 class="text-lg font-medium mb-4">System Status</h3>
        <ul class="space-y-3">
          <li class="flex justify-between">
            <span class="text-gray-600">Service</span>
            <span class="font-medium text-green-600">Running</span>
          </li>
          <li class="flex justify-between">
            <span class="text-gray-600">Authentication</span>
            <span class="font-medium text-green-600">Active</span>
          </li>
          <li class="flex justify-between">
            <span class="text-gray-600">Database</span>
            <span class="font-medium text-green-600">Connected</span>
          </li>
          <li class="flex justify-between">
            <span class="text-gray-600">API</span>
            <span class="font-medium text-green-600">Available</span>
          </li>
          <li class="flex justify-between">
            <span class="text-gray-600">WebSocket</span>
            <span class="font-medium text-yellow-600">Limited</span>
          </li>
        </ul>
      </div>
      
      <div class="card">
        <h3 class="text-lg font-medium mb-4">Recent Activities</h3>
        <ul class="space-y-3">
          <li class="flex">
            <div class="flex-shrink-0 h-4 w-4 mt-0.5 rounded-full bg-green-500"></div>
            <div class="ml-3">
              <p class="text-sm text-gray-700">Server restarted</p>
              <p class="text-xs text-gray-500">10 minutes ago</p>
            </div>
          </li>
          <li class="flex">
            <div class="flex-shrink-0 h-4 w-4 mt-0.5 rounded-full bg-blue-500"></div>
            <div class="ml-3">
              <p class="text-sm text-gray-700">User 'admin' logged in</p>
              <p class="text-xs text-gray-500">30 minutes ago</p>
            </div>
          </li>
          <li class="flex">
            <div class="flex-shrink-0 h-4 w-4 mt-0.5 rounded-full bg-yellow-500"></div>
            <div class="ml-3">
              <p class="text-sm text-gray-700">Configuration updated</p>
              <p class="text-xs text-gray-500">1 hour ago</p>
            </div>
          </li>
          <li class="flex">
            <div class="flex-shrink-0 h-4 w-4 mt-0.5 rounded-full bg-red-500"></div>
            <div class="ml-3">
              <p class="text-sm text-gray-700">Session terminated</p>
              <p class="text-xs text-gray-500">2 hours ago</p>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</main>
