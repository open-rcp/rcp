# RCP Implementation Guidelines

This guide provides detailed instructions for implementing and using the Rust/Remote Control Protocol (RCP) in your applications.

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture Overview](#architecture-overview)
3. [Protocol Library (rcpcore)](#protocol-library)
4. [Client Implementation (rcpcli)](#client-implementation)
5. [RCP Daemon Implementation (rcpdaemon)](#daemon-implementation)
   - [Server Component](#server-component)
   - [API Component](#api-component)
   - [CLI Component](#cli-component)
6. [Authentication](#authentication)
7. [Error Handling](#error-handling)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Introduction

The Rust/Remote Control Protocol (RCP) is a lightweight, secure protocol for remote control and service sharing between applications. It is designed to be efficient, extensible, and easy to integrate into Rust applications.

Key features:
- Binary protocol with minimal overhead
- Secure authentication mechanisms
- Extensible service architecture
- Efficient data streaming
- Reconnection support
- Cross-platform compatibility
- Runtime management via daemon
- Integrated CLI and optional API interface

## Architecture Overview

RCP follows a modular architecture with the following key components:

1. **Protocol Library (rcpcore)**: Defines the basic message format, framing, and protocol state machine.
2. **Client Library (rcpcli)**: Provides client connectivity and control interface.
3. **Daemon (rcpdaemon)**: Provides server functionality, session management, and administration.
4. **Authentication**: Multiple methods for secure client authentication.
5. **Runtime Service**: System service that manages application lifecycle.
6. **CLI**: Command-line interface for administration.
7. **API**: RESTful API for remote management.
8. **Desk**: Web and desktop administrative interface.

### Protocol Flow

1. Client connects to server
2. Client authenticates with server
3. Server creates a session for the client
4. Client subscribes to services
5. Client and server exchange service-specific messages
6. Client unsubscribes from services
7. Client disconnects

### Management Flow

1. User interacts with Desk UI, CLI, or API
2. Management commands are sent to RCP Service
3. RCP Service applies configuration and controls RCP Server
4. RCP Service provides feedback and status information

## Client Implementation

### Basic Client Usage

```rust
use rcp_client::{Client, ClientConfig, AuthMethod};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn example_client() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration
    let config = ClientConfig {
        host: "example.com".to_string(),
        port: 8716,
        auth_method: AuthMethod::PreSharedKey,
        psk: Some("your_secret_key".to_string()),
        ..Default::default()
    };
    
    // Create and connect client
    let mut client = Client::new(config);
    client.connect().await?;
    
    // Authenticate
    let session = client.authenticate().await?;
    println!("Connected with session ID: {}", session.session_id);
    
    // Create shared client reference for services
    let client_arc = Arc::new(Mutex::new(client));
    
    // Work with services...
    
    // Disconnect when done
    let mut client = client_arc.lock().await;
    client.disconnect().await?;
    
    Ok(())
}
```

### SSH-Like Connection Strings

RCP now supports SSH-like connection string formatting for easier client connection setup. This allows you to specify all connection parameters in a single string using a familiar format:

```
[user[:password]@]host[:port][/path]
```

**Examples:**

```
localhost:8716                  # Connect to localhost on port 8716
user:pass@192.168.1.100:8716    # Connect with username/password
admin:secretkey@server.example.com:8716/custom-path
```

**Client usage with connection string:**

```rust
use rcp_client::{Client, ServiceType};

async fn connect_with_string() -> Result<(), Box<dyn std::error::Error>> {
    // Create client using connection string
    let client = Client::builder()
        .connection_string("user:password@host:8716")
        .unwrap()
        .build();
    
    // Connect and authenticate in one step
    client.connect_and_authenticate().await?;
    
    // Start the client message processor
    client.start().await?;
    
    // Use the client...
    
    // Disconnect when done
    client.disconnect().await?;
    
    Ok(())
}
```

**Command line usage:**

```bash
# Connect using an SSH-like connection string
rcpcli connect user:pass@host:8716

# Execute a command using connection string
rcpcli execute user:pass@host:8716 my_command arg1 arg2
```

### Working with Services

```rust
use rcp_client::{DisplayService, InputService, ClipboardService};

async fn use_services(client_arc: Arc<Mutex<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    // Create service clients
    let display = DisplayService::new(Arc::clone(&client_arc));
    let input = InputService::new(Arc::clone(&client_arc));
    let clipboard = ClipboardService::new(Arc::clone(&client_arc));
    
    // Subscribe to services
    display.subscribe().await?;
    input.subscribe().await?;
    clipboard.subscribe().await?;
    
    // Use display service
    display.set_quality(90).await?;
    
    // Use input service
    input.send_mouse_move(100, 200).await?;
    input.send_key(0x41, true).await?;  // Press 'A'
    input.send_key(0x41, false).await?; // Release 'A'
    
    // Use clipboard service
    clipboard.send_clipboard("Shared clipboard text").await?;
    
    // Unsubscribe when done
    display.unsubscribe().await?;
    input.unsubscribe().await?;
    clipboard.unsubscribe().await?;
    
    Ok(())
}
```

### Handling Events

```rust
use futures_util::StreamExt;
use rcp_client::ClientEvent;

async fn handle_events(client: &mut Client) {
    let mut receiver = client.event_receiver();
    
    while let Some(event) = receiver.next().await {
        match event {
            ClientEvent::StateChanged(state) => {
                println!("Client state changed to {:?}", state);
            }
            ClientEvent::FrameReceived(frame) => {
                println!("Received frame: command={:02x}, size={} bytes",
                         frame.command_id(), frame.payload().len());
                
                // Handle specific frame types
                if frame.command_id() == rcp_client::CommandId::VideoFrame as u8 {
                    // Process video frame...
                }
            }
            ClientEvent::Disconnected(reason) => {
                println!("Disconnected: {:?}", reason);
                break;
            }
            ClientEvent::Error(error) => {
                println!("Error: {}", error);
            }
            _ => {}
        }
    }
}
```

## Daemon Implementation

The RCP Daemon (rcpdaemon) is a unified system daemon that includes server and API capabilities in a single component.

### Server Component Implementation

The server component is fully integrated into the RCP Daemon.

```rust
use rcpdaemon::{DaemonConfig, Daemon, ServerConfig, AuthMethod, AuthConfig};

async fn run_daemon_with_server() -> Result<(), Box<dyn std::error::Error>> {
    // Configure authentication
    let auth_config = AuthConfig::new()
        .with_psk("your_secret_key")
        .with_allowed_methods(&[AuthMethod::PreSharedKey]);
    
    // Create daemon configuration with server settings
    let mut config = DaemonConfig::default();
    config.server = Some(ServerConfig {
        bind_address: "0.0.0.0".to_string(),
        port: 8716,
        auth_config,
        ..Default::default()
    });
    
    // Create and start daemon with integrated server
    let mut daemon = Daemon::new(config);
    daemon.start().await?;
    
    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    
    // Graceful shutdown
    daemon.stop().await?;
    
    Ok(())
}
```

### Implementing Services

```rust
use rcp_server::{Service, ServiceContext, ServiceConfig, Session, Frame};
use async_trait::async_trait;

struct MyCustomService {
    // Service state...
}

#[async_trait]
impl Service for MyCustomService {
    async fn initialize(&mut self, _config: ServiceConfig) -> rcp_server::Result<()> {
        // Initialize service
        Ok(())
    }
    
    async fn client_subscribed(&mut self, session: &Session, ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle new client subscription
        println!("Client {} subscribed to service", session.client_name);
        Ok(())
    }
    
    async fn handle_frame(&mut self, frame: Frame, session: &Session, ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle client request
        println!("Received frame from client {}: {:02x}", session.client_name, frame.command_id());
        
        // Send response if needed
        let response = Frame::new(0x42, b"response data".to_vec());
        ctx.send_frame(response, session).await?;
        
        Ok(())
    }
    
    async fn client_unsubscribed(&mut self, session: &Session, _ctx: &mut ServiceContext) -> rcp_server::Result<()> {
        // Handle client unsubscription
        println!("Client {} unsubscribed from service", session.client_name);
        Ok(())
    }
}
```

### Registering Services with the Server

```rust
async fn register_services(service: &mut Service) -> Result<(), Box<dyn std::error::Error>> {
    // Get server component from the integrated service
    let server = service.get_server_component()?;
    
    // Register built-in services
    server.register_service("display", Box::new(DisplayService::new())).await?;
    server.register_service("input", Box::new(InputService::new())).await?;
    server.register_service("clipboard", Box::new(ClipboardService::new())).await?;
    
    // Register custom services
    server.register_service("custom", Box::new(MyCustomService{})).await?;
    
    Ok(())
}
```

### Basic Daemon Implementation

This example shows how to configure and run the unified RCP daemon:

```rust
use rcpdaemon::{Daemon, DaemonConfig, DaemonError};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = std::env::var("RCP_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/etc/rcp/config.toml"));
    
    let config = DaemonConfig::from_file(config_path)?;
    
    // Create and start the unified daemon
    // (includes server capabilities by default)
    let mut daemon = Daemon::new(config);
    
    // Initialize the daemon
    daemon.init().await?;
    
    // Start the daemon (which will start the integrated server)
    daemon.start().await?;
    
    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    
    // Graceful shutdown
    daemon.stop().await?;
    
    Ok(())
}
```

### Configuration Management

```rust
use rcpdaemon::{ConfigManager, ConfigError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    app_name: String,
    app_path: String,
    arguments: Vec<String>,
    allowed_users: Vec<String>,
}

async fn manage_config() -> Result<(), ConfigError> {
    let mut config_manager = ConfigManager::new("/etc/rcp/apps.toml");
    
    // Load existing configuration
    config_manager.load().await?;
    
    // Add a new app configuration
    let app = AppConfig {
        app_name: "Notepad".to_string(),
        app_path: "C:\\Windows\\System32\\notepad.exe".to_string(),
        arguments: vec![],
        allowed_users: vec!["admin".to_string(), "user1".to_string()],
    };
    
    config_manager.set("apps.notepad", app).await?;
    
    // Save configuration
    config_manager.save().await?;
    
    Ok(())
}
```

### Server Management

```rust
use rcpdaemon::{ServerManager, ServerConfig, ServerError};

async fn manage_servers(daemon: &mut Daemon) -> Result<(), ServerError> {
    // Get the server manager
    let server_mgr = daemon.server_manager();
    
    // Create a new server configuration
    let config = ServerConfig {
        name: "main".to_string(),
        port: 8716,
        max_connections: 100,
        tls_enabled: false,
        ..Default::default()
    };
    
    // Create and start a new server
    server_mgr.create_server("main", config).await?;
    server_mgr.start_server("main").await?;
    
    // Get server status
    let status = server_mgr.get_server_status("main").await?;
    println!("Server status: {:?}", status);
    
    // Stop the server when needed
    server_mgr.stop_server("main").await?;
    
    Ok(())
}
```

## CLI Implementation

Implementing CLI commands for administration:

### Basic CLI Command Structure

```rust
use rcp_cli::{Cli, CliConfig, Command, Result as CliResult};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "rcp-cli", about = "RCP command-line interface")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Server management commands
    Server {
        #[clap(subcommand)]
        action: ServerAction,
    },
    
    /// User management commands
    User {
        #[clap(subcommand)]
        action: UserAction,
    },
    
    /// Show status information
    Status,
}

#[derive(Subcommand)]
enum ServerAction {
    /// Start the server
    Start {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },
    /// Stop the server
    Stop {
        /// Server name
        #[clap(default_value = "default")]
        name: String,
    },
}

#[derive(Subcommand)]
enum UserAction {
    /// List users
    List,
    /// Add a user
    Add {
        /// Username
        username: String,
        /// Password
        #[clap(short, long)]
        password: Option<String>,
    },
}

#[tokio::main]
async fn main() -> CliResult<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Create CLI configuration
    let config = CliConfig::default();
    
    // Create CLI instance
    let mut cli = Cli::new(config);
    
    // Connect to service
    cli.connect().await?;
    
    // Handle commands
    match args.command {
        Commands::Server { action } => match action {
            ServerAction::Start { name } => {
                println!("Starting server '{}'...", name);
                cli.start_server(&name).await?;
                println!("Server started successfully!");
            }
            ServerAction::Stop { name } => {
                println!("Stopping server '{}'...", name);
                cli.stop_server(&name).await?;
                println!("Server stopped successfully!");
            }
        },
        Commands::User { action } => match action {
            UserAction::List => {
                let users = cli.list_users().await?;
                println!("Users:");
                for user in users {
                    println!("- {} ({})", user.username, user.role);
                }
            }
            UserAction::Add { username, password } => {
                // Prompt for password if not provided
                let password = match password {
                    Some(pass) => pass,
                    None => rpassword::prompt_password("Enter password: ")?,
                };
                
                cli.add_user(&username, &password, "user").await?;
                println!("User '{}' added successfully!", username);
            }
        },
        Commands::Status => {
            let status = cli.get_status().await?;
            println!("RCP Status:");
            println!("- Service: {}", status.service_status);
            println!("- Uptime: {} seconds", status.uptime);
            println!("- Active servers: {}", status.active_servers.len());
            println!("- Active connections: {}", status.active_connections);
        }
    }
    
    // Disconnect from service
    cli.disconnect().await?;
    
    Ok(())
}
```

### API Component Implementation

The API component is integrated into the RCP Service and enabled via the "api" feature flag:

```rust
// Cargo.toml
// [dependencies]
// rcpdaemon = { version = "0.2.0", features = ["api"] }

use rcpdaemon::{ServiceConfig, Service, ApiConfig};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service configuration with API settings
    let mut config = ServiceConfig::default();
    
    // Configure the integrated API component
    config.api = Some(ApiConfig {
        bind_address: "0.0.0.0".to_string(),
        port: 8080,
        auth_token_expiry: 3600,
        ..Default::default()
    });
    
    // Create service with API component enabled via feature flag
    let mut service = Service::new(config);
    
    // Start the service (which will start both server and API components)
    service.start().await?;
    println!("Service started with integrated API on http://0.0.0.0:8080");
    
    // Wait for shutdown signal
    signal::ctrl_c().await?;
    
    // Gracefully shutdown
    service.stop().await?;
    
    Ok(())
}
```

### API Routes Implementation

```rust
use rcp_api::{Router, handler, Json, Response, State};

// Define state
struct AppState {
    service_client: rcpdaemon::client::ServiceClient,
}

// Create router
fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/status", handler::get(get_status))
        .route("/api/v1/servers", handler::get(list_servers))
        .route("/api/v1/servers/:id", handler::get(get_server).delete(delete_server))
        .route("/api/v1/users", handler::get(list_users).post(create_user))
}

// Handler functions
async fn get_status(State(state): State<AppState>) -> Result<Json<serde_json::Value>, Response> {
    let status = state.service_client.get_status().await?;
    let json = serde_json::json!({
        "service": {
            "status": status.state,
            "uptime": status.uptime,
            "version": env!("CARGO_PKG_VERSION"),
        },
        "resources": {
            "cpu_usage": status.cpu_usage,
            "memory_usage": status.memory_usage,
        }
    });
    
    Ok(Json(json))
}

async fn list_servers(State(state): State<AppState>) -> Result<Json<serde_json::Value>, Response> {
    let servers = state.service_client.list_servers().await?;
    let servers_json = servers.into_iter().map(|s| {
        serde_json::json!({
            "id": s.id,
            "name": s.name,
            "status": s.status,
            "port": s.port,
            "connections": s.active_connections,
        })
    }).collect::<Vec<_>>();
    
    Ok(Json(serde_json::json!({ "servers": servers_json })))
}

// Other handler functions...
```

## Desk Implementation

The RCP Desk provides a unified management interface with both web and desktop support:

### SvelteKit Web Interface Structure

```typescript
// src/routes/+layout.svelte
<script lang="ts">
  import { page } from '$app/stores';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Header from '$lib/components/Header.svelte';
  import { onMount } from 'svelte';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  onMount(() => {
    // Check authentication on mount
    if (!$authStore.authenticated && !page.url.pathname.startsWith('/login')) {
      goto('/login');
    }
  });
</script>

<div class="app-container">
  {#if $authStore.authenticated}
    <Sidebar />
    <div class="content">
      <Header />
      <main>
        <slot />
      </main>
    </div>
  {:else}
    <main class="auth-layout">
      <slot />
    </main>
  {/if}
</div>

<style>
  /* Component styles... */
</style>
```

### API Integration

```typescript
// src/lib/api/client.ts
import { authStore } from '../stores/auth';
import { get } from 'svelte/store';

const API_BASE = import.meta.env.VITE_API_URL || '/api/v1';

export type ApiResponse<T = any> = {
  data?: T;
  error?: string;
  status: number;
};

async function request<T = any>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  const { token } = get(authStore);
  
  const headers = new Headers(options.headers);
  
  if (token) {
    headers.set('Authorization', `Bearer ${token}`);
  }
  
  headers.set('Content-Type', 'application/json');
  
  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers
  });
  
  const status = response.status;
  
  try {
    if (status === 204) {
      return { status, data: null as any };
    }
    
    const data = await response.json();
    
    if (status >= 400) {
      return {
        status,
        error: data.message || 'Unknown error'
      };
    }
    
    return { status, data };
  } catch (e) {
    return {
      status,
      error: 'Failed to parse response'
    };
  }
}

export const api = {
  get: <T = any>(endpoint: string) => request<T>(endpoint),
  
  post: <T = any>(endpoint: string, data: any) => request<T>(endpoint, {
    method: 'POST',
    body: JSON.stringify(data)
  }),
  
  put: <T = any>(endpoint: string, data: any) => request<T>(endpoint, {
    method: 'PUT',
    body: JSON.stringify(data)
  }),
  
  delete: <T = any>(endpoint: string) => request<T>(endpoint, {
    method: 'DELETE'
  })
};
```

### Tauri Desktop Integration

```rust
// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};
use tauri_plugin_store::StoreBuilder;
use std::path::PathBuf;

// RCP Service client integration
mod rcp_client;

// Command handlers
mod commands {
    use super::*;
    use tauri::{command, State};
    use serde::{Serialize, Deserialize};
    
    pub struct ServiceClientState(pub rcp_client::ServiceClient);
    
    #[derive(Serialize)]
    pub struct StatusResponse {
        status: String,
        uptime: u64,
        connections: u32,
    }
    
    #[command]
    pub async fn get_status(
        client: State<'_, ServiceClientState>
    ) -> Result<StatusResponse, String> {
        client.0.get_status()
            .await
            .map(|s| StatusResponse {
                status: s.status,
                uptime: s.uptime,
                connections: s.active_connections,
            })
            .map_err(|e| e.to_string())
    }
    
    // Other command handlers...
}

fn main() {
    let context = tauri::generate_context!();
    
    tauri::Builder::default()
        .setup(|app| {
            // Setup secure storage
            let store_path = PathBuf::from(".settings.dat");
            
            let store = StoreBuilder::new(app.handle(), store_path)
                .encrypted()
                .build();
                
            app.manage(store);
            
            // Create RCP service client
            let service_client = rcp_client::ServiceClient::new();
            app.manage(commands::ServiceClientState(service_client));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            // Other command handlers...
        ])
        .run(context)
        .expect("error while running tauri application");
}
```

## Authentication

RCP supports multiple authentication mechanisms:

### Pre-Shared Key (PSK)

The simplest authentication method using a shared secret key.

**Server configuration:**
```rust
let auth_config = AuthConfig::new()
    .with_psk("your_secret_key")
    .with_allowed_methods(&[AuthMethod::PreSharedKey]);
```

**Client configuration:**
```rust
let config = ClientConfig {
    auth_method: AuthMethod::PreSharedKey,
    psk: Some("your_secret_key".to_string()),
    ..Default::default()
};
```

### Certificate-Based (Future Implementation)

For enhanced security, certificate-based authentication will be supported in future versions.

## Error Handling

RCP uses proper error types and propagation throughout the codebase:

```rust
use rcp_client::Error;

async fn handle_errors() -> Result<(), Error> {
    let mut client = Client::new(ClientConfig::default());
    
    match client.connect().await {
        Ok(_) => println!("Connected successfully"),
        Err(Error::Connection(msg)) => println!("Connection error: {}", msg),
        Err(Error::Timeout(msg)) => println!("Connection timed out: {}", msg),
        Err(e) => println!("Other error: {}", e),
    }
    
    Ok(())
}
```

## Best Practices

1. **Authentication**: Always use secure authentication methods and protect credentials.
2. **Error Handling**: Implement proper error handling and recovery mechanisms.
3. **Reconnection**: Enable automatic reconnection for better user experience during network issues.
4. **Resource Management**: Properly close connections and unsubscribe from services when done.
5. **Permissions**: Implement appropriate permissions on the server to restrict client actions.
6. **Service Management**: Use the RCP Service for application lifecycle management.
7. **Configuration**: Keep configuration in a centralized location with appropriate access controls.
8. **Monitoring**: Implement health checks and monitoring for all components.
9. **Security**: Follow the principle of least privilege for all operations.
10. **Logging**: Implement structured logging for easier debugging and audit trails.

## Examples

See the `examples/` directory for complete working examples:

- `client_example.rs`: Basic RCP client usage
- `integrated_service_example.rs`: Using the integrated service with server component
- `integrated_api_example.rs`: Using the integrated service with API component enabled
- `custom_service.rs`: Implementing a custom service
- `service_example.rs`: RCP Service configuration
- `cli_example.rs`: CLI implementation examples
- `desk_example/`: Desk UI examples for both web and desktop

---

For more information, refer to the [Architecture Overview](architecture.md), [Protocol Specification](protocol-specification.md), [rcpdaemon](rcpdaemon.md), [RCP CLI](rcp-cli.md), [RCP API](rcp-api.md), [RCP Admin](rcp-admin.md), and [RCP Desk](rcp-desk.md) documents.