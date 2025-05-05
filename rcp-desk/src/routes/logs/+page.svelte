<script lang="ts">
  import { onMount } from 'svelte';
  
  // Define log entry type
  interface LogEntry {
    id: string;
    timestamp: string;
    level: 'info' | 'warning' | 'error' | 'debug';
    message: string;
    source: string;
    details?: string;
  }
  
  // Log state
  let logs: LogEntry[] = [];
  let loading = true;
  let error: string | null = null;
  let activeLogId: string | null = null;
  
  // Filter state
  let levelFilter: string = 'all';
  let sourceFilter: string = 'all';
  let searchQuery: string = '';
  let startDate: string = '';
  let endDate: string = '';
  
  // Pagination
  let currentPage = 1;
  let totalPages = 1;
  let logsPerPage = 50;
  
  // Get available log sources
  const logSources = ['server', 'client', 'database', 'auth', 'system'];
  
  // Get logs from API
  async function fetchLogs() {
    try {
      loading = true;
      
      // This is a placeholder - in a real application, you would call your API
      // For now, we'll use mock data
      setTimeout(() => {
        logs = generateMockLogs();
        totalPages = Math.ceil(logs.length / logsPerPage);
        loading = false;
      }, 500);
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'An error occurred while fetching logs';
      loading = false;
    }
  }
  
  // Generate mock log data
  function generateMockLogs(): LogEntry[] {
    const mockLogs: LogEntry[] = [];
    const levels = ['info', 'warning', 'error', 'debug'];
    const messages = [
      'User logged in successfully',
      'Failed login attempt',
      'Database connection established',
      'Request processed successfully',
      'Invalid request parameters',
      'Resource not found',
      'Operation timed out',
      'Configuration loaded',
      'Session expired',
      'Data validation failed'
    ];
    
    // Generate 100 random logs
    for (let i = 0; i < 100; i++) {
      const date = new Date();
      date.setMinutes(date.getMinutes() - i * 30); // Space out the timestamps
      
      mockLogs.push({
        id: `log-${i}`,
        timestamp: date.toISOString(),
        level: levels[Math.floor(Math.random() * levels.length)] as 'info' | 'warning' | 'error' | 'debug',
        message: messages[Math.floor(Math.random() * messages.length)],
        source: logSources[Math.floor(Math.random() * logSources.length)],
        details: `Detailed information for log entry ${i}.\nThis would contain stack traces, request data, or other context-specific information.`
      });
    }
    
    return mockLogs;
  }
  
  // Apply filters to logs
  $: filteredLogs = logs.filter(log => {
    // Filter by level
    if (levelFilter !== 'all' && log.level !== levelFilter) return false;
    
    // Filter by source
    if (sourceFilter !== 'all' && log.source !== sourceFilter) return false;
    
    // Filter by search query
    if (searchQuery && !log.message.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    
    // Filter by date range
    if (startDate) {
      const logDate = new Date(log.timestamp);
      const filterStartDate = new Date(startDate);
      if (logDate < filterStartDate) return false;
    }
    
    if (endDate) {
      const logDate = new Date(log.timestamp);
      const filterEndDate = new Date(endDate);
      filterEndDate.setHours(23, 59, 59, 999); // End of day
      if (logDate > filterEndDate) return false;
    }
    
    return true;
  });
  
  // Get paginated logs
  $: paginatedLogs = filteredLogs.slice(
    (currentPage - 1) * logsPerPage, 
    currentPage * logsPerPage
  );
  
  // Handle pagination
  function goToPage(page: number) {
    if (page < 1) page = 1;
    if (page > totalPages) page = totalPages;
    currentPage = page;
    
    // Scroll to top when changing page
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
  
  // Show log details
  function toggleLogDetails(logId: string) {
    if (activeLogId === logId) {
      activeLogId = null;
    } else {
      activeLogId = logId;
    }
  }
  
  // Reset filters
  function resetFilters() {
    levelFilter = 'all';
    sourceFilter = 'all';
    searchQuery = '';
    startDate = '';
    endDate = '';
  }
  
  // Format timestamp for display
  function formatTimestamp(isoString: string): string {
    const date = new Date(isoString);
    return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`;
  }
  
  // Get CSS class for log level
  function getLevelClass(level: string): string {
    switch (level) {
      case 'info': return 'bg-blue-100 text-blue-800';
      case 'warning': return 'bg-yellow-100 text-yellow-800';
      case 'error': return 'bg-red-100 text-red-800';
      case 'debug': return 'bg-gray-100 text-gray-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  }
  
  // Fetch logs when component mounts
  onMount(() => {
    fetchLogs();
  });
</script>

<svelte:head>
  <title>Logs - RCP Desk</title>
</svelte:head>

<div class="container mx-auto py-6 px-4">
  <h1 class="text-2xl font-bold text-gray-800 mb-6">System Logs</h1>
  
  <!-- Filters -->
  <div class="bg-white rounded-lg shadow-md p-4 mb-6">
    <h2 class="text-lg font-medium mb-4">Filters</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-4">
      <div>
        <label for="levelFilter" class="block text-sm font-medium text-gray-700 mb-1">Log Level</label>
        <select
          id="levelFilter"
          bind:value={levelFilter}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">All Levels</option>
          <option value="info">Info</option>
          <option value="warning">Warning</option>
          <option value="error">Error</option>
          <option value="debug">Debug</option>
        </select>
      </div>
      
      <div>
        <label for="sourceFilter" class="block text-sm font-medium text-gray-700 mb-1">Source</label>
        <select
          id="sourceFilter"
          bind:value={sourceFilter}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        >
          <option value="all">All Sources</option>
          {#each logSources as source}
            <option value={source}>{source.charAt(0).toUpperCase() + source.slice(1)}</option>
          {/each}
        </select>
      </div>
      
      <div>
        <label for="startDate" class="block text-sm font-medium text-gray-700 mb-1">Start Date</label>
        <input
          id="startDate"
          type="date"
          bind:value={startDate}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        />
      </div>
      
      <div>
        <label for="endDate" class="block text-sm font-medium text-gray-700 mb-1">End Date</label>
        <input
          id="endDate"
          type="date"
          bind:value={endDate}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        />
      </div>
    </div>
    
    <div class="flex flex-col sm:flex-row gap-4">
      <div class="flex-grow">
        <label for="searchQuery" class="block text-sm font-medium text-gray-700 mb-1">Search</label>
        <input
          id="searchQuery"
          type="text"
          placeholder="Search log messages..."
          bind:value={searchQuery}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
        />
      </div>
      
      <div class="flex items-end">
        <button
          on:click={resetFilters}
          class="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
        >
          Reset Filters
        </button>
      </div>
    </div>
  </div>
  
  <!-- Log Table -->
  <div class="bg-white rounded-lg shadow-md overflow-hidden">
    {#if loading}
      <div class="p-8 text-center">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600 mb-2"></div>
        <p class="text-gray-600">Loading logs...</p>
      </div>
    {:else if error}
      <div class="p-8 text-center">
        <div class="bg-red-50 border-l-4 border-red-500 p-4 rounded">
          <p class="text-red-700">{error}</p>
        </div>
      </div>
    {:else if filteredLogs.length === 0}
      <div class="p-8 text-center">
        <p class="text-gray-600">No logs found matching the current filters.</p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Timestamp</th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Level</th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Source</th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Message</th>
              <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each paginatedLogs as log (log.id)}
              <tr class="hover:bg-gray-50">
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                  {formatTimestamp(log.timestamp)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm">
                  <span class={`px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full ${getLevelClass(log.level)}`}>
                    {log.level.toUpperCase()}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                  {log.source}
                </td>
                <td class="px-6 py-4 text-sm text-gray-800">
                  {log.message}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <button
                    on:click={() => toggleLogDetails(log.id)}
                    class="text-primary-600 hover:text-primary-800 focus:outline-none"
                  >
                    {activeLogId === log.id ? 'Hide Details' : 'View Details'}
                  </button>
                </td>
              </tr>
              
              {#if activeLogId === log.id}
                <tr class="bg-gray-50">
                  <td colspan="5" class="px-6 py-4">
                    <div class="text-sm">
                      <h4 class="font-medium text-gray-800 mb-2">Details</h4>
                      <pre class="bg-gray-100 p-3 rounded-md overflow-x-auto whitespace-pre-wrap">{log.details || 'No additional details available.'}</pre>
                    </div>
                  </td>
                </tr>
              {/if}
            {/each}
          </tbody>
        </table>
      </div>
      
      <!-- Pagination -->
      {#if filteredLogs.length > logsPerPage}
        <div class="px-6 py-4 flex items-center justify-between border-t border-gray-200">
          <div class="text-sm text-gray-700">
            Showing <span class="font-medium">{(currentPage - 1) * logsPerPage + 1}</span> to <span class="font-medium">{Math.min(currentPage * logsPerPage, filteredLogs.length)}</span> of <span class="font-medium">{filteredLogs.length}</span> results
          </div>
          
          <div class="flex items-center space-x-2">
            <button
              on:click={() => goToPage(currentPage - 1)}
              disabled={currentPage === 1}
              class="px-3 py-1 rounded-md text-sm font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Previous
            </button>
            
            {#if totalPages <= 5}
              {#each Array(totalPages) as _, i}
                <button
                  on:click={() => goToPage(i + 1)}
                  class={`px-3 py-1 rounded-md text-sm font-medium ${currentPage === i + 1 ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500`}
                >
                  {i + 1}
                </button>
              {/each}
            {:else}
              <!-- First page -->
              <button
                on:click={() => goToPage(1)}
                class={`px-3 py-1 rounded-md text-sm font-medium ${currentPage === 1 ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500`}
              >
                1
              </button>
              
              <!-- Page range ellipsis -->
              {#if currentPage > 3}
                <span class="text-gray-500">...</span>
              {/if}
              
              <!-- Middle pages -->
              {#each Array(3) as _, i}
                {#if currentPage > 2 && currentPage < totalPages - 1 && i + currentPage - 1 > 1 && i + currentPage - 1 < totalPages}
                  <button
                    on:click={() => goToPage(i + currentPage - 1)}
                    class={`px-3 py-1 rounded-md text-sm font-medium ${currentPage === i + currentPage - 1 ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500`}
                  >
                    {i + currentPage - 1}
                  </button>
                {/if}
              {/each}
              
              <!-- Page range ellipsis -->
              {#if currentPage < totalPages - 2}
                <span class="text-gray-500">...</span>
              {/if}
              
              <!-- Last page -->
              <button
                on:click={() => goToPage(totalPages)}
                class={`px-3 py-1 rounded-md text-sm font-medium ${currentPage === totalPages ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500`}
              >
                {totalPages}
              </button>
            {/if}
            
            <button
              on:click={() => goToPage(currentPage + 1)}
              disabled={currentPage === totalPages}
              class="px-3 py-1 rounded-md text-sm font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Next
            </button>
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>