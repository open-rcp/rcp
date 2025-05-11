// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Use the crate name to import the lib functionality
use rcp_admin_lib as lib;

fn main() {
    // Call the run function from our lib module
    lib::run()
}
