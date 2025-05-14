use rcp_sdl_client::{AppState, EguiSDL2, RcpClient};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::error::Error;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use egui::Color32;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create the tokio runtime manually
    let rt = Runtime::new()?;
    
    // Execute the async main function on the runtime
    rt.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    // Create window
    let window = video_subsystem
        .window("RCP Client", 1024, 768)
        .position_centered()
        .resizable()
        .build()?;
    
    // Create renderer
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()?;
        
    // Initialize egui integration
    let mut egui = EguiSDL2::new();
    
    // Initialize app state
    let state = Arc::new(Mutex::new(AppState::new()));
    
    // Event loop
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle SDL events
        for event in event_pump.poll_iter() {
            // Forward events to egui
            egui.process_event(&event);
            
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        
        // Begin egui frame
        egui.begin_frame(canvas.window());
        
        // All UI rendering happens in this block
        {
            let ctx = egui.context();
            let state_ref = state.clone();
            let mut state = state_ref.lock().unwrap();
            
            // If there's an error, show it
            let error_to_display = state.error_message.clone();
            if let Some(error) = error_to_display {
                egui::Window::new("Error")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(&ctx, |ui| {
                        ui.label(&error);
                        if ui.button("OK").clicked() {
                            state.error_message = None;
                        }
                    });
            }
            
            // Connection screen
            if !state.connected {
                egui::Window::new("Connect to RCP Server")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .fixed_size([320.0, 200.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(&ctx, |ui| {
                        ui.heading("RCP Client");
                        
                        ui.add_space(10.0);
                        ui.label("Server:");
                        ui.text_edit_singleline(&mut state.host);
                        
                        ui.add_space(5.0);
                        ui.label("Port:");
                        ui.add(egui::DragValue::new(&mut state.port).speed(1));
                        
                        ui.add_space(15.0);
                        if ui.button("Connect").clicked() {
                            // Clone only the necessary fields for connection
                            let host = state.host.clone();
                            let port = state.port;
                            let state_for_closure = state_ref.clone();
                            
                            // Drop the mutex guard before starting async work
                            drop(state);
                            
                            // Create a separate task to connect
                            tokio::spawn(async move {
                                // Create client and connect
                                let client = rcp_client::ClientBuilder::new()
                                    .host(&host)
                                    .port(port)
                                    .client_name("RCP-SDL-Client")
                                    .build();
                                
                                let result = client.connect().await;
                                
                                // Update state after connection
                                let mut state = state_for_closure.lock().unwrap();
                                match result {
                                    Ok(_) => {
                                        state.connected = true;
                                        state.error_message = None;
                                        // Store the client in our wrapper
                                        state.client = RcpClient::new();
                                        state.client.set_client(client);
                                    },
                                    Err(e) => {
                                        let error_msg = format!("Failed to connect: {}", e);
                                        state.error_message = Some(error_msg);
                                    }
                                }
                            });
                        }
                    });
            } 
            // Login screen
            else if !state.authenticated {
                egui::Window::new("Login")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .fixed_size([320.0, 230.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(&ctx, |ui| {
                        ui.heading("Log In");
                        
                        ui.add_space(10.0);
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut state.username);
                        
                        ui.add_space(5.0);
                        ui.label("Password:");
                        ui.add(egui::TextEdit::singleline(&mut state.password)
                            .password(true));
                        
                        ui.add_space(10.0);
                        ui.checkbox(&mut state.remember_credentials, "Remember credentials");
                        
                        ui.add_space(15.0);
                        if ui.button("Log In").clicked() {
                            // Clone only the necessary fields and drop the MutexGuard
                            let username = state.username.clone();
                            let password = state.password.clone();
                            let state_for_closure = state_ref.clone();
                            
                            // Check if we have a client before dropping the mutex
                            let has_client = state.client.is_connected();
                            
                            // Drop the mutex guard before starting async work
                            drop(state);
                            
                            // Only proceed if we have a client
                            if has_client {
                                // Break down the async operation into stages to avoid holding MutexGuard across .await
                                tokio::spawn(async move {
                                    // Step 1: Get client reference and extract necessary data
                                    let client_op_result = {
                                        let state_guard = state_for_closure.lock().unwrap();
                                        if state_guard.client.is_connected() {
                                            Ok(())
                                        } else {
                                            Err("Not connected to server")
                                        }
                                    };
                                    
                                    // Step 2: Execute the authentication operations
                                    let auth_result = if client_op_result.is_ok() {
                                        // Authenticate with the server using a builder-like approach
                                        let client_builder = rcp_client::ClientBuilder::new()
                                            .host("localhost") // Use defaults for new connection
                                            .port(8717)
                                            .client_name("RCP-SDL-Client-Login");
                                        
                                        let mut temp_client = client_builder.build();
                                        
                                        // Try connecting with this temporary client
                                        match temp_client.connect().await {
                                            Ok(_) => {
                                                // Set auth method and authenticate
                                                match temp_client.set_auth_method(rcp_core::AuthMethod::Password(username, password)).await {
                                                    Ok(_) => {
                                                        match temp_client.authenticate().await {
                                                            Ok(_) => {
                                                                match temp_client.start().await {
                                                                    Ok(_) => Ok(temp_client),
                                                                    Err(e) => Err(format!("Failed to start client: {}", e))
                                                                }
                                                            },
                                                            Err(e) => Err(format!("Authentication failed: {}", e))
                                                        }
                                                    },
                                                    Err(e) => Err(format!("Failed to set auth method: {}", e))
                                                }
                                            },
                                            Err(e) => Err(format!("Failed to connect temporary client: {}", e))
                                        }
                                    } else {
                                        Err("Client not connected".to_string())
                                    };
                                    
                                    // Step 3: Update state after authentication
                                    match auth_result {
                                        Ok(authenticated_client) => {
                                            // Update the state with success
                                            {
                                                let mut state = state_for_closure.lock().unwrap();
                                                state.authenticated = true;
                                                state.error_message = None;
                                                
                                                // Replace the client with the authenticated one
                                                state.client = RcpClient::new();
                                                state.client.set_client(authenticated_client);
                                            }
                                            
                                            // Step 4: Load available applications
                                            // This is executed outside of the MutexGuard scope
                                            let apps_result: Result<Vec<rcp_sdl_client::AppInfo>, Box<dyn std::error::Error + Send + Sync>> = {
                                                // Simulate loading apps
                                                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                                                
                                                // Create sample apps (in a real implementation, this would query the server)
                                                Ok(vec![
                                                    rcp_sdl_client::AppInfo {
                                                        id: "notepad".to_string(),
                                                        name: "Notepad".to_string(),
                                                        description: "Simple text editor".to_string(),
                                                        last_used: Some("2025-05-08T14:30:00Z".to_string()),
                                                    },
                                                    rcp_sdl_client::AppInfo {
                                                        id: "calculator".to_string(),
                                                        name: "Calculator".to_string(),
                                                        description: "Basic calculator application".to_string(),
                                                        last_used: None,
                                                    },
                                                    rcp_sdl_client::AppInfo {
                                                        id: "browser".to_string(),
                                                        name: "Web Browser".to_string(),
                                                        description: "Secure web browser".to_string(),
                                                        last_used: Some("2025-05-07T09:15:00Z".to_string()),
                                                    },
                                                ])
                                            };
                                            
                                            // Update the state with the apps list
                                            let mut state = state_for_closure.lock().unwrap();
                                            match apps_result {
                                                Ok(apps) => {
                                                    state.apps = apps;
                                                },
                                                Err(e) => {
                                                    state.error_message = Some(format!("Failed to load applications: {}", e));
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            let mut state = state_for_closure.lock().unwrap();
                                            state.error_message = Some(format!("Login failed: {}", e));
                                        }
                                    }
                                });
                            }
                        }
                    });
            }
            // Application streaming screen
            else if let Some(app_id) = &state.active_app_id {
                let app_name = state.apps
                    .iter()
                    .find(|app| app.id == *app_id)
                    .map(|app| app.name.clone())
                    .unwrap_or_else(|| "Application".to_string());
                
                egui::Window::new(&app_name)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .default_size([800.0, 600.0])
                    .collapsible(false)
                    .resizable(true)
                    .show(&ctx, |ui| {
                        // This would be where the application streaming happens
                        // For now, just show placeholder
                        let available_size = ui.available_size();
                        ui.add_sized(
                            available_size - egui::vec2(0.0, 30.0),
                            egui::Label::new(
                                egui::RichText::new("Application stream would appear here")
                                .heading()
                            ).wrap(true),
                        );
                        
                        ui.add_space(5.0);
                        if ui.button("Close Application").clicked() {
                            // Store a clone for the closure
                            let state_for_closure = state_ref.clone();
                            let app_id = state.active_app_id.clone();
                            let client_available = state.client.is_connected();
                            
                            // Drop the mutex guard before starting async work
                            drop(state);
                            
                            // Only proceed if we have an app_id and a client
                            if let Some(app_id_to_close) = app_id {
                                if client_available {
                                    // Spawn a new task to close the app
                                    tokio::spawn(async move {
                                        // Step 1: Check connection status
                                        let is_connected = {
                                            let state_guard = state_for_closure.lock().unwrap();
                                            state_guard.client.is_connected()
                                        };
                                        
                                        // Step 2: Perform the close operation
                                        let close_result = if is_connected {
                                            // Simulate app closing for now
                                            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                                            println!("Closing application: {}", app_id_to_close);
                                            Ok(())
                                        } else {
                                            Err(format!("Not connected to server"))
                                        };
                                        
                                        // Step 3: Update state after closing
                                        let mut state = state_for_closure.lock().unwrap();
                                        match close_result {
                                            Ok(_) => {
                                                state.active_app_id = None;
                                                state.error_message = None;
                                            },
                                            Err(_) => {
                                                // Force close the app anyway
                                                state.active_app_id = None;
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    });
            }
            // Application launcher screen
            else {
                egui::Window::new("RCP Applications")
                    .default_size([800.0, 600.0])
                    .show(&ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Applications");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(&format!("User: {}", state.username));
                                if ui.button("Log Out").clicked() {
                                    state.authenticated = false;
                                }
                            });
                        });
                        
                        ui.separator();                                // Clone the apps to avoid borrow checker issues
                                let apps_to_display = state.apps.clone();
                                
                                // Display applications in a grid
                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        let available_width = ui.available_width();
                                        let card_width = 200.0;
                                        let cards_per_row = (available_width / (card_width + 10.0)).floor().max(1.0) as usize;
                                        
                                        egui::Grid::new("apps_grid")
                                            .num_columns(cards_per_row)
                                            .spacing([10.0, 10.0])
                                            .show(ui, |ui| {
                                                for (index, app) in apps_to_display.iter().enumerate() {
                                            ui.vertical(|ui| {
                                                // App card
                                                let response = {
                                                    let frame = egui::Frame::none()
                                                        .fill(Color32::from_rgb(250, 250, 250))
                                                        .stroke(egui::Stroke::new(1.0, Color32::from_gray(220)))
                                                        .rounding(egui::Rounding::same(8.0))
                                                        .inner_margin(egui::Margin::same(10.0))
                                                        .outer_margin(egui::Margin::same(0.0));
                                                    
                                                    frame.show(ui, |ui| {
                                                        ui.set_min_width(card_width);
                                                        ui.set_min_height(150.0);
                                                        
                                                        ui.heading(&app.name);
                                                        ui.label(&app.description);
                                                        
                                                        if let Some(last_used) = &app.last_used {
                                                            ui.label(format!("Last used: {}", last_used));
                                                        }
                                                        
                                                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {                                                                            if ui.button("Launch").clicked() {
                                                                                let app_id = app.id.clone(); 
                                                                                let app_name = app.name.clone();
                                                                                let state_for_closure = state_ref.clone();
                                                                                
                                                                                // Clone the needed info and drop the mutex guard
                                                                                // Create a new scope to prevent use-after-move error
                                                                                let client_available = {
                                                                                    let client_check = state.client.is_connected();
                                                                                    client_check
                                                                                };
                                                                                
                                                                                if client_available {
                                                                                    tokio::spawn(async move {
                                                                                        // Launch the app in separate stages to avoid MutexGuard across await
                                                                                        
                                                                                        // Step 1: Just check if we're still connected
                                                                                        let check_result = {
                                                                                            let state_guard = state_for_closure.lock().unwrap();
                                                                                            state_guard.client.is_connected()
                                                                                        };
                                                                                        
                                                                                        // Step 2: Perform the launch operation
                                                                                        let launch_result = if check_result {
                                                                                            // Simulate launching the application
                                                                                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                                                                            println!("Launching application: {}", app_id);
                                                                                            Ok(())
                                                                                        } else {
                                                                                            Err(format!("Client not connected"))
                                                                                        };
                                                                        
                                                                        // Step 3: Update the state after launch
                                                                        let mut state = state_for_closure.lock().unwrap();
                                                                        match launch_result {
                                                                            Ok(_) => {
                                                                                state.active_app_id = Some(app_id.clone());
                                                                                state.error_message = None;
                                                                            },
                                                                            Err(e) => {
                                                                                state.error_message = Some(format!("Failed to launch {}: {}", app_name, e));
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        });
                                                    }).response
                                                };
                                                
                                                response.on_hover_text(&app.name);
                                            });
                                            
                                            // Break to a new row after cards_per_row
                                            if (index + 1) % cards_per_row == 0 && index > 0 {
                                                ui.end_row();
                                            }
                                        }
                                    });
                            });
                    });
            }
        }
        
        // End egui frame and get primitives
        let (primitives, textures_delta) = egui.end_frame();
        
        // Paint the egui primitives to our SDL canvas
        egui.paint_primitives(&mut canvas, primitives, textures_delta);
        
        // Wait a bit to limit FPS
        std::thread::sleep(Duration::from_millis(10));
    }
    
    Ok(())
}

// End of main.rs
