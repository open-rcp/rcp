// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import the lib module so we can access it
mod lib;

fn main() {
    // Call the run function from our lib module
    lib::run()
}
