import axios from 'axios';

const API_BASE_URL = '/api';

// Create axios instance with base configuration
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  }
});

// Add authentication token to requests if available
apiClient.interceptors.request.use(
  (config: any) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error: any) => Promise.reject(error)
);

// API service with methods to interact with the management API
export const apiService = {
  // Auth methods
  login: async (username: string, password: string) => {
    const response = await apiClient.post('/auth/login', { username, password });
    if (response.data.token) {
      localStorage.setItem('auth_token', response.data.token);
    }
    return response.data;
  },
  logout: () => {
    localStorage.removeItem('auth_token');
  },
  
  // Server management methods
  getServerStatus: () => apiClient.get('/server/status'),
  startServer: () => apiClient.post('/server/start'),
  stopServer: () => apiClient.post('/server/stop'),
  restartServer: () => apiClient.post('/server/restart'),
  
  // Configuration methods
  getConfig: () => apiClient.get('/config'),
  updateConfig: (config: any) => apiClient.put('/config', config),
  
  // User management methods
  getUsers: () => apiClient.get('/users'),
  getUser: (id: string) => apiClient.get(`/users/${id}`),
  createUser: (user: any) => apiClient.post('/users', user),
  updateUser: (id: string, user: any) => apiClient.put(`/users/${id}`, user),
  deleteUser: (id: string) => apiClient.delete(`/users/${id}`),
  
  // Session management
  getSessions: () => apiClient.get('/sessions'),
  getSession: (id: string) => apiClient.get(`/sessions/${id}`),
  terminateSession: (id: string) => apiClient.delete(`/sessions/${id}`),
  
  // Logs
  getLogs: (params?: { level?: string, limit?: number }) => 
    apiClient.get('/logs', { params })
};

export default apiService;