# RCP API

This document specifies the API for the Rust/Remote Control Protocol (RCP), providing a RESTful interface for managing RCP servers, client connections, and monitoring system usage.

> **Note:** The API functionality is now integrated directly into the `rcpdaemon` (RCP Daemon) component and is available when built with the `api` feature enabled. There is no longer a separate `rcp-api` component.

## API Overview

The RCP API enables remote administration of RCP deployments with capabilities including:

- Server status monitoring and configuration
- User and permission management
- Session monitoring and control
- Service enablement and configuration
- Statistics and reporting
- Logs and diagnostics

The API serves as a critical integration point between:
- The RCP Daemon (rcpdaemon) runtime
- The RCP Desk management interface
- Third-party management systems
- Custom automation scripts

## Authentication

The API uses JWT (JSON Web Token) for authentication:

```
POST /api/v1/auth/login
```

**Request:**
```json
{
  "username": "admin",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2025-05-04T14:30:00Z",
  "user": {
    "id": "admin",
    "roles": ["admin"]
  }
}
```

All subsequent requests must include the token in the Authorization header:
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## API Endpoints

### Server Management

#### Get Server Status

```
GET /api/v1/server/status
```

**Response:**
```json
{
  "status": "running",
  "version": "1.0.0",
  "uptime": 86400,
  "connections": {
    "total": 5,
    "active": 2
  },
  "resources": {
    "cpu_usage": 2.5,
    "memory_usage": 128.4
  }
}
```

#### Update Server Configuration

```
PUT /api/v1/server/config
```

**Request:**
```json
{
  "port": 8716,
  "max_connections": 50,
  "authentication": {
    "required": true,
    "methods": ["psk", "public_key"]
  },
  "services": {
    "enabled": ["display", "input", "app", "clipboard"]
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Configuration updated successfully",
  "requires_restart": true
}
```

#### Restart Server

```
POST /api/v1/server/restart
```

**Response:**
```json
{
  "success": true,
  "message": "Server restarting",
  "estimated_downtime_seconds": 5
}
```

### Daemon Management

```
GET /api/v1/daemon/status
```

**Response:**
```json
{
  "status": "running",
  "version": "1.0.0",
  "uptime": 152435,
  "managed_servers": 2,
  "total_connections": 12
}
```

#### Daemon Control

```
POST /api/v1/daemon/restart
```

**Response:**
```json
{
  "success": true,
  "message": "Daemon restarting",
  "estimated_downtime_seconds": 10
}
```

### Session Management

#### List Active Sessions

```
GET /api/v1/sessions
```

**Response:**
```json
{
  "sessions": [
    {
      "id": "52023038-41a6-41ff-bb87-e18b24163e92",
      "client_id": "eb9b6298-0ee9-457c-a88d-b89b616ce275",
      "client_name": "UserWorkstation",
      "client_address": "192.168.1.50",
      "started_at": "2025-05-04T12:15:30Z",
      "subscribed_services": ["display", "input", "app"],
      "idle": false
    },
    {
      "id": "8fd9429f-0ae4-4360-9b42-27b7d5f09d63",
      "client_id": "9e613a4e-97e5-4f7c-9877-e9784fecc083",
      "client_name": "MobileApp",
      "client_address": "192.168.1.100",
      "started_at": "2025-05-04T12:30:45Z",
      "subscribed_services": ["display", "clipboard"],
      "idle": true
    }
  ],
  "total": 2
}
```

#### Get Session Details

```
GET /api/v1/sessions/{session_id}
```

**Response:**
```json
{
  "id": "52023038-41a6-41ff-bb87-e18b24163e92",
  "client_id": "eb9b6298-0ee9-457c-a88d-b89b616ce275",
  "client_name": "UserWorkstation",
  "client_address": "192.168.1.50",
  "started_at": "2025-05-04T12:15:30Z",
  "authenticated_at": "2025-05-04T12:15:32Z",
  "permissions": ["display", "input", "app", "app:launch"],
  "subscribed_services": ["display", "input", "app"],
  "statistics": {
    "bytes_received": 15728640,
    "bytes_sent": 104857600,
    "frames_received": 1250,
    "frames_sent": 3600
  },
  "idle": false,
  "last_activity": "2025-05-04T12:45:10Z"
}
```

#### Terminate Session

```
DELETE /api/v1/sessions/{session_id}
```

**Response:**
```json
{
  "success": true,
  "message": "Session terminated successfully"
}
```

### Service Management

#### List Available Services

```
GET /api/v1/services
```

**Response:**
```json
{
  "services": [
    {
      "id": "display",
      "name": "Display Service",
      "description": "Screen sharing service",
      "enabled": true
    },
    {
      "id": "input",
      "name": "Input Service",
      "description": "Keyboard and mouse input service",
      "enabled": true
    },
    {
      "id": "app",
      "name": "Application Service",
      "description": "Application launching and control",
      "enabled": false
    },
    {
      "id": "clipboard",
      "name": "Clipboard Service",
      "description": "Clipboard synchronization",
      "enabled": true
    },
    {
      "id": "file-transfer",
      "name": "File Transfer Service",
      "description": "File operations between peers",
      "enabled": true
    }
  ]
}
```

#### Update Service Configuration

```
PUT /api/v1/services/{service_id}
```

**Request:**
```json
{
  "enabled": true,
  "config": {
    "max_quality": 90,
    "fps_limit": 30,
    "encryption": "aes256"
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Service configuration updated"
}
```

### User Management

#### List Users

```
GET /api/v1/users
```

**Response:**
```json
{
  "users": [
    {
      "id": "admin",
      "name": "Administrator",
      "roles": ["admin"],
      "created_at": "2025-01-01T00:00:00Z"
    },
    {
      "id": "user1",
      "name": "Regular User",
      "roles": ["user"],
      "created_at": "2025-03-15T14:30:00Z"
    }
  ]
}
```

#### Create User

```
POST /api/v1/users
```

**Request:**
```json
{
  "id": "newuser",
  "name": "New User",
  "password": "secure_password",
  "roles": ["user"]
}
```

**Response:**
```json
{
  "success": true,
  "user": {
    "id": "newuser",
    "name": "New User",
    "roles": ["user"],
    "created_at": "2025-05-04T13:00:00Z"
  }
}
```

#### Update User

```
PUT /api/v1/users/{user_id}
```

**Request:**
```json
{
  "name": "Updated User Name",
  "roles": ["user", "operator"]
}
```

**Response:**
```json
{
  "success": true,
  "message": "User updated successfully"
}
```

#### Delete User

```
DELETE /api/v1/users/{user_id}
```

**Response:**
```json
{
  "success": true,
  "message": "User deleted successfully"
}
```

### Statistics and Monitoring

#### Get System Metrics

```
GET /api/v1/metrics
```

**Response:**
```json
{
  "current_connections": 2,
  "total_connections_today": 15,
  "peak_connections": 8,
  "peak_time": "2025-05-04T10:15:00Z",
  "bandwidth": {
    "total_bytes_received": 1073741824,
    "total_bytes_sent": 5368709120,
    "current_bps_in": 512000,
    "current_bps_out": 2048000
  },
  "cpu_usage": 2.5,
  "memory_usage": 128.4
}
```

#### Get Historical Data

```
GET /api/v1/metrics/history?from=2025-05-03T00:00:00Z&to=2025-05-04T23:59:59Z&interval=hour
```

**Response:**
```json
{
  "interval": "hour",
  "data": [
    {
      "timestamp": "2025-05-03T00:00:00Z",
      "connections": 3,
      "cpu_usage": 1.2,
      "memory_usage": 110.5,
      "bandwidth_in": 52428800,
      "bandwidth_out": 268435456
    },
    {
      "timestamp": "2025-05-03T01:00:00Z",
      "connections": 2,
      "cpu_usage": 0.8,
      "memory_usage": 105.2,
      "bandwidth_in": 41943040,
      "bandwidth_out": 167772160
    }
  ]
}
```

### Logs and Diagnostics

#### Get System Logs

```
GET /api/v1/logs?level=error&from=2025-05-04T00:00:00Z&limit=100
```

**Response:**
```json
{
  "logs": [
    {
      "timestamp": "2025-05-04T08:15:22Z",
      "level": "error",
      "source": "session",
      "message": "Connection error: Connection closed during authentication"
    },
    {
      "timestamp": "2025-05-04T09:30:45Z",
      "level": "error",
      "source": "service.app",
      "message": "Failed to launch application: Permission denied"
    }
  ],
  "total": 2
}
```

#### Run Diagnostics

```
POST /api/v1/diagnostics/run
```

**Request:**
```json
{
  "tests": ["network", "services", "auth"]
}
```

**Response:**
```json
{
  "success": true,
  "results": {
    "network": {
      "status": "pass",
      "details": "All network interfaces operational"
    },
    "services": {
      "status": "warning",
      "details": "App service not responding correctly"
    },
    "auth": {
      "status": "pass",
      "details": "Authentication system operational"
    }
  }
}
```

## Error Responses

All API endpoints return standard HTTP status codes with a consistent error format:

```json
{
  "error": true,
  "code": "AUTH_FAILED",
  "message": "Authentication failed",
  "details": "Invalid username or password"
}
```

Common error codes include:

- `AUTH_FAILED`: Authentication issues
- `PERMISSION_DENIED`: Insufficient permissions
- `RESOURCE_NOT_FOUND`: Requested resource not found
- `VALIDATION_ERROR`: Invalid request data
- `INTERNAL_ERROR`: Server-side error

## Versioning

The API version is specified in the URL path (e.g., `/api/v1/`). Future revisions will increment the version number.

## Rate Limiting

API requests are subject to rate limiting:
- 60 requests per minute for standard endpoints
- 20 requests per minute for authentication endpoints

Rate limit headers are included in each response:
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 58
X-RateLimit-Reset: 1620147600
```

## Implementation Plan

1. **Initial Version (v1)**
   - Core server management
   - Basic session control
   - Authentication endpoints
   - System status endpoints

2. **Expansion (v1.1)**
   - User management
   - Detailed metrics
   - Service configuration

3. **Advanced Features (v2)**
   - Full historic data
   - Advanced diagnostics
   - Programmatic daemon control
   - WebSocket real-time events
   - Integration webhooks