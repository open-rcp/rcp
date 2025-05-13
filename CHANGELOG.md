## v0.2.0-beta (2025-05-13)

### Features
* feat: Integrate RCP server and API functionality into unified RCP service component
* feat: Add feature-gated API capabilities with "api" feature flag
* feat: Add library target to rcp-service for reusable component architecture
* feat: Create re-exports of common types and functionality in service library
* feat: Implement unified configuration system for integrated components
* feat: Update all build scripts to support integrated service architecture

### Bug Fixes
* fix: Update import paths in tests to use library interface
* fix: Correct feature-gating for API components
* fix: Fix doc comment format in API configuration files
* fix: Resolve compatibility issues with service client connections
* fix: Address build errors in dependency references

### Improvements
* refactor: Move server code into service component
* refactor: Create clean internal interfaces between components
* refactor: Implement proper module exports in service lib.rs
* refactor: Redesign service manager for better integration
* refactor: Keep CLI as separate component for separation of concerns

### Documentation
* docs: Update architecture.md to reflect unified service architecture
* docs: Update rcp-service.md with integrated server and API capabilities
* docs: Update rcp-cli.md to explain separation from service component
* docs: Create integration-progress.md and integration-changes.md to track changes
* docs: Update server-service-integration.md with implementation status
* docs: Add rational for keeping CLI separate from the service
* docs: Update diagrams to show current component architecture

## v0.1.0-beta (2025-05-11)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: add comprehensive integration tests for RCP core library functionality
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: remove .svelte-kit directory from .gitignore and update version hash in internal.js
* fix: remove duplicate environment variable declarations in ambient.d.ts
* fix: remove unused imports from config module and clean up commands exposure
* fix: update .gitignore to include specific node_modules and .svelte-kit directories
* fix: update SvelteKit adapter and dependencies, improve server management UI
* fix: update SvelteKit adapter to version 3.0.8 and clean up server status handling in admin interface
* fix: update daemonize dependency to version 0.5 and clean up import in main.rs
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines
* fix: update documentation to reflect the correct name "Rust/Remote Control Protocol" across multiple files

### Improvements
* refactor: clean up unused imports and re-export statements in CLI and service modules
* refactor: remove unused UnixPlatform export from platform module

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

## v0.1.0-beta (2025-05-05)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines

### Improvements

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

## v0.1.0-beta (2025-05-05)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines

### Improvements

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

## v0.1.0-beta (2025-05-04)

### Features

### Bug Fixes
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines

### Improvements

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

## v0.1.0-beta (2025-05-04)

### Features

### Bug Fixes
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines

### Improvements

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

## v0.1.0-beta (2025-05-04)

### Features

### Bug Fixes
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines

### Improvements

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-pre

## v0.1.0-pre (2025-05-04)

### Features
* feat: add initial implementation of RCP service with command-line interface and service management features
* feat: update release workflow to use GitHub Script for creating and uploading release assets

### Bug Fixes

### Improvements

### Documentation

## v0.1.0+7 (2025-05-04)

### Features
* feat: Add rcp-management-api module with SurrealDB integration
* feat: Implement RCP Management UI with Svelte and Axios
* feat: Implement authentication handlers for login, logout, and user profile retrieval
* feat: Refactor examples and integrate authentication handlers in management API
* feat: enhance build script to setup LLVM/Clang and improve server build process
* feat: enhance changelog workflow to support autonomous releases and improve version detection
* feat: enhance management API handling and improve server run logic
* feat: implement management API with status endpoint using axum
* feat: implement server state management and uptime tracking in RCP server
* feat: remove outdated documentation for RCP Flutter Client, Management API, and Management Dashboard
* feat: update bincode to version 2.0.1 and refactor serialization/deserialization functions
* feat: update build documentation and dependencies; enhance platform-specific instructions and adjust bincode version
* feat: update dependencies and refactor management API integration
* feat: update documentation and build scripts for RCP project; remove outdated scripts and enhance build instructions
* feat: update error handling and build scripts to improve compatibility and simplify setup

### Bug Fixes
* fix: Add missing dependencies to rcp-examples in Cargo.toml and Cargo.lock
* fix: Correct examples path in workspace members for clarity
* fix: Remove unnecessary dependencies from rcp-examples in Cargo.lock
* fix: Update Cargo.toml to streamline authors and license fields, and adjust dependency paths
* fix: correct link to milestone roadmap in README
* fix: format message sending for improved readability in WebSocket connection handling
* fix: improve error handling in bincode serialization and deserialization functions
* fix: remove unnecessary features from bincode dependency in Cargo.toml files
* fix: remove unused AWS-LC dependencies and update rustls and webpki-roots versions
* fix: revert rustls and webpki-roots versions to previous stable releases
* fix: update AWS-LC configuration and remove problematic bindgen flag
* fix: update error handling in WebSocket connection responses
* fix: update link to legacy roadmap in README
* fix: update rustls and webpki-roots dependencies for compatibility

### Improvements
* refactor: remove management API implementation and related dependencies
* refactor: remove management UI and API references from build scripts and configuration
* refactor: remove unnecessary blank line before server run

### Documentation
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7

## v0.1.0+7 (2025-05-04)

### Features
* feat: Add rcp-management-api module with SurrealDB integration
* feat: Implement RCP Management UI with Svelte and Axios
* feat: Implement authentication handlers for login, logout, and user profile retrieval
* feat: Refactor examples and integrate authentication handlers in management API
* feat: enhance build script to setup LLVM/Clang and improve server build process
* feat: enhance changelog workflow to support autonomous releases and improve version detection
* feat: enhance management API handling and improve server run logic
* feat: implement management API with status endpoint using axum
* feat: implement server state management and uptime tracking in RCP server
* feat: remove outdated documentation for RCP Flutter Client, Management API, and Management Dashboard
* feat: update bincode to version 2.0.1 and refactor serialization/deserialization functions
* feat: update build documentation and dependencies; enhance platform-specific instructions and adjust bincode version
* feat: update dependencies and refactor management API integration
* feat: update documentation and build scripts for RCP project; remove outdated scripts and enhance build instructions
* feat: update error handling and build scripts to improve compatibility and simplify setup
* feat: Add rcp-management-api module with SurrealDB integration
* feat: Implement RCP Management UI with Svelte and Axios
* feat: Implement authentication handlers for login, logout, and user profile retrieval
* feat: Refactor examples and integrate authentication handlers in management API
* feat: enhance build script to setup LLVM/Clang and improve server build process
* feat: enhance changelog workflow to support autonomous releases and improve version detection
* feat: enhance management API handling and improve server run logic
* feat: implement management API with status endpoint using axum
* feat: implement server state management and uptime tracking in RCP server
* feat: remove outdated documentation for RCP Flutter Client, Management API, and Management Dashboard
* feat: update bincode to version 2.0.1 and refactor serialization/deserialization functions
* feat: update build documentation and dependencies; enhance platform-specific instructions and adjust bincode version
* feat: update dependencies and refactor management API integration
* feat: update documentation and build scripts for RCP project; remove outdated scripts and enhance build instructions
* feat: update error handling and build scripts to improve compatibility and simplify setup
* feat: Add rcp-management-api module with SurrealDB integration
* feat: Implement RCP Management UI with Svelte and Axios
* feat: Implement authentication handlers for login, logout, and user profile retrieval
* feat: Refactor examples and integrate authentication handlers in management API
* feat: enhance build script to setup LLVM/Clang and improve server build process
* feat: enhance changelog workflow to support autonomous releases and improve version detection
* feat: enhance management API handling and improve server run logic
* feat: implement management API with status endpoint using axum
* feat: implement server state management and uptime tracking in RCP server
* feat: remove outdated documentation for RCP Flutter Client, Management API, and Management Dashboard
* feat: update bincode to version 2.0.1 and refactor serialization/deserialization functions
* feat: update build documentation and dependencies; enhance platform-specific instructions and adjust bincode version
* feat: update dependencies and refactor management API integration
* feat: update documentation and build scripts for RCP project; remove outdated scripts and enhance build instructions
* feat: update error handling and build scripts to improve compatibility and simplify setup

### Bug Fixes
* fix: Add missing dependencies to rcp-examples in Cargo.toml and Cargo.lock
* fix: Correct examples path in workspace members for clarity
* fix: Remove unnecessary dependencies from rcp-examples in Cargo.lock
* fix: Update Cargo.toml to streamline authors and license fields, and adjust dependency paths
* fix: correct link to milestone roadmap in README
* fix: format message sending for improved readability in WebSocket connection handling
* fix: improve error handling in bincode serialization and deserialization functions
* fix: remove unnecessary features from bincode dependency in Cargo.toml files
* fix: remove unused AWS-LC dependencies and update rustls and webpki-roots versions
* fix: revert rustls and webpki-roots versions to previous stable releases
* fix: update AWS-LC configuration and remove problematic bindgen flag
* fix: update error handling in WebSocket connection responses
* fix: update link to legacy roadmap in README
* fix: update rustls and webpki-roots dependencies for compatibility
* fix: Add missing dependencies to rcp-examples in Cargo.toml and Cargo.lock
* fix: Correct examples path in workspace members for clarity
* fix: Remove unnecessary dependencies from rcp-examples in Cargo.lock
* fix: Update Cargo.toml to streamline authors and license fields, and adjust dependency paths
* fix: correct link to milestone roadmap in README
* fix: format message sending for improved readability in WebSocket connection handling
* fix: improve error handling in bincode serialization and deserialization functions
* fix: remove unnecessary features from bincode dependency in Cargo.toml files
* fix: remove unused AWS-LC dependencies and update rustls and webpki-roots versions
* fix: revert rustls and webpki-roots versions to previous stable releases
* fix: update AWS-LC configuration and remove problematic bindgen flag
* fix: update error handling in WebSocket connection responses
* fix: update link to legacy roadmap in README
* fix: update rustls and webpki-roots dependencies for compatibility
* fix: Add missing dependencies to rcp-examples in Cargo.toml and Cargo.lock
* fix: Correct examples path in workspace members for clarity
* fix: Remove unnecessary dependencies from rcp-examples in Cargo.lock
* fix: Update Cargo.toml to streamline authors and license fields, and adjust dependency paths
* fix: correct link to milestone roadmap in README
* fix: format message sending for improved readability in WebSocket connection handling
* fix: improve error handling in bincode serialization and deserialization functions
* fix: remove unnecessary features from bincode dependency in Cargo.toml files
* fix: remove unused AWS-LC dependencies and update rustls and webpki-roots versions
* fix: revert rustls and webpki-roots versions to previous stable releases
* fix: update AWS-LC configuration and remove problematic bindgen flag
* fix: update error handling in WebSocket connection responses
* fix: update link to legacy roadmap in README
* fix: update rustls and webpki-roots dependencies for compatibility

### Improvements
* refactor: remove management API implementation and related dependencies
* refactor: remove management UI and API references from build scripts and configuration
* refactor: remove unnecessary blank line before server run

### Documentation
* docs: update changelog for v0.1.0+7
