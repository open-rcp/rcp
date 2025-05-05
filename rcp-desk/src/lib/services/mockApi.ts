/**
 * Mock API handler for development environment
 * This simulates API responses when the actual backend is not available
 */

// Mock user data
const mockUsers = [
  {
    id: "1",
    name: "Administrator",
    email: "admin@rcpdesk.local",
    roles: ["admin", "user"]
  },
  {
    id: "2",
    name: "Test User",
    email: "user@rcpdesk.local",
    roles: ["user"]
  }
];

// Mock authentication tokens (refreshed on login)
const mockTokens = {
  accessToken: "dev-mock-token-" + Date.now(),
  refreshToken: "dev-mock-refresh-" + Date.now()
};

// Handler for mock API requests
export async function handleMockRequest(path: string, method: string, data?: any): Promise<any> {
  console.log(`[Mock API] ${method} ${path}`, data);
  
  // Simulate network latency
  await new Promise(resolve => setTimeout(resolve, 300));

  // Authentication endpoints
  if (path === "/auth/login" && method === "POST") {
    return handleLogin(data);
  }
  
  if (path === "/auth/refresh" && method === "POST") {
    return handleRefreshToken(data);
  }

  if (path === "/profile" && method === "GET") {
    return handleGetProfile();
  }
  
  if (path === "/health" && method === "GET") {
    return { status: "ok", version: "dev-mock" };
  }
  
  // Default 404 response for unhandled routes
  return {
    error: "Not found",
    status: 404,
    success: false
  };
}

// Handle login requests
function handleLogin(data: { username: string, password: string }): any {
  const adminUser = import.meta.env.VITE_DEV_ADMIN_USERNAME || "admin";
  const adminPass = import.meta.env.VITE_DEV_ADMIN_PASSWORD || "rcpAdmin!2025";
  
  const testUser = "user";
  const testPass = "password123";
  
  // Admin login
  if (data.username === adminUser && data.password === adminPass) {
    return {
      success: true,
      user: mockUsers[0],
      token: mockTokens.accessToken,
      refreshToken: mockTokens.refreshToken
    };
  }
  
  // Test user login
  if (data.username === testUser && data.password === testPass) {
    return {
      success: true,
      user: mockUsers[1],
      token: mockTokens.accessToken,
      refreshToken: mockTokens.refreshToken
    };
  }
  
  // Failed login
  return {
    success: false,
    error: "Invalid username or password",
    status: 401
  };
}

// Handle token refresh
function handleRefreshToken(data: { refreshToken: string }): any {
  if (data.refreshToken) {
    // Generate new tokens
    mockTokens.accessToken = "dev-mock-token-" + Date.now();
    mockTokens.refreshToken = "dev-mock-refresh-" + Date.now();
    
    return {
      success: true,
      token: mockTokens.accessToken,
      refreshToken: mockTokens.refreshToken
    };
  }
  
  return {
    success: false,
    error: "Invalid refresh token",
    status: 401
  };
}

// Handle getting user profile
function handleGetProfile(): any {
  return {
    success: true,
    user: mockUsers[0]
  };
}