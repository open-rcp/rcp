// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create a flag to track clean shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up ctrl-c handler for graceful shutdown
    ctrlc::set_handler(move || {
        println!("Received termination signal, shutting down gracefully...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
            // Any additional setup can go here
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // Prevent the app from exiting immediately
                api.prevent_exit();
                
                if running.load(Ordering::SeqCst) {
                    // Normal exit requested by user/system
                    std::process::exit(0);
                } else {
                    // Ctrl+C was pressed, exit gracefully
                    std::process::exit(0);
                }
            }
        });
}
