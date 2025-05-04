use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let static_dir = Path::new(&out_dir).join("static");
    
    // Create the static directory if it doesn't exist
    std::fs::create_dir_all(&static_dir).unwrap();
    
    // Only run npm commands if npm is available
    if Command::new("npm").arg("--version").output().is_ok() {
        println!("Building Svelte frontend...");
        
        // Install dependencies if node_modules doesn't exist
        if !Path::new("node_modules").exists() {
            println!("Installing dependencies...");
            let status = Command::new("npm")
                .arg("install")
                .status()
                .expect("Failed to run npm install");
                
            if !status.success() {
                panic!("Failed to install dependencies");
            }
        }
        
        // Build the Svelte app
        println!("Building Svelte app...");
        let status = Command::new("npm")
            .arg("run")
            .arg("build")
            .status()
            .expect("Failed to run npm build");
            
        if !status.success() {
            panic!("Failed to build Svelte app");
        }
        
        // Copy the built files to the output directory
        println!("Copying built files to output directory...");
        if cfg!(windows) {
            Command::new("powershell")
                .args(["-c", &format!("Copy-Item -Path build/* -Destination {} -Recurse -Force", static_dir.display())])
                .status()
                .expect("Failed to copy built files");
        } else {
            Command::new("cp")
                .args(["-r", "build/", &static_dir.to_string_lossy()])
                .status()
                .expect("Failed to copy built files");
        }
        
        println!("Frontend build completed successfully!");
    } else {
        println!("npm not found, skipping frontend build");
    }
}