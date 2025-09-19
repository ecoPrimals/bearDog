use super::models::*;
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tokio::sync::RwLock;

pub struct ChaosController {
    active_faults: Arc<RwLock<HashMap<String, ActiveFault>>>,
    fault_history: Arc<Mutex<VecDeque<FaultEvent>>>,
    is_running: Arc<AtomicBool>,
    config: ChaosTestConfig,
}

impl ChaosController {
    pub fn new(config: ChaosTestConfig) -> Self {
        Self {
            active_faults: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            fault_history: Arc::new(Mutex::new(VecDeque::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            config,
        }
    }

    pub async fn track_active_fault(&self, fault: ActiveFault) {
        let mut active_faults = self.active_faults.write();
        active_faults.insert(fault.id.clone(), fault);
    }

    pub async fn remove_active_fault(&self, fault_id: &str) {
        let mut active_faults = self.active_faults.write();
        active_faults.remove(fault_id);
    }

    pub async fn get_active_faults(&self) -> Vec<ActiveFault> {
        let active_faults = self.active_faults.read();
        active_faults.values().cloned().collect()
    }

    pub async fn add_fault_to_history(&self, fault_event: FaultEvent) {
        let mut history = self.fault_history.lock().unwrap_or_else(|poisoned| {
            tracing::warn!("Mutex poisoned, recovering");
            poisoned.into_inner()
        });
        history.push_back(fault_event);

        if history.len() > 1000 {
            history.pop_front();
        }
    }

    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}
