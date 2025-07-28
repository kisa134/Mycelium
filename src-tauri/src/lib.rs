mod p2p;
mod system;

use p2p::{RealP2PNode, P2PEvent};
use system::{SystemMonitor, SystemInfo};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tauri::{Emitter, Runtime};

/// Application state containing P2P node and system monitoring
pub struct AppState {
    p2p_node: Mutex<Option<RealP2PNode>>,
    system_monitor: Mutex<SystemMonitor>,
    event_sender: Mutex<Option<mpsc::UnboundedSender<P2PEvent>>>,
}

impl Default for AppState {
    /// Creates a new default AppState instance
    /// 
    /// This function initializes all components with their default values:
    /// - P2P node is set to None (not running)
    /// - System monitor is created with default configuration
    /// - Event sender is set to None (no active sender)
    /// 
    /// # Returns
    /// 
    /// Returns a new AppState instance with all components in their default state
    fn default() -> Self {
        Self {
            p2p_node: Mutex::new(None),
            system_monitor: Mutex::new(SystemMonitor::new()),
            event_sender: Mutex::new(None),
        }
    }
}

/// Starts the P2P node and begins network operations
/// 
/// # Arguments
/// 
/// * `window` - Tauri window for emitting events to frontend
/// * `state` - Application state containing P2P node
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn start_node<R: Runtime>(window: tauri::Window<R>, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // Check if node is already running
    {
        let p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
        if p2p_guard.is_some() {
            return Err("P2P node is already running".to_string());
        }
    }

    let (mut p2p_node, mut event_receiver) = RealP2PNode::new()
        .await
        .map_err(|e| format!("Failed to create P2P node: {}", e))?;

    p2p_node.start()
        .await
        .map_err(|e| format!("Failed to start P2P node: {}", e))?;

    let event_sender = p2p_node.event_sender.clone();
    {
        let mut event_guard = state.event_sender.lock().map_err(|e| e.to_string())?;
        *event_guard = Some(event_sender);
    }

    // Start event loop in a separate task
    let window_clone = window.clone();
    tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                P2PEvent::StatusUpdate { status_text } => {
                    let _ = window_clone.emit("p2p_event", serde_json::json!({
                        "type": "STATUS_UPDATE",
                        "payload": { "status_text": status_text }
                    }));
                }
                P2PEvent::NetworkStatusUpdate { status } => {
                    // Safely serialize status to JSON with error handling
                    match serde_json::to_value(status) {
                        Ok(json_value) => {
                            let _ = window_clone.emit("network-status-update", json_value);
                        }
                        Err(e) => {
                            log::error!("Failed to serialize network status: {}", e);
                            let _ = window_clone.emit("p2p_event", serde_json::json!({
                                "type": "ERROR",
                                "payload": { "error": format!("Failed to serialize network status: {}", e) }
                            }));
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // Start the main event loop in a separate task
    let window_clone = window.clone();
    let mut p2p_node_clone = p2p_node.clone();
    tokio::spawn(async move {
        if let Err(e) = p2p_node_clone.run_event_loop().await {
            let _ = window_clone.emit("p2p_event", serde_json::json!({
                "type": "ERROR",
                "payload": { "error": e.to_string() }
            }));
        }
    });

    // Store the P2P node
    {
        let mut p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
        *p2p_guard = Some(p2p_node);
    }

    Ok(())
}

/// Stops the P2P node and cleans up resources
/// 
/// # Arguments
/// 
/// * `state` - Application state containing P2P node
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn stop_node(state: tauri::State<'_, AppState>) -> Result<(), String> {
    {
        let mut p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
        
        if p2p_guard.is_none() {
            return Err("P2P node is not running".to_string());
        }

        // Clear the event sender
        {
            let mut event_guard = state.event_sender.lock().map_err(|e| e.to_string())?;
            *event_guard = None;
        }
        
        // Drop the P2P node
        *p2p_guard = None;
    }

    Ok(())
}

/// Gets current system information including CPU and RAM usage
/// 
/// # Arguments
/// 
/// * `state` - Application state containing system monitor
/// 
/// # Returns
/// 
/// Returns SystemInfo on success, or an error message on failure
#[tauri::command]
fn get_system_info(state: tauri::State<'_, AppState>) -> Result<SystemInfo, String> {
    let mut monitor = state.system_monitor.lock().map_err(|e| e.to_string())?;
    Ok(monitor.get_system_info())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            start_node,
            stop_node,
            get_system_info
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Failed to run Tauri application: {}", e);
            std::process::exit(1);
        });
}
