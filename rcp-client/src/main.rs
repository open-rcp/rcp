use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rcp_client::Client;
use rcp_core::AuthMethod;
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

/// RCP Client - Command line interface for Rust Control Protocol
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Server hostname or IP address
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    /// Server port
    #[arg(short, long, default_value_t = rcp_client::DEFAULT_PORT)]
    port: u16,

    /// Client name/description
    #[arg(long, default_value = "RCP CLI Client")]
    client_name: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a remote server
    Connect {
        /// Connection string in the format [user[:pass]@]host[:port][/path]
        #[arg(value_name = "CONNECTION_STRING")]
        connection_string: Option<String>,

        /// Pre-shared key for authentication
        #[arg(short, long)]
        psk: Option<String>,
    },

    /// Execute a command on the remote server
    Execute {
        /// Connection string in the format [user[:pass]@]host[:port][/path]
        #[arg(value_name = "CONNECTION_STRING")]
        connection_string: Option<String>,

        /// Command to execute
        command: String,

        /// Command arguments
        args: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Configure logging
    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    // Initialize the logging subscriber
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    // Process command
    match &cli.command {
        Some(Commands::Connect { connection_string, psk }) => {
            // Create client builder based on connection string or command line arguments
            let mut builder = Client::builder();
            
            if let Some(conn_str) = connection_string {
                // Use connection string
                builder = builder.connection_string(conn_str)
                    .context("Failed to parse connection string")?;
                
                // Log connection details from the parsed connection string
                tracing::info!("Connecting using connection string: {}", conn_str);
            } else {
                // Use command line arguments
                builder = builder
                    .host(cli.host.clone())
                    .port(cli.port)
                    .client_name(cli.client_name.clone());
                
                tracing::info!("Connecting to server at {}:{}", cli.host, cli.port);
            }
            
            // Set authentication method and PSK if provided
            builder = builder
                .client_id(Uuid::new_v4())
                .auth_method(AuthMethod::PreSharedKey);
                
            if let Some(auth_psk) = psk {
                builder = builder.auth_psk(auth_psk);
            }
            
            // Build the client
            let client = builder.build();

            // Connect and authenticate
            client.connect().await?;
            tracing::info!("Connected successfully, authenticating...");
            client.authenticate().await?;
            tracing::info!("Authentication successful");

            // Start the client message processor
            client.start().await?;
            tracing::info!("Client started, press Ctrl+C to disconnect");

            // Keep the connection open until user interrupts
            tokio::signal::ctrl_c().await?;
            tracing::info!("Received interrupt signal, disconnecting...");

            // Disconnect
            client.disconnect().await?;
            tracing::info!("Disconnected successfully");
        }

        Some(Commands::Execute { connection_string, command, args }) => {
            // Create client builder based on connection string or command line arguments
            let mut builder = Client::builder();
            
            if let Some(conn_str) = connection_string {
                // Use connection string
                builder = builder.connection_string(conn_str)
                    .context("Failed to parse connection string")?;
                
                // Log connection details from the parsed connection string
                tracing::info!("Connecting using connection string: {}", conn_str);
            } else {
                // Use command line arguments
                builder = builder
                    .host(cli.host.clone())
                    .port(cli.port)
                    .client_name(cli.client_name.clone());
                
                tracing::info!("Connecting to server at {}:{}", cli.host, cli.port);
            }
            
            // Set authentication method
            builder = builder
                .client_id(Uuid::new_v4())
                .auth_method(AuthMethod::PreSharedKey);
                
            // Build the client
            let client = builder.build();

            // Connect and authenticate
            client.connect_and_authenticate().await?;
            tracing::info!("Connection established and authenticated successfully");

            tracing::info!("Executing command: {} {:?}", command, args);
            // You would implement command execution logic here
            // For example:
            // client.execute_command(&command, &args).await?;

            tracing::info!("Command executed successfully");

            // Disconnect
            client.disconnect().await?;
        }

        None => {
            tracing::info!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
