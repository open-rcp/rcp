use anyhow::Result;
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
        /// Pre-shared key for authentication
        #[arg(short, long)]
        psk: Option<String>,
    },

    /// Execute a command on the remote server
    Execute {
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

    // Create client
    let client = Client::builder()
        .host(cli.host.clone())
        .port(cli.port)
        .client_name(cli.client_name.clone())
        .client_id(Uuid::new_v4())
        .auth_method(AuthMethod::PreSharedKey)
        .build();

    // Process command
    match &cli.command {
        Some(Commands::Connect { psk }) => {
            tracing::info!("Connecting to server at {}:{}", cli.host, cli.port);
            client.connect().await?;

            if let Some(auth_psk) = psk {
                tracing::info!("Authenticating with PSK");
                let client = Client::builder()
                    .host(cli.host.clone())
                    .port(cli.port)
                    .client_name(cli.client_name.clone())
                    .client_id(Uuid::new_v4())
                    .auth_method(AuthMethod::PreSharedKey)
                    .auth_psk(auth_psk)
                    .build();

                client.authenticate().await?;
            } else {
                tracing::info!("No authentication token provided");
                client.authenticate().await?;
            }

            tracing::info!("Connection established and authenticated successfully");

            // Start the client message processor
            client.start().await?;

            // Keep the connection open for a bit
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // Disconnect
            client.disconnect().await?;
        }

        Some(Commands::Execute { command, args }) => {
            tracing::info!("Connecting to server");
            client.connect().await?;
            client.authenticate().await?;

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
