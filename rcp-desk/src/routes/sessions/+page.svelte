<script lang="ts">
  import { onMount } from 'svelte';
  
  // Mock data for sessions
  interface Session {
    id: string;
    client_id: string;
    client_name: string;
    client_address: string;
    server_id: string;
    server_name: string;
    started_at: string;
    subscribed_services: string[];
    idle: boolean;
    duration: number; // in seconds
    bytes_received: number;
    bytes_sent: number;
  }

  interface Filters {
    server: string;
    status: string;
    service: string;
    search: string;
  }

  let sessions: Session[] = $state([
    {
      id: "52023038-41a6-41ff-bb87-e18b24163e92",
      client_id: "eb9b6298-0ee9-457c-a88d-b89b616ce275",
      client_name: "UserWorkstation",
      client_address: "192.168.1.50",
      server_id: "main-server",
      server_name: "Main Server",
      started_at: "2025-05-04T12:15:30Z",
      subscribed_services: ["display", "input", "app"],
      idle: false,
      duration: 7500, // in seconds
      bytes_received: 15728640,
      bytes_sent: 104857600
    },
    {
      id: "8fd9429f-0ae4-4360-9b42-27b7d5f09d63",
      client_id: "9e613a4e-97e5-4f7c-9877-e9784fecc083",
      client_name: "MobileApp",
      client_address: "192.168.1.100",
      server_id: "main-server",
      server_name: "Main Server",
      started_at: "2025-05-04T12:30:45Z",
      subscribed_services: ["display", "clipboard"],
      idle: true,
      duration: 6800, // in seconds
      bytes_received: 5242880,
      bytes_sent: 31457280
    },
    {
      id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      client_id: "c5f7e3d1-b2a4-9876-5432-0987fedcba98",
      client_name: "TabletDevice",
      client_address: "192.168.1.75",
      server_id: "dev-server",
      server_name: "Development Server",
      started_at: "2025-05-04T13:10:20Z",
      subscribed_services: ["display", "input", "file-transfer"],
      idle: false,
      duration: 4500, // in seconds
      bytes_received: 8388608,
      bytes_sent: 41943040
    }
  ]);
  
  let filters: Filters = $state({
    server: "",
    status: "",
    service: "",
    search: ""
  });
  
  let filteredSessions: Session[] = $derived(sessions.filter(session => {
    // Apply server filter
    if (filters.server && session.server_id !== filters.server) {
      return false;
    }
    
    // Apply status filter
    if (filters.status) {
      if (filters.status === 'active' && session.idle) {
        return false;
      }
      if (filters.status === 'idle' && !session.idle) {
        return false;
      }
    }
    
    // Apply service filter
    if (filters.service && !session.subscribed_services.includes(filters.service)) {
      return false;
    }
    
    // Apply search filter
    if (filters.search) {
      const searchLower = filters.search.toLowerCase();
      return (
        session.client_name.toLowerCase().includes(searchLower) ||
        session.client_address.toLowerCase().includes(searchLower) ||
        session.id.toLowerCase().includes(searchLower)
      );
    }
    
    return true;
  }));
  
  // Format bytes to human-readable format
  function formatBytes(bytes: number, decimals = 2): string {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
  }
  
  // Format uptime/duration
  function formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    return `${hours}h ${minutes}m ${secs}s`;
  }
  
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
  
  function terminateSession(sessionId: string): void {
    if (confirm("Are you sure you want to terminate this session?")) {
      // In a real app, this would call the API to terminate the session
      sessions = sessions.filter(session => session.id !== sessionId);
    }
  }
  
  function resetFilters(): void {
    filters = {
      server: "",
      status: "",
      service: "",
      search: ""
    };
  }
  
  onMount(() => {
    // Initialize filtered sessions
    filteredSessions = [...sessions];
    
    // In a real app, we would fetch sessions from the API here
  });
</script>

<svelte:head>
  <title>Session Management | RCP Desk</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-800">Active Sessions</h1>
    <div>
      <button class="btn-primary flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
        </svg>
        Refresh
      </button>
    </div>
  </div>
  
  <!-- Filters -->
  <div class="card">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-medium">Filters</h2>
      <button class="text-primary-600 text-sm" onclick={resetFilters}>Reset</button>
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div>
        <label for="server-filter" class="form-label">Server</label>
        <select 
          id="server-filter" 
          class="form-input w-full rounded-md" 
          bind:value={filters.server}
        >
          <option value="">All Servers</option>
          <option value="main-server">Main Server</option>
          <option value="backup-server">Backup Server</option>
          <option value="dev-server">Development Server</option>
        </select>
      </div>
      
      <div>
        <label for="status-filter" class="form-label">Status</label>
        <select 
          id="status-filter" 
          class="form-input w-full rounded-md" 
          bind:value={filters.status}
        >
          <option value="">All Statuses</option>
          <option value="active">Active</option>
          <option value="idle">Idle</option>
        </select>
      </div>
      
      <div>
        <label for="service-filter" class="form-label">Service</label>
        <select 
          id="service-filter" 
          class="form-input w-full rounded-md" 
          bind:value={filters.service}
        >
          <option value="">All Services</option>
          <option value="display">Display</option>
          <option value="input">Input</option>
          <option value="app">Application</option>
          <option value="clipboard">Clipboard</option>
          <option value="file-transfer">File Transfer</option>
        </select>
      </div>
      
      <div>
        <label for="search-filter" class="form-label">Search</label>
        <input
          id="search-filter"
          type="text"
          class="form-input w-full rounded-md"
          placeholder="Search by name, address or ID..."
          bind:value={filters.search}
        />
      </div>
    </div>
  </div>
  
  <!-- Sessions Table -->
  <div class="card overflow-hidden" style="padding: 0;">
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead>
          <tr>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Client</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Server</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Started</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Duration</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Bandwidth</th>
            <th class="px-6 py-3 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          {#each filteredSessions as session}
            <tr>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm font-medium text-gray-900">{session.client_name}</div>
                <div class="text-sm text-gray-500">{session.client_address}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{session.server_name}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{timeAgo(session.started_at)}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{formatDuration(session.duration)}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                {#if session.idle}
                  <span class="status-inactive">Idle</span>
                {:else}
                  <span class="status-active">Active</span>
                {/if}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">
                  <span title="Data received from client">↓ {formatBytes(session.bytes_received)}</span>
                  <br/>
                  <span title="Data sent to client">↑ {formatBytes(session.bytes_sent)}</span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a href={`/sessions/${session.id}`} class="text-primary-600 hover:text-primary-900 mr-3">
                  <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><!-- Icon from Mage Icons by MageIcons - https://github.com/Mage-Icons/mage-icons/blob/main/License.txt --><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"><path stroke-width="1.5" d="M12 21.5a9.5 9.5 0 1 0 0-19a9.5 9.5 0 0 0 0 19m.005-4.222v-6.333"/><path stroke-width="2" d="M11.956 7.443h.01"/></g></svg>
                  Details
                </a>
                <button onclick={() => terminateSession(session.id)} class="text-red-600 hover:text-red-900">
                  <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><!-- Icon from Mage Icons by MageIcons - https://github.com/Mage-Icons/mage-icons/blob/main/License.txt --><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 21.5a9.5 9.5 0 1 0 0-19a9.5 9.5 0 0 0 0 19m6.713-2.787L5.287 5.287"/></svg>
                  Block
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    
    {#if filteredSessions.length === 0}
      <div class="py-10 text-center">
        <p class="text-gray-500">No sessions match the current filters</p>
        <button class="mt-2 text-primary-600" onclick={resetFilters}>Clear filters</button>
      </div>
    {/if}
  </div>
  
  <!-- Stats -->
  <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
    <div class="card">
      <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Total Sessions</h2>
      <div class="text-2xl font-bold text-primary-600">{filteredSessions.length}</div>
    </div>
    
    <div class="card">
      <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Active Sessions</h2>
      <div class="text-2xl font-bold text-primary-600">
        {filteredSessions.filter(s => !s.idle).length}
      </div>
    </div>
    
    <div class="card">
      <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Total Bandwidth</h2>
      <div class="text-2xl font-bold text-primary-600">
        {formatBytes(filteredSessions.reduce((sum, session) => sum + session.bytes_received + session.bytes_sent, 0))}
      </div>
    </div>
    
    <div class="card">
      <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-2">Average Duration</h2>
      <div class="text-2xl font-bold text-primary-600">
        {filteredSessions.length ? formatDuration(Math.floor(filteredSessions.reduce((sum, session) => sum + session.duration, 0) / filteredSessions.length)) : "N/A"}
      </div>
    </div>
  </div>
</div>