mod p2p;
mod system;
mod ui_api;

use p2p::{RealP2PNode, P2PEvent};
use system::{SystemMonitor, SystemInfo};
use ui_api::*;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tauri::{Emitter, Runtime};
use chrono::Utc;

/// Application state containing P2P node and system monitoring
pub struct AppState {
    p2p_node: Mutex<Option<RealP2PNode>>,
    system_monitor: Mutex<SystemMonitor>,
    event_sender: Mutex<Option<mpsc::UnboundedSender<P2PEvent>>>,
    /// Dashboard data cache
    dashboard_data: Mutex<Option<DashboardData>>,
    /// Active tasks cache
    active_tasks: Mutex<Vec<ActiveTask>>,
    /// Conversations cache
    conversations: Mutex<Vec<Conversation>>,
    /// Permission profiles cache
    permission_profiles: Mutex<Vec<PermissionProfile>>,
}

impl Default for AppState {
    /// Creates a new default AppState instance
    /// 
    /// This function initializes all components with their default values:
    /// - P2P node is set to None (not running)
    /// - System monitor is created with default configuration
    /// - Event sender is set to None (no active sender)
    /// - UI data caches are initialized as empty
    /// 
    /// # Returns
    /// 
    /// Returns a new AppState instance with all components in their default state
    fn default() -> Self {
        Self {
            p2p_node: Mutex::new(None),
            system_monitor: Mutex::new(SystemMonitor::new()),
            event_sender: Mutex::new(None),
            dashboard_data: Mutex::new(None),
            active_tasks: Mutex::new(Vec::new()),
            conversations: Mutex::new(Vec::new()),
            permission_profiles: Mutex::new(Vec::new()),
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
        *event_guard = Some(event_sender.clone());
    }

    // Start the main event loop in a separate task
    let window_clone = window.clone();
    tokio::spawn(async move {
        // Real P2P event loop
        while let Some(event) = event_receiver.recv().await {
            let event_json = serde_json::json!({
                "type": match &event {
                    P2PEvent::PeerConnected { .. } => "PEER_CONNECTED",
                    P2PEvent::PeerDisconnected { .. } => "PEER_DISCONNECTED",
                    P2PEvent::StatusUpdate { .. } => "STATUS_UPDATE",
                    P2PEvent::PeerCount { .. } => "PEER_COUNT",
                    P2PEvent::NetworkStatusUpdate { .. } => "NETWORK_STATUS",
                },
                "payload": match &event {
                    P2PEvent::PeerConnected { peer_id } => serde_json::json!({ "peer_id": peer_id }),
                    P2PEvent::PeerDisconnected { peer_id } => serde_json::json!({ "peer_id": peer_id }),
                    P2PEvent::StatusUpdate { status_text } => serde_json::json!({ "status": status_text }),
                    P2PEvent::PeerCount { count } => serde_json::json!({ "count": count }),
                    P2PEvent::NetworkStatusUpdate { status } => serde_json::to_value(status).unwrap(),
                }
            });
            
            let _ = window_clone.emit("p2p_event", event_json);
        }
    });

    // Store the P2P node
    {
        let mut p2p_guard = state.p2p_node.lock().map_err(|e| e.to_string())?;
        *p2p_guard = Some(p2p_node);
    }

    // Initialize dashboard data
    initialize_dashboard_data(&state).await;

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

// ============================================================================
// DASHBOARD API COMMANDS
// ============================================================================

/// Gets dashboard data for the main interface
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns DashboardData on success, or an error message on failure
#[tauri::command]
async fn get_dashboard_data(state: tauri::State<'_, AppState>) -> Result<DashboardData, String> {
    {
        let dashboard_guard = state.dashboard_data.lock().map_err(|e| e.to_string())?;
        
        if let Some(data) = &*dashboard_guard {
            return Ok(data.clone());
        }
    }
    
    // Generate default dashboard data if not available
    let default_data = generate_default_dashboard_data().await;
    Ok(default_data)
}

/// Updates dashboard data with new information
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `data` - New dashboard data
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn update_dashboard_data(state: tauri::State<'_, AppState>, data: DashboardData) -> Result<(), String> {
    let mut dashboard_guard = state.dashboard_data.lock().map_err(|e| e.to_string())?;
    *dashboard_guard = Some(data);
    Ok(())
}

// ============================================================================
// SYNAPSE PROTOCOL API COMMANDS
// ============================================================================

/// Gets active tasks for the Synapse protocol
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns Vec<ActiveTask> on success, or an error message on failure
#[tauri::command]
async fn get_active_tasks(state: tauri::State<'_, AppState>) -> Result<Vec<ActiveTask>, String> {
    let tasks_guard = state.active_tasks.lock().map_err(|e| e.to_string())?;
    Ok(tasks_guard.clone())
}

/// Gets detailed information about a specific task
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `task_id` - Task ID to get details for
/// 
/// # Returns
/// 
/// Returns TaskDetails on success, or an error message on failure
#[tauri::command]
async fn get_task_details(state: tauri::State<'_, AppState>, task_id: String) -> Result<TaskDetails, String> {
    // For now, return a mock task details
    // In real implementation, this would query the actual task data
    let mock_task = ActiveTask {
        id: task_id.clone(),
        name: "ML Model Training".to_string(),
        aibox_id: "AIbox #8472".to_string(),
        progress: 67,
        time_remaining: Some("1ч 26м".to_string()),
        priority: TaskPriority::Critical,
        reward_tokens: 45,
        status: TaskStatus::Running,
        resource_usage: ResourceUsage {
            cpu_percent: 85,
            ram_gb: 2.1,
            gpu_percent: 92,
        },
    };

    let task_details = TaskDetails {
        task: mock_task,
        task_type: TaskType::ModelTraining,
        model: Some("GPT-4 Fine-tuning".to_string()),
        data_size_gb: 2.3,
        complexity: TaskComplexity::High,
        verification: VerificationMethod::CryptographicSignature,
        security: SecurityLevel::Isolated,
        detailed_resource_usage: DetailedResourceUsage {
            cpu_percent: 85,
            ram_gb: 2.1,
            gpu_percent: 92,
            gpu_memory_gb: 4.0,
        },
    };

    Ok(task_details)
}

/// Pauses a running task
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `task_id` - Task ID to pause
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn pause_task(state: tauri::State<'_, AppState>, task_id: String) -> Result<(), String> {
    log::info!("Pausing task: {}", task_id);
    // In real implementation, this would pause the actual task
    Ok(())
}

/// Resumes a paused task
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `task_id` - Task ID to resume
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn resume_task(state: tauri::State<'_, AppState>, task_id: String) -> Result<(), String> {
    log::info!("Resuming task: {}", task_id);
    // In real implementation, this would resume the actual task
    Ok(())
}

/// Cancels a task
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `task_id` - Task ID to cancel
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn cancel_task(state: tauri::State<'_, AppState>, task_id: String) -> Result<(), String> {
    log::info!("Cancelling task: {}", task_id);
    // In real implementation, this would cancel the actual task
    Ok(())
}

// ============================================================================
// CHRONICLE PROTOCOL API COMMANDS
// ============================================================================

/// Gets storage summary for the Chronicle protocol
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns ChronicleSummary on success, or an error message on failure
#[tauri::command]
async fn get_storage_summary(state: tauri::State<'_, AppState>) -> Result<ChronicleSummary, String> {
    // Mock storage summary
    let summary = ChronicleSummary {
        allocated_storage_gb: 50.0,
        used_storage_gb: 23.4,
        fragment_count: 1247,
        data_integrity: 99,
        geographic_distribution: GeographicDistribution {
            europe: RegionInfo { percentage: 45, fragment_count: 562 },
            asia: RegionInfo { percentage: 32, fragment_count: 399 },
            america: RegionInfo { percentage: 23, fragment_count: 286 },
        },
        storage_security: StorageSecurity {
            encryption_algorithm: "AES-256-GCM".to_string(),
            redundancy_factor: 3,
        },
    };

    Ok(summary)
}

/// Gets active fragments for the Chronicle protocol
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns Vec<ActiveFragment> on success, or an error message on failure
#[tauri::command]
async fn get_active_fragments(state: tauri::State<'_, AppState>) -> Result<Vec<ActiveFragment>, String> {
    // Mock active fragments
    let fragments = vec![
        ActiveFragment {
            name: "AI Model Data".to_string(),
            aibox_id: "AIbox #8472".to_string(),
            size_gb: 2.3,
            fragment_count: 156,
            status: FragmentStatus::Active,
        },
        ActiveFragment {
            name: "Training Dataset".to_string(),
            aibox_id: "AIbox #1563".to_string(),
            size_gb: 8.7,
            fragment_count: 543,
            status: FragmentStatus::Active,
        },
        ActiveFragment {
            name: "Configuration Files".to_string(),
            aibox_id: "AIbox #2341".to_string(),
            size_gb: 0.5,
            fragment_count: 32,
            status: FragmentStatus::Recovering,
        },
    ];

    Ok(fragments)
}

// ============================================================================
// CONTACT PROTOCOL API COMMANDS
// ============================================================================

/// Gets conversations for the Contact protocol
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns Vec<Conversation> on success, or an error message on failure
#[tauri::command]
async fn get_conversations(state: tauri::State<'_, AppState>) -> Result<Vec<Conversation>, String> {
    let conversations_guard = state.conversations.lock().map_err(|e| e.to_string())?;
    Ok(conversations_guard.clone())
}

/// Sends a message to an AIbox
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `aibox_id` - AIbox ID to send message to
/// * `content` - Message content
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn send_message(state: tauri::State<'_, AppState>, aibox_id: String, content: String) -> Result<(), String> {
    log::info!("Sending message to AIbox {}: {}", aibox_id, content);
    // In real implementation, this would send the actual message
    Ok(())
}

// ============================================================================
// COVENANT PROTOCOL API COMMANDS
// ============================================================================

/// Gets permission profiles
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns Vec<PermissionProfile> on success, or an error message on failure
#[tauri::command]
async fn get_permission_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<PermissionProfile>, String> {
    let profiles_guard = state.permission_profiles.lock().map_err(|e| e.to_string())?;
    Ok(profiles_guard.clone())
}

/// Updates permission settings
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// * `settings` - New permission settings
/// 
/// # Returns
/// 
/// Returns Ok(()) on success, or an error message on failure
#[tauri::command]
async fn update_permission_settings(state: tauri::State<'_, AppState>, settings: PermissionSettings) -> Result<(), String> {
    log::info!("Updating permission settings: CPU {}%, RAM {}GB, GPU {}%", 
               settings.cpu_percent, settings.ram_gb, settings.gpu_percent);
    // In real implementation, this would update the actual permissions
    Ok(())
}

// ============================================================================
// ANALYTICS API COMMANDS
// ============================================================================

/// Gets analytics data
/// 
/// # Arguments
/// 
/// * `state` - Application state
/// 
/// # Returns
/// 
/// Returns AnalyticsData on success, or an error message on failure
#[tauri::command]
async fn get_analytics_data(state: tauri::State<'_, AppState>) -> Result<AnalyticsData, String> {
    // Mock analytics data
    let analytics = AnalyticsData {
        network_stats: NetworkStatistics {
            total_active_nodes: 1247,
            total_compute_power: 15432.0,
            total_storage_tb: 1250.0,
            avg_node_reliability: 98.5,
        },
        performance_metrics: PerformanceMetrics {
            your_performance_score: 85.2,
            network_avg_performance: 78.9,
            performance_ranking: 156,
            performance_trends: vec![
                PerformanceTrend {
                    timestamp: Utc::now(),
                    value: 85.2,
                },
            ],
        },
        trend_analysis: TrendAnalysis {
            network_growth_trend: TrendDirection::Increasing,
            performance_trend: TrendDirection::Stable,
            token_earning_trend: TrendDirection::Increasing,
        },
        comparison_data: ComparisonData {
            your_vs_network: ComparisonMetrics {
                performance: 1.08,
                reliability: 1.02,
                contribution: 0.95,
            },
            your_vs_top_performers: ComparisonMetrics {
                performance: 0.85,
                reliability: 0.98,
                contribution: 0.75,
            },
        },
        development_forecasts: DevelopmentForecasts {
            predicted_network_size_6m: 2500,
            predicted_compute_power_6m: 25000.0,
            predicted_token_value_6m: 1.25,
        },
    };

    Ok(analytics)
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Initializes dashboard data with default values
async fn initialize_dashboard_data(state: &tauri::State<'_, AppState>) {
    let dashboard_data = generate_default_dashboard_data().await;
    let mut dashboard_guard = state.dashboard_data.lock().unwrap();
    *dashboard_guard = Some(dashboard_data);
}

/// Generates default dashboard data for initial display
async fn generate_default_dashboard_data() -> DashboardData {
    DashboardData {
        network_status: NetworkStatus {
            is_connected: false,
            active_nodes: 0,
            total_compute_power: 0.0,
            network_health: 0,
            connection_quality: ConnectionQuality::Poor,
        },
        node_stats: NodeStats {
            contribution_percentage: 0.0,
            ranking: 0,
            reliability: 0,
            avg_response_time: 0.0,
        },
        aibox_status: AiboxStatus {
            is_active: false,
            mood: AiboxMood::Neutral,
            trust_level: 0,
            last_activity: Utc::now(),
            current_activity: None,
        },
        protocol_summaries: ProtocolSummaries {
            synapse: SynapseSummary {
                active_tasks: 0,
                available_resources: ResourceUsage {
                    cpu_percent: 0,
                    ram_gb: 0.0,
                    gpu_percent: 0,
                },
                total_earned_tokens: 0,
                weekly_token_growth: 0.0,
                network_performance: NetworkPerformance {
                    total_compute_power: 0.0,
                    active_nodes: 0,
                    avg_completion_time: 0.0,
                    reliability: 0,
                    your_contribution: 0.0,
                    your_ranking: 0,
                },
            },
            chronicle: ChronicleSummary {
                allocated_storage_gb: 0.0,
                used_storage_gb: 0.0,
                fragment_count: 0,
                data_integrity: 0,
                geographic_distribution: GeographicDistribution {
                    europe: RegionInfo { percentage: 0, fragment_count: 0 },
                    asia: RegionInfo { percentage: 0, fragment_count: 0 },
                    america: RegionInfo { percentage: 0, fragment_count: 0 },
                },
                storage_security: StorageSecurity {
                    encryption_algorithm: "None".to_string(),
                    redundancy_factor: 0,
                },
            },
            contact: ContactSummary {
                aibox_status: AiboxStatus {
                    is_active: false,
                    mood: AiboxMood::Neutral,
                    trust_level: 0,
                    last_activity: Utc::now(),
                    current_activity: None,
                },
                new_messages: 0,
                urgent_messages: 0,
                active_conversations: 0,
                avg_response_time: "0 мин".to_string(),
            },
            covenant: CovenantSummary {
                compute: ComputeSummary {
                    cpu_percent: 0,
                    ram_gb: 0.0,
                    gpu_allowed: false,
                    max_concurrent_tasks: 0,
                    allowed_task_types: vec![],
                },
                storage: StorageSummary {
                    max_storage_gb: 0.0,
                    allowed_storage_types: vec![],
                    data_retention_days: 0,
                    encryption_required: false,
                },
                communication: CommunicationSummary {
                    direct_communication_allowed: true,
                    max_messages_per_hour: 10,
                    emergency_contact_allowed: true,
                },
                token: TokenSummary {
                    max_monthly_earnings: 1000,
                    min_task_reward: 5,
                    max_single_task_reward: 100,
                },
            },
        },
        recent_activity: vec![],
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            start_node,
            stop_node,
            get_system_info,
            get_dashboard_data,
            update_dashboard_data,
            get_active_tasks,
            get_task_details,
            pause_task,
            resume_task,
            cancel_task,
            get_storage_summary,
            get_active_fragments,
            get_conversations,
            send_message,
            get_permission_profiles,
            update_permission_settings,
            get_analytics_data
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Failed to run Tauri application: {}", e);
            std::process::exit(1);
        });
}
