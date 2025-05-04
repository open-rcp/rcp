<script>
  import { onMount, onDestroy } from 'svelte';
  import apiService from '$lib/api/api';
  
  let serverStatus = 'loading';
  let serverStats = null;
  let errorMessage = '';
  let interval;
  
  // Fetch server status
  async function fetchServerStatus() {
    try {
      const response = await apiService.getServerStatus();
      serverStatus = response.data.status;
      serverStats = response.data.stats;
      errorMessage = '';
    } catch (error) {
      console.error('Error fetching server status:', error);
      errorMessage = 'Failed to fetch server status. Please try again.';
    }
  }

  async function controlServer(action) {
    try {
      errorMessage = '';
      serverStatus = 'loading';
      
      let response;
      switch(action) {
        case 'start':
          response = await apiService.startServer();
          break;
        case 'stop':
          response = await apiService.stopServer();
          break;
        case 'restart':
          response = await apiService.restartServer();
          break;
      }
      
      // Refresh status after action
      fetchServerStatus();
    } catch (error) {
      console.error(`Error performing server ${action}:`, error);
      errorMessage = `Failed to ${action} the server. Please try again.`;
      fetchServerStatus();
    }
  }
  
  onMount(() => {
    fetchServerStatus();
    // Poll server status every 5 seconds
    interval = setInterval(fetchServerStatus, 5000);
  });
  
  onDestroy(() => {
    if (interval) clearInterval(interval);
  });
</script>

<div class="dashboard">
  <h1>RCP Server Dashboard</h1>
  
  {#if errorMessage}
    <div class="alert alert-danger" role="alert">
      {errorMessage}
    </div>
  {/if}
  
  <div class="card mb-4">
    <div class="card-header">
      <h2 class="card-title h5 mb-0">Server Status</h2>
    </div>
    <div class="card-body">
      {#if serverStatus === 'loading'}
        <div class="d-flex justify-content-center">
          <div class="spinner-border" role="status">
            <span class="visually-hidden">Loading...</span>
          </div>
        </div>
      {:else if serverStatus === 'running'}
        <div class="alert alert-success">
          <strong>Server is running</strong>
        </div>
        <div class="server-controls">
          <button class="btn btn-danger me-2" on:click={() => controlServer('stop')}>
            Stop Server
          </button>
          <button class="btn btn-warning" on:click={() => controlServer('restart')}>
            Restart Server
          </button>
        </div>
      {:else if serverStatus === 'stopped'}
        <div class="alert alert-secondary">
          <strong>Server is stopped</strong>
        </div>
        <div class="server-controls">
          <button class="btn btn-success" on:click={() => controlServer('start')}>
            Start Server
          </button>
        </div>
      {:else}
        <div class="alert alert-warning">
          <strong>Unknown server status: {serverStatus}</strong>
        </div>
      {/if}
    </div>
  </div>
  
  {#if serverStats && serverStatus === 'running'}
    <div class="row">
      <div class="col-md-4">
        <div class="card mb-4">
          <div class="card-header">
            <h3 class="card-title h5 mb-0">Active Sessions</h3>
          </div>
          <div class="card-body">
            <h4 class="display-4 text-center">{serverStats.active_sessions}</h4>
          </div>
        </div>
      </div>
      
      <div class="col-md-4">
        <div class="card mb-4">
          <div class="card-header">
            <h3 class="card-title h5 mb-0">Connected Clients</h3>
          </div>
          <div class="card-body">
            <h4 class="display-4 text-center">{serverStats.connected_clients}</h4>
          </div>
        </div>
      </div>
      
      <div class="col-md-4">
        <div class="card mb-4">
          <div class="card-header">
            <h3 class="card-title h5 mb-0">Uptime</h3>
          </div>
          <div class="card-body">
            <h4 class="display-4 text-center">{serverStats.uptime}</h4>
          </div>
        </div>
      </div>
    </div>
    
    <div class="card mb-4">
      <div class="card-header">
        <h2 class="card-title h5 mb-0">Resource Usage</h2>
      </div>
      <div class="card-body">
        <div class="mb-3">
          <label class="form-label">CPU Usage</label>
          <div class="progress">
            <div 
              class="progress-bar" 
              role="progressbar" 
              style="width: {serverStats.cpu_usage}%;" 
              aria-valuenow={serverStats.cpu_usage} 
              aria-valuemin="0" 
              aria-valuemax="100"
            >
              {serverStats.cpu_usage}%
            </div>
          </div>
        </div>
        
        <div class="mb-3">
          <label class="form-label">Memory Usage</label>
          <div class="progress">
            <div 
              class="progress-bar" 
              role="progressbar" 
              style="width: {serverStats.memory_usage_percent}%;" 
              aria-valuenow={serverStats.memory_usage_percent} 
              aria-valuemin="0" 
              aria-valuemax="100"
            >
              {serverStats.memory_usage_percent}% ({serverStats.memory_usage})
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .card {
    box-shadow: 0 0.125rem 0.25rem rgba(0, 0, 0, 0.075);
  }
  
  .server-controls {
    margin-top: 1rem;
  }
</style>