use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Check if we're on macOS
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        // Try to find the SDL2 Framework location using the sdl2-config tool
        let output = Command::new("sh")
            .arg("-c")
            .arg("which sdl2-config")
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                // We found sdl2-config, use it to get the library paths
                let sdl2_config = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                // Get the library paths
                let lib_path_output = Command::new(&sdl2_config)
                    .arg("--libs")
                    .output()
                    .expect("Failed to execute sdl2-config");
                
                if lib_path_output.status.success() {
                    let lib_path = String::from_utf8_lossy(&lib_path_output.stdout)
                        .trim()
                        .to_string();
                    println!("SDL2 lib path: {}", lib_path);
                    
                    // Just pass these flags directly to the linker
                    println!("cargo:rustc-flags={}", lib_path);
                }
            } else {
                // Fallback to standard Homebrew paths on macOS
                println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
                println!("cargo:rustc-link-lib=SDL2");
                println!("cargo:rustc-link-lib=SDL2_image");
                println!("cargo:rustc-link-lib=SDL2_ttf");
                println!("cargo:rustc-link-lib=SDL2_gfx");
            }
        } else {
            // Fallback to standard Homebrew paths on macOS
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:rustc-link-lib=SDL2");
            println!("cargo:rustc-link-lib=SDL2_image");
            println!("cargo:rustc-link-lib=SDL2_ttf");
            println!("cargo:rustc-link-lib=SDL2_gfx");
        }
    } else {
        // For other platforms, use pkg-config (Linux, etc.)
        if let Ok(_) = pkg_config::find_library("sdl2") {
            println!("Using pkg-config to find SDL2");
            return;
        }
        
        // Fallback: Link against SDL2 directly
        println!("cargo:rustc-link-lib=SDL2");
    }
    
    // Tell cargo to invalidate the built crate whenever this build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
