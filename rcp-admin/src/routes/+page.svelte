<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  
  // Admin state
  let username = $state('admin');
  let password = $state('');
  let authenticated = $state(false);
  let activeTab = $state('dashboard');
  let error = $state('');
  
  // Server stats
  let serverStatus = $state<{
    running: boolean;
    uptime: string;
    version: string;
    connections: number;
    cpuUsage: number;
    memoryUsage: number;
  }>({
    running: false,
    uptime: '0',
    version: '0.1.0',
    connections: 0,
    cpuUsage: 0,
    memoryUsage: 0
  });
  
  // Apps management
  let virtualApps = $state<Array<{
    id: string;
    name: string;
    path: string;
    enabled: boolean;
    fileAssociations: string[];
    permissions: string[];
    sessions: number;
  }>>([]);
  
  // Users management
  let users = $state<Array<{
    id: string;
    username: string;
    role: string;
    lastLogin: string;
    active: boolean;
  }>>([]);
  
  // New app form
  let newApp = $state({
    name: '',
    path: '',
    args: '',
    permissions: [] as string[],
  });
  
  // New user form
  let newUser = $state({
    username: '',
    password: '',
    role: 'user'
  });

  onMount(async () => {
    // Check if server is running locally
    try {
      const status = await invoke('get_server_status');
      serverStatus = status;
      
      // If server is running, set up polling for stats
      if (status.running) {
        setInterval(async () => {
          const updatedStats = await invoke('get_server_stats');
          serverStatus = { ...serverStatus, ...updatedStats };
        }, 5000);
      }
    } catch (e) {
      error = e.message;
    }
    
    // Listen for log events
    await listen('server-log', (event) => {
      addLogEntry(event.payload);
    });
  });

  async function login() {
    try {
      const result = await invoke('admin_login', { username, password });
      if (result.success) {
        authenticated = true;
        await loadDashboard();
      }
    } catch (e) {
      error = e.message;
    }
  }
  
  async function loadDashboard() {
    try {
      // Load all dashboard data
      const [apps, usersList, stats] = await Promise.all([
        invoke('get_configured_apps'),
        invoke('get_users'),
        invoke('get_server_stats')
      ]);
      
      virtualApps = apps;
      users = usersList;
      serverStatus = { ...serverStatus, ...stats };
      error = '';
    } catch (e) {
      error = e.message;
    }
  }
  
  async function toggleServer() {
    try {
      if (serverStatus.running) {
        await invoke('stop_server');
        serverStatus.running = false;
      } else {
        await invoke('start_server');
        serverStatus.running = true;
        // Refresh stats after server starts
        setTimeout(async () => {
          const stats = await invoke('get_server_stats');
          serverStatus = { ...serverStatus, ...stats };
        }, 1000);
      }
    } catch (e) {
      error = e.message;
    }
  }
  
  async function addApplication() {
    try {
      await invoke('add_application', { 
        app: {
          name: newApp.name,
          path: newApp.path,
          args: newApp.args.split(' '),
          permissions: newApp.permissions
        }
      });
      
      // Reset form and reload apps
      newApp.name = '';
      newApp.path = '';
      newApp.args = '';
      newApp.permissions = [];
      
      // Reload apps list
      virtualApps = await invoke('get_configured_apps');
    } catch (e) {
      error = e.message;
    }
  }
  
  async function toggleAppStatus(appId: string) {
    try {
      await invoke('toggle_app', { appId });
      
      // Update local state
      virtualApps = virtualApps.map(app => {
        if (app.id === appId) {
          return { ...app, enabled: !app.enabled };
        }
        return app;
      });
    } catch (e) {
      error = e.message;
    }
  }
  
  async function addUser() {
    try {
      await invoke('add_user', newUser);
      
      // Reset form and reload users
      newUser.username = '';
      newUser.password = '';
      newUser.role = 'user';
      
      // Reload users list
      users = await invoke('get_users');
    } catch (e) {
      error = e.message;
    }
  }
  
  async function toggleUserActive(userId: string) {
    try {
      await invoke('toggle_user', { userId });
      
      // Update local state
      users = users.map(user => {
        if (user.id === userId) {
          return { ...user, active: !user.active };
        }
        return user;
      });
    } catch (e) {
      error = e.message;
    }
  }
  
  function addLogEntry(log) {
    // Log handling logic here
  }
  
  function selectTab(tab: string) {
    activeTab = tab;
  }
</script>

<main class="admin-container">
  <h1>RCP Server Admin</h1>

  {#if error}
    <div class="error-banner">
      {error}
      <button on:click={() => error = ''}>✕</button>
    </div>
  {/if}

  {#if !authenticated}
    <div class="login-container">
      <div class="login-form">
        <h2>Admin Login</h2>
        <div class="form-group">
          <label for="username">Username</label>
          <input id="username" type="text" bind:value={username} />
        </div>
        <div class="form-group">
          <label for="password">Password</label>
          <input id="password" type="password" bind:value={password} />
        </div>
        <button class="primary-btn" on:click={login}>Login</button>
      </div>
    </div>
  {:else}
    <div class="admin-panel">
      <nav class="sidebar">
        <ul>
          <li class={activeTab === 'dashboard' ? 'active' : ''}>
            <button on:click={() => selectTab('dashboard')}>Dashboard</button>
          </li>
          <li class={activeTab === 'applications' ? 'active' : ''}>
            <button on:click={() => selectTab('applications')}>Applications</button>
          </li>
          <li class={activeTab === 'users' ? 'active' : ''}>
            <button on:click={() => selectTab('users')}>Users</button>
          </li>
          <li class={activeTab === 'settings' ? 'active' : ''}>
            <button on:click={() => selectTab('settings')}>Settings</button>
          </li>
          <li class={activeTab === 'logs' ? 'active' : ''}>
            <button on:click={() => selectTab('logs')}>Logs</button>
          </li>
        </ul>
      </nav>
      
      <div class="content">
        {#if activeTab === 'dashboard'}
          <div class="dashboard">
            <h2>Dashboard</h2>
            
            <div class="status-card">
              <h3>Server Status</h3>
              <div class="server-status">
                <div class="status-indicator {serverStatus.running ? 'running' : 'stopped'}"></div>
                <span>{serverStatus.running ? 'Running' : 'Stopped'}</span>
                <button class="action-btn" on:click={toggleServer}>
                  {serverStatus.running ? 'Stop Server' : 'Start Server'}
                </button>
              </div>
              
              <div class="stats">
                <div class="stat-item">
                  <span class="stat-label">Version:</span>
                  <span class="stat-value">{serverStatus.version}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Uptime:</span>
                  <span class="stat-value">{serverStatus.uptime}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Active Connections:</span>
                  <span class="stat-value">{serverStatus.connections}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">CPU Usage:</span>
                  <span class="stat-value">{serverStatus.cpuUsage.toFixed(1)}%</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Memory Usage:</span>
                  <span class="stat-value">{serverStatus.memoryUsage.toFixed(1)} MB</span>
                </div>
              </div>
            </div>
            
            <div class="summary-cards">
              <div class="summary-card">
                <h3>Applications</h3>
                <span class="count">{virtualApps.length}</span>
                <button class="text-btn" on:click={() => selectTab('applications')}>Manage</button>
              </div>
              
              <div class="summary-card">
                <h3>Users</h3>
                <span class="count">{users.length}</span>
                <button class="text-btn" on:click={() => selectTab('users')}>Manage</button>
              </div>
              
              <div class="summary-card">
                <h3>Active Sessions</h3>
                <span class="count">{virtualApps.reduce((acc, app) => acc + app.sessions, 0)}</span>
                <button class="text-btn" on:click={() => selectTab('applications')}>View</button>
              </div>
            </div>
          </div>
          
        {:else if activeTab === 'applications'}
          <div class="applications">
            <h2>Applications</h2>
            
            <div class="app-list">
              <table>
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>Path</th>
                    <th>Status</th>
                    <th>Active Sessions</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {#each virtualApps as app}
                    <tr>
                      <td>{app.name}</td>
                      <td class="path-cell">{app.path}</td>
                      <td>
                        <span class="status {app.enabled ? 'enabled' : 'disabled'}">
                          {app.enabled ? 'Enabled' : 'Disabled'}
                        </span>
                      </td>
                      <td>{app.sessions}</td>
                      <td>
                        <button class="icon-btn" on:click={() => toggleAppStatus(app.id)}>
                          {app.enabled ? 'Disable' : 'Enable'}
                        </button>
                        <button class="icon-btn">Edit</button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
            
            <div class="add-form">
              <h3>Add New Application</h3>
              <div class="form-row">
                <div class="form-group">
                  <label for="app-name">Name</label>
                  <input id="app-name" type="text" bind:value={newApp.name} placeholder="e.g. Notepad" />
                </div>
                
                <div class="form-group">
                  <label for="app-path">Path</label>
                  <input id="app-path" type="text" bind:value={newApp.path} placeholder="e.g. /usr/bin/gedit" />
                </div>
              </div>
              
              <div class="form-group">
                <label for="app-args">Arguments (Optional)</label>
                <input id="app-args" type="text" bind:value={newApp.args} placeholder="e.g. --new-window" />
              </div>
              
              <div class="form-group">
                <label>Permissions</label>
                <div class="checkbox-group">
                  <label>
                    <input type="checkbox" value="file-access" 
                           bind:group={newApp.permissions} />
                    File Access
                  </label>
                  <label>
                    <input type="checkbox" value="network" 
                           bind:group={newApp.permissions} />
                    Network Access
                  </label>
                  <label>
                    <input type="checkbox" value="printing" 
                           bind:group={newApp.permissions} />
                    Printing
                  </label>
                </div>
              </div>
              
              <button class="primary-btn" on:click={addApplication}>Add Application</button>
            </div>
          </div>
          
        {:else if activeTab === 'users'}
          <div class="users">
            <h2>Users</h2>
            
            <div class="user-list">
              <table>
                <thead>
                  <tr>
                    <th>Username</th>
                    <th>Role</th>
                    <th>Status</th>
                    <th>Last Login</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {#each users as user}
                    <tr>
                      <td>{user.username}</td>
                      <td>{user.role}</td>
                      <td>
                        <span class="status {user.active ? 'enabled' : 'disabled'}">
                          {user.active ? 'Active' : 'Disabled'}
                        </span>
                      </td>
                      <td>{user.lastLogin || 'Never'}</td>
                      <td>
                        <button class="icon-btn" on:click={() => toggleUserActive(user.id)}>
                          {user.active ? 'Disable' : 'Enable'}
                        </button>
                        <button class="icon-btn">Edit</button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
            
            <div class="add-form">
              <h3>Add New User</h3>
              <div class="form-row">
                <div class="form-group">
                  <label for="user-name">Username</label>
                  <input id="user-name" type="text" bind:value={newUser.username} />
                </div>
                
                <div class="form-group">
                  <label for="user-pass">Password</label>
                  <input id="user-pass" type="password" bind:value={newUser.password} />
                </div>
              </div>
              
              <div class="form-group">
                <label for="user-role">Role</label>
                <select id="user-role" bind:value={newUser.role}>
                  <option value="admin">Administrator</option>
                  <option value="user">Standard User</option>
                  <option value="guest">Guest</option>
                </select>
              </div>
              
              <button class="primary-btn" on:click={addUser}>Add User</button>
            </div>
          </div>
          
        {:else if activeTab === 'settings'}
          <div class="settings">
            <h2>Server Settings</h2>
            <!-- Settings content here -->
          </div>
          
        {:else if activeTab === 'logs'}
          <div class="logs">
            <h2>Server Logs</h2>
            <!-- Logs content here -->
          </div>
        {/if}
      </div>
    </div>
  {/if}
</main>

<style>
  .admin-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  
  h1 {
    padding: 1rem;
    margin: 0;
    background-color: #0066cc;
    color: white;
  }
  
  .error-banner {
    background-color: #f44336;
    color: white;
    padding: 0.75rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .error-banner button {
    background: none;
    border: none;
    color: white;
    font-weight: bold;
    cursor: pointer;
  }
  
  .login-container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-grow: 1;
    background-color: #f5f5f5;
  }
  
  .login-form {
    background-color: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    width: 400px;
  }
  
  .form-group {
    margin-bottom: 1rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  input[type="text"],
  input[type="password"],
  select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }
  
  .primary-btn {
    background-color: #0066cc;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    font-weight: 600;
    cursor: pointer;
    width: 100%;
  }
  
  .admin-panel {
    display: flex;
    flex-grow: 1;
  }
  
  .sidebar {
    width: 250px;
    background-color: #f5f5f5;
    border-right: 1px solid #ddd;
  }
  
  .sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .sidebar li {
    border-bottom: 1px solid #ddd;
  }
  
  .sidebar li.active {
    background-color: #e6f0ff;
    border-left: 4px solid #0066cc;
  }
  
  .sidebar button {
    display: block;
    width: 100%;
    padding: 1rem;
    background: none;
    border: none;
    text-align: left;
    font-size: 1rem;
    cursor: pointer;
  }
  
  .sidebar li.active button {
    font-weight: bold;
    color: #0066cc;
  }
  
  .content {
    flex-grow: 1;
    padding: 2rem;
    background-color: #fff;
    overflow-y: auto;
  }
  
  .status-card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
    margin-bottom: 2rem;
  }
  
  .server-status {
    display: flex;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .status-indicator {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    margin-right: 0.5rem;
  }
  
  .status-indicator.running {
    background-color: #4caf50;
    box-shadow: 0 0 8px rgba(76, 175, 80, 0.6);
  }
  
  .status-indicator.stopped {
    background-color: #f44336;
  }
  
  .action-btn {
    margin-left: auto;
    background-color: #f5f5f5;
    border: 1px solid #ddd;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .stats {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .stat-item {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    background-color: #f9f9f9;
    border-radius: 4px;
  }
  
  .stat-label {
    font-size: 0.9rem;
    color: #666;
  }
  
  .stat-value {
    font-size: 1.2rem;
    font-weight: 600;
    margin-top: 0.25rem;
  }
  
  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1.5rem;
  }
  
  .summary-card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
    text-align: center;
  }
  
  .summary-card .count {
    display: block;
    font-size: 2.5rem;
    font-weight: 700;
    margin: 1rem 0;
    color: #0066cc;
  }
  
  .text-btn {
    background: none;
    border: none;
    color: #0066cc;
    text-decoration: underline;
    cursor: pointer;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 2rem;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  
  th {
    background-color: #f5f5f5;
    font-weight: 600;
  }
  
  .path-cell {
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .status {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.85rem;
  }
  
  .status.enabled {
    background-color: #e6f7e6;
    color: #2e7d32;
  }
  
  .status.disabled {
    background-color: #ffebee;
    color: #c62828;
  }
  
  .icon-btn {
    background: none;
    border: none;
    color: #0066cc;
    cursor: pointer;
    margin-right: 0.5rem;
  }
  
  .add-form {
    background-color: #f9f9f9;
    border-radius: 8px;
    padding: 1.5rem;
  }
  
  .form-row {
    display: flex;
    gap: 1rem;
  }
  
  .form-row .form-group {
    flex: 1;
  }
  
  .checkbox-group {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
  
  .checkbox-group label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
