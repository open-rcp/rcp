use rcpc::{Client, ClientBuilder, AuthMethod};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("RCP Simple Client");
    println!("----------------");
    
    // Build client
    let mut client = ClientBuilder::new()
        .host("127.0.0.1")
        .port(8717)  // RCP Server port from your rcpd_config.toml
        .client_name("RCP-Test-Client")
        .build();
    
    println!("Connecting to RCP server at 127.0.0.1:8717...");
    
    // Connect to server
    client.connect().await?;
    println!("Connected to server!");
    
    // Set authentication method with PSK
    client.set_auth_method(AuthMethod::PreSharedKey("customkey".to_string())).await?;
    println!("Authentication method set (PSK)");
    
    // Authenticate
    client.authenticate().await?;
    println!("Authentication successful!");
    
    // Start client command processing
    client.start().await?;
    println!("Client started!");
    
    // List available applications
    println!("\nListing available applications...");
    let apps = client.get_available_applications().await?;
    
    if apps.is_empty() {
        println!("No applications available.");
    } else {
        println!("{} application(s) available:", apps.len());
        
        for (i, app) in apps.iter().enumerate() {
            println!("{}. {} (ID: {})", i + 1, app.name, app.id);
            println!("   - Path: {}", app.executable_path);
            if !app.args.is_empty() {
                println!("   - Args: {:?}", app.args);
            }
        }
        
        // Ask user if they want to launch an app
        println!("\nWould you like to launch an app? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase() == "y" {
            println!("Enter the app number to launch:");
            let mut app_num = String::new();
            std::io::stdin().read_line(&mut app_num)?;
            
            if let Ok(num) = app_num.trim().parse::<usize>() {
                if num > 0 && num <= apps.len() {
                    let app = &apps[num - 1];
                    println!("Launching {}...", app.name);
                    
                    // Launch the application
                    let session_id = client.launch_application(&app.id, vec![]).await?;
                    println!("Application launched! Session ID: {}", session_id);
                    
                    // Wait for user input before closing
                    println!("\nPress Enter to close the application...");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    
                    // Close the application
                    client.close_application(&session_id).await?;
                    println!("Application closed!");
                } else {
                    println!("Invalid app number!");
                }
            } else {
                println!("Invalid input!");
            }
        }
    }
    
    // Disconnect when done
    println!("\nDisconnecting...");
    client.disconnect().await?;
    println!("Disconnected!");
    
    Ok(())
}
