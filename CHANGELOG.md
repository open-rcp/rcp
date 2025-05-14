## v0.1.0-beta (2025-05-14)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add comprehensive RCP project milestone roadmap documentation
* feat: Add initial configuration for RCP service and server with TLS and application settings
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Enhance user and application management in RCP CLI
* feat: Implement RCP header structure and serialization
* feat: Implement RCP server configuration, error handling, and session management
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: Implement simple client example with application launching and PSK authentication
* feat: Integrate RCP server and service components into a unified RCP service
* feat: add comprehensive integration tests for RCP core library functionality
* feat: add extensive tests for authentication, command, frame, and protocol functionalities
* feat: add server configuration and server tests for improved functionality
* feat: add session tests for connection state and server configuration validation
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: Correct typo in RCPD documentation for connection handling
* fix: Downgrade @tauri-apps/api to beta version and update icon configuration in Tauri settings
* fix: Update CI workflow to include platform-specific dependency installations and improve test execution conditions
* fix: Update Tauri configuration for development URL and adjust dependencies
* fix: change mutable connection to immutable in state transitions test
* fix: remove .svelte-kit directory from .gitignore and update version hash in internal.js
* fix: remove duplicate environment variable declarations in ambient.d.ts
* fix: remove unused imports from config module and clean up commands exposure
* fix: update .gitignore to include specific node_modules and .svelte-kit directories
* fix: update SvelteKit adapter and dependencies, improve server management UI
* fix: update SvelteKit adapter to version 3.0.8 and clean up server status handling in admin interface
* fix: update clap version and improve service installation logic for macOS and Linux
* fix: update daemonize dependency to version 0.5 and clean up import in main.rs
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines
* fix: update documentation to reflect the correct name "Rust/Remote Control Protocol" across multiple files

### Improvements
* Merge pull request #13 from open-rcp/dev
* Merge pull request #15 from open-rcp/dev
* Merge pull request #21 from open-rcp/dev
* refactor: Remove unused imports and improve variable declarations in CLI command modules
* refactor: Rename rcp-client and rcp-core to rcpc and rcpp respectively across documentation and scripts
* refactor: Update build script and dependencies to use 'rcpd' instead of 'rcp-service'
* refactor: add #[allow(dead_code)] annotations to unused functions and methods across CLI and service modules
* refactor: add missing attribute to MockService id field for clarity
* refactor: add user management methods for deleting, updating roles, and resetting passwords
* refactor: adjust health check mock priorities and improve command endpoint path formatting
* refactor: clean up unused imports and re-export statements in CLI and service modules
* refactor: enhance CLI structure, add configuration handling, and improve test coverage
* refactor: enhance error handling tests and update service command assertions
* refactor: enhance user management commands with add, remove, update role, and reset password functionalities
* refactor: implement user management functionality including add, delete, update, and list users
* refactor: improve connection string parsing error handling and add comprehensive tests for client and service functionality
* refactor: improve test assertions for clarity and consistency across multiple test files
* refactor: remove auto-fix step from clippy in CI workflow
* refactor: remove unused UnixPlatform export from platform module
* refactor: streamline CI workflow and enhance default implementations for RcpState and UserService
* refactor: streamline app command handling by consolidating parameters into a single options struct
* refactor: update auth failure test to reflect server error and adjust health check mock responses
* refactor: update error handling test for server error response and improve service test documentation
* refactor: update token property checks and expiration validation in db tests

### Documentation
* docs: Add comprehensive installation guide for RCPD
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre
* feat: Implement RCP header structure and serialization

## v0.1.0-beta (2025-05-14)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add comprehensive RCP project milestone roadmap documentation
* feat: Add initial configuration for RCP service and server with TLS and application settings
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Enhance user and application management in RCP CLI
* feat: Implement RCP header structure and serialization
* feat: Implement RCP server configuration, error handling, and session management
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: Implement simple client example with application launching and PSK authentication
* feat: Integrate RCP server and service components into a unified RCP service
* feat: add comprehensive integration tests for RCP core library functionality
* feat: add extensive tests for authentication, command, frame, and protocol functionalities
* feat: add server configuration and server tests for improved functionality
* feat: add session tests for connection state and server configuration validation
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: Correct typo in RCPD documentation for connection handling
* fix: Downgrade @tauri-apps/api to beta version and update icon configuration in Tauri settings
* fix: Update CI workflow to include platform-specific dependency installations and improve test execution conditions
* fix: Update Tauri configuration for development URL and adjust dependencies
* fix: change mutable connection to immutable in state transitions test
* fix: remove .svelte-kit directory from .gitignore and update version hash in internal.js
* fix: remove duplicate environment variable declarations in ambient.d.ts
* fix: remove unused imports from config module and clean up commands exposure
* fix: update .gitignore to include specific node_modules and .svelte-kit directories
* fix: update SvelteKit adapter and dependencies, improve server management UI
* fix: update SvelteKit adapter to version 3.0.8 and clean up server status handling in admin interface
* fix: update clap version and improve service installation logic for macOS and Linux
* fix: update daemonize dependency to version 0.5 and clean up import in main.rs
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines
* fix: update documentation to reflect the correct name "Rust/Remote Control Protocol" across multiple files

### Improvements
* Merge pull request #13 from open-rcp/dev
* Merge pull request #15 from open-rcp/dev
* refactor: Remove unused imports and improve variable declarations in CLI command modules
* refactor: Update build script and dependencies to use 'rcpd' instead of 'rcp-service'
* refactor: add #[allow(dead_code)] annotations to unused functions and methods across CLI and service modules
* refactor: add missing attribute to MockService id field for clarity
* refactor: add user management methods for deleting, updating roles, and resetting passwords
* refactor: adjust health check mock priorities and improve command endpoint path formatting
* refactor: clean up unused imports and re-export statements in CLI and service modules
* refactor: enhance CLI structure, add configuration handling, and improve test coverage
* refactor: enhance error handling tests and update service command assertions
* refactor: enhance user management commands with add, remove, update role, and reset password functionalities
* refactor: implement user management functionality including add, delete, update, and list users
* refactor: improve connection string parsing error handling and add comprehensive tests for client and service functionality
* refactor: improve test assertions for clarity and consistency across multiple test files
* refactor: remove auto-fix step from clippy in CI workflow
* refactor: remove unused UnixPlatform export from platform module
* refactor: streamline CI workflow and enhance default implementations for RcpState and UserService
* refactor: streamline app command handling by consolidating parameters into a single options struct
* refactor: update auth failure test to reflect server error and adjust health check mock responses
* refactor: update error handling test for server error response and improve service test documentation
* refactor: update token property checks and expiration validation in db tests

### Documentation
* docs: Add comprehensive installation guide for RCPD
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre
* feat: Implement RCP header structure and serialization

## v0.1.0-beta (2025-05-14)

### Features
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* feat: Add initial configuration for RCP service and server with TLS and application settings
* feat: Add initial package.json for rcp-desk with dependencies and scripts
* feat: Enhance user and application management in RCP CLI
* feat: Implement RCP server configuration, error handling, and session management
* feat: Implement initial version of rcp-cli with configuration management, service commands, and user management
* feat: Implement simple client example with application launching and PSK authentication
* feat: Integrate RCP server and service components into a unified RCP service
* feat: add comprehensive integration tests for RCP core library functionality
* feat: add extensive tests for authentication, command, frame, and protocol functionalities
* feat: add server configuration and server tests for improved functionality
* feat: add session tests for connection state and server configuration validation
* feat: initialize Tauri + SvelteKit application with basic greeting functionality

### Bug Fixes
* chore: update Cargo.toml to include rcp-desk and change license to Apache-2.0
* fix: Correct typo in RCPD documentation for connection handling
* fix: Downgrade @tauri-apps/api to beta version and update icon configuration in Tauri settings
* fix: Update CI workflow to include platform-specific dependency installations and improve test execution conditions
* fix: Update Tauri configuration for development URL and adjust dependencies
* fix: change mutable connection to immutable in state transitions test
* fix: remove .svelte-kit directory from .gitignore and update version hash in internal.js
* fix: remove duplicate environment variable declarations in ambient.d.ts
* fix: remove unused imports from config module and clean up commands exposure
* fix: update .gitignore to include specific node_modules and .svelte-kit directories
* fix: update SvelteKit adapter and dependencies, improve server management UI
* fix: update SvelteKit adapter to version 3.0.8 and clean up server status handling in admin interface
* fix: update clap version and improve service installation logic for macOS and Linux
* fix: update daemonize dependency to version 0.5 and clean up import in main.rs
* fix: update documentation links for consistency in naming
* fix: update documentation links for protocol specification and development guidelines
* fix: update documentation to reflect the correct name "Rust/Remote Control Protocol" across multiple files

### Improvements
* Merge pull request #13 from open-rcp/dev
* Merge pull request #15 from open-rcp/dev
* refactor: Update build script and dependencies to use 'rcpd' instead of 'rcp-service'
* refactor: add #[allow(dead_code)] annotations to unused functions and methods across CLI and service modules
* refactor: add missing attribute to MockService id field for clarity
* refactor: add user management methods for deleting, updating roles, and resetting passwords
* refactor: adjust health check mock priorities and improve command endpoint path formatting
* refactor: clean up unused imports and re-export statements in CLI and service modules
* refactor: enhance CLI structure, add configuration handling, and improve test coverage
* refactor: enhance error handling tests and update service command assertions
* refactor: enhance user management commands with add, remove, update role, and reset password functionalities
* refactor: implement user management functionality including add, delete, update, and list users
* refactor: improve connection string parsing error handling and add comprehensive tests for client and service functionality
* refactor: improve test assertions for clarity and consistency across multiple test files
* refactor: remove auto-fix step from clippy in CI workflow
* refactor: remove unused UnixPlatform export from platform module
* refactor: streamline CI workflow and enhance default implementations for RcpState and UserService
* refactor: streamline app command handling by consolidating parameters into a single options struct
* refactor: update auth failure test to reflect server error and adjust health check mock responses
* refactor: update error handling test for server error response and improve service test documentation
* refactor: update token property checks and expiration validation in db tests

### Documentation
* docs: Add comprehensive installation guide for RCPD
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0+7
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-beta
* docs: update changelog for v0.1.0-pre

