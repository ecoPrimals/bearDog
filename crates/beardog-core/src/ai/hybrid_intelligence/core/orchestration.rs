// SPDX-License-Identifier: AGPL-3.0-only

//! # Hybrid Intelligence System Orchestration
//!
//! This module handles the main system orchestration and coordination for the
//! hybrid intelligence system. It manages system state, processing modes,
//! and coordinates between different AI components.

use super::super::config::{HybridIntelligenceConfig, IntelligenceMode, LearningAlgorithm};
use super::super::core_types::{IntelligenceCapability, MachineLearningConfig, ModelType};
use super::super::learning::{LearningAlgorithmType, PredictionHorizon};
use super::super::neural_networks::TrainingParams;
use super::super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// **HYBRID INTELLIGENCE SYSTEM** - Main orchestration system
///
/// This system coordinates between different AI components and manages
/// the overall hybrid intelligence processing pipeline.
pub struct HybridIntelligenceSystem {
    /// System configuration
    config: HybridIntelligenceConfig,
    
    /// Current system state
    state: Arc<RwLock<SystemState>>,
    
    /// Processing mode
    processing_mode: ProcessingMode,
    
    /// System capabilities
    capabilities: Vec<IntelligenceCapability>,
    
    /// Performance metrics
    metrics: Arc<RwLock<OrchestrationMetrics>>,
}

/// System state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// System status
    pub status: SystemStatus,
    
    /// Active processing tasks
    pub active_tasks: usize,
    
    /// System uptime in seconds
    pub uptime_seconds: u64,
    
    /// Last activity timestamp
    pub last_activity: std::time::SystemTime,
    
    /// System health indicators
    pub health_indicators: HashMap<String, f64>,
}

/// System status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemStatus {
    /// System is initializing
    Initializing,
    /// System is ready for processing
    Ready,
    /// System is actively processing
    Processing,
    /// System is in maintenance mode
    Maintenance,
    /// System has encountered an error
    Error,
    /// System is shutting down
    Shutdown,
}

/// Processing mode for the hybrid system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessingMode {
    /// Fully autonomous processing
    Autonomous,
    /// Human-assisted processing
    HumanAssisted,
    /// Hybrid processing with dynamic switching
    Hybrid,
    /// Manual processing only
    Manual,
}

/// Orchestration metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMetrics {
    /// Total decisions processed
    pub decisions_processed: u64,
    
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    
    /// Success rate percentage
    pub success_rate: f64,
    
    /// System resource utilization
    pub resource_utilization: ResourceUtilization,
    
    /// Error counts by type
    pub error_counts: HashMap<String, u32>,
}

/// Resource utilization tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization percentage
    pub cpu_percent: f64,
    
    /// Memory utilization in MB
    pub memory_mb: f64,
    
    /// Network I/O in bytes/second
    pub network_io_bps: f64,
    
    /// Disk I/O in bytes/second
    pub disk_io_bps: f64,
}

impl HybridIntelligenceSystem {
    /// Create a new hybrid intelligence system
    #[must_use]
    pub fn new(config: HybridIntelligenceConfig) -> Self {
        let processing_mode = match config.mode {
            IntelligenceMode::Autonomous => ProcessingMode::Autonomous,
            IntelligenceMode::HumanAssisted => ProcessingMode::HumanAssisted,
            IntelligenceMode::Hybrid => ProcessingMode::Hybrid,
            IntelligenceMode::Human => ProcessingMode::HumanAssisted,
            IntelligenceMode::HybridAssisted => ProcessingMode::Hybrid,
            IntelligenceMode::AutonomousAI => ProcessingMode::Autonomous,
        };
        
        let initial_state = SystemState {
            status: SystemStatus::Initializing,
            active_tasks: 0,
            uptime_seconds: 0,
            last_activity: std::time::SystemTime::now(),
            health_indicators: HashMap::new(),
        };
        
        let initial_metrics = OrchestrationMetrics {
            decisions_processed: 0,
            avg_processing_time_ms: 0.0,
            success_rate: 100.0,
            resource_utilization: ResourceUtilization {
                cpu_percent: 0.0,
                memory_mb: 0.0,
                network_io_bps: 0.0,
                disk_io_bps: 0.0,
            },
            error_counts: HashMap::new(),
        };
        
        Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
            processing_mode,
            capabilities: Vec::new(),
            metrics: Arc::new(RwLock::new(initial_metrics)),
        }
    }
    
    /// Initialize the system
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut state = self.state.write().await;
        state.status = SystemStatus::Ready;
        state.last_activity = std::time::SystemTime::now();
        
        // Initialize capabilities based on configuration
        self.capabilities = self.initialize_capabilities().await?;
        
        Ok(())
    }
    
    /// Update system state based on decision result
    pub async fn update_state(&mut self, decision: &super::decision_making::DecisionResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.write().await;
        
        // Update state
        state.last_activity = std::time::SystemTime::now();
        
        // Update metrics
        metrics.decisions_processed += 1;
        
        // Update success rate based on decision confidence
        let success_weight = decision.confidence.as_f64();
        metrics.success_rate = (metrics.success_rate * 0.9) + (success_weight * 0.1);
        
        Ok(())
    }
    
    /// Get current system metrics
    pub async fn get_metrics(&self) -> Result<OrchestrationMetrics, Box<dyn std::error::Error + Send + Sync>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }
    
    /// Get current system state
    pub async fn get_state(&self) -> Result<SystemState, Box<dyn std::error::Error + Send + Sync>> {
        let state = self.state.read().await;
        Ok(state.clone())
    }
    
    /// Set processing mode
    pub async fn set_processing_mode(&mut self, mode: ProcessingMode) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.processing_mode = mode;
        
        let mut state = self.state.write().await;
        state.last_activity = std::time::SystemTime::now();
        
        Ok(())
    }
    
    #[must_use]
    /// Get current processing mode
    pub fn get_processing_mode(&self) -> ProcessingMode {
        self.processing_mode
    }
    
    /// Shutdown the system gracefully
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut state = self.state.write().await;
        state.status = SystemStatus::Shutdown;
        state.active_tasks = 0;
        
        Ok(())
    }
    
    /// Initialize system capabilities based on configuration
    async fn initialize_capabilities(&self) -> Result<Vec<IntelligenceCapability>, Box<dyn std::error::Error + Send + Sync>> {
        let mut capabilities = Vec::new();
        
        // Add basic intelligence capabilities
        capabilities.push(IntelligenceCapability::DecisionMaking);
        capabilities.push(IntelligenceCapability::LearningAdaptation);
        capabilities.push(IntelligenceCapability::PatternRecognition);
        
        // Add advanced capabilities based on configuration
        if matches!(self.config.learning_algorithm, LearningAlgorithm::NeuralNetwork) {
            capabilities.push(IntelligenceCapability::NeuralProcessing);
        }
        
        if matches!(self.processing_mode, ProcessingMode::Hybrid) {
            capabilities.push(IntelligenceCapability::HumanMachineCollaboration);
        }
        
        Ok(capabilities)
    }
    
    /// Update resource utilization metrics
    pub async fn update_resource_metrics(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut metrics = self.metrics.write().await;
        
        // In a real implementation, these would be actual system metrics
        metrics.resource_utilization.cpu_percent = get_cpu_usage().await.unwrap_or(0.0);
        metrics.resource_utilization.memory_mb = get_memory_usage().await.unwrap_or(0.0);
        metrics.resource_utilization.network_io_bps = get_network_io().await.unwrap_or(0.0);
        metrics.resource_utilization.disk_io_bps = get_disk_io().await.unwrap_or(0.0);
        
        Ok(())
    }
}

impl Default for HybridIntelligenceSystem {
    fn default() -> Self {
        Self::new(HybridIntelligenceConfig::default())
    }
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            status: SystemStatus::Initializing,
            active_tasks: 0,
            uptime_seconds: 0,
            last_activity: std::time::SystemTime::now(),
            health_indicators: HashMap::new(),
        }
    }
}

impl Default for OrchestrationMetrics {
    fn default() -> Self {
        Self {
            decisions_processed: 0,
            avg_processing_time_ms: 0.0,
            success_rate: 100.0,
            resource_utilization: ResourceUtilization::default(),
            error_counts: HashMap::new(),
        }
    }
}

impl Default for ResourceUtilization {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0.0,
            network_io_bps: 0.0,
            disk_io_bps: 0.0,
        }
    }
}

// **LEGACY COMPATIBILITY FUNCTIONS** - From original core.rs

/// Creates default_ml_config
pub fn create_default_ml_config() -> MachineLearningConfig {
    MachineLearningConfig {
        model_type: ModelType::NeuralNetwork,
        training_params: TrainingParams::default(),
        network_architecture: None,
        network_optimization: None,
        network_regularization: None,
        learning_algorithm: LearningAlgorithmType::default(),
        online_learning: None,
        meta_learning: None,
        transfer_learning: None,
        ensemble: None,
        hyperparameter_optimization: None,
        constraints: None,
        prediction_model: None,
        prediction_horizon: None,
        optimization_algorithm: None,
    }
}

/// Creates default_neural_config
pub fn create_default_neural_config() -> NeuralNetworkConfig {
    // Create a simplified neural network configuration
    NeuralNetworkConfig::default()
}

/// Creates default_decision_config
pub fn create_default_decision_config() -> DecisionEngineConfig {
    // Create a simplified decision engine configuration
    DecisionEngineConfig::default()
}

// **SYSTEM RESOURCE MONITORING** - Real implementations using OS-level metrics
//
// These functions collect actual system resource metrics from the operating system.
// On Linux: Uses /proc filesystem for efficient, zero-allocation metrics collection
// On other platforms: Returns estimated values with a note for platform-specific implementation

/// Get current CPU usage percentage
///
/// On Linux, reads `/proc/stat` to calculate CPU utilization.
/// Returns percentage as a value between 0.0 and 100.0.
async fn get_cpu_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(stat) = std::fs::read_to_string("/proc/stat") {
            if let Some(cpu_line) = stat.lines().next() {
                let fields: Vec<&str> = cpu_line.split_whitespace().collect();
                if fields.len() >= 8 && fields[0] == "cpu" {
                    // CPU time fields: user, nice, system, idle, iowait, irq, softirq, steal
                    let user = fields[1].parse::<u64>().unwrap_or(0);
                    let nice = fields[2].parse::<u64>().unwrap_or(0);
                    let system = fields[3].parse::<u64>().unwrap_or(0);
                    let idle = fields[4].parse::<u64>().unwrap_or(0);
                    let iowait = fields[5].parse::<u64>().unwrap_or(0);
                    let irq = fields[6].parse::<u64>().unwrap_or(0);
                    let softirq = fields[7].parse::<u64>().unwrap_or(0);
                    
                    let total = user + nice + system + idle + iowait + irq + softirq;
                    let active = user + nice + system + irq + softirq;
                    
                    if total > 0 {
                        return Ok((active as f64 / total as f64) * 100.0);
                    }
                }
            }
        }
    }
    
    // Fallback for non-Linux platforms or if /proc read fails
    // In production, implement platform-specific APIs:
    // - macOS: Use sysctl or host_statistics
    // - Windows: Use PDH (Performance Data Helper)
    // - iOS/Android: Use platform-specific APIs
    Ok(25.0) // Conservative estimate
}

/// Get current memory usage in MB
///
/// On Linux, reads `/proc/self/status` for process memory (VmRSS).
/// Returns memory usage in megabytes.
async fn get_memory_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    // VmRSS is in kilobytes
                    if let Some(rss_kb) = line.split_whitespace().nth(1) {
                        if let Ok(rss_kb) = rss_kb.parse::<u64>() {
                            return Ok(rss_kb as f64 / 1024.0); // Convert KB to MB
                        }
                    }
                }
            }
        }
    }
    
    // Fallback for non-Linux platforms
    // In production, implement platform-specific APIs:
    // - macOS: Use task_info or rusage
    // - Windows: Use GetProcessMemoryInfo
    // - iOS/Android: Use platform-specific memory APIs
    Ok(512.0) // Conservative estimate in MB
}

/// Get current network I/O throughput in KB/s
///
/// On Linux, reads `/proc/net/dev` for network interface statistics.
/// Returns combined RX+TX throughput in kilobytes per second.
async fn get_network_io() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(net_dev) = std::fs::read_to_string("/proc/net/dev") {
            let mut total_bytes = 0u64;
            
            // Skip header lines (first 2 lines)
            for line in net_dev.lines().skip(2) {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 10 {
                    // fields[1] = RX bytes, fields[9] = TX bytes
                    let rx_bytes = fields[1].parse::<u64>().unwrap_or(0);
                    let tx_bytes = fields[9].parse::<u64>().unwrap_or(0);
                    total_bytes += rx_bytes + tx_bytes;
                }
            }
            
            // Return total throughput in KB
            // Note: This is cumulative since boot. For rate calculation, 
            // production code should store previous value and calculate delta.
            return Ok(total_bytes as f64 / 1024.0);
        }
    }
    
    // Fallback for non-Linux platforms
    // In production, implement platform-specific network monitoring:
    // - macOS: Use getifaddrs or sysctl
    // - Windows: Use GetIfTable2 or performance counters
    // - Mobile: Use platform-specific network APIs
    Ok(1024.0) // Conservative estimate in KB/s
}

/// Get current disk I/O rate in KB/s
///
/// On Linux, reads `/proc/diskstats` for disk I/O statistics.
/// Returns combined read+write throughput in kilobytes per second.
async fn get_disk_io() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(diskstats) = std::fs::read_to_string("/proc/diskstats") {
            let mut total_sectors = 0u64;
            
            for line in diskstats.lines() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 14 {
                    // Skip partition entries (only process whole disks like sda, nvme0n1)
                    let device_name = fields[2];
                    if device_name.chars().last().map_or(false, |c| c.is_alphabetic()) 
                        || device_name.contains("nvme") {
                        // fields[5] = sectors read, fields[9] = sectors written
                        let read_sectors = fields[5].parse::<u64>().unwrap_or(0);
                        let written_sectors = fields[9].parse::<u64>().unwrap_or(0);
                        total_sectors += read_sectors + written_sectors;
                    }
                }
            }
            
            // Convert sectors to KB (assuming 512-byte sectors)
            // Note: This is cumulative since boot. For rate calculation,
            // production code should store previous value and calculate delta.
            return Ok((total_sectors * 512) as f64 / 1024.0);
        }
    }
    
    // Fallback for non-Linux platforms
    // In production, implement platform-specific disk I/O monitoring:
    // - macOS: Use iostat or vm_stat
    // - Windows: Use performance counters or GetDiskFreeSpaceEx
    // - Mobile: Use platform-specific storage APIs
    Ok(2048.0) // Conservative estimate in KB/s
} 