<script lang="ts">
  import { onMount } from 'svelte';
  
  // Define the user interface
  interface User {
    id: string;
    name: string;
    email: string;
    roles: string[];
    created_at: string;
    last_login: string | null;
    is_active: boolean;
    password?: string; // Optional password field for editing
  }

  interface NewUser {
    id: string;
    name: string;
    email: string;
    password: string;
    roles: string[];
  }

  interface Role {
    id: string;
    name: string;
    description: string;
  }
  
  // Mock data for users
  let users = $state<User[]>([
    {
      id: "admin",
      name: "Administrator",
      email: "admin@example.com",
      roles: ["admin"],
      created_at: "2025-01-01T00:00:00Z",
      last_login: "2025-05-04T13:15:30Z",
      is_active: true
    },
    {
      id: "user1",
      name: "Regular User",
      email: "user1@example.com",
      roles: ["user"],
      created_at: "2025-03-15T14:30:00Z",
      last_login: "2025-05-03T09:45:12Z",
      is_active: true
    },
    {
      id: "operator",
      name: "System Operator",
      email: "operator@example.com",
      roles: ["operator"],
      created_at: "2025-02-10T08:20:15Z",
      last_login: "2025-05-02T16:30:45Z",
      is_active: true
    },
    {
      id: "guest",
      name: "Guest User",
      email: "guest@example.com",
      roles: ["guest"],
      created_at: "2025-04-20T11:25:30Z",
      last_login: "2025-04-25T14:15:20Z",
      is_active: false
    }
  ]);
  
  let isAddUserModalOpen = $state(false);
  let isEditUserModalOpen = $state(false);
  let editingUser = $state<User | null>(null);
  
  let newUser = $state<NewUser>({
    id: "",
    name: "",
    email: "",
    password: "",
    roles: ["user"]
  });
  
  // Available roles
  const availableRoles: Role[] = [
    { id: "admin", name: "Administrator", description: "Full system access" },
    { id: "operator", name: "Operator", description: "Can manage servers and sessions" },
    { id: "user", name: "User", description: "Standard user permissions" },
    { id: "guest", name: "Guest", description: "Limited read-only access" }
  ];
  
  // Format relative time (e.g., "2 days ago")
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
  
  function toggleAddUserModal(): void {
    isAddUserModalOpen = !isAddUserModalOpen;
    
    if (!isAddUserModalOpen) {
      // Reset form when closing
      newUser = {
        id: "",
        name: "",
        email: "",
        password: "",
        roles: ["user"]
      };
    }
  }
  
  function addUser(): void {
    // In a real application, this would call the RCP API to create a user
    users = [...users, {
      id: newUser.id,
      name: newUser.name,
      email: newUser.email,
      roles: [...newUser.roles],
      created_at: new Date().toISOString(),
      last_login: null,
      is_active: true
    }];
    
    // Close modal
    toggleAddUserModal();
  }
  
  function openEditUserModal(user: User): void {
    editingUser = { ...user, password: "" };
    isEditUserModalOpen = true;
  }
  
  function closeEditUserModal(): void {
    editingUser = null;
    isEditUserModalOpen = false;
  }
  
  function updateUser(): void {
    if (!editingUser) return;
    
    // In a real application, this would call the RCP API to update the user
    users = users.map(user => {
      if (user.id === editingUser?.id) {
        return { 
          ...user,
          name: editingUser.name,
          email: editingUser.email,
          roles: [...editingUser.roles],
          is_active: editingUser.is_active
        };
      }
      return user;
    });
    
    // Close modal
    closeEditUserModal();
  }
  
  function toggleUserStatus(userId: string): void {
    // In a real application, this would call the RCP API to update user status
    users = users.map(user => {
      if (user.id === userId) {
        return { ...user, is_active: !user.is_active };
      }
      return user;
    });
  }
  
  function deleteUser(userId: string): void {
    if (confirm("Are you sure you want to delete this user?")) {
      // In a real application, this would call the RCP API to delete the user
      users = users.filter(user => user.id !== userId);
    }
  }
  
  function toggleRole(roles: string[], role: string): string[] {
    if (roles.includes(role)) {
      return roles.filter(r => r !== role);
    } else {
      return [...roles, role];
    }
  }
  
  onMount(() => {
    // In a real app, we would fetch users from the API here
  });
</script>

<svelte:head>
  <title>User Management | RCP Desk</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-800">User Management</h1>
    <div>
      <button 
        class="btn-primary flex items-center" 
        onclick={toggleAddUserModal}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
        </svg>
        Add User
      </button>
    </div>
  </div>
  
  <!-- Users Table -->
  <div class="card overflow-hidden">
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead>
          <tr>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">User</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ID</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Role</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Created</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Last Login</th>
            <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
            <th class="px-6 py-3 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          {#each users as user}
            <tr>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm font-medium text-gray-900">{user.name}</div>
                <div class="text-sm text-gray-500">{user.email}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {user.id}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                {#each user.roles as role, i}
                  <span class={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                    ${role === 'admin' ? 'bg-red-100 text-red-800' : ''}
                    ${role === 'operator' ? 'bg-blue-100 text-blue-800' : ''}
                    ${role === 'user' ? 'bg-green-100 text-green-800' : ''}
                    ${role === 'guest' ? 'bg-yellow-100 text-yellow-800' : ''}
                  `}>
                    {role}
                  </span>
                  {#if i < user.roles.length - 1} &nbsp; {/if}
                {/each}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {timeAgo(user.created_at)}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {user.last_login ? timeAgo(user.last_login) : 'Never'}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                {#if user.is_active}
                  <span class="status-active">Active</span>
                {:else}
                  <span class="status-inactive">Inactive</span>
                {/if}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <button 
                  onclick={() => openEditUserModal(user)} 
                  class="text-primary-600 hover:text-primary-900 mr-3"
                >
                  Edit
                </button>
                <button 
                  onclick={() => toggleUserStatus(user.id)} 
                  class={user.is_active ? "text-yellow-600 hover:text-yellow-900 mr-3" : "text-green-600 hover:text-green-900 mr-3"}
                >
                  {user.is_active ? 'Deactivate' : 'Activate'}
                </button>
                {#if user.id !== 'admin'}
                  <button 
                    onclick={() => deleteUser(user.id)} 
                    class="text-red-600 hover:text-red-900"
                  >
                    Delete
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    
    {#if users.length === 0}
      <div class="py-10 text-center">
        <p class="text-gray-500">No users found</p>
      </div>
    {/if}
  </div>
  
  <!-- Role Descriptions -->
  <div class="card">
    <h2 class="text-lg font-semibold mb-4">User Roles</h2>
    <div class="space-y-3">
      {#each availableRoles as role}
        <div class="flex items-start">
          <div class={`px-2 py-0.5 text-xs leading-5 font-semibold rounded-full mr-2
            ${role.id === 'admin' ? 'bg-red-100 text-red-800' : ''}
            ${role.id === 'operator' ? 'bg-blue-100 text-blue-800' : ''}
            ${role.id === 'user' ? 'bg-green-100 text-green-800' : ''}
            ${role.id === 'guest' ? 'bg-yellow-100 text-yellow-800' : ''}
          `}>
            {role.id}
          </div>
          <div>
            <div class="font-medium">{role.name}</div>
            <div class="text-sm text-gray-500">{role.description}</div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Add User Modal -->
{#if isAddUserModalOpen}
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-medium">Add New User</h3>
        <button class="text-gray-400 hover:text-gray-600" onclick={toggleAddUserModal} aria-label="Close dialog">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <div class="space-y-4">
        <div>
          <label for="user-id" class="form-label">Username</label>
          <input
            id="user-id"
            type="text"
            class="form-input w-full rounded-md"
            placeholder="username"
            bind:value={newUser.id}
          />
        </div>
        
        <div>
          <label for="user-name" class="form-label">Full Name</label>
          <input
            id="user-name"
            type="text"
            class="form-input w-full rounded-md"
            placeholder="John Doe"
            bind:value={newUser.name}
          />
        </div>
        
        <div>
          <label for="user-email" class="form-label">Email</label>
          <input
            id="user-email"
            type="email"
            class="form-input w-full rounded-md"
            placeholder="john@example.com"
            bind:value={newUser.email}
          />
        </div>
        
        <div>
          <label for="user-password" class="form-label">Password</label>
          <input
            id="user-password"
            type="password"
            class="form-input w-full rounded-md"
            placeholder="••••••••"
            bind:value={newUser.password}
          />
        </div>
        
        <div>
          <label id="add-user-roles-label" class="form-label" for="add-user-roles">Roles</label>
          <div id="add-user-roles" class="mt-2 space-y-2" aria-labelledby="add-user-roles-label">
            {#each availableRoles as role}
              <div class="flex items-center">
                <input 
                  id={`role-${role.id}`}
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  checked={newUser.roles.includes(role.id)}
                  onchange={() => newUser.roles = toggleRole(newUser.roles, role.id)}
                />
                <label for={`role-${role.id}`} class="ml-2 text-sm text-gray-900">
                  {role.name}
                </label>
              </div>
            {/each}
          </div>
        </div>
      </div>
      
      <div class="mt-6 flex justify-end space-x-2">
        <button class="btn-outline" onclick={toggleAddUserModal}>
          Cancel
        </button>
        <button class="btn-primary" onclick={addUser}>
          Add User
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit User Modal -->
{#if isEditUserModalOpen && editingUser}
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-medium">Edit User: {editingUser.id}</h3>
        <button class="text-gray-400 hover:text-gray-600" onclick={closeEditUserModal} aria-label="Close dialog">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <div class="space-y-4">
        <div>
          <label for="edit-user-name" class="form-label">Full Name</label>
          <input
            id="edit-user-name"
            type="text"
            class="form-input w-full rounded-md"
            bind:value={editingUser.name}
          />
        </div>
        
        <div>
          <label for="edit-user-email" class="form-label">Email</label>
          <input
            id="edit-user-email"
            type="email"
            class="form-input w-full rounded-md"
            bind:value={editingUser.email}
          />
        </div>
        
        <div>
          <label for="edit-user-password" class="form-label">New Password (leave blank to keep current)</label>
          <input
            id="edit-user-password"
            type="password"
            class="form-input w-full rounded-md"
            placeholder="••••••••"
            bind:value={editingUser.password}
          />
        </div>
        
        <div class="flex items-center">
          <input
            id="edit-user-active"
            type="checkbox"
            class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            bind:checked={editingUser.is_active}
          />
          <label for="edit-user-active" class="ml-2 text-sm text-gray-900">
            Active
          </label>
        </div>
        
        <div>
          <label id="edit-user-roles-label" class="form-label" for="edit-user-roles">Roles</label>
          <div id="edit-user-roles" class="mt-2 space-y-2" aria-labelledby="edit-user-roles-label">
            {#each availableRoles as role}
              <div class="flex items-center">
                <input 
                  id={`edit-role-${role.id}`}
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  checked={editingUser?.roles.includes(role.id)}
                  onchange={() => {
                    if (editingUser) {
                      editingUser.roles = toggleRole(editingUser.roles, role.id);
                    }
                  }}
                  disabled={editingUser?.id === 'admin' && role.id === 'admin'} 
                />
                <label for={`edit-role-${role.id}`} class="ml-2 text-sm text-gray-900">
                  {role.name}
                </label>
              </div>
            {/each}
          </div>
        </div>
      </div>
      
      <div class="mt-6 flex justify-end space-x-2">
        <button class="btn-outline" onclick={closeEditUserModal}>
          Cancel
        </button>
        <button class="btn-primary" onclick={updateUser}>
          Update User
        </button>
      </div>
    </div>
  </div>
{/if}