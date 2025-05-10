<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  // App state using Svelte stores
  const host = writable('localhost');
  const port = writable(8716);
  const username = writable('');
  const password = writable('');
  const rememberCredentials = writable(false);
  const connected = writable(false);
  const authenticated = writable(false);
  const error = writable('');
  
  // Application display
  const apps = writable<{
    id: string;
    name: string;
    icon: string;
    description: string;
    lastUsed: Date | null;
  }[]>([]);
  
  // Active application
  const activeAppId = writable<string | null>(null);
  const appStreaming = writable(false);
  
  // Create helper variables to use the stores
  let $host, $port, $username, $password, $rememberCredentials, $connected, $authenticated, $error, $apps, $activeAppId, $appStreaming;
  $: $host = $host;
  $: $port = $port;
  $: $username = $username;
  $: $password = $password;
  $: $rememberCredentials = $rememberCredentials;
  $: $connected = $connected;
  $: $authenticated = $authenticated;
  $: $error = $error;
  $: $apps = $apps;
  $: $activeAppId = $activeAppId;
  $: $appStreaming = $appStreaming;
  
  // Initialize connection events
  onMount(async () => {
    // Listen for connection state changes
    await listen('connection-state', (event: any) => {
      connected.set(event.payload.connected);
      error.set(event.payload.error || '');
    });
    
    // Listen for stream events
    await listen('app-stream', (event: any) => {
      if (event.payload.type === 'frame') {
        updateAppFrame(event.payload.frame);
      }
    });
    
    // Check for saved credentials
    const savedCredentials = await invoke('get_saved_credentials');
    if (savedCredentials) {
      host.set(savedCredentials.host);
      port.set(savedCredentials.port);
      username.set(savedCredentials.username);
      rememberCredentials.set(true);
    }
  });
  });
  
  async function connect() {
    try {
      await invoke('connect', { host: $host, port: $port });
    } catch (e: any) {
      error.set(e.message);
    }
  }
  
  async function login() {
    try {
      const result = await invoke('login', { 
        username: $username, 
        password: $password,
        rememberCredentials: $rememberCredentials 
      });
      
      if (result.success) {
        authenticated.set(true);
        await loadApps();
      }
    } catch (e: any) {
      error.set(e.message);
    }
  }
  
  async function loadApps() {
    try {
      apps.set(await invoke('get_available_apps'));
    } catch (e: any) {
      error.set(e.message);
    }
  }
  
  async function launchApp(appId: string) {
    try {
      activeAppId.set(appId);
      appStreaming.set(true);
      await invoke('launch_app', { appId });
    } catch (e: any) {
      error.set(e.message);
      activeAppId.set(null);
      appStreaming.set(false);
    }
  }
  
  function updateAppFrame(frameData: string) {
    const streamCanvas = document.getElementById('app-stream') as HTMLCanvasElement;
    if (!streamCanvas) return;
    
    const ctx = streamCanvas.getContext('2d');
    if (!ctx) return;
    
    const img = new Image();
    img.onload = () => {
      ctx.drawImage(img, 0, 0, streamCanvas.width, streamCanvas.height);
    };
    img.src = `data:image/jpeg;base64,${frameData}`;
  }
  
  function closeApp() {
    invoke('close_app', { appId: $activeAppId });
    activeAppId.set(null);
    appStreaming.set(false);
  }
</script>

<main class="app-container">
  {#if $error}
    <div class="error-message">
      <span>{$error}</span>
      <button on:click={() => error.set('')}>✕</button>
    </div>
  {/if}
  
  {#if !$connected}
    <div class="connect-panel">
      <h1>RCP Client</h1>
      <div class="form-group">
        <label for="host">Server</label>
        <input id="host" type="text" bind:value={$host} />
      </div>
      <div class="form-group">
        <label for="port">Port</label>
        <input id="port" type="number" bind:value={$port} />
      </div>
      <button class="primary" on:click={connect}>Connect</button>
    </div>
    
  {:else if !$authenticated}
    <div class="login-panel">
      <h1>Log In</h1>
      <div class="form-group">
        <label for="username">Username</label>
        <input id="username" type="text" bind:value={$username} />
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input id="password" type="password" bind:value={$password} />
      </div>
      <div class="form-check">
        <input id="remember" type="checkbox" bind:checked={$rememberCredentials} />
        <label for="remember">Remember credentials</label>
      </div>
      <button class="primary" on:click={login}>Log In</button>
    </div>
    
  {:else if $appStreaming}
    <div class="app-streaming">
      <div class="stream-header">
        <h2>{$apps.find(a => a.id === $activeAppId)?.name || 'Application'}</h2>
        <button on:click={closeApp}>Close</button>
      </div>
      <div class="stream-container">
        <canvas id="app-stream" width="1280" height="720"></canvas>
      </div>
    </div>
    
  {:else}
    <div class="app-launcher">
      <div class="header">
        <h1>Applications</h1>
        <div class="user-info">
          <span>{$username}</span>
          <button on:click={() => authenticated.set(false)}>Log Out</button>
        </div>
      </div>
      
      <div class="app-grid">
        {#each $apps as app}
          <div class="app-card" on:click={() => launchApp(app.id)}>
            <img src={app.icon || '/app-icon.png'} alt={app.name} />
            <h3>{app.name}</h3>
            <p>{app.description}</p>
            {#if app.lastUsed}
              <span class="last-used">Last used: {app.lastUsed.toLocaleDateString()}</span>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</main>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 0;
    margin: 0;
    background-color: #f5f5f5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  
  .connect-panel, .login-panel {
    max-width: 400px;
    margin: 100px auto;
    padding: 2rem;
    background-color: #ffffff;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  .form-group {
    margin-bottom: 1rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .form-check {
    display: flex;
    align-items: center;
    margin: 1rem 0;
  }
  
  .form-check label {
    margin-left: 0.5rem;
  }
  
  input[type="text"],
  input[type="password"],
  input[type="number"] {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }
  
  button {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-weight: 600;
    cursor: pointer;
  }
  
  button.primary {
    background-color: #0066cc;
    color: white;
    width: 100%;
    margin-top: 1rem;
  }
  
  button.primary:hover {
    background-color: #0052a3;
  }
  
  .error-message {
    position: fixed;
    top: 1rem;
    left: 50%;
    transform: translateX(-50%);
    background-color: #f44336;
    color: white;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    display: flex;
    align-items: center;
    gap: 1rem;
    z-index: 1000;
  }
  
  .error-message button {
    background: none;
    border: none;
    color: white;
    font-weight: bold;
    padding: 0 0.5rem;
  }
  
  .app-launcher {
    padding: 2rem;
    height: 100%;
  }
  
  .app-launcher .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .user-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .app-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 2rem;
  }
  
  .app-card {
    background-color: white;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s, box-shadow 0.2s;
    cursor: pointer;
  }
  
  .app-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
  
  .app-card img {
    width: 100%;
    height: 160px;
    object-fit: cover;
  }
  
  .app-card h3 {
    margin: 1rem;
    font-size: 1.2rem;
  }
  
  .app-card p {
    margin: 0 1rem 1rem;
    color: #666;
    font-size: 0.9rem;
  }
  
  .app-card .last-used {
    display: block;
    margin: 0 1rem 1rem;
    font-size: 0.8rem;
    color: #999;
  }
  
  .app-streaming {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }
  
  .stream-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: #f5f5f5;
    border-bottom: 1px solid #ddd;
  }
  
  .stream-container {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #222;
  }
  
  #app-stream {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }
</style>
