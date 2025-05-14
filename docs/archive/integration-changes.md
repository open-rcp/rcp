# Server and API Integration Changes

This document summarizes the changes made to integrate both the RCP server and RCP API functionality into the RCP service component.

## Overview

The integration combines three previously separate components:
- `rcp-server`: Server implementation
- `rcp-api`: API implementation
- `rcp-service`: Service manager

Into a single component:
- `rcp-service`: Unified component with integrated server and optional API capabilities

## Directory Structure Changes

The integrated service now has the following directory structure:
```
rcp-service/
├── src/
│   ├── config.rs          # Service configuration
│   ├── error.rs           # Error handling
│   ├── main.rs            # Entry point
│   ├── manager.rs         # Service manager implementation
│   ├── service.rs         # Service interface
│   ├── server/            # Integrated server functionality
│   │   ├── config.rs      # Server configuration
│   │   ├── mod.rs         # Server module definition
│   │   ├── server.rs      # Server implementation
│   │   └── session.rs     # Client session handling
│   └── api/               # Integrated API functionality (feature-gated)
│       ├── config.rs      # API configuration
│       ├── mod.rs         # API module definition
│       ├── server.rs      # API server implementation
│       └── handlers.rs    # API request handlers
```

## Feature Gates

The API functionality is now feature-gated with the `api` feature:

```toml
[features]
default = []
api = [
    "axum", 
    "tower-http", 
    "sqlx", 
    "tower", 
    "serde_urlencoded", 
    "serde_with",
    "jsonwebtoken",
    "uuid/serde"
] 
    "mime"
]
```

## Build Script Updates

All build scripts have been updated to build `rcp-service` instead of the individual `rcp-server` component:

- Updated Windows build script (`scripts/windows/build.bat`)
- Updated Linux build script (`scripts/linux/build.sh`)
- Updated macOS build script (`scripts/macos/build.sh`)
- Updated Linux combined build & run script (`scripts/linux/build_and_run.sh`)

## GitHub Actions Workflow Updates

The GitHub release workflow has been updated:

- Changed to build `rcp-service` instead of `rcp-server`
- Updated all package naming and archives to `rcp-service-*` instead of `rcp-server-*`

## Documentation Updates

Various documentation files have been updated:

- Updated the main `README.md` to reflect the new component structure
- Updated architecture diagrams and component descriptions in `docs/architecture.md`
- Added notes to `docs/rcp-api.md` about the integration
- Updated the project outline in `docs/project-outline.md`

## Client Updates

The RCP CLI continues to use the API endpoints without significant changes, but now points to the integrated API endpoints provided by the `rcp-service` when built with the `api` feature.

## Library Interface Integration

The RCP service component now exposes a library interface through `lib.rs`, allowing other components to import and use its functionality directly. This includes:

- Re-exporting common types like `ServiceConfig`, `ServiceError`, `Result`, `ServiceManager`, and `Service`
- Conditional compilation of API functionality through the `api` feature gate
- Making all core functionality available through public modules

## Test Updates

Tests have been updated to use the library interface rather than directly importing modules via path attributes. This improves:

- Test maintainability
- Code organization 
- Ease of refactoring

The following test files have been updated or created:

- `tests/config_tests.rs`: Updated to use library imports
- `tests/manager_test.rs`: Created as a replacement for direct path imports
- `tests/manager_tests.rs`: Updated to use library imports

All tests now pass successfully with the integrated architecture.
- `config_tests.rs` - Tests for service configuration
- `manager_test.rs` - Tests for service manager functionality

## Known Issues and TODOs

There are still some unused code warnings that need to be addressed as part of the integration cleanup:

- Several unused methods and types remain in the codebase
- Some fields are marked as unused but may be needed for future functionality
- The API feature flag needs broader testing to ensure it works correctly in all contexts
