import { api } from './api';
import { authStore } from '../stores/auth';

// Types
interface LoginCredentials {
  username: string;
  password: string;
}

interface LoginResponse {
  user: {
    id: string;
    name: string;
    email: string;
    roles: string[];
  };
  token: string;
  refreshToken: string;
}

class AuthService {
  /**
   * Attempt to login with credentials
   */
  async login(credentials: LoginCredentials): Promise<boolean> {
    const response = await api.post<LoginResponse>(
      '/auth/login', 
      credentials, 
      { requiresAuth: false }
    );

    if (response.success && response.data) {
      // Store auth data
      const { user, token } = response.data;
      
      // Save refresh token to localStorage for persistence
      if (response.data.refreshToken) {
        localStorage.setItem('refreshToken', response.data.refreshToken);
      }
      
      // Update auth store
      authStore.login(user, token);
      return true;
    }
    
    return false;
  }

  /**
   * Attempt to refresh the authentication token
   */
  async refreshToken(): Promise<boolean> {
    const refreshToken = localStorage.getItem('refreshToken');
    
    if (!refreshToken) {
      return false;
    }

    const response = await api.post<{ token: string; refreshToken: string }>(
      '/auth/refresh',
      { refreshToken },
      { requiresAuth: false }
    );

    if (response.success && response.data) {
      // Get current auth state
      const auth = authStore;
      let user;
      
      // Subscribe to get current user
      const unsubscribe = auth.subscribe(state => {
        user = state.user;
      });
      unsubscribe();
      
      if (user) {
        // Update tokens
        if (response.data.refreshToken) {
          localStorage.setItem('refreshToken', response.data.refreshToken);
        }
        
        // Update auth store with new token
        authStore.login(user, response.data.token);
        return true;
      }
    }
    
    return false;
  }

  /**
   * Logout the current user
   */
  async logout(): Promise<void> {
    try {
      // Call logout endpoint (don't worry if it fails)
      await api.post('/auth/logout', {});
    } catch (error) {
      console.error('Error during logout:', error);
    } finally {
      // Remove refresh token
      localStorage.removeItem('refreshToken');
      
      // Clear auth store
      authStore.logout();
    }
  }

  /**
   * Check if the user is authenticated
   */
  isAuthenticated(): boolean {
    let isAuth = false;
    
    // Subscribe to get current auth state
    const unsubscribe = authStore.subscribe(state => {
      isAuth = state.isAuthenticated && !!state.token;
    });
    unsubscribe();
    
    return isAuth;
  }

  /**
   * Initialize authentication from stored refresh token
   */
  async initAuth(): Promise<boolean> {
    // Check if there's a refresh token
    const refreshToken = localStorage.getItem('refreshToken');
    
    if (refreshToken) {
      // Try to refresh the token
      return await this.refreshToken();
    }
    
    return false;
  }

  /**
   * Get user profile from the server
   */
  async getProfile(): Promise<any> {
    const response = await api.get('/profile');
    return response.success ? response.data : null;
  }
}

export const authService = new AuthService();