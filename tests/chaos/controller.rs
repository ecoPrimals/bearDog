#![allow(unused_imports, unused_variables, dead_code, unused_comparisons, clippy::all)]

// Chaos Testing Controller
// Migrated October 7, 2025 - Updated for modular architecture

use super::models::*;
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::SystemTime,
};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Fault event for history tracking
#[derive(Debug, Clone)]
pub struct FaultEvent {
    pub fault_id: String,
    pub fault_type: FaultType,
    pub timestamp: SystemTime,
    pub severity: FaultSeverity,
    pub component: String,
}

/// Chaos controller for orchestrating fault injection
pub struct ChaosController {
    active_faults: Arc<RwLock<HashMap<String, ActiveFault>>>,
    fault_history: Arc<Mutex<VecDeque<FaultEvent>>>,
    is_running: Arc<AtomicBool>,
    config: ChaosTestConfig,
}

impl ChaosController {
    /// Create a new chaos controller
    pub fn new(config: ChaosTestConfig) -> Self {
        Self {
            active_faults: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            fault_history: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            is_running: Arc::new(AtomicBool::new(false)),
            config,
        }
    }

    /// Track an active fault
    pub async fn track_active_fault(&self, fault: ActiveFault) {
        let mut active_faults = self.active_faults.write().await;
        let fault_id = fault.id.to_string();
        
        info!(
            "Tracking active fault '{}' on component '{}'",
            fault_id, fault.target_component
        );
        
        active_faults.insert(fault_id, fault);
    }

    /// Remove an active fault
    pub async fn remove_active_fault(&self, fault_id: &str) {
        let mut active_faults = self.active_faults.write().await;
        
        if active_faults.remove(fault_id).is_some() {
            info!("Removed active fault '{}'", fault_id);
        } else {
            warn!("Attempted to remove non-existent fault '{}'", fault_id);
        }
    }

    /// Get all currently active faults
    pub async fn get_active_faults(&self) -> Vec<ActiveFault> {
        let active_faults = self.active_faults.read().await;
        active_faults.values().cloned().collect()
    }

    /// Get the number of active faults
    pub async fn active_fault_count(&self) -> usize {
        let active_faults = self.active_faults.read().await;
        active_faults.len()
    }

    /// Check if we can inject more faults based on configuration
    pub async fn can_inject_fault(&self) -> bool {
        let count = self.active_fault_count().await;
        count < self.config.max_concurrent_faults as usize
    }

    /// Add a fault event to history
    pub fn add_fault_to_history(&self, fault_event: FaultEvent) {
        let mut history = self.fault_history.lock().unwrap_or_else(|poisoned| {
            warn!("Mutex poisoned while adding fault to history, recovering");
            poisoned.into_inner()
        });
        
        history.push_back(fault_event);

        // Keep history size bounded
        if history.len() > 1000 {
            history.pop_front();
        }
    }

    /// Get fault history
    pub fn get_fault_history(&self) -> Vec<FaultEvent> {
        let history = self.fault_history.lock().unwrap_or_else(|poisoned| {
            warn!("Mutex poisoned while reading fault history, recovering");
            poisoned.into_inner()
        });
        
        history.iter().cloned().collect()
    }

    /// Start the chaos controller
    pub fn start(&self) {
        info!("Starting chaos controller");
        self.is_running.store(true, Ordering::SeqCst);
    }

    /// Stop the chaos controller
    pub fn stop(&self) {
        info!("Stopping chaos controller");
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Check if the controller is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Get the controller configuration
    pub fn config(&self) -> &ChaosTestConfig {
        &self.config
    }

    /// Clear all active faults
    pub async fn clear_all_faults(&self) {
        let mut active_faults = self.active_faults.write().await;
        let count = active_faults.len();
        active_faults.clear();
        info!("Cleared {} active faults", count);
    }

    /// Get statistics about chaos testing
    pub async fn get_statistics(&self) -> ChaosStatistics {
        let active_faults = self.active_faults.read().await;
        let history = self.fault_history.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });

        ChaosStatistics {
            active_fault_count: active_faults.len(),
            total_faults_injected: history.len(),
            is_running: self.is_running(),
        }
    }
}

/// Statistics about chaos testing
#[derive(Debug, Clone)]
pub struct ChaosStatistics {
    pub active_fault_count: usize,
    pub total_faults_injected: usize,
    pub is_running: bool,
}

