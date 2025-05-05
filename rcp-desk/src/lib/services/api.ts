// Base API service for making HTTP requests to the backend
import { authStore } from '../stores/auth';
import { get } from 'svelte/store';

// API configuration
const API_BASE_URL = import.meta.env.VITE_API_URL || '/api/v1';

// Types
interface RequestOptions {
  method?: string;
  headers?: Record<string, string>;
  body?: any;
  requiresAuth?: boolean;
  retryWithRefresh?: boolean; // Flag to prevent infinite retry loops
}

interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  status: number;
}

// API service class
class ApiService {
  // Base URL for API requests
  private baseUrl: string;
  private isRefreshing: boolean = false;
  private refreshPromise: Promise<boolean> | null = null;

  constructor(baseUrl: string = API_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  // Generic request method
  async request<T = any>(endpoint: string, options: RequestOptions = {}): Promise<ApiResponse<T>> {
    const {
      method = 'GET',
      headers = {},
      body,
      requiresAuth = true,
      retryWithRefresh = true
    } = options;

    // Build request headers
    const requestHeaders: Record<string, string> = {
      'Content-Type': 'application/json',
      ...headers
    };

    // Add auth token if required
    if (requiresAuth) {
      const auth = get(authStore);
      if (auth.token) {
        requestHeaders['Authorization'] = `Bearer ${auth.token}`;
      } else {
        return {
          success: false,
          error: 'Authentication required',
          status: 401
        };
      }
    }

    try {
      // Prepare request options
      const requestOptions: RequestInit = {
        method,
        headers: requestHeaders
      };

      // Add body for non-GET requests
      if (body && method !== 'GET') {
        requestOptions.body = JSON.stringify(body);
      }

      // Make the request
      const response = await fetch(`${this.baseUrl}${endpoint}`, requestOptions);
      const status = response.status;
      
      // Try to parse response as JSON
      let data;
      try {
        data = await response.json();
      } catch (e) {
        // If not JSON, use text or empty object
        data = await response.text() || {};
      }
      
      // Handle response
      if (response.ok) {
        return {
          success: true,
          data,
          status
        };
      }
      
      // Handle token expiration (401 Unauthorized)
      if (status === 401 && requiresAuth && retryWithRefresh) {
        // Try to refresh the token and retry the request
        const refreshed = await this.refreshTokenIfNeeded();
        
        if (refreshed) {
          // Retry the request with the new token
          return this.request<T>(endpoint, { ...options, retryWithRefresh: false });
        }
      }
      
      // Handle error response
      return {
        success: false,
        error: data.error || response.statusText,
        data,
        status
      };
    } catch (error) {
      // Handle network errors
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Network error',
        status: 0
      };
    }
  }

  // Refresh token if needed
  private async refreshTokenIfNeeded(): Promise<boolean> {
    // If already refreshing, wait for the existing refresh promise
    if (this.isRefreshing && this.refreshPromise) {
      return this.refreshPromise;
    }

    this.isRefreshing = true;
    
    // Create a new refresh promise
    this.refreshPromise = new Promise<boolean>(async (resolve) => {
      try {
        const refreshToken = localStorage.getItem('refreshToken');
        
        if (!refreshToken) {
          resolve(false);
          return;
        }
        
        // Call refresh token endpoint
        const response = await fetch(`${this.baseUrl}/auth/refresh`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ refreshToken })
        });
        
        if (!response.ok) {
          resolve(false);
          return;
        }
        
        const data = await response.json();
        
        if (data.token) {
          // Update auth store with new token
          const auth = get(authStore);
          
          if (auth.user) {
            authStore.login(auth.user, data.token);
            
            // Update refresh token if provided
            if (data.refreshToken) {
              localStorage.setItem('refreshToken', data.refreshToken);
            }
            
            resolve(true);
            return;
          }
        }
        
        resolve(false);
      } catch (error) {
        console.error('Token refresh error:', error);
        resolve(false);
      } finally {
        this.isRefreshing = false;
        this.refreshPromise = null;
      }
    });
    
    return this.refreshPromise;
  }

  // Convenience methods
  async get<T = any>(endpoint: string, options: Omit<RequestOptions, 'method' | 'body'> = {}): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { ...options, method: 'GET' });
  }

  async post<T = any>(endpoint: string, body: any, options: Omit<RequestOptions, 'method' | 'body'> = {}): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { ...options, method: 'POST', body });
  }

  async put<T = any>(endpoint: string, body: any, options: Omit<RequestOptions, 'method' | 'body'> = {}): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { ...options, method: 'PUT', body });
  }

  async delete<T = any>(endpoint: string, options: Omit<RequestOptions, 'method'> = {}): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { ...options, method: 'DELETE' });
  }
}

// Create and export API service instance
export const api = new ApiService();