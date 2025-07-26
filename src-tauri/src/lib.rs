mod p2p;
mod system;

use p2p::{P2PNode, P2PEvent};
use system::{SystemMonitor, SystemInfo};
use std::sync::Mutex;
use tokio::sync::mpsc;

pub struct AppState {
    p2p_node: Mutex<Option<P2PNode>>,
    system_monitor: Mutex<SystemMonitor>,
    event_sender: Mutex<Option<mpsc::UnboundedSender<P2PEvent>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            p2p_node: Mutex::new(None),
            system_monitor: Mutex::new(SystemMonitor::new()),
            event_sender: Mutex::new(None),
        }
    }
}

#[tauri::command]
async fn start_node(window: tauri::Window, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
    
    if p2p_guard.is_some() {
        return Err("P2P node is already running".to_string());
    }

    let (mut p2p_node, mut event_receiver) = P2PNode::new()
        .await
        .map_err(|e| format!("Failed to create P2P node: {}", e))?;

    p2p_node.start()
        .await
        .map_err(|e| format!("Failed to start P2P node: {}", e))?;

    let event_sender = p2p_node.event_sender.clone();
    *state.event_sender.lock().map_err(|e| e.to_string())? = Some(event_sender);

    // Start event loop in a separate task
    let window_clone = window.clone();
    tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                P2PEvent::PeerConnected { peer_id } => {
                    let _ = window_clone.emit("p2p_event", serde_json::json!({
                        "type": "PEER_CONNECTED",
                        "payload": { "peer_id": peer_id }
                    }));
                }
                P2PEvent::PeerDisconnected { peer_id } => {
                    let _ = window_clone.emit("p2p_event", serde_json::json!({
                        "type": "PEER_DISCONNECTED",
                        "payload": { "peer_id": peer_id }
                    }));
                }
                P2PEvent::StatusUpdate { status_text } => {
                    let _ = window_clone.emit("p2p_event", serde_json::json!({
                        "type": "STATUS_UPDATE",
                        "payload": { "status_text": status_text }
                    }));
                }
                P2PEvent::PeerCount { count } => {
                    let _ = window_clone.emit("p2p_event", serde_json::json!({
                        "type": "PEER_COUNT",
                        "payload": { "count": count }
                    }));
                }
            }
        }
    });

    // Start the main event loop in a separate task
    let window_clone = window.clone();
    tokio::spawn(async move {
        if let Err(e) = p2p_node.run_event_loop().await {
            let _ = window_clone.emit("p2p_event", serde_json::json!({
                "type": "ERROR",
                "payload": { "error": e.to_string() }
            }));
        }
    });

    *p2p_guard = Some(p2p_node);

    Ok(())
}

#[tauri::command]
async fn stop_node(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
    
    if p2p_guard.is_none() {
        return Err("P2P node is not running".to_string());
    }

    // Clear the event sender
    *state.event_sender.lock().map_err(|e| e.to_string())? = None;
    
    // Drop the P2P node
    *p2p_guard = None;

    Ok(())
}

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
        .expect("error while running tauri application");
}
