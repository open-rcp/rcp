use anyhow::Result;
use rcpcli::{service::ServiceType, ClientBuilder};
use rcpcore::{AuthMethod, CommandId, Frame};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::time::Duration;
use uuid::Uuid;

// Structure that matches rcpcore::LaunchAppCommand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchAppCommand {
    /// Launch flags
    pub flags: u32,

    /// Application path to launch
    pub application_path: String,

    /// Command line arguments, if any
    pub args: Option<String>,
}

// Constants for application paths
const APP_NOTEPAD: &str = "default:notepad";
const APP_TEXTEDIT: &str = "default:textedit";
const APP_CALCULATOR: &str = "default:calculator";
const APP_BROWSER: &str = "default:browser";
const APP_TERMINAL: &str = "default:terminal";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Build client
    let client = ClientBuilder::new()
        .host("127.0.0.1") // Connect to local server
        .port(8716) // Default port
        .client_name("AppLaunchExample")
        .client_id(Uuid::new_v4())
        .auth_method(AuthMethod::PreSharedKey)
        .auth_psk("customkey".to_string())
        .build();

    println!("Connecting to RCP server...");

    // Connect to the server
    client.connect().await?;
    println!("Connected! Authenticating...");

    // Authenticate
    client.authenticate().await?;
    println!("Authentication successful!");

    // Start command processing
    client.start().await?;

    // Subscribe to the app service
    let app_service = client.subscribe_service(ServiceType::App).await?;
    println!("Subscribed to app service!");

    // Main menu loop
    loop {
        print_menu();

        match get_user_choice() {
            1 => launch_app(&app_service, APP_NOTEPAD, None).await?,
            2 => launch_app(&app_service, APP_TEXTEDIT, None).await?,
            3 => launch_app(&app_service, APP_CALCULATOR, None).await?,
            4 => launch_app(&app_service, APP_BROWSER, None).await?,
            5 => launch_app(&app_service, APP_TERMINAL, None).await?,
            6 => {
                println!("Enter the path to the application:");
                let path = get_user_input();

                println!("Enter any arguments (or leave empty for none):");
                let args = get_user_input();
                let args = if args.trim().is_empty() {
                    None
                } else {
                    Some(args)
                };

                launch_app(&app_service, &path, args).await?;
            }
            0 => break,
            _ => println!("Invalid selection, please try again."),
        }

        // Brief pause to see results
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    println!("Disconnecting...");
    client.disconnect().await?;
    println!("Disconnected. Goodbye!");

    Ok(())
}

/// Print the main menu
fn print_menu() {
    println!("\n--- RCP Application Launcher ---");
    println!("1. Launch Notepad (Windows)");
    println!("2. Launch TextEdit (macOS) / Text Editor (Linux)");
    println!("3. Launch Calculator");
    println!("4. Launch Web Browser");
    println!("5. Launch Terminal");
    println!("6. Launch Custom Application");
    println!("0. Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

/// Get user menu selection
fn get_user_choice() -> u8 {
    let input = get_user_input();
    input.trim().parse().unwrap_or(99)
}

/// Read a line of user input
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

/// Launch an application with optional arguments
async fn launch_app(
    service: &rcpcli::service::ServiceClient,
    app_path: &str,
    args: Option<String>,
) -> Result<()> {
    // Create the launch command
    let command = LaunchAppCommand {
        flags: 0, // Default flags
        application_path: app_path.to_string(),
        args,
    };

    // Serialize the command to binary format
    let payload = serde_json::to_vec(&command)?;

    // Create frame for the launch command
    let frame = Frame::new(CommandId::LaunchApp as u8, payload);

    // Send the frame to the server using the service
    service.send_fire_and_forget(frame).await?;

    println!("Launch command sent to server!");

    Ok(())
}
