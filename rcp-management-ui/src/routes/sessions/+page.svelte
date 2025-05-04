<script>
  import { onMount, onDestroy } from 'svelte';
  import apiService from '$lib/api/api';
  
  let sessions = [];
  let isLoading = true;
  let errorMessage = '';
  let successMessage = '';
  let interval;
  
  onMount(() => {
    loadSessions();
    // Refresh sessions data every 10 seconds
    interval = setInterval(loadSessions, 10000);
  });
  
  onDestroy(() => {
    if (interval) clearInterval(interval);
  });
  
  async function loadSessions() {
    try {
      const response = await apiService.getSessions();
      sessions = response.data;
      errorMessage = '';
    } catch (error) {
      console.error('Error loading sessions:', error);
      errorMessage = 'Failed to load active sessions. Please try again.';
    } finally {
      isLoading = false;
    }
  }
  
  async function terminateSession(sessionId) {
    if (!confirm('Are you sure you want to terminate this session?')) {
      return;
    }
    
    try {
      await apiService.terminateSession(sessionId);
      successMessage = 'Session terminated successfully!';
      
      // Remove the terminated session from the list
      sessions = sessions.filter(session => session.id !== sessionId);
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        successMessage = '';
      }, 3000);
    } catch (error) {
      console.error('Error terminating session:', error);
      errorMessage = 'Failed to terminate session. Please try again.';
    }
  }
  
  function formatDuration(startTime) {
    const start = new Date(startTime);
    const now = new Date();
    const diffMs = now - start;
    
    const seconds = Math.floor(diffMs / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    
    if (days > 0) {
      return `${days}d ${hours % 24}h ${minutes % 60}m`;
    } else if (hours > 0) {
      return `${hours}h ${minutes % 60}m ${seconds % 60}s`;
    } else if (minutes > 0) {
      return `${minutes}m ${seconds % 60}s`;
    } else {
      return `${seconds}s`;
    }
  }
  
  function getSessionStatusClass(status) {
    switch (status) {
      case 'connected':
        return 'bg-success';
      case 'connecting':
        return 'bg-warning';
      case 'disconnected':
        return 'bg-secondary';
      case 'error':
        return 'bg-danger';
      default:
        return 'bg-info';
    }
  }
</script>

<div class="sessions-page">
  <div class="d-flex justify-content-between align-items-center mb-4">
    <h1>Active Sessions</h1>
    <button class="btn btn-outline-primary" on:click={loadSessions}>
      <i class="bi bi-arrow-clockwise"></i> Refresh
    </button>
  </div>
  
  {#if errorMessage}
    <div class="alert alert-danger" role="alert">
      {errorMessage}
    </div>
  {/if}
  
  {#if successMessage}
    <div class="alert alert-success" role="alert">
      {successMessage}
    </div>
  {/if}
  
  {#if isLoading}
    <div class="d-flex justify-content-center my-5">
      <div class="spinner-border" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    </div>
  {:else if sessions.length === 0}
    <div class="alert alert-info" role="alert">
      No active sessions found.
    </div>
  {:else}
    <div class="table-responsive">
      <table class="table table-striped table-hover">
        <thead>
          <tr>
            <th>ID</th>
            <th>Client</th>
            <th>User</th>
            <th>Status</th>
            <th>Duration</th>
            <th>IP Address</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each sessions as session (session.id)}
            <tr>
              <td>{session.id.substring(0, 8)}...</td>
              <td>
                <div>{session.client_name}</div>
                <small class="text-muted">{session.client_id}</small>
              </td>
              <td>{session.user || 'Anonymous'}</td>
              <td>
                <span class="badge {getSessionStatusClass(session.status)}">
                  {session.status}
                </span>
              </td>
              <td>{formatDuration(session.start_time)}</td>
              <td>{session.ip_address}</td>
              <td>
                <button 
                  class="btn btn-sm btn-danger" 
                  on:click={() => terminateSession(session.id)}
                  title="Terminate Session"
                >
                  <i class="bi bi-x-circle"></i> Terminate
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    
    <div class="mt-3">
      <p>Total active sessions: {sessions.length}</p>
    </div>
  {/if}
</div>

<style>
  .sessions-page {
    margin-bottom: 2rem;
  }
</style>