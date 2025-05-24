use rcpcli::ClientBuilder;
use std::error::Error;
use std::io::{self};
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("RCP Enhanced Client");
    println!("-----------------");

    // Build client
    let client = ClientBuilder::new()
        .host("127.0.0.1")
        .port(8717) // RCP Server port from your rcpdaemon_config.toml
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

    println!("Connection successful. Testing API connectivity...");

    // Call the RCP API directly to verify connectivity
    let status_output = Command::new("curl")
        .arg("http://localhost:8718/v1/status")
        .output()?;

    if status_output.status.success() {
        let status_str = String::from_utf8_lossy(&status_output.stdout);
        println!("API Status: {}", status_str);
    } else {
        println!("Failed to get API status");
    }

    // Define our application structure
    struct AppInfo {
        id: String,
        name: String,
        executable_path: String,
        args: Vec<String>,
    }

    // In a real implementation, we would get this from client.get_applications()
    // For now we'll use the known applications from rcpdaemon_config.toml
    let apps = vec![
        AppInfo {
            id: "brave".to_string(),
            name: "Brave Browser".to_string(),
            executable_path: "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser"
                .to_string(),
            args: vec!["--private-window".to_string()],
        },
        AppInfo {
            id: "safari".to_string(),
            name: "Safari Browser".to_string(),
            executable_path: "/Applications/Safari.app/Contents/MacOS/Safari".to_string(),
            args: vec![],
        },
    ];

    // List available applications
    if apps.is_empty() {
        println!("No applications available.");
    } else {
        println!("\n{} application(s) available:", apps.len());

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
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            println!("Enter the app number to launch:");
            let mut app_num = String::new();
            io::stdin().read_line(&mut app_num)?;

            if let Ok(num) = app_num.trim().parse::<usize>() {
                if num > 0 && num <= apps.len() {
                    let app = &apps[num - 1];
                    println!("Launching {}...", app.name);

                    // Try to launch the app using the RCP protocol
                    println!("Sending launch request for app ID: {}", app.id);

                    // Since we don't have a send_command method implemented yet
                    // we'll go directly to the fallback mechanism
                    println!("No direct RCP application launch method available");

                    // Launch the application directly
                    println!("Attempting to launch directly as a fallback...");

                    let mut command = Command::new(&app.executable_path);
                    if !app.args.is_empty() {
                        command.args(&app.args);
                    }

                    match command.stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
                        Ok(mut child) => {
                            println!("Application launched directly! PID: {:?}", child.id());

                            // Wait for user input before closing
                            println!("\nPress Enter to close the application...");
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;

                            // Try to terminate the process
                            match child.kill() {
                                Ok(_) => println!("Application terminated"),
                                Err(e) => println!("Failed to terminate application: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to launch application directly: {}", e),
                    }
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
