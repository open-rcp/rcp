use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Get the output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let _target_dir = out_dir.ancestors().nth(3).unwrap(); // Go up to target/debug or target/release

    // No external libraries are currently being used
    println!("cargo:warning=No external libraries required");
    
    // Uncomment the following line if you need to copy DLL files
    // let _ = copy_files(&_target_dir, &out_dir);
}

#[allow(dead_code)]
fn copy_files(from_dir: &Path, to_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "dll") {
            let file_name = path.file_name().unwrap();
            let target_path = to_dir.join(file_name);
            println!(
                "cargo:warning=Copying {} to {}",
                path.display(),
                target_path.display()
            );
            fs::copy(&path, &target_path)?;
        }
    }
    Ok(())
}
