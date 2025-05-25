/**
 * API functions for communicating with the RCP daemon
 */

// Base URL for API requests
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';

// Type definitions
export interface ApiResponse<T> {
  data?: T;
  error?: {
    code: string;
    message: string;
  };
}

export interface ServerStatus {
  status: 'online' | 'offline' | 'degraded';
  version: string;
  uptime: number;
  cpu_usage: number;
  memory_usage: number;
  connections_active: number;
}

export interface SessionData {
  id: string;
  client_name: string;
  client_address: string;
  client_version?: string;
  authenticated: boolean;
  connected_at: string;
  last_activity?: string;
  application?: string;
}

export interface ApplicationData {
  id: string;
  name: string;
  description: string;
  type: string;
  command: string;
  working_directory?: string;
  version?: string;
  icon?: string;
  created_at: string;
  updated_at: string;
  status: 'active' | 'inactive';
  launch_args?: string[];
  env_vars?: Record<string, string>;
}

export interface SystemMetrics {
  cpu_usage: number;
  memory_usage: number;
  uptime: number;
  disk_usage: number;
  connections_active: number;
  connections_total: number;
  connections_peak: number;
  requests_per_minute: number;
}

/**
 * Generic API fetcher with error handling
 */
async function apiFetch<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  try {
    const url = `${API_BASE_URL}${endpoint.startsWith('/') ? endpoint : `/${endpoint}`}`;
    
    const defaultHeaders: HeadersInit = {
      'Content-Type': 'application/json',
    };
    
    const response = await fetch(url, {
      ...options,
      headers: {
        ...defaultHeaders,
        ...options.headers,
      },
    });
    
    const data = await response.json();
    
    if (!response.ok) {
      return {
        error: {
          code: data.code || `${response.status}`,
          message: data.message || 'An error occurred while fetching data',
        },
      };
    }
    
    return { data };
  } catch (error) {
    console.error('API fetch error:', error);
    return {
      error: {
        code: 'client_error',
        message: error instanceof Error ? error.message : 'An unknown error occurred',
      },
    };
  }
}

/**
 * Get RCP daemon server status
 */
export async function getServerStatus(): Promise<ApiResponse<ServerStatus>> {
  return apiFetch<ServerStatus>('/status');
}

/**
 * Get all active sessions
 */
export async function getSessions(): Promise<ApiResponse<SessionData[]>> {
  return apiFetch<SessionData[]>('/sessions');
}

/**
 * Get a specific session by ID
 */
export async function getSession(id: string): Promise<ApiResponse<SessionData>> {
  return apiFetch<SessionData>(`/sessions/${id}`);
}

/**
 * Close a client session
 */
export async function closeSession(id: string): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>(`/sessions/${id}`, {
    method: 'DELETE',
  });
}

/**
 * Get all registered applications
 */
export async function getApplications(): Promise<ApiResponse<ApplicationData[]>> {
  return apiFetch<ApplicationData[]>('/applications');
}

/**
 * Get a specific application by ID
 */
export async function getApplication(id: string): Promise<ApiResponse<ApplicationData>> {
  return apiFetch<ApplicationData>(`/applications/${id}`);
}

/**
 * Create a new application
 */
export async function createApplication(data: Omit<ApplicationData, 'id' | 'created_at' | 'updated_at'>): Promise<ApiResponse<ApplicationData>> {
  return apiFetch<ApplicationData>('/applications', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

/**
 * Update an existing application
 */
export async function updateApplication(id: string, data: Partial<ApplicationData>): Promise<ApiResponse<ApplicationData>> {
  return apiFetch<ApplicationData>(`/applications/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
}

/**
 * Delete an application
 */
export async function deleteApplication(id: string): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>(`/applications/${id}`, {
    method: 'DELETE',
  });
}

/**
 * Launch an application on a specific client
 */
export async function launchApplication(applicationId: string, sessionId: string, options?: { args?: string[] }): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>(`/applications/${applicationId}/launch`, {
    method: 'POST',
    body: JSON.stringify({
      session_id: sessionId,
      ...options,
    }),
  });
}

/**
 * Get system metrics
 */
export async function getSystemMetrics(): Promise<ApiResponse<SystemMetrics>> {
  return apiFetch<SystemMetrics>('/metrics');
}

/**
 * Update server configuration
 */
export async function updateServerConfig(config: Record<string, any>): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>('/config/server', {
    method: 'PUT',
    body: JSON.stringify(config),
  });
}

/**
 * Update security configuration
 */
export async function updateSecurityConfig(config: Record<string, any>): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>('/config/security', {
    method: 'PUT',
    body: JSON.stringify(config),
  });
}

/**
 * Update logging configuration
 */
export async function updateLoggingConfig(config: Record<string, any>): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>('/config/logging', {
    method: 'PUT',
    body: JSON.stringify(config),
  });
}

/**
 * Restart the RCP daemon
 */
export async function restartServer(): Promise<ApiResponse<{ success: boolean }>> {
  return apiFetch<{ success: boolean }>('/admin/restart', {
    method: 'POST',
  });
}

/**
 * Download server logs
 */
export async function downloadLogs(): Promise<Blob> {
  const url = `${API_BASE_URL}/admin/logs/download`;
  const response = await fetch(url);
  
  if (!response.ok) {
    throw new Error('Failed to download logs');
  }
  
  return response.blob();
}