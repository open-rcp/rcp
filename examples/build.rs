use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get the paths to our SDL DLL directories
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = Path::new(&manifest_dir).parent().unwrap();
    
    let sdl2_bin = root_dir.join("sdl2-bin");
    let sdl2_image_bin = root_dir.join("sdl2_image-bin");
    let sdl2_image_optional = sdl2_image_bin.join("optional");
    
    // Copy SDL DLLs to target directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.ancestors().nth(3).unwrap(); // Go up to target/debug or target/release
    
    // Copy SDL2.dll
    if let Err(e) = copy_files(&sdl2_bin, target_dir) {
        println!("cargo:warning=Failed to copy SDL2 DLLs: {}", e);
    } else {
        println!("cargo:warning=Copied SDL2.dll to target directory");
    }
    
    // Copy SDL2_image.dll
    if let Err(e) = copy_files(&sdl2_image_bin, target_dir) {
        println!("cargo:warning=Failed to copy SDL2_image DLLs: {}", e);
    } else {
        println!("cargo:warning=Copied SDL2_image.dll to target directory");
    }
    
    // Copy optional dependency DLLs for SDL2_image
    if sdl2_image_optional.exists() {
        if let Err(e) = copy_files(&sdl2_image_optional, target_dir) {
            println!("cargo:warning=Failed to copy optional SDL2_image dependency DLLs: {}", e);
        } else {
            println!("cargo:warning=Copied optional SDL2_image dependency DLLs to target directory");
        }
    }
    
    // Since we're using the bundled feature with static linking,
    // we don't need to specify library search paths
    println!("cargo:warning=Using SDL2 bundled feature - libraries will be compiled from source");
}

fn copy_files(from_dir: &Path, to_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "dll") {
            let file_name = path.file_name().unwrap();
            let target_path = to_dir.join(file_name);
            println!("cargo:warning=Copying {} to {}", path.display(), target_path.display());
            fs::copy(&path, &target_path)?;
        }
    }
    Ok(())
}