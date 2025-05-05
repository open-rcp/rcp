<script lang="ts">
  import { onMount } from 'svelte';
  
  // Mock data for servers
  let servers = $state([
    {
      id: "main-server",
      name: "Main Server",
      status: "running",
      port: 8716,
      connections: 3,
      uptime: 86400, // in seconds
      resources: {
        cpu_usage: 2.5,
        memory_usage: 128.4
      }
    },
    {
      id: "backup-server",
      name: "Backup Server",
      status: "stopped",
      port: 8717,
      connections: 0,
      uptime: 0,
      resources: {
        cpu_usage: 0,
        memory_usage: 0
      }
    },
    {
      id: "dev-server",
      name: "Development Server",
      status: "running",
      port: 8718,
      connections: 1,
      uptime: 3600,
      resources: {
        cpu_usage: 1.2,
        memory_usage: 90.5
      }
    }
  ]);
  
  let isAddServerModalOpen = $state(false);
  let newServer = $state({
    name: "",
    port: 8716,
    maxConnections: 100,
    tlsEnabled: false
  });
  
  // Format uptime
  function formatUptime(seconds: number): string {
    if (seconds === 0) return "Not Running";
    
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
  
  function toggleAddServerModal() {
    isAddServerModalOpen = !isAddServerModalOpen;
  }
  
  function addServer() {
    // In a real application, this would call the RCP API to create a server
    servers = [...servers, {
      id: `server-${Date.now()}`,
      name: newServer.name,
      status: "stopped",
      port: newServer.port,
      connections: 0,
      uptime: 0,
      resources: {
        cpu_usage: 0,
        memory_usage: 0
      }
    }];
    
    // Reset form and close modal
    newServer = {
      name: "",
      port: 8716,
      maxConnections: 100,
      tlsEnabled: false
    };
    isAddServerModalOpen = false;
  }
  
  function toggleServerStatus(serverId: string): void {
    servers = servers.map(server => {
      if (server.id === serverId) {
        const newStatus = server.status === "running" ? "stopped" : "running";
        return { 
          ...server, 
          status: newStatus,
          uptime: newStatus === "stopped" ? 0 : (newStatus === "running" ? 1 : server.uptime)
        };
      }
      return server;
    });
  }
  
  function deleteServer(serverId: string): void {
    if (confirm("Are you sure you want to delete this server?")) {
      servers = servers.filter(server => server.id !== serverId);
    }
  }
</script>

<svelte:head>
  <title>Server Management | RCP Desk</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-800">Server Management</h1>
    <div>
      <button 
        class="btn-primary flex items-center"
        onclick={toggleAddServerModal}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
        </svg>
        Add Server
      </button>
    </div>
  </div>
  
  <!-- Server List -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    {#each servers as server}
      <div class="card">
        <div class="flex justify-between items-start">
          <div>
            <h2 class="text-xl font-semibold">{server.name}</h2>
            <p class="text-sm text-gray-500">Port: {server.port}</p>
          </div>
          <div>
            {#if server.status === "running"}
              <span class="status-active">Running</span>
            {:else}
              <span class="status-inactive">Stopped</span>
            {/if}
          </div>
        </div>
        
        <div class="mt-4">
          <div class="flex justify-between text-sm mt-2">
            <span>Uptime:</span>
            <span>{formatUptime(server.uptime)}</span>
          </div>
          <div class="flex justify-between text-sm mt-2">
            <span>Connections:</span>
            <span>{server.connections}</span>
          </div>
          <div class="flex justify-between text-sm mt-2">
            <span>CPU Usage:</span>
            <span>{server.resources.cpu_usage}%</span>
          </div>
          <div class="flex justify-between text-sm mt-2">
            <span>Memory Usage:</span>
            <span>{server.resources.memory_usage} MB</span>
          </div>
        </div>
        
        <div class="mt-6 grid grid-cols-2 gap-2">
          <button 
            class={server.status === "running" ? "btn-outline text-red-600 hover:bg-red-50" : "btn-outline text-green-600 hover:bg-green-50"} 
            onclick={() => toggleServerStatus(server.id)}
          >
            {server.status === "running" ? "Stop" : "Start"}
          </button>
          <button class="btn-outline" onclick={() => window.location.href = `/servers/${server.id}`}>
            Configure
          </button>
          <button class="btn-outline col-span-2" onclick={() => window.location.href = `/servers/${server.id}/logs`}>
            View Logs
          </button>
          {#if server.status !== "running"}
            <button 
              class="btn-outline text-red-600 hover:bg-red-50 col-span-2"
              onclick={() => deleteServer(server.id)}
            >
              Delete
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Add Server Modal -->
{#if isAddServerModalOpen}
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-medium">Add New Server</h3>
        <button class="text-gray-400 hover:text-gray-600" onclick={toggleAddServerModal} aria-label="Close dialog">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <div class="space-y-4">
        <div>
          <label for="server-name" class="form-label">Server Name</label>
          <input
            id="server-name"
            type="text"
            class="form-input w-full rounded-md"
            placeholder="Main Server"
            bind:value={newServer.name}
          />
        </div>
        
        <div>
          <label for="server-port" class="form-label">Port</label>
          <input
            id="server-port"
            type="number"
            class="form-input w-full rounded-md"
            placeholder="8716"
            bind:value={newServer.port}
          />
        </div>
        
        <div>
          <label for="max-connections" class="form-label">Max Connections</label>
          <input
            id="max-connections"
            type="number"
            class="form-input w-full rounded-md"
            placeholder="100"
            bind:value={newServer.maxConnections}
          />
        </div>
        
        <div class="flex items-center">
          <input
            id="tls-enabled"
            type="checkbox"
            class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            bind:checked={newServer.tlsEnabled}
          />
          <label for="tls-enabled" class="ml-2 block text-sm text-gray-900">
            Enable TLS
          </label>
        </div>
      </div>
      
      <div class="mt-6 flex justify-end space-x-2">
        <button class="btn-outline" onclick={toggleAddServerModal}>
          Cancel
        </button>
        <button class="btn-primary" onclick={addServer}>
          Add Server
        </button>
      </div>
    </div>
  </div>
{/if}