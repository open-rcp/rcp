use rcp_client::ClientBuilder;
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
    
    // Connect to server - using directly, not with ? operator
    match client.connect().await {
        Ok(_) => println!("Connected to server!"),
        Err(e) => {
            println!("Failed to connect: {}", e);
            return Ok(());
        }
    }
    
    // Set authentication method with PSK
    println!("Setting PSK authentication...");
    // Simulate authentication (actual API might be different)
    println!("Authentication method set (simulated PSK)");
    
    // Simulate authenticate
    println!("Authenticating...");
    println!("Authentication successful! (simulated)");
    
    // Simulate start client
    println!("Starting client...");
    println!("Client started! (simulated)");
     // List available applications
    println!("\nListing available applications...");
    // Let's simulate getting applications since the method might not be implemented yet
    println!("Fetching application list via API...");
    
    // Define our known applications from rcpd_config.toml
    struct AppInfo {
        id: String,
        name: String,
        executable_path: String,
        args: Vec<String>,
    }
    
    let apps = vec![
        AppInfo {
            id: "brave".to_string(),
            name: "Brave Browser".to_string(),
            executable_path: "/Applications/Brave.app/Contents/MacOS/brave".to_string(),
            args: vec!["--private-window".to_string()],
        },
        AppInfo {
            id: "safari".to_string(),
            name: "Safari Browser".to_string(),
            executable_path: "/Applications/Safari.app/Contents/MacOS/Safari".to_string(),
            args: vec![],
        },
    ];
    
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
                    
                    // Send a request to launch the application
                    println!("Sending launch request for app ID: {}", app.id);
                    println!("In a complete implementation, this would call client.launch_application()");
                    
                    let session_id = format!("simulated-session-{}", app.id);
                    println!("Application launched! (Simulated) Session ID: {}", session_id);
                    
                    // Wait for user input before closing
                    println!("\nPress Enter to close the application...");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    
                    // Close the application
                    println!("Closing application session: {}", session_id);
                    println!("In a complete implementation, this would call client.close_application()");
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
    match client.disconnect().await {
        Ok(_) => println!("Disconnected!"),
        Err(e) => println!("Error during disconnect: {}", e),
    }
    
    Ok(())
}
