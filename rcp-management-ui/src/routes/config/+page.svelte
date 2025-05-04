<script>
  import { onMount } from 'svelte';
  import apiService from '$lib/api/api';
  
  let config = null;
  let isLoading = true;
  let isSaving = false;
  let errorMessage = '';
  let successMessage = '';
  
  onMount(async () => {
    await loadConfig();
  });
  
  async function loadConfig() {
    try {
      isLoading = true;
      errorMessage = '';
      successMessage = '';
      
      const response = await apiService.getConfig();
      config = response.data;
    } catch (error) {
      console.error('Error loading config:', error);
      errorMessage = 'Failed to load configuration. Please try again.';
    } finally {
      isLoading = false;
    }
  }
  
  async function saveConfig() {
    try {
      isSaving = true;
      errorMessage = '';
      
      await apiService.updateConfig(config);
      successMessage = 'Configuration saved successfully!';
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        successMessage = '';
      }, 3000);
    } catch (error) {
      console.error('Error saving config:', error);
      errorMessage = 'Failed to save configuration. Please try again.';
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="config-page">
  <h1>Server Configuration</h1>
  
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
  {:else if config}
    <form on:submit|preventDefault={saveConfig} class="config-form">
      <div class="card mb-4">
        <div class="card-header">
          <h3 class="card-title h5 mb-0">Server Settings</h3>
        </div>
        <div class="card-body">
          <div class="row">
            <div class="col-md-6 mb-3">
              <label for="serverHost" class="form-label">Server Host</label>
              <input
                type="text"
                class="form-control"
                id="serverHost"
                bind:value={config.server.host}
              />
            </div>
            
            <div class="col-md-6 mb-3">
              <label for="serverPort" class="form-label">Server Port</label>
              <input
                type="number"
                class="form-control"
                id="serverPort"
                bind:value={config.server.port}
                min="1"
                max="65535"
              />
            </div>
          </div>
          
          <div class="mb-3">
            <label for="maxConnections" class="form-label">Maximum Connections</label>
            <input
              type="number"
              class="form-control"
              id="maxConnections"
              bind:value={config.server.max_connections}
              min="1"
            />
          </div>
          
          <div class="mb-3 form-check">
            <input
              type="checkbox"
              class="form-check-input"
              id="enableTls"
              bind:checked={config.server.tls_enabled}
            />
            <label class="form-check-label" for="enableTls">Enable TLS</label>
          </div>
          
          {#if config.server.tls_enabled}
            <div class="mb-3">
              <label for="certPath" class="form-label">TLS Certificate Path</label>
              <input
                type="text"
                class="form-control"
                id="certPath"
                bind:value={config.server.tls_cert_path}
              />
            </div>
            
            <div class="mb-3">
              <label for="keyPath" class="form-label">TLS Key Path</label>
              <input
                type="text"
                class="form-control"
                id="keyPath"
                bind:value={config.server.tls_key_path}
              />
            </div>
          {/if}
        </div>
      </div>
      
      <div class="card mb-4">
        <div class="card-header">
          <h3 class="card-title h5 mb-0">Authentication Settings</h3>
        </div>
        <div class="card-body">
          <div class="mb-3">
            <label for="authMethod" class="form-label">Authentication Method</label>
            <select class="form-select" id="authMethod" bind:value={config.auth.method}>
              <option value="none">None</option>
              <option value="pre_shared_key">Pre-shared Key</option>
              <option value="password">Password</option>
              <option value="oauth">OAuth</option>
            </select>
          </div>
          
          {#if config.auth.method === 'pre_shared_key'}
            <div class="mb-3">
              <label for="pskValue" class="form-label">Pre-shared Key</label>
              <div class="input-group">
                <input
                  type="password"
                  class="form-control"
                  id="pskValue"
                  bind:value={config.auth.psk_value}
                />
                <button
                  class="btn btn-outline-secondary"
                  type="button"
                  on:click={() => {
                    const input = document.getElementById('pskValue');
                    if (input.type === 'password') {
                      input.type = 'text';
                    } else {
                      input.type = 'password';
                    }
                  }}
                >
                  Show/Hide
                </button>
              </div>
            </div>
          {/if}
          
          <div class="mb-3 form-check">
            <input
              type="checkbox"
              class="form-check-input"
              id="requireAuth"
              bind:checked={config.auth.require_auth}
            />
            <label class="form-check-label" for="requireAuth">Require Authentication</label>
          </div>
        </div>
      </div>
      
      <div class="card mb-4">
        <div class="card-header">
          <h3 class="card-title h5 mb-0">Logging Settings</h3>
        </div>
        <div class="card-body">
          <div class="mb-3">
            <label for="logLevel" class="form-label">Log Level</label>
            <select class="form-select" id="logLevel" bind:value={config.logging.level}>
              <option value="error">Error</option>
              <option value="warn">Warn</option>
              <option value="info">Info</option>
              <option value="debug">Debug</option>
              <option value="trace">Trace</option>
            </select>
          </div>
          
          <div class="mb-3 form-check">
            <input
              type="checkbox"
              class="form-check-input"
              id="logToFile"
              bind:checked={config.logging.to_file}
            />
            <label class="form-check-label" for="logToFile">Log to File</label>
          </div>
          
          {#if config.logging.to_file}
            <div class="mb-3">
              <label for="logPath" class="form-label">Log File Path</label>
              <input
                type="text"
                class="form-control"
                id="logPath"
                bind:value={config.logging.file_path}
              />
            </div>
          {/if}
        </div>
      </div>
      
      <div class="form-actions">
        <button type="button" class="btn btn-secondary me-2" on:click={loadConfig}>
          Reset
        </button>
        <button type="submit" class="btn btn-primary" disabled={isSaving}>
          {#if isSaving}
            <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
            Saving...
          {:else}
            Save Configuration
          {/if}
        </button>
      </div>
    </form>
  {:else}
    <div class="alert alert-warning">
      No configuration found. Please try refreshing the page.
    </div>
  {/if}
</div>

<style>
  .config-form {
    margin-bottom: 2rem;
  }
  
  .form-actions {
    margin-top: 1rem;
  }
</style>