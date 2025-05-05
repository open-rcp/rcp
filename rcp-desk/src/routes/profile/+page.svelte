<script lang="ts">
  import Gravatar from '$components/Gravatar.svelte';
  import { authStore } from '$stores/auth';
</script>

<div class="container mx-auto px-4 py-8">
  <div class="max-w-2xl mx-auto bg-white rounded-lg shadow-md overflow-hidden">
    <div class="p-6">
      <div class="flex flex-col items-center mb-6">
        <Gravatar 
          email={$authStore?.user?.email || ''} 
          size={128} 
          defaultImage="identicon" 
          className="rounded-full mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-800">
          {$authStore?.user?.name || 'User Profile'}
        </h1>
        <p class="text-gray-600">{$authStore?.user?.email || 'No email available'}</p>
      </div>
      
      <div class="border-t border-gray-200 pt-4">
        <h2 class="text-xl font-semibold mb-4">Profile Information</h2>
        
        <div class="space-y-3">
          <div class="flex flex-col md:flex-row md:items-center">
            <span class="text-gray-600 md:w-1/4">User ID:</span>
            <span class="font-medium">{$authStore?.user?.id || 'Not available'}</span>
          </div>
          
          <div class="flex flex-col md:flex-row md:items-center">
            <span class="text-gray-600 md:w-1/4">Roles:</span>
            <div class="flex flex-wrap gap-2">
              {#if $authStore?.user?.roles && $authStore.user.roles.length > 0}
                {#each $authStore.user.roles as role}
                  <span class="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded">{role}</span>
                {/each}
              {:else}
                <span class="text-gray-500">No roles assigned</span>
              {/if}
            </div>
          </div>
          
          <div class="flex flex-col md:flex-row md:items-center">
            <span class="text-gray-600 md:w-1/4">Account Status:</span>
            <span class="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded">Active</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>