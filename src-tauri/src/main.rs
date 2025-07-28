// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

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
    // Initialize logging
    env_logger::init();
    
    // Run the Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get the main window
            let window = app.get_window("main").unwrap();
            
            // Set window properties for desktop app
            window.set_title("Mycelium - Protocol Symbiosis").unwrap();
            window.set_size(tauri::LogicalSize::new(1200.0, 800.0)).unwrap();
            window.set_resizable(true).unwrap();
            window.set_min_size(Some(tauri::LogicalSize::new(800.0, 600.0))).unwrap();
            
            // Show the window
            window.show().unwrap();
            
            Ok(())
        })
        .manage(mycelium_app_lib::AppState::default())
        .invoke_handler(tauri::generate_handler![
            mycelium_app_lib::start_node,
            mycelium_app_lib::stop_node,
            mycelium_app_lib::get_system_info,
            mycelium_app_lib::get_dashboard_data,
            mycelium_app_lib::update_dashboard_data,
            mycelium_app_lib::get_active_tasks,
            mycelium_app_lib::get_task_details,
            mycelium_app_lib::pause_task,
            mycelium_app_lib::resume_task,
            mycelium_app_lib::cancel_task,
            mycelium_app_lib::get_storage_summary,
            mycelium_app_lib::get_active_fragments,
            mycelium_app_lib::get_conversations,
            mycelium_app_lib::send_message,
            mycelium_app_lib::get_permission_profiles,
            mycelium_app_lib::update_permission_settings,
            mycelium_app_lib::get_analytics_data
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Failed to run Tauri application: {}", e);
            std::process::exit(1);
        });
}
