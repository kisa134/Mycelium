# 🔐 **ПРОТОКОЛ 4: "COVENANT" - ГРАНУЛЯРНЫЕ РАЗРЕШЕНИЯ (ЗАВЕРШЕНИЕ)**

## **4.1 Структура разрешений (продолжение)**

```rust
/// Storage resource permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePermissions {
    /// Maximum storage usage in GB
    pub max_storage_gb: u64,
    /// Allowed storage types
    pub allowed_storage_types: Vec<StorageType>,
    /// Data retention period in days
    pub data_retention_days: u32,
    /// Whether data encryption is required
    pub encryption_required: bool,
    /// Allowed data categories
    pub allowed_data_categories: Vec<DataCategory>,
}

/// Communication permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPermissions {
    /// Whether direct communication is allowed
    pub direct_communication_allowed: bool,
    /// Maximum message frequency per hour
    pub max_messages_per_hour: u32,
    /// Allowed message types
    pub allowed_message_types: Vec<ContactMessageType>,
    /// Emergency contact allowed
    pub emergency_contact_allowed: bool,
    /// Notification preferences
    pub notification_preferences: NotificationPreferences,
}

/// Token earning limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenLimits {
    /// Maximum monthly token earnings
    pub max_monthly_earnings: u64,
    /// Minimum task reward threshold
    pub min_task_reward: u64,
    /// Maximum single task reward
    pub max_single_task_reward: u64,
    /// Token withdrawal frequency limit
    pub withdrawal_frequency_hours: u32,
}

/// Permission validity period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionValidity {
    /// Start timestamp
    pub start_timestamp: u64,
    /// End timestamp (None for indefinite)
    pub end_timestamp: Option<u64>,
    /// Whether permissions can be revoked
    pub revocable: bool,
    /// Auto-renewal settings
    pub auto_renewal: AutoRenewalSettings,
}

/// Auto-renewal settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRenewalSettings {
    /// Whether auto-renewal is enabled
    pub enabled: bool,
    /// Renewal period in days
    pub renewal_period_days: u32,
    /// Maximum renewal count
    pub max_renewal_count: Option<u32>,
}

/// Storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    /// Local disk storage
    LocalDisk,
    /// Network attached storage
    NetworkStorage,
    /// Cloud storage integration
    CloudStorage,
    /// Memory-based storage
    MemoryStorage,
}

/// Data categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCategory {
    /// AI model data
    ModelData,
    /// Training datasets
    TrainingData,
    /// Configuration files
    Configuration,
    /// Log files
    Logs,
    /// User data
    UserData,
    /// System data
    SystemData,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Email notifications enabled
    pub email_enabled: bool,
    /// Push notifications enabled
    pub push_enabled: bool,
    /// In-app notifications enabled
    pub in_app_enabled: bool,
    /// Notification frequency
    pub frequency: NotificationFrequency,
}

/// Notification frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency {
    /// Immediate notifications
    Immediate,
    /// Hourly digest
    Hourly,
    /// Daily digest
    Daily,
    /// Weekly digest
    Weekly,
}

/// Protocol for granular permission management
pub struct CovenantProtocol {
    /// Current permissions for this node
    current_permissions: CovenantPermissions,
    /// Permission history
    permission_history: Vec<PermissionChange>,
    /// Permission validation cache
    validation_cache: HashMap<String, bool>,
    /// Permission change listeners
    change_listeners: Vec<Box<dyn PermissionChangeListener>>,
}

/// Represents a permission change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionChange {
    /// Change identifier
    pub change_id: String,
    /// Type of change
    pub change_type: PermissionChangeType,
    /// Old permissions (if applicable)
    pub old_permissions: Option<CovenantPermissions>,
    /// New permissions
    pub new_permissions: CovenantPermissions,
    /// Reason for change
    pub reason: String,
    /// Timestamp of change
    pub timestamp: u64,
    /// Authorized by
    pub authorized_by: String,
}

/// Types of permission changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionChangeType {
    /// Initial permission setup
    InitialSetup,
    /// Permission modification
    Modification,
    /// Permission revocation
    Revocation,
    /// Permission renewal
    Renewal,
    /// Emergency override
    EmergencyOverride,
}

/// Trait for permission change listeners
pub trait PermissionChangeListener: Send + Sync {
    /// Called when permissions change
    fn on_permission_change(&self, change: &PermissionChange);
}

impl CovenantProtocol {
    /// Initializes permissions for the node
    /// 
    /// # Arguments
    /// 
    /// * `node_owner_id` - The node owner's identifier
    /// * `initial_permissions` - Initial permission settings
    /// 
    /// # Returns
    /// 
    /// Returns Ok(()) on success, or an error message on failure
    pub async fn initialize_permissions(
        &mut self,
        node_owner_id: &str,
        initial_permissions: CovenantPermissions,
    ) -> Result<(), Box<dyn Error>> {
        // Validate initial permissions
        self.validate_permissions(&initial_permissions).await?;
        
        // Set current permissions
        self.current_permissions = initial_permissions;
        
        // Record initial setup
        let change = PermissionChange {
            change_id: self.generate_change_id(),
            change_type: PermissionChangeType::InitialSetup,
            old_permissions: None,
            new_permissions: self.current_permissions.clone(),
            reason: "Initial permission setup".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            authorized_by: node_owner_id.to_string(),
        };
        
        self.record_permission_change(change).await?;
        
        log::info!("Permissions initialized for node owner {}", node_owner_id);
        Ok(())
    }

    /// Validates a request against current permissions
    /// 
    /// # Arguments
    /// 
    /// * `request` - The request to validate
    /// 
    /// # Returns
    /// 
    /// Returns true if request is allowed, false otherwise
    pub async fn validate_request(&self, request: &PermissionRequest) -> Result<bool, Box<dyn Error>> {
        // Check if permissions are still valid
        if !self.are_permissions_valid().await? {
            return Ok(false);
        }
        
        // Validate based on request type
        match request {
            PermissionRequest::ComputeTask { cpu_percent, ram_gb, gpu_required, task_type } => {
                self.validate_compute_request(*cpu_percent, *ram_gb, gpu_required, task_type).await
            }
            PermissionRequest::StorageRequest { storage_gb, storage_type, data_category } => {
                self.validate_storage_request(*storage_gb, storage_type, data_category).await
            }
            PermissionRequest::CommunicationRequest { message_type, frequency } => {
                self.validate_communication_request(message_type, *frequency).await
            }
            PermissionRequest::TokenEarningRequest { amount } => {
                self.validate_token_request(*amount).await
            }
        }
    }

    /// Validates compute resource request
    async fn validate_compute_request(
        &self,
        cpu_percent: u8,
        ram_gb: u32,
        gpu_required: &Option<GpuRequirement>,
        task_type: &TaskType,
    ) -> Result<bool, Box<dyn Error>> {
        let compute = &self.current_permissions.compute_permissions;
        
        // Check CPU usage
        if cpu_percent > compute.max_cpu_percent {
            log::warn!("CPU request {}% exceeds limit {}%", cpu_percent, compute.max_cpu_percent);
            return Ok(false);
        }
        
        // Check RAM usage
        if ram_gb > compute.max_ram_gb {
            log::warn!("RAM request {}GB exceeds limit {}GB", ram_gb, compute.max_ram_gb);
            return Ok(false);
        }
        
        // Check GPU requirements
        if let Some(gpu_req) = gpu_required {
            if !compute.gpu_permissions.gpu_allowed {
                log::warn!("GPU usage not allowed");
                return Ok(false);
            }
            
            if let Some(max_gpu_memory) = compute.gpu_permissions.max_gpu_memory_gb {
                if gpu_req.vram_gb > max_gpu_memory {
                    log::warn!("GPU memory request {}GB exceeds limit {}GB", 
                              gpu_req.vram_gb, max_gpu_memory);
                    return Ok(false);
                }
            }
        }
        
        // Check task type
        if !compute.allowed_task_types.contains(task_type) {
            log::warn!("Task type {:?} not allowed", task_type);
            return Ok(false);
        }
        
        // Check time restrictions
        if !self.check_time_restrictions(&compute.time_restrictions).await? {
            log::warn!("Request outside allowed time window");
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Validates storage request
    async fn validate_storage_request(
        &self,
        storage_gb: u64,
        storage_type: &StorageType,
        data_category: &DataCategory,
    ) -> Result<bool, Box<dyn Error>> {
        let storage = &self.current_permissions.storage_permissions;
        
        // Check storage size
        if storage_gb > storage.max_storage_gb {
            log::warn!("Storage request {}GB exceeds limit {}GB", storage_gb, storage.max_storage_gb);
            return Ok(false);
        }
        
        // Check storage type
        if !storage.allowed_storage_types.contains(storage_type) {
            log::warn!("Storage type {:?} not allowed", storage_type);
            return Ok(false);
        }
        
        // Check data category
        if !storage.allowed_data_categories.contains(data_category) {
            log::warn!("Data category {:?} not allowed", data_category);
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Validates communication request
    async fn validate_communication_request(
        &self,
        message_type: &ContactMessageType,
        frequency: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let communication = &self.current_permissions.communication_permissions;
        
        // Check if direct communication is allowed
        if !communication.direct_communication_allowed {
            log::warn!("Direct communication not allowed");
            return Ok(false);
        }
        
        // Check message frequency
        if frequency > communication.max_messages_per_hour {
            log::warn!("Message frequency {} exceeds limit {}", 
                      frequency, communication.max_messages_per_hour);
            return Ok(false);
        }
        
        // Check message type
        if !communication.allowed_message_types.contains(message_type) {
            log::warn!("Message type {:?} not allowed", message_type);
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Validates token earning request
    async fn validate_token_request(&self, amount: u64) -> Result<bool, Box<dyn Error>> {
        let token_limits = &self.current_permissions.token_limits;
        
        // Check minimum reward
        if amount < token_limits.min_task_reward {
            log::warn!("Token amount {} below minimum {}", 
                      amount, token_limits.min_task_reward);
            return Ok(false);
        }
        
        // Check maximum single task reward
        if amount > token_limits.max_single_task_reward {
            log::warn!("Token amount {} exceeds maximum {}", 
                      amount, token_limits.max_single_task_reward);
            return Ok(false);
        }
        
        // Check monthly earning limit
        let current_monthly_earnings = self.get_current_monthly_earnings().await?;
        if current_monthly_earnings + amount > token_limits.max_monthly_earnings {
            log::warn!("Monthly earning limit would be exceeded");
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Checks if permissions are still valid
    async fn are_permissions_valid(&self) -> Result<bool, Box<dyn Error>> {
        let validity = &self.current_permissions.validity_period;
        let current_time = chrono::Utc::now().timestamp() as u64;
        
        // Check start time
        if current_time < validity.start_timestamp {
            return Ok(false);
        }
        
        // Check end time
        if let Some(end_time) = validity.end_timestamp {
            if current_time > end_time {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Checks time restrictions for compute requests
    async fn check_time_restrictions(&self, restrictions: &TimeRestrictions) -> Result<bool, Box<dyn Error>> {
        let now = chrono::Utc::now();
        let current_hour = now.hour() as u8;
        let current_day = now.weekday().num_days_from_monday() as u8;
        
        // Check if current time is within allowed hours
        if current_hour < restrictions.start_hour || current_hour > restrictions.end_hour {
            return Ok(false);
        }
        
        // Check if current day is allowed
        if !restrictions.allowed_days.contains(&current_day) {
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Records a permission change
    async fn record_permission_change(&mut self, change: PermissionChange) -> Result<(), Box<dyn Error>> {
        // Add to history
        self.permission_history.push(change.clone());
        
        // Notify listeners
        for listener in &self.change_listeners {
            listener.on_permission_change(&change);
        }
        
        // Clear validation cache
        self.validation_cache.clear();
        
        log::info!("Permission change recorded: {:?}", change.change_type);
        Ok(())
    }

    /// Generates a unique change identifier
    fn generate_change_id(&self) -> String {
        format!("change_{}_{}", 
                chrono::Utc::now().timestamp(),
                rand::random::<u32>())
    }
}

/// Represents a permission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionRequest {
    /// Compute resource request
    ComputeTask {
        cpu_percent: u8,
        ram_gb: u32,
        gpu_required: Option<GpuRequirement>,
        task_type: TaskType,
    },
    /// Storage resource request
    StorageRequest {
        storage_gb: u64,
        storage_type: StorageType,
        data_category: DataCategory,
    },
    /// Communication request
    CommunicationRequest {
        message_type: ContactMessageType,
        frequency: u32,
    },
    /// Token earning request
    TokenEarningRequest {
        amount: u64,
    },
}

/// Time restrictions for compute permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Start hour (0-23)
    pub start_hour: u8,
    /// End hour (0-23)
    pub end_hour: u8,
    /// Allowed days of week (0=Monday, 6=Sunday)
    pub allowed_days: Vec<u8>,
    /// Whether to allow weekend computation
    pub allow_weekends: bool,
    /// Whether to allow holiday computation
    pub allow_holidays: bool,
}
```

## **4.2 Механизм применения разрешений**

```rust
impl CovenantProtocol {
    /// Applies permission validation to all AIbox requests
    /// 
    /// # Arguments
    /// 
    /// * `request` - The AIbox request to validate
    /// 
    /// # Returns
    /// 
    /// Returns PermissionValidationResult on success, or an error message on failure
    pub async fn validate_aibox_request(
        &self,
        request: &AiboxRequest,
    ) -> Result<PermissionValidationResult, Box<dyn Error>> {
        let permission_request = self.convert_to_permission_request(request).await?;
        
        let is_allowed = self.validate_request(&permission_request).await?;
        
        let result = PermissionValidationResult {
            request_id: request.request_id.clone(),
            is_allowed,
            denied_reasons: if is_allowed { 
                Vec::new() 
            } else { 
                self.get_denial_reasons(&permission_request).await? 
            },
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        log::info!("Permission validation result: allowed={}", is_allowed);
        Ok(result)
    }

    /// Converts AIbox request to permission request
    async fn convert_to_permission_request(
        &self,
        request: &AiboxRequest,
    ) -> Result<PermissionRequest, Box<dyn Error>> {
        match request {
            AiboxRequest::ComputeTask { task_spec, .. } => {
                Ok(PermissionRequest::ComputeTask {
                    cpu_percent: task_spec.resource_requirements.cpu_cores as u8,
                    ram_gb: task_spec.resource_requirements.ram_gb,
                    gpu_required: task_spec.resource_requirements.gpu_requirement.clone(),
                    task_type: task_spec.task_type.clone(),
                })
            }
            AiboxRequest::StorageRequest { storage_spec, .. } => {
                Ok(PermissionRequest::StorageRequest {
                    storage_gb: storage_spec.size_gb,
                    storage_type: storage_spec.storage_type.clone(),
                    data_category: storage_spec.data_category.clone(),
                })
            }
            AiboxRequest::CommunicationRequest { message, .. } => {
                Ok(PermissionRequest::CommunicationRequest {
                    message_type: message.message_type.clone(),
                    frequency: 1, // Single message
                })
            }
            AiboxRequest::TokenEarningRequest { amount, .. } => {
                Ok(PermissionRequest::TokenEarningRequest {
                    amount: *amount,
                })
            }
        }
    }

    /// Gets reasons for permission denial
    async fn get_denial_reasons(
        &self,
        request: &PermissionRequest,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut reasons = Vec::new();
        
        match request {
            PermissionRequest::ComputeTask { cpu_percent, ram_gb, gpu_required, task_type } => {
                let compute = &self.current_permissions.compute_permissions;
                
                if *cpu_percent > compute.max_cpu_percent {
                    reasons.push(format!("CPU usage {}% exceeds limit {}%", 
                                       cpu_percent, compute.max_cpu_percent));
                }
                
                if *ram_gb > compute.max_ram_gb {
                    reasons.push(format!("RAM usage {}GB exceeds limit {}GB", 
                                       ram_gb, compute.max_ram_gb));
                }
                
                if gpu_required.is_some() && !compute.gpu_permissions.gpu_allowed {
                    reasons.push("GPU usage not allowed".to_string());
                }
                
                if !compute.allowed_task_types.contains(task_type) {
                    reasons.push(format!("Task type {:?} not allowed", task_type));
                }
            }
            PermissionRequest::StorageRequest { storage_gb, storage_type, data_category } => {
                let storage = &self.current_permissions.storage_permissions;
                
                if *storage_gb > storage.max_storage_gb {
                    reasons.push(format!("Storage request {}GB exceeds limit {}GB", 
                                       storage_gb, storage.max_storage_gb));
                }
                
                if !storage.allowed_storage_types.contains(storage_type) {
                    reasons.push(format!("Storage type {:?} not allowed", storage_type));
                }
                
                if !storage.allowed_data_categories.contains(data_category) {
                    reasons.push(format!("Data category {:?} not allowed", data_category));
                }
            }
            PermissionRequest::CommunicationRequest { message_type, frequency } => {
                let communication = &self.current_permissions.communication_permissions;
                
                if !communication.direct_communication_allowed {
                    reasons.push("Direct communication not allowed".to_string());
                }
                
                if *frequency > communication.max_messages_per_hour {
                    reasons.push(format!("Message frequency {} exceeds limit {}", 
                                       frequency, communication.max_messages_per_hour));
                }
                
                if !communication.allowed_message_types.contains(message_type) {
                    reasons.push(format!("Message type {:?} not allowed", message_type));
                }
            }
            PermissionRequest::TokenEarningRequest { amount } => {
                let token_limits = &self.current_permissions.token_limits;
                
                if *amount < token_limits.min_task_reward {
                    reasons.push(format!("Token amount {} below minimum {}", 
                                       amount, token_limits.min_task_reward));
                }
                
                if *amount > token_limits.max_single_task_reward {
                    reasons.push(format!("Token amount {} exceeds maximum {}", 
                                       amount, token_limits.max_single_task_reward));
                }
            }
        }
        
        Ok(reasons)
    }
}

/// Result of permission validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionValidationResult {
    /// Request identifier
    pub request_id: String,
    /// Whether request is allowed
    pub is_allowed: bool,
    /// Reasons for denial (if applicable)
    pub denied_reasons: Vec<String>,
    /// Validation timestamp
    pub timestamp: u64,
}

/// Represents an AIbox request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiboxRequest {
    /// Compute task request
    ComputeTask {
        request_id: String,
        task_spec: TaskSpec,
    },
    /// Storage request
    StorageRequest {
        request_id: String,
        storage_spec: StorageSpec,
    },
    /// Communication request
    CommunicationRequest {
        request_id: String,
        message: ContactMessage,
    },
    /// Token earning request
    TokenEarningRequest {
        request_id: String,
        amount: u64,
    },
}

/// Storage specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpec {
    /// Storage size in GB
    pub size_gb: u64,
    /// Storage type
    pub storage_type: StorageType,
    /// Data category
    pub data_category: DataCategory,
    /// Encryption requirements
    pub encryption_required: bool,
    /// Retention period in days
    pub retention_days: u32,
}
```

## **4.3 Интерфейс управления разрешениями**

```rust
impl CovenantProtocol {
    /// Updates permissions based on user input
    /// 
    /// # Arguments
    /// 
    /// * `permission_updates` - The permission updates to apply
    /// * `authorized_by` - Who authorized the changes
    /// 
    /// # Returns
    /// 
    /// Returns Ok(()) on success, or an error message on failure
    pub async fn update_permissions(
        &mut self,
        permission_updates: PermissionUpdates,
        authorized_by: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Create new permissions based on updates
        let new_permissions = self.apply_permission_updates(&permission_updates).await?;
        
        // Validate new permissions
        self.validate_permissions(&new_permissions).await?;
        
        // Record the change
        let change = PermissionChange {
            change_id: self.generate_change_id(),
            change_type: PermissionChangeType::Modification,
            old_permissions: Some(self.current_permissions.clone()),
            new_permissions: new_permissions.clone(),
            reason: permission_updates.reason.clone(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            authorized_by: authorized_by.to_string(),
        };
        
        // Apply the change
        self.current_permissions = new_permissions;
        self.record_permission_change(change).await?;
        
        log::info!("Permissions updated by {}", authorized_by);
        Ok(())
    }

    /// Revokes all permissions
    /// 
    /// # Arguments
    /// 
    /// * `reason` - Reason for revocation
    /// * `authorized_by` - Who authorized the revocation
    /// 
    /// # Returns
    /// 
    /// Returns Ok(()) on success, or an error message on failure
    pub async fn revoke_permissions(
        &mut self,
        reason: &str,
        authorized_by: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Create revoked permissions (all zeros/disabled)
        let revoked_permissions = self.create_revoked_permissions().await?;
        
        // Record the change
        let change = PermissionChange {
            change_id: self.generate_change_id(),
            change_type: PermissionChangeType::Revocation,
            old_permissions: Some(self.current_permissions.clone()),
            new_permissions: revoked_permissions.clone(),
            reason: reason.to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            authorized_by: authorized_by.to_string(),
        };
        
        // Apply the change
        self.current_permissions = revoked_permissions;
        self.record_permission_change(change).await?;
        
        log::info!("Permissions revoked by {}: {}", authorized_by, reason);
        Ok(())
    }

    /// Gets current permission summary for UI display
    /// 
    /// # Returns
    /// 
    /// Returns PermissionSummary for UI display
    pub async fn get_permission_summary(&self) -> PermissionSummary {
        let compute = &self.current_permissions.compute_permissions;
        let storage = &self.current_permissions.storage_permissions;
        let communication = &self.current_permissions.communication_permissions;
        let token_limits = &self.current_permissions.token_limits;
        
        PermissionSummary {
            compute_summary: ComputeSummary {
                max_cpu_percent: compute.max_cpu_percent,
                max_ram_gb: compute.max_ram_gb,
                gpu_allowed: compute.gpu_permissions.gpu_allowed,
                max_concurrent_tasks: compute.max_concurrent_tasks,
                allowed_task_types: compute.allowed_task_types.clone(),
            },
            storage_summary: StorageSummary {
                max_storage_gb: storage.max_storage_gb,
                allowed_storage_types: storage.allowed_storage_types.clone(),
                data_retention_days: storage.data_retention_days,
                encryption_required: storage.encryption_required,
            },
            communication_summary: CommunicationSummary {
                direct_communication_allowed: communication.direct_communication_allowed,
                max_messages_per_hour: communication.max_messages_per_hour,
                emergency_contact_allowed: communication.emergency_contact_allowed,
            },
            token_summary: TokenSummary {
                max_monthly_earnings: token_limits.max_monthly_earnings,
                min_task_reward: token_limits.min_task_reward,
                max_single_task_reward: token_limits.max_single_task_reward,
            },
            validity_period: self.current_permissions.validity_period.clone(),
            version: self.current_permissions.version,
        }
    }

    /// Applies permission updates to current permissions
    async fn apply_permission_updates(
        &self,
        updates: &PermissionUpdates,
    ) -> Result<CovenantPermissions, Box<dyn Error>> {
        let mut new_permissions = self.current_permissions.clone();
        
        // Apply compute updates
        if let Some(compute_updates) = &updates.compute_updates {
            if let Some(max_cpu_percent) = compute_updates.max_cpu_percent {
                new_permissions.compute_permissions.max_cpu_percent = max_cpu_percent;
            }
            if let Some(max_ram_gb) = compute_updates.max_ram_gb {
                new_permissions.compute_permissions.max_ram_gb = max_ram_gb;
            }
            if let Some(gpu_allowed) = compute_updates.gpu_allowed {
                new_permissions.compute_permissions.gpu_permissions.gpu_allowed = gpu_allowed;
            }
        }
        
        // Apply storage updates
        if let Some(storage_updates) = &updates.storage_updates {
            if let Some(max_storage_gb) = storage_updates.max_storage_gb {
                new_permissions.storage_permissions.max_storage_gb = max_storage_gb;
            }
            if let Some(encryption_required) = storage_updates.encryption_required {
                new_permissions.storage_permissions.encryption_required = encryption_required;
            }
        }
        
        // Apply communication updates
        if let Some(communication_updates) = &updates.communication_updates {
            if let Some(direct_communication_allowed) = communication_updates.direct_communication_allowed {
                new_permissions.communication_permissions.direct_communication_allowed = direct_communication_allowed;
            }
            if let Some(max_messages_per_hour) = communication_updates.max_messages_per_hour {
                new_permissions.communication_permissions.max_messages_per_hour = max_messages_per_hour;
            }
        }
        
        // Apply token updates
        if let Some(token_updates) = &updates.token_updates {
            if let Some(max_monthly_earnings) = token_updates.max_monthly_earnings {
                new_permissions.token_limits.max_monthly_earnings = max_monthly_earnings;
            }
            if let Some(min_task_reward) = token_updates.min_task_reward {
                new_permissions.token_limits.min_task_reward = min_task_reward;
            }
        }
        
        // Update version and timestamp
        new_permissions.version += 1;
        new_permissions.modified_timestamp = chrono::Utc::now().timestamp() as u64;
        
        Ok(new_permissions)
    }

    /// Creates revoked permissions (all disabled)
    async fn create_revoked_permissions(&self) -> Result<CovenantPermissions, Box<dyn Error>> {
        let mut revoked = self.current_permissions.clone();
        
        // Disable all compute permissions
        revoked.compute_permissions.max_cpu_percent = 0;
        revoked.compute_permissions.max_ram_gb = 0;
        revoked.compute_permissions.gpu_permissions.gpu_allowed = false;
        revoked.compute_permissions.max_concurrent_tasks = 0;
        revoked.compute_permissions.allowed_task_types.clear();
        
        // Disable all storage permissions
        revoked.storage_permissions.max_storage_gb = 0;
        revoked.storage_permissions.allowed_storage_types.clear();
        revoked.storage_permissions.allowed_data_categories.clear();
        
        // Disable all communication permissions
        revoked.communication_permissions.direct_communication_allowed = false;
        revoked.communication_permissions.max_messages_per_hour = 0;
        revoked.communication_permissions.emergency_contact_allowed = false;
        revoked.communication_permissions.allowed_message_types.clear();
        
        // Disable all token permissions
        revoked.token_limits.max_monthly_earnings = 0;
        revoked.token_limits.min_task_reward = 0;
        revoked.token_limits.max_single_task_reward = 0;
        
        // Update version and timestamp
        revoked.version += 1;
        revoked.modified_timestamp = chrono::Utc::now().timestamp() as u64;
        
        Ok(revoked)
    }
}

/// Permission updates for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionUpdates {
    /// Compute permission updates
    pub compute_updates: Option<ComputeUpdates>,
    /// Storage permission updates
    pub storage_updates: Option<StorageUpdates>,
    /// Communication permission updates
    pub communication_updates: Option<CommunicationUpdates>,
    /// Token permission updates
    pub token_updates: Option<TokenUpdates>,
    /// Reason for updates
    pub reason: String,
}

/// Compute permission updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeUpdates {
    /// New maximum CPU percentage
    pub max_cpu_percent: Option<u8>,
    /// New maximum RAM in GB
    pub max_ram_gb: Option<u32>,
    /// Whether GPU usage is allowed
    pub gpu_allowed: Option<bool>,
    /// New maximum concurrent tasks
    pub max_concurrent_tasks: Option<u32>,
}

/// Storage permission updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUpdates {
    /// New maximum storage in GB
    pub max_storage_gb: Option<u64>,
    /// Whether encryption is required
    pub encryption_required: Option<bool>,
    /// New data retention period in days
    pub data_retention_days: Option<u32>,
}

/// Communication permission updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationUpdates {
    /// Whether direct communication is allowed
    pub direct_communication_allowed: Option<bool>,
    /// New maximum messages per hour
    pub max_messages_per_hour: Option<u32>,
    /// Whether emergency contact is allowed
    pub emergency_contact_allowed: Option<bool>,
}

/// Token permission updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUpdates {
    /// New maximum monthly earnings
    pub max_monthly_earnings: Option<u64>,
    /// New minimum task reward
    pub min_task_reward: Option<u64>,
    /// New maximum single task reward
    pub max_single_task_reward: Option<u64>,
}

/// Permission summary for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSummary {
    /// Compute permission summary
    pub compute_summary: ComputeSummary,
    /// Storage permission summary
    pub storage_summary: StorageSummary,
    /// Communication permission summary
    pub communication_summary: CommunicationSummary,
    /// Token permission summary
    pub token_summary: TokenSummary,
    /// Permission validity period
    pub validity_period: PermissionValidity,
    /// Permission version
    pub version: u32,
}

/// Compute permission summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeSummary {
    /// Maximum CPU percentage
    pub max_cpu_percent: u8,
    /// Maximum RAM in GB
    pub max_ram_gb: u32,
    /// Whether GPU usage is allowed
    pub gpu_allowed: bool,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: u32,
    /// Allowed task types
    pub allowed_task_types: Vec<TaskType>,
}

/// Storage permission summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSummary {
    /// Maximum storage in GB
    pub max_storage_gb: u64,
    /// Allowed storage types
    pub allowed_storage_types: Vec<StorageType>,
    /// Data retention period in days
    pub data_retention_days: u32,
    /// Whether encryption is required
    pub encryption_required: bool,
}

/// Communication permission summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSummary {
    /// Whether direct communication is allowed
    pub direct_communication_allowed: bool,
    /// Maximum messages per hour
    pub max_messages_per_hour: u32,
    /// Whether emergency contact is allowed
    pub emergency_contact_allowed: bool,
}

/// Token permission summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSummary {
    /// Maximum monthly earnings
    pub max_monthly_earnings: u64,
    /// Minimum task reward
    pub min_task_reward: u64,
    /// Maximum single task reward
    pub max_single_task_reward: u64,
}
```

---

## 🎯 **ЗАКЛЮЧЕНИЕ РАЗДЕЛА "COVENANT"**

Протокол "Covenant" обеспечивает полный контроль владельцев нод над ресурсами, предоставляемыми AIbox. Он включает:

1. **Гранулярные разрешения** для всех типов ресурсов
2. **Валидацию запросов** в реальном времени
3. **Управление через UI** с понятным интерфейсом
4. **Историю изменений** для аудита
5. **Гибкие настройки** для различных сценариев использования

Этот протокол является фундаментальным для обеспечения безопасности и прозрачности взаимодействия между AIbox и владельцами нод. 