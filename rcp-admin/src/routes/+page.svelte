<script lang="ts">
  import { onMount } from 'svelte';
  // @ts-ignore
  import { invoke } from '@tauri-apps/api/tauri';

  // Simple state variables
  let error = '';
  let status = {
    running: false,
    uptime: '-',
    version: '-',
    connections: 0,
    cpu_usage: 0,
    memory_usage: 0
  };

  onMount(async () => {
    try {
      // Use a try-catch for each call to pinpoint any issues
      try {
        console.log("Trying to call greet command...");
        const greeting = await invoke('greet', { name: 'RCP Admin' });
        console.log("Greeting result:", greeting);
      } catch (greetError: unknown) {
        console.error("Greeting error:", greetError);
        error = `Error with greeting: ${greetError instanceof Error ? greetError.message : String(greetError)}`;
      }
      
      // Get server status as a separate try block
      try {
        console.log("Trying to call get_server_status command...");
        const serverStatus = await invoke('get_server_status');
        console.log("Status result:", serverStatus);
        if (serverStatus) {
          status = serverStatus as typeof status;
        }
      } catch (statusError: unknown) {
        console.error("Status error:", statusError);
        error = `Failed to get server status: ${statusError instanceof Error ? statusError.message : String(statusError)}`;
        
        // Fallback to placeholder data
        status = {
          running: true,
          uptime: '1h 23m',
          version: '0.1.0',
          connections: 3,
          cpu_usage: 2.5,
          memory_usage: 128.4
        };
      }
    } catch (e: unknown) {
      console.error('General error in RCP Admin:', e);
      error = `Communication error: ${e instanceof Error ? e.message : String(e)}`;
    }
  });
</script>

<main>
  <h1>RCP Admin</h1>
  
  {#if error}
    <div class="error">
      <p>Error: {error}</p>
    </div>
  {/if}
  
  <div class="status-card">
    <h2>Server Status</h2>
    <p>Running: {status.running ? 'Yes' : 'No'}</p>
    <p>Version: {status.version}</p>
    <p>Uptime: {status.uptime}</p>
    <p>Connections: {status.connections}</p>
    <p>CPU Usage: {status.cpu_usage ? status.cpu_usage.toFixed(1) : '0.0'}%</p>
    <p>Memory Usage: {status.memory_usage ? status.memory_usage.toFixed(1) : '0.0'} MB</p>
  </div>
</main>

<style>
  main {
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    padding: 2rem;
  }
  
  .error {
    background-color: #ffdddd;
    border-left: 4px solid #f44336;
    padding: 0.5rem 1rem;
    margin-bottom: 1rem;
  }
  
  .status-card {
    background-color: #f9f9f9;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
</style>
