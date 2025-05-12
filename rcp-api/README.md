# RCP API

The RCP API component serves as the management and control API gateway for the RCP ecosystem. Its primary responsibility is to provide HTTP APIs for server-side operations, handle authentication/session management, and delegate user management, permissions, and application configuration to the RCP Server component.

## Architecture

The RCP API follows a layered architecture:

1. **API Layer**: Handles HTTP requests/responses and authentication
2. **Service Layer**: Communicates with the RCP Service for command execution
3. **Data Layer**: Manages API tokens and sessions

## Key Components

- **Config**: API server configuration
- **DB**: Database operations for API tokens and sessions
- **Service Client**: Client for communicating with the RCP Service
- **Routes**: API route definitions
- **Handlers**: API endpoint implementations
- **Error Handling**: Standardized API error responses

## Running Tests

To run the test suite:

```bash
cargo test
```

## API Endpoints

- **Health**: `/health` - API server health check
- **Authentication**: `/auth/token` - API token management
- **Sessions**: `/sessions` - Session information
- **Servers**: `/servers` - Server management (via RCP Service)
- **Status**: `/status` - System status information

## Configuration

Configuration can be provided via:

1. Configuration file (TOML)
2. Environment variables

### Environment Variables

- `RCP_API_BIND_ADDRESS`: API server bind address
- `RCP_API_PORT`: API server port
- `RCP_API_DATABASE_URL`: Database connection URL
- `RCP_API_SERVICE_CONNECTION`: RCP Service connection string
- `RCP_API_JWT_SECRET`: Secret for JWT tokens
- `RCP_API_JWT_EXPIRATION`: Token expiration time in minutes
- `RCP_API_ENABLE_CORS`: Enable/disable CORS support
- `RCP_API_CORS_ORIGINS`: Allowed CORS origins
- `RCP_API_ENABLE_COMPRESSION`: Enable/disable response compression
- `RCP_API_LOG_LEVEL`: Log level (debug, info, warn, error)

## Integration with RCP Components

- **RCP Service**: For server management and command execution
- **RCP Server**: For user management, permissions, and application configuration
