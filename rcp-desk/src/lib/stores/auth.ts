import { writable } from 'svelte/store';

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

// Create the initial store value
const initialState: AuthStore = {
  isAuthenticated: false,
  user: null,
  token: null
};

// Create the writable store
const createAuthStore = () => {
  const { subscribe, set, update } = writable<AuthStore>(initialState);
  
  return {
    subscribe,
    
    // Set user information after successful login
    login: (user: User, token: string) => {
      update(state => ({
        ...state,
        isAuthenticated: true,
        user,
        token
      }));
    },
    
    // Clear auth information on logout
    logout: () => {
      set(initialState);
    },
    
    // Update user information
    updateUser: (user: User) => {
      update(state => ({
        ...state,
        user
      }));
    }
  };
};

// Export the store
export const authStore = createAuthStore();