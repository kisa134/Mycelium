use sysinfo::System;
use serde::{Deserialize, Serialize};

/// Represents current system information including CPU and memory usage
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub ram_total: u64,
    pub ram_used: u64,
    pub ram_usage_percent: f32,
}

/// System monitor that tracks CPU and memory usage
pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    /// Creates a new system monitor and initializes system information
    /// 
    /// # Returns
    /// 
    /// A new SystemMonitor instance with refreshed system data
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self { system }
    }

    /// Gets current system information including CPU and RAM usage
    /// 
    /// This function refreshes all system data and returns current metrics.
    /// 
    /// # Returns
    /// 
    /// SystemInfo containing current CPU usage, total RAM, used RAM, and RAM usage percentage
    pub fn get_system_info(&mut self) -> SystemInfo {
        self.system.refresh_all();
        
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let ram_total = self.system.total_memory();
        let ram_used = self.system.used_memory();
        let ram_usage_percent = (ram_used as f32 / ram_total as f32) * 100.0;

        SystemInfo {
            cpu_usage,
            ram_total,
            ram_used,
            ram_usage_percent,
        }
    }
} 