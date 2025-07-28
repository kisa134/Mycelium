//! UI API structures for Mycelium Symbiosis Protocol
//! 
//! This module provides all the data structures and types needed for
//! communication between the Rust backend and the Svelte frontend.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


// ============================================================================
// DASHBOARD API STRUCTURES
// ============================================================================

/// Main dashboard data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Network connection status
    pub network_status: NetworkStatus,
    /// Node statistics
    pub node_stats: NodeStats,
    /// AIbox status
    pub aibox_status: AiboxStatus,
    /// Protocol summaries
    pub protocol_summaries: ProtocolSummaries,
    /// Recent activity
    pub recent_activity: Vec<ActivityItem>,
}

/// Network connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Whether connected to Mycelium network
    pub is_connected: bool,
    /// Number of active nodes in network
    pub active_nodes: u32,
    /// Total network compute power in TFLOPS
    pub total_compute_power: f64,
    /// Network health percentage
    pub network_health: u8,
    /// Connection quality
    pub connection_quality: ConnectionQuality,
}

/// Connection quality levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Disconnected,
}

/// Node statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    /// Your node's contribution percentage
    pub contribution_percentage: f32,
    /// Your node's ranking
    pub ranking: u32,
    /// Reliability percentage
    pub reliability: u8,
    /// Average response time
    pub avg_response_time: f64,
}

/// AIbox status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiboxStatus {
    /// Whether AIbox is active
    pub is_active: bool,
    /// AIbox mood/emotion
    pub mood: AiboxMood,
    /// Trust level with this node
    pub trust_level: u8,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Current activity description
    pub current_activity: Option<String>,
}

/// AIbox mood states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiboxMood {
    Happy,
    Content,
    Neutral,
    Concerned,
    Stressed,
}

/// Protocol summaries for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSummaries {
    /// Synapse protocol summary
    pub synapse: SynapseSummary,
    /// Chronicle protocol summary
    pub chronicle: ChronicleSummary,
    /// Contact protocol summary
    pub contact: ContactSummary,
    /// Covenant protocol summary
    pub covenant: CovenantSummary,
}

/// Recent activity item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    /// Activity ID
    pub id: String,
    /// Activity type
    pub activity_type: ActivityType,
    /// Activity description
    pub description: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Associated AIbox ID (if applicable)
    pub aibox_id: Option<String>,
    /// Status
    pub status: ActivityStatus,
}

/// Activity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    MessageReceived,
    MessageSent,
    PermissionChanged,
    ResourceAllocated,
    StorageFragmentStored,
}

/// Activity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityStatus {
    InProgress,
    Completed,
    Failed,
    Pending,
}

// ============================================================================
// SYNAPSE PROTOCOL API STRUCTURES
// ============================================================================

/// Synapse protocol summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseSummary {
    /// Number of active tasks
    pub active_tasks: u32,
    /// Available resources
    pub available_resources: ResourceUsage,
    /// Total earned tokens
    pub total_earned_tokens: u64,
    /// Weekly token growth percentage
    pub weekly_token_growth: f32,
    /// Network performance metrics
    pub network_performance: NetworkPerformance,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: u8,
    /// RAM usage in GB
    pub ram_gb: f32,
    /// GPU usage percentage
    pub gpu_percent: u8,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformance {
    /// Total network compute power in TFLOPS
    pub total_compute_power: f64,
    /// Number of active nodes
    pub active_nodes: u32,
    /// Average task completion time in seconds
    pub avg_completion_time: f64,
    /// Network reliability percentage
    pub reliability: u8,
    /// Your node's contribution percentage
    pub your_contribution: f32,
    /// Your node's ranking
    pub your_ranking: u32,
}

/// Active task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTask {
    /// Task ID
    pub id: String,
    /// Task name
    pub name: String,
    /// Associated AIbox ID
    pub aibox_id: String,
    /// Task progress percentage
    pub progress: u8,
    /// Time remaining
    pub time_remaining: Option<String>,
    /// Task priority
    pub priority: TaskPriority,
    /// Reward in VOID tokens
    pub reward_tokens: u64,
    /// Task status
    pub status: TaskStatus,
    /// Resource usage
    pub resource_usage: ResourceUsage,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Task details for detailed view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDetails {
    /// Basic task information
    pub task: ActiveTask,
    /// Task type
    pub task_type: TaskType,
    /// Model information (if applicable)
    pub model: Option<String>,
    /// Data size in GB
    pub data_size_gb: f32,
    /// Complexity level
    pub complexity: TaskComplexity,
    /// Verification method
    pub verification: VerificationMethod,
    /// Security level
    pub security: SecurityLevel,
    /// Detailed resource usage
    pub detailed_resource_usage: DetailedResourceUsage,
}

/// Task types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    MachineLearning,
    DataProcessing,
    ImageRecognition,
    TextAnalysis,
    ModelTraining,
    Inference,
}

/// Task complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskComplexity {
    Low,
    Medium,
    High,
    Extreme,
}

/// Verification methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    CryptographicSignature,
    ProofOfWork,
    Consensus,
    Manual,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Standard,
    Isolated,
    Sandboxed,
    Encrypted,
}

/// Detailed resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: u8,
    /// RAM usage in GB
    pub ram_gb: f32,
    /// GPU usage percentage
    pub gpu_percent: u8,
    /// GPU memory usage in GB
    pub gpu_memory_gb: f32,
}

// ============================================================================
// CHRONICLE PROTOCOL API STRUCTURES
// ============================================================================

/// Chronicle protocol summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleSummary {
    /// Allocated storage in GB
    pub allocated_storage_gb: f64,
    /// Used storage in GB
    pub used_storage_gb: f64,
    /// Number of data fragments
    pub fragment_count: u32,
    /// Data integrity percentage
    pub data_integrity: u8,
    /// Geographic distribution
    pub geographic_distribution: GeographicDistribution,
    /// Storage security info
    pub storage_security: StorageSecurity,
}

/// Geographic distribution of data fragments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicDistribution {
    /// Europe percentage and fragment count
    pub europe: RegionInfo,
    /// Asia percentage and fragment count
    pub asia: RegionInfo,
    /// America percentage and fragment count
    pub america: RegionInfo,
}

/// Region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    /// Percentage of fragments in this region
    pub percentage: u8,
    /// Number of fragments in this region
    pub fragment_count: u32,
}

/// Storage security information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSecurity {
    /// Encryption algorithm
    pub encryption_algorithm: String,
    /// Redundancy factor
    pub redundancy_factor: u8,
}

/// Active fragment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveFragment {
    /// Fragment name
    pub name: String,
    /// Associated AIbox ID
    pub aibox_id: String,
    /// Fragment size in GB
    pub size_gb: f64,
    /// Number of fragments
    pub fragment_count: u32,
    /// Fragment status
    pub status: FragmentStatus,
}

/// Fragment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FragmentStatus {
    Active,
    Recovering,
    Corrupted,
    Missing,
}

/// Fragment details for detailed view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentDetails {
    /// Basic fragment information
    pub fragment: ActiveFragment,
    /// Fragment size in GB
    pub size_gb: f64,
    /// Number of fragments
    pub fragment_count: u32,
    /// Encoding type
    pub encoding_type: String,
    /// Encryption algorithm
    pub encryption_algorithm: String,
    /// Key size in bits
    pub key_size_bits: u32,
    /// Last storage proof update
    pub last_proof_update: DateTime<Utc>,
    /// Integrity status
    pub integrity_status: bool,
    /// Availability percentage
    pub availability_percent: u8,
    /// Node distribution
    pub node_distribution: Vec<NodeFragmentInfo>,
    /// Minimum fragments needed for recovery
    pub min_fragments_for_recovery: u32,
}

/// Node fragment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFragmentInfo {
    /// Node ID
    pub node_id: String,
    /// Whether this is the user's node
    pub is_your_node: bool,
    /// Number of fragments stored on this node
    pub fragment_count: u32,
}

// ============================================================================
// CONTACT PROTOCOL API STRUCTURES
// ============================================================================

/// Contact protocol summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactSummary {
    /// AIbox status
    pub aibox_status: AiboxStatus,
    /// Number of new messages
    pub new_messages: u32,
    /// Number of urgent messages
    pub urgent_messages: u32,
    /// Number of active conversations
    pub active_conversations: u32,
    /// Average response time
    pub avg_response_time: String,
}

/// Message information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: String,
    /// Sender AIbox ID
    pub sender_id: String,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    /// Message content
    pub content: String,
    /// Message type
    pub message_type: MessageType,
    /// Message priority
    pub priority: MessagePriority,
    /// Whether message has been read
    pub is_read: bool,
    /// Whether message requires response
    pub requires_response: bool,
}

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ResourceRequest,
    PhilosophicalQuestion,
    SystemNotification,
    FeedbackRequest,
    EmergencyAlert,
    Greeting,
    TaskUpdate,
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Conversation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Conversation ID
    pub id: String,
    /// AIbox ID
    pub aibox_id: String,
    /// AIbox status
    pub aibox_status: AiboxStatus,
    /// Conversation messages
    pub messages: Vec<Message>,
    /// Interaction statistics
    pub interaction_stats: InteractionStats,
}

/// Interaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStats {
    /// Total number of messages
    pub total_messages: u32,
    /// Average response time in minutes
    pub avg_response_time_minutes: f32,
    /// Success rate percentage
    pub success_rate_percent: u8,
    /// Number of conflicts
    pub conflict_count: u32,
}

// ============================================================================
// COVENANT PROTOCOL API STRUCTURES
// ============================================================================

/// Covenant protocol summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantSummary {
    /// Compute permissions summary
    pub compute: ComputeSummary,
    /// Storage permissions summary
    pub storage: StorageSummary,
    /// Communication permissions summary
    pub communication: CommunicationSummary,
    /// Token permissions summary
    pub token: TokenSummary,
}

/// Compute permissions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeSummary {
    /// CPU limit percentage
    pub cpu_percent: u8,
    /// RAM limit in GB
    pub ram_gb: f32,
    /// GPU usage allowed
    pub gpu_allowed: bool,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: u32,
    /// Allowed task types
    pub allowed_task_types: Vec<TaskType>,
}

/// Storage permissions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSummary {
    /// Maximum storage in GB
    pub max_storage_gb: f64,
    /// Allowed storage types
    pub allowed_storage_types: Vec<StorageType>,
    /// Data retention period in days
    pub data_retention_days: u32,
    /// Encryption required
    pub encryption_required: bool,
}

/// Storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    LocalDisk,
    NetworkStorage,
    CloudStorage,
    MemoryStorage,
}

/// Communication permissions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSummary {
    /// Direct communication allowed
    pub direct_communication_allowed: bool,
    /// Maximum messages per hour
    pub max_messages_per_hour: u32,
    /// Emergency contact allowed
    pub emergency_contact_allowed: bool,
}

/// Token permissions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSummary {
    /// Maximum monthly earnings
    pub max_monthly_earnings: u64,
    /// Minimum task reward
    pub min_task_reward: u64,
    /// Maximum single task reward
    pub max_single_task_reward: u64,
}

/// Resource usage for permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageForPermissions {
    /// CPU usage percentage
    pub cpu_percent: u8,
    /// RAM usage in GB
    pub ram_gb: f32,
    /// GPU usage percentage
    pub gpu_percent: u8,
    /// Storage usage in GB
    pub storage_gb: f64,
    /// Warning message (if any)
    pub warning_message: Option<String>,
}

/// Permission profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionProfile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Whether profile is active
    pub is_active: bool,
    /// Profile settings
    pub settings: PermissionSettings,
}

/// Permission settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSettings {
    /// CPU limit percentage
    pub cpu_percent: u8,
    /// RAM limit in GB
    pub ram_gb: f32,
    /// GPU limit percentage
    pub gpu_percent: u8,
    /// Storage limit in GB
    pub storage_gb: f64,
    /// Communication settings
    pub communication: CommunicationSettings,
    /// Token settings
    pub token: TokenSettings,
    /// Time restrictions
    pub time_restrictions: TimeRestrictions,
}

/// Communication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSettings {
    /// Direct communication allowed
    pub direct_communication_allowed: bool,
    /// Maximum messages per hour
    pub max_messages_per_hour: u32,
    /// Emergency contact allowed
    pub emergency_contact_allowed: bool,
}

/// Token settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSettings {
    /// Maximum monthly earnings
    pub max_monthly_earnings: u64,
    /// Minimum task reward
    pub min_task_reward: u64,
    /// Maximum single task reward
    pub max_single_task_reward: u64,
}

/// Time restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Start hour (0-23)
    pub start_hour: u8,
    /// End hour (0-23)
    pub end_hour: u8,
    /// Allowed days of week (0=Monday, 6=Sunday)
    pub allowed_days: Vec<u8>,
    /// Allow weekends
    pub allow_weekends: bool,
    /// Allow holidays
    pub allow_holidays: bool,
}

// ============================================================================
// ANALYTICS API STRUCTURES
// ============================================================================

/// Analytics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsData {
    /// Network statistics
    pub network_stats: NetworkStatistics,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Trend analysis
    pub trend_analysis: TrendAnalysis,
    /// Comparison data
    pub comparison_data: ComparisonData,
    /// Development forecasts
    pub development_forecasts: DevelopmentForecasts,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    /// Total active nodes
    pub total_active_nodes: u32,
    /// Total compute power in TFLOPS
    pub total_compute_power: f64,
    /// Total storage in TB
    pub total_storage_tb: f64,
    /// Average node reliability
    pub avg_node_reliability: f32,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Your node's performance score
    pub your_performance_score: f32,
    /// Network average performance score
    pub network_avg_performance: f32,
    /// Performance ranking
    pub performance_ranking: u32,
    /// Performance trends
    pub performance_trends: Vec<PerformanceTrend>,
}

/// Performance trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Performance value
    pub value: f32,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Network growth trend
    pub network_growth_trend: TrendDirection,
    /// Performance trend
    pub performance_trend: TrendDirection,
    /// Token earning trend
    pub token_earning_trend: TrendDirection,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Stable,
    Decreasing,
}

/// Comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonData {
    /// Your node vs network average
    pub your_vs_network: ComparisonMetrics,
    /// Your node vs top performers
    pub your_vs_top_performers: ComparisonMetrics,
}

/// Comparison metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonMetrics {
    /// Performance comparison
    pub performance: f32,
    /// Reliability comparison
    pub reliability: f32,
    /// Contribution comparison
    pub contribution: f32,
}

/// Development forecasts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentForecasts {
    /// Predicted network size in 6 months
    pub predicted_network_size_6m: u32,
    /// Predicted compute power in 6 months
    pub predicted_compute_power_6m: f64,
    /// Predicted token value in 6 months
    pub predicted_token_value_6m: f64,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// UI API error types
#[derive(Debug, thiserror::Error)]
pub enum UiApiError {
    #[error("Network connection failed: {0}")]
    NetworkError(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Resource allocation failed: {0}")]
    ResourceAllocationFailed(String),
    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),
    #[error("Storage operation failed: {0}")]
    StorageOperationFailed(String),
    #[error("Communication failed: {0}")]
    CommunicationFailed(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("System error: {0}")]
    SystemError(String),
}

/// Result type for UI API operations
pub type UiApiResult<T> = Result<T, UiApiError>; 