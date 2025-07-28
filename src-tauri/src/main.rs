// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Main entry point for the Mycelium application
/// 
/// This function initializes the Tauri application and starts the main event loop.
/// It handles the desktop application lifecycle and delegates to the library's run function.
/// 
/// # Returns
/// 
/// This function does not return as it runs the application event loop.
/// 
/// # Errors
/// 
/// If the application fails to start, it will log an error and exit with code 1.
fn main() {
    mycelium_app_lib::run()
}
