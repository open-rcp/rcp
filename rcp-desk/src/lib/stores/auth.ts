import { writable } from 'svelte/store';
import { browser } from '../utils/environment';

// Define the user interface
interface User {
  id: string;
  name: string;
  email: string;
  roles: string[];
}

// Define the auth store interface
interface AuthStore {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
}

// Try to load initial state from localStorage
function loadInitialState(): AuthStore {
  // Default state if nothing is stored
  const defaultState: AuthStore = {
    isAuthenticated: false,
    user: null,
    token: null
  };
  
  // Only try to read localStorage in browser environment
  if (browser) {
    try {
      const storedAuth = localStorage.getItem('authState');
      if (storedAuth) {
        return JSON.parse(storedAuth) as AuthStore;
      }
    } catch (e) {
      console.error('Error loading auth state from localStorage:', e);
    }
  }
  
  return defaultState;
}

// Create the writable store with initial state from localStorage
const createAuthStore = () => {
  const initialState = loadInitialState();
  const { subscribe, set, update } = writable<AuthStore>(initialState);
  
  return {
    subscribe,
    
    // Set user information after successful login and persist to localStorage
    login: (user: User, token: string) => {
      const newState: AuthStore = {
        isAuthenticated: true,
        user,
        token
      };
      
      // Persist to localStorage
      if (browser) {
        localStorage.setItem('authState', JSON.stringify(newState));
      }
      
      // Update the store
      set(newState);
    },
    
    // Clear auth information on logout
    logout: () => {
      const defaultState: AuthStore = {
        isAuthenticated: false,
        user: null,
        token: null
      };
      
      // Remove from localStorage
      if (browser) {
        localStorage.removeItem('authState');
      }
      
      set(defaultState);
    },
    
    // Update user information
    updateUser: (user: User) => {
      update(state => {
        const newState = {
          ...state,
          user
        };
        
        // Persist to localStorage
        if (browser) {
          localStorage.setItem('authState', JSON.stringify(newState));
        }
        
        return newState;
      });
    }
  };
};

// Export the store
export const authStore = createAuthStore();