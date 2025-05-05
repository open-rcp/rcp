<script lang="ts">
  import { onMount } from 'svelte';
  
  // Mock data for system configuration
  let config = $state({
    server: {
      default_port: 8716,
      max_connections: 100,
      connection_timeout: 30,
      keep_alive_interval: 15,
      tls_enabled: true,
      certificate_path: "/etc/rcp/certificates/cert.pem",
      private_key_path: "/etc/rcp/certificates/key.pem",
      bind_address: "0.0.0.0"
    },
    authentication: {
      required: true,
      methods: ["psk", "public_key"],
      default_method: "psk",
      token_expiry: 86400,
      failed_attempts_lockout: 5,
      lockout_duration: 900
    },
    services: {
      display: {
        enabled: true,
        max_quality: 90,
        fps_limit: 30,
        encryption: "aes256"
      },
      input: {
        enabled: true,
        keyboard_enabled: true,
        mouse_enabled: true
      },
      app: {
        enabled: true,
        allowed_apps: ["notepad.exe", "calc.exe", "mspaint.exe"],
        user_app_validation: true
      },
      clipboard: {
        enabled: true,
        max_size: 10485760, // 10MB
        allow_files: false
      },
      file_transfer: {
        enabled: true,
        max_file_size: 104857600, // 100MB
        allowed_extensions: ["txt", "pdf", "doc", "docx", "xls", "xlsx", "jpg", "png"]
      }
    },
    logging: {
      level: "info",
      file_enabled: true,
      file_path: "/var/log/rcp/server.log",
      max_file_size: 10485760, // 10MB
      max_files: 5,
      console_enabled: true
    },
    system: {
      auto_start: true,
      update_check: true,
      update_channel: "stable",
      metrics_collection: true,
      metrics_retention_days: 30
    }
  });
  
  let activeTab: string = $state("server");
  let isEditingMode: boolean = $state(false);
  let originalConfig: any = null;
  let isRestarting: boolean = $state(false);
  let saveSuccess: boolean = $state(false);
  let saveFail: boolean = $state(false);
  
  // Switch between tabs
  function switchTab(tabName: string): void {
    activeTab = tabName;
  }
  
  // Save configuration changes
  function saveConfig(): void {
    // In a real application, this would call the RCP API to update configuration
    saveSuccess = true;
    setTimeout(() => saveSuccess = false, 3000);
    
    // Exit editing mode
    isEditingMode = false;
    originalConfig = null;
  }
  
  // Toggle editing mode
  function toggleEditMode(): void {
    if (!isEditingMode) {
      // Store original config for potential cancellation
      originalConfig = JSON.parse(JSON.stringify(config));
      isEditingMode = true;
    } else {
      // Cancel editing - restore original config
      config = JSON.parse(JSON.stringify(originalConfig));
      isEditingMode = false;
      originalConfig = null;
    }
  }
  
  // Reset configuration to defaults
  function resetToDefaults(): void {
    if (confirm("Are you sure you want to reset all settings to default values? This cannot be undone.")) {
      // In a real application, this would call the API to reset config
      // For now, we'll just simulate it by resetting form controls
      
      // Would typically get defaults from the API, but for now we'll just use our initial mock data
      // In reality we'd do: const response = await api.get('/config/defaults');
      
      config = {
        server: {
          default_port: 8716,
          max_connections: 50,
          connection_timeout: 60,
          keep_alive_interval: 30,
          tls_enabled: false,
          certificate_path: "",
          private_key_path: "",
          bind_address: "0.0.0.0"
        },
        authentication: {
          required: true,
          methods: ["psk"],
          default_method: "psk",
          token_expiry: 86400,
          failed_attempts_lockout: 3,
          lockout_duration: 300
        },
        services: {
          display: {
            enabled: true,
            max_quality: 80,
            fps_limit: 30,
            encryption: "none"
          },
          input: {
            enabled: true,
            keyboard_enabled: true,
            mouse_enabled: true
          },
          app: {
            enabled: false,
            allowed_apps: [],
            user_app_validation: true
          },
          clipboard: {
            enabled: true,
            max_size: 1048576, // 1MB
            allow_files: false
          },
          file_transfer: {
            enabled: false,
            max_file_size: 10485760, // 10MB
            allowed_extensions: ["txt", "pdf", "jpg", "png"]
          }
        },
        logging: {
          level: "info",
          file_enabled: true,
          file_path: "/var/log/rcp/server.log",
          max_file_size: 5242880, // 5MB
          max_files: 3,
          console_enabled: true
        },
        system: {
          auto_start: false,
          update_check: true,
          update_channel: "stable",
          metrics_collection: false,
          metrics_retention_days: 7
        }
      };
    }
  }
  
  // Restart system
  function restartSystem(): void {
    if (confirm("Are you sure you want to restart the RCP service? All active connections will be terminated.")) {
      isRestarting = true;
      
      // In a real application, this would call the API to restart the service
      setTimeout(() => {
        isRestarting = false;
      }, 5000); // Simulate a 5-second restart
    }
  }
  
  // Format bytes to human-readable format
  function formatBytes(bytes: number, decimals: number = 0): string {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
  }
  
  function addAllowedApp(): void {
    // Get input element
    const input = document.getElementById('new-allowed-app') as HTMLInputElement;
    const value = input.value.trim();
    
    if (value) {
      config.services.app.allowed_apps = [...config.services.app.allowed_apps, value];
      input.value = '';
    }
  }
  
  function removeAllowedApp(app: string): void {
    config.services.app.allowed_apps = config.services.app.allowed_apps.filter(a => a !== app);
  }
  
  function addAllowedExtension(): void {
    // Get input element
    const input = document.getElementById('new-allowed-extension') as HTMLInputElement;
    const value = input.value.trim();
    
    if (value) {
      // Remove any leading dots
      const extension = value.startsWith('.') ? value.substring(1) : value;
      config.services.file_transfer.allowed_extensions = [...config.services.file_transfer.allowed_extensions, extension];
      input.value = '';
    }
  }
  
  function removeAllowedExtension(ext: string): void {
    config.services.file_transfer.allowed_extensions = config.services.file_transfer.allowed_extensions.filter(e => e !== ext);
  }
  
  onMount((): void => {
    // In a real app, we would fetch configuration from the API here
  });
</script>

<svelte:head>
  <title>Configuration | RCP Desk</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-800">System Configuration</h1>
    <div class="flex space-x-2">
      {#if !isEditingMode}
        <button class="btn-primary flex items-center" onclick={toggleEditMode}>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
            <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" /> 
          </svg>
          Edit Configuration
        </button>
      {:else}
        <button class="btn-outline flex items-center" onclick={toggleEditMode}>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
          Cancel
        </button>
        <button class="btn-primary flex items-center" onclick={saveConfig}>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
          </svg>
          Save
        </button>
      {/if}
      <button class="btn-outline" onclick={resetToDefaults}>
        Reset to Defaults
      </button>
    </div>
  </div>

  {#if saveSuccess}
    <div class="bg-green-50 border-l-4 border-green-400 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-green-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-green-700">
            Configuration saved successfully. Some changes may require restarting the service.
          </p>
        </div>
      </div>
    </div>
  {/if}
  
  {#if saveFail}
    <div class="bg-red-50 border-l-4 border-red-400 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-red-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-red-700">
            Failed to save configuration. Please try again.
          </p>
        </div>
      </div>
    </div>
  {/if}
  
  {#if isRestarting}
    <div class="bg-blue-50 border-l-4 border-blue-400 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="animate-spin h-5 w-5 text-blue-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-blue-700">
            Restarting RCP service, please wait...
          </p>
        </div>
      </div>
    </div>
  {/if}
  
  <div class="card">
    <!-- Tab navigation -->
    <div class="border-b border-gray-200">
      <nav class="-mb-px flex space-x-6 overflow-x-auto" aria-label="Tabs">
        <button 
          onclick={() => switchTab('server')} 
          class={`py-4 px-1 border-b-2 font-medium text-sm ${activeTab === 'server' ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`}>
          Server
        </button>
        <button 
          onclick={() => switchTab('authentication')} 
          class={`py-4 px-1 border-b-2 font-medium text-sm ${activeTab === 'authentication' ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`}>
          Authentication
        </button>
        <button 
          onclick={() => switchTab('services')} 
          class={`py-4 px-1 border-b-2 font-medium text-sm ${activeTab === 'services' ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`}>
          Services
        </button>
        <button 
          onclick={() => switchTab('logging')} 
          class={`py-4 px-1 border-b-2 font-medium text-sm ${activeTab === 'logging' ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`}>
          Logging
        </button>
        <button 
          onclick={() => switchTab('system')} 
          class={`py-4 px-1 border-b-2 font-medium text-sm ${activeTab === 'system' ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`}>
          System
        </button>
      </nav>
    </div>
    
    <!-- Tab content -->
    <div class="py-6 px-2">
      <!-- Server Settings -->
      {#if activeTab === 'server'}
        <div class="space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="default_port" class="form-label">Default Port</label>
              <input
                id="default_port"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.server.default_port}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">The default port for RCP server connections</p>
            </div>
            
            <div>
              <label for="max_connections" class="form-label">Max Connections</label>
              <input
                id="max_connections"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.server.max_connections}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Maximum number of concurrent connections</p>
            </div>
            
            <div>
              <label for="connection_timeout" class="form-label">Connection Timeout (seconds)</label>
              <input
                id="connection_timeout"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.server.connection_timeout}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Time before inactive connections are closed</p>
            </div>
            
            <div>
              <label for="keep_alive_interval" class="form-label">Keep Alive Interval (seconds)</label>
              <input
                id="keep_alive_interval"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.server.keep_alive_interval}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Interval between keep-alive messages</p>
            </div>
            
            <div>
              <label for="bind_address" class="form-label">Bind Address</label>
              <input
                id="bind_address"
                type="text"
                class="form-input w-full rounded-md"
                bind:value={config.server.bind_address}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Network address to bind the server to (0.0.0.0 for all interfaces)</p>
            </div>
          </div>
          
          <div class="border-t border-gray-200 pt-4">
            <h3 class="text-lg font-medium">TLS Configuration</h3>
            
            <div class="mt-4">
              <div class="flex items-center">
                <input
                  id="tls_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.server.tls_enabled}
                  disabled={!isEditingMode}
                />
                <label for="tls_enabled" class="ml-2 block text-sm text-gray-900">
                  Enable TLS encryption
                </label>
              </div>
              <p class="mt-1 text-xs text-gray-500">Encrypt all communications with TLS (recommended)</p>
            </div>
            
            {#if config.server.tls_enabled}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                <div>
                  <label for="certificate_path" class="form-label">Certificate Path</label>
                  <input
                    id="certificate_path"
                    type="text"
                    class="form-input w-full rounded-md"
                    bind:value={config.server.certificate_path}
                    disabled={!isEditingMode}
                  />
                </div>
                
                <div>
                  <label for="private_key_path" class="form-label">Private Key Path</label>
                  <input
                    id="private_key_path"
                    type="text"
                    class="form-input w-full rounded-md"
                    bind:value={config.server.private_key_path}
                    disabled={!isEditingMode}
                  />
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}
      
      <!-- Authentication Settings -->
      {#if activeTab === 'authentication'}
        <div class="space-y-6">
          <div class="flex items-center">
            <input
              id="auth_required"
              type="checkbox"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              bind:checked={config.authentication.required}
              disabled={!isEditingMode}
            />
            <label for="auth_required" class="ml-2 block text-sm text-gray-900">
              Require Authentication
            </label>
          </div>
          <p class="text-xs text-gray-500">When enabled, all connections must be authenticated</p>
          
          <div>
            <fieldset>
              <legend id="auth-methods-label" class="form-label">Authentication Methods</legend>
              <div class="mt-2 space-y-2" aria-labelledby="auth-methods-label">
                <div class="flex items-center">
                  <input
                    id="auth_method_psk"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    checked={config.authentication.methods.includes("psk")}
                    onchange={() => {
                      if (isEditingMode) {
                        if (config.authentication.methods.includes("psk")) {
                          config.authentication.methods = config.authentication.methods.filter(m => m !== "psk");
                        } else {
                          config.authentication.methods = [...config.authentication.methods, "psk"];
                        }
                      }
                    }}
                    disabled={!isEditingMode}
                  />
                  <label for="auth_method_psk" class="ml-2 text-sm text-gray-900">
                    Pre-shared Key
                  </label>
                </div>
                
                <div class="flex items-center">
                  <input
                    id="auth_method_public_key"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    checked={config.authentication.methods.includes("public_key")}
                    onchange={() => {
                      if (isEditingMode) {
                        if (config.authentication.methods.includes("public_key")) {
                          config.authentication.methods = config.authentication.methods.filter(m => m !== "public_key");
                        } else {
                          config.authentication.methods = [...config.authentication.methods, "public_key"];
                        }
                      }
                    }}
                    disabled={!isEditingMode}
                  />
                  <label for="auth_method_public_key" class="ml-2 text-sm text-gray-900">
                    Public Key
                  </label>
                </div>
              </div>
            </fieldset>
          </div>
          
          <div>
            <label for="default_method" class="form-label">Default Authentication Method</label>
            <select
              id="default_method"
              class="form-input w-full rounded-md"
              bind:value={config.authentication.default_method}
              disabled={!isEditingMode}
            >
              {#if config.authentication.methods.includes("psk")}
                <option value="psk">Pre-shared Key</option>
              {/if}
              {#if config.authentication.methods.includes("public_key")}
                <option value="public_key">Public Key</option>
              {/if}
            </select>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="token_expiry" class="form-label">Token Expiry (seconds)</label>
              <input
                id="token_expiry"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.authentication.token_expiry}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Time before authentication tokens expire</p>
            </div>
            
            <div>
              <label for="failed_attempts_lockout" class="form-label">Failed Attempts Before Lockout</label>
              <input
                id="failed_attempts_lockout"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.authentication.failed_attempts_lockout}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Number of authentication failures before account lockout</p>
            </div>
            
            <div>
              <label for="lockout_duration" class="form-label">Lockout Duration (seconds)</label>
              <input
                id="lockout_duration"
                type="number"
                class="form-input w-full rounded-md"
                bind:value={config.authentication.lockout_duration}
                disabled={!isEditingMode}
              />
              <p class="mt-1 text-xs text-gray-500">Time an account remains locked after too many failed attempts</p>
            </div>
          </div>
        </div>
      {/if}
      
      <!-- Services Settings -->
      {#if activeTab === 'services'}
        <div class="space-y-8">
          <!-- Display Service -->
          <div class="border-b border-gray-200 pb-6">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium">Display Service</h3>
              <div class="flex items-center">
                <input
                  id="display_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.services.display.enabled}
                  disabled={!isEditingMode}
                />
                <label for="display_enabled" class="ml-2 block text-sm text-gray-900">
                  Enabled
                </label>
              </div>
            </div>
            
            {#if config.services.display.enabled}
              <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4">
                <div>
                  <label for="display_max_quality" class="form-label">Maximum Quality (0-100)</label>
                  <input
                    id="display_max_quality"
                    type="number"
                    min="0"
                    max="100"
                    class="form-input w-full rounded-md"
                    bind:value={config.services.display.max_quality}
                    disabled={!isEditingMode}
                  />
                </div>
                
                <div>
                  <label for="display_fps_limit" class="form-label">FPS Limit</label>
                  <input
                    id="display_fps_limit"
                    type="number"
                    class="form-input w-full rounded-md"
                    bind:value={config.services.display.fps_limit}
                    disabled={!isEditingMode}
                  />
                </div>
                
                <div>
                  <label for="display_encryption" class="form-label">Encryption</label>
                  <select
                    id="display_encryption"
                    class="form-input w-full rounded-md"
                    bind:value={config.services.display.encryption}
                    disabled={!isEditingMode}
                  >
                    <option value="none">None</option>
                    <option value="aes256">AES-256</option>
                    <option value="chacha20">ChaCha20</option>
                  </select>
                </div>
              </div>
            {/if}
          </div>
          
          <!-- Input Service -->
          <div class="border-b border-gray-200 pb-6">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium">Input Service</h3>
              <div class="flex items-center">
                <input
                  id="input_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.services.input.enabled}
                  disabled={!isEditingMode}
                />
                <label for="input_enabled" class="ml-2 block text-sm text-gray-900">
                  Enabled
                </label>
              </div>
            </div>
            
            {#if config.services.input.enabled}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                <div class="flex items-center">
                  <input
                    id="keyboard_enabled"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    bind:checked={config.services.input.keyboard_enabled}
                    disabled={!isEditingMode}
                  />
                  <label for="keyboard_enabled" class="ml-2 block text-sm text-gray-900">
                    Keyboard Input Enabled
                  </label>
                </div>
                
                <div class="flex items-center">
                  <input
                    id="mouse_enabled"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    bind:checked={config.services.input.mouse_enabled}
                    disabled={!isEditingMode}
                  />
                  <label for="mouse_enabled" class="ml-2 block text-sm text-gray-900">
                    Mouse Input Enabled
                  </label>
                </div>
              </div>
            {/if}
          </div>
          
          <!-- Application Service -->
          <div class="border-b border-gray-200 pb-6">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium">Application Service</h3>
              <div class="flex items-center">
                <input
                  id="app_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.services.app.enabled}
                  disabled={!isEditingMode}
                />
                <label for="app_enabled" class="ml-2 block text-sm text-gray-900">
                  Enabled
                </label>
              </div>
            </div>
            
            {#if config.services.app.enabled}
              <div class="mt-4">
                <div class="flex items-center">
                  <input
                    id="user_app_validation"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    bind:checked={config.services.app.user_app_validation}
                    disabled={!isEditingMode}
                  />
                  <label for="user_app_validation" class="ml-2 block text-sm text-gray-900">
                    Require User Confirmation for App Launch
                  </label>
                </div>
                
                <div class="mt-4">
                  <label id="allowed-apps-label" class="form-label" for="allowed-apps-list">Allowed Applications</label>
                  <div id="allowed-apps-list" class="mt-2 flex flex-wrap gap-2" aria-labelledby="allowed-apps-label">
                    {#each config.services.app.allowed_apps as app}
                      <div class="bg-gray-100 rounded-full px-3 py-1 text-sm flex items-center">
                        <span>{app}</span>
                        {#if isEditingMode}
                          <button 
                            class="ml-2 text-gray-400 hover:text-gray-600"
                            onclick={() => removeAllowedApp(app)}
                            aria-label="Remove app"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                          </button>
                        {/if}
                      </div>
                    {/each}
                  </div>
                  
                  {#if isEditingMode}
                    <div class="mt-2 flex">
                      <input
                        id="new-allowed-app"
                        type="text"
                        class="form-input rounded-l-md flex-1"
                        placeholder="e.g. notepad.exe"
                      />
                      <button 
                        class="bg-gray-200 hover:bg-gray-300 text-gray-700 px-3 rounded-r-md"
                        onclick={addAllowedApp}
                      >
                        Add
                      </button>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
          
          <!-- Clipboard Service -->
          <div class="border-b border-gray-200 pb-6">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium">Clipboard Service</h3>
              <div class="flex items-center">
                <input
                  id="clipboard_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.services.clipboard.enabled}
                  disabled={!isEditingMode}
                />
                <label for="clipboard_enabled" class="ml-2 block text-sm text-gray-900">
                  Enabled
                </label>
              </div>
            </div>
            
            {#if config.services.clipboard.enabled}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                <div>
                  <label for="clipboard_max_size" class="form-label">Maximum Size</label>
                  <div class="flex items-center">
                    <input
                      id="clipboard_max_size"
                      type="number"
                      class="form-input w-full rounded-l-md"
                      bind:value={config.services.clipboard.max_size}
                      disabled={!isEditingMode}
                    />
                    <span class="inline-flex items-center px-3 rounded-r-md border border-l-0 border-gray-300 bg-gray-50 text-gray-500 text-sm">
                      bytes
                    </span>
                  </div>
                  <p class="mt-1 text-xs text-gray-500">Current: {formatBytes(config.services.clipboard.max_size)}</p>
                </div>
                
                <div class="flex items-center">
                  <input
                    id="allow_clipboard_files"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    bind:checked={config.services.clipboard.allow_files}
                    disabled={!isEditingMode}
                  />
                  <label for="allow_clipboard_files" class="ml-2 block text-sm text-gray-900">
                    Allow File Transfers via Clipboard
                  </label>
                </div>
              </div>
            {/if}
          </div>
          
          <!-- File Transfer Service -->
          <div>
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium">File Transfer Service</h3>
              <div class="flex items-center">
                <input
                  id="file_transfer_enabled"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  bind:checked={config.services.file_transfer.enabled}
                  disabled={!isEditingMode}
                />
                <label for="file_transfer_enabled" class="ml-2 block text-sm text-gray-900">
                  Enabled
                </label>
              </div>
            </div>
            
            {#if config.services.file_transfer.enabled}
              <div class="mt-4">
                <div>
                  <label for="file_max_size" class="form-label">Maximum File Size</label>
                  <div class="flex items-center">
                    <input
                      id="file_max_size"
                      type="number"
                      class="form-input w-full rounded-l-md"
                      bind:value={config.services.file_transfer.max_file_size}
                      disabled={!isEditingMode}
                    />
                    <span class="inline-flex items-center px-3 rounded-r-md border border-l-0 border-gray-300 bg-gray-50 text-gray-500 text-sm">
                      bytes
                    </span>
                  </div>
                  <p class="mt-1 text-xs text-gray-500">Current: {formatBytes(config.services.file_transfer.max_file_size)}</p>
                </div>
                
                <div class="mt-4">
                  <label id="allowed-extensions-label" class="form-label" for="allowed-extensions-list">Allowed File Extensions</label>
                  <div id="allowed-extensions-list" class="mt-2 flex flex-wrap gap-2" aria-labelledby="allowed-extensions-label">
                    {#each config.services.file_transfer.allowed_extensions as ext}
                      <div class="bg-gray-100 rounded-full px-3 py-1 text-sm flex items-center">
                        <span>.{ext}</span>
                        {#if isEditingMode}
                          <button 
                            class="ml-2 text-gray-400 hover:text-gray-600" 
                            onclick={() => removeAllowedExtension(ext)}
                            aria-label="Remove extension"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                          </button>
                        {/if}
                      </div>
                    {/each}
                  </div>
                  
                  {#if isEditingMode}
                    <div class="mt-2 flex">
                      <input
                        id="new-allowed-extension"
                        type="text"
                        class="form-input rounded-l-md flex-1"
                        placeholder="e.g. pdf"
                      />
                      <button 
                        class="bg-gray-200 hover:bg-gray-300 text-gray-700 px-3 rounded-r-md"
                        onclick={addAllowedExtension}
                      >
                        Add
                      </button>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}
      
      <!-- Logging Settings -->
      {#if activeTab === 'logging'}
        <div class="space-y-6">
          <div>
            <label for="log_level" class="form-label">Log Level</label>
            <select
              id="log_level"
              class="form-input w-full rounded-md"
              bind:value={config.logging.level}
              disabled={!isEditingMode}
            >
              <option value="debug">Debug</option>
              <option value="info">Info</option>
              <option value="warn">Warning</option>
              <option value="error">Error</option>
            </select>
            <p class="mt-1 text-xs text-gray-500">Controls the verbosity of log messages</p>
          </div>
          
          <div class="border-t border-gray-200 pt-6">
            <h3 class="text-lg font-medium mb-4">File Logging</h3>
            
            <div class="flex items-center mb-4">
              <input
                id="file_logging_enabled"
                type="checkbox"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                bind:checked={config.logging.file_enabled}
                disabled={!isEditingMode}
              />
              <label for="file_logging_enabled" class="ml-2 block text-sm text-gray-900">
                Enable File Logging
              </label>
            </div>
            
            {#if config.logging.file_enabled}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label for="log_file_path" class="form-label">Log File Path</label>
                  <input
                    id="log_file_path"
                    type="text"
                    class="form-input w-full rounded-md"
                    bind:value={config.logging.file_path}
                    disabled={!isEditingMode}
                  />
                </div>
                
                <div>
                  <label for="max_file_size" class="form-label">Maximum Log File Size</label>
                  <div class="flex items-center">
                    <input
                      id="max_file_size"
                      type="number"
                      class="form-input w-full rounded-l-md"
                      bind:value={config.logging.max_file_size}
                      disabled={!isEditingMode}
                    />
                    <span class="inline-flex items-center px-3 rounded-r-md border border-l-0 border-gray-300 bg-gray-50 text-gray-500 text-sm">
                      bytes
                    </span>
                  </div>
                  <p class="mt-1 text-xs text-gray-500">Current: {formatBytes(config.logging.max_file_size)}</p>
                </div>
                
                <div>
                  <label for="max_files" class="form-label">Maximum Log Files</label>
                  <input
                    id="max_files"
                    type="number"
                    class="form-input w-full rounded-md"
                    bind:value={config.logging.max_files}
                    disabled={!isEditingMode}
                  />
                  <p class="mt-1 text-xs text-gray-500">Number of log files to keep before rotation</p>
                </div>
              </div>
            {/if}
          </div>
          
          <div class="border-t border-gray-200 pt-6">
            <h3 class="text-lg font-medium mb-4">Console Logging</h3>
            
            <div class="flex items-center">
              <input
                id="console_logging_enabled"
                type="checkbox"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                bind:checked={config.logging.console_enabled}
                disabled={!isEditingMode}
              />
              <label for="console_logging_enabled" class="ml-2 block text-sm text-gray-900">
                Enable Console Logging
              </label>
            </div>
            <p class="mt-1 text-xs text-gray-500">Output logs to the console/terminal</p>
          </div>
        </div>
      {/if}
      
      <!-- System Settings -->
      {#if activeTab === 'system'}
        <div class="space-y-6">
          <div class="flex items-center">
            <input
              id="auto_start"
              type="checkbox"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              bind:checked={config.system.auto_start}
              disabled={!isEditingMode}
            />
            <label for="auto_start" class="ml-2 block text-sm text-gray-900">
              Start RCP Service on System Boot
            </label>
          </div>
          
          <div class="border-t border-gray-200 pt-6">
            <h3 class="text-lg font-medium mb-4">Updates</h3>
            
            <div class="flex items-center mb-4">
              <input
                id="update_check"
                type="checkbox"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                bind:checked={config.system.update_check}
                disabled={!isEditingMode}
              />
              <label for="update_check" class="ml-2 block text-sm text-gray-900">
                Automatically Check for Updates
              </label>
            </div>
            
            <div>
              <label for="update_channel" class="form-label">Update Channel</label>
              <select
                id="update_channel"
                class="form-input w-full rounded-md"
                bind:value={config.system.update_channel}
                disabled={!isEditingMode || !config.system.update_check}
              >
                <option value="stable">Stable</option>
                <option value="beta">Beta</option>
                <option value="alpha">Alpha</option>
              </select>
            </div>
          </div>
          
          <div class="border-t border-gray-200 pt-6">
            <h3 class="text-lg font-medium mb-4">Metrics</h3>
            
            <div class="flex items-center mb-4">
              <input
                id="metrics_collection"
                type="checkbox"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                bind:checked={config.system.metrics_collection}
                disabled={!isEditingMode}
              />
              <label for="metrics_collection" class="ml-2 block text-sm text-gray-900">
                Collect Usage Metrics
              </label>
            </div>
            
            {#if config.system.metrics_collection}
              <div>
                <label for="metrics_retention" class="form-label">Metrics Retention (days)</label>
                <input
                  id="metrics_retention"
                  type="number"
                  class="form-input w-full rounded-md"
                  bind:value={config.system.metrics_retention_days}
                  disabled={!isEditingMode}
                />
                <p class="mt-1 text-xs text-gray-500">How long to keep metrics data</p>
              </div>
            {/if}
          </div>
          
          <div class="border-t border-gray-200 pt-6">
            <h3 class="text-lg font-medium mb-4">Maintenance</h3>
            
            <button 
              class="btn-outline text-yellow-600 hover:bg-yellow-50"
              onclick={restartSystem}
              disabled={isRestarting}
            >
              {#if isRestarting}
                <svg class="animate-spin h-5 w-5 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Restarting...
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                </svg>
                Restart RCP Service
              {/if}
            </button>
            <p class="mt-2 text-xs text-gray-500">Restarts the RCP service. All active connections will be terminated.</p>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>