// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Fault Injection Components
//!
//! Fault injector trait and implementations for different subsystems
//! including network, security, database, and resource fault injectors.

use super::models::*;
use super::ChaosTestFramework;
use beardog::{BearDogError, BearDogResult};
use std::time::Instant;
use tracing::{error, info};
use uuid::Uuid;

/// Fault injector trait for different subsystems
/// **MODERNIZED** - Uses native async fn instead of async_trait for zero-cost abstractions
#[allow(async_fn_in_trait)]
pub trait FaultInjector: Send + Sync {
    /// Inject a fault into the target system
    async fn inject_fault(&self, fault: FaultType) -> BearDogResult<String>;
    
    /// Remove a fault from the target system
    async fn remove_fault(&self, fault_id: &str) -> BearDogResult<()>;
    
    /// Check if a fault is currently active
    async fn is_fault_active(&self, fault_id: &str) -> BearDogResult<bool>;
    
    /// Get the component this injector targets
    fn target_component(&self) -> String;
}

/// Inject a fault and monitor its impact
pub async fn inject_and_monitor_fault(framework: &mut ChaosTestFramework, fault: FaultType) -> BearDogResult<FaultResult> {
    let fault_id = Uuid::new_v4().to_string();
    let component = determine_target_component(&fault);
    
    info!("💥 Injecting fault: {:?} on component: {}", fault, component);
    
    let start_time = Instant::now();
    
    // Inject the fault
    let injector = framework.fault_injectors.get(&component)
        .ok_or_else(|| BearDogError::internal(format!("No fault injector for component: {}", component)))?;
    
    let injection_result = injector.inject_fault(fault.clone()).await;
    
    if let Err(e) = &injection_result {
        error!("❌ Failed to inject fault: {}", e);
        return Ok(FaultResult {
            fault_id,
            fault_type: fault,
            target_component: component,
            injection_success: false,
            impact_metrics: SystemImpact::default(),
            recovery_time_ms: None,
        });
    }
    
    // Monitor system during fault
    let impact_metrics = framework.measure_fault_impact().await?;
    
    // Track active fault
    let active_fault = ActiveFault {
        id: fault_id.clone(),
        fault_type: fault.clone(),
        start_time,
        duration: std::time::Duration::from_millis(get_fault_duration(&fault)),
        target_component: component.clone(),
        severity: determine_fault_severity(&fault),
    };
    
    framework.chaos_controller.track_active_fault(active_fault).await;
    
    // Wait for fault duration and then recover
    let fault_duration = get_fault_duration(&fault);
    tokio::time::sleep(std::time::Duration::from_millis(fault_duration)).await;
    
    // Remove the fault
    let _ = injector.remove_fault(&fault_id).await;
    
    // Wait for recovery
    let recovery_time = framework.wait_for_recovery(&component).await?;
    
    // Remove from active tracking
    framework.chaos_controller.remove_active_fault(&fault_id).await;
    
    Ok(FaultResult {
        fault_id,
        fault_type: fault,
        target_component: component,
        injection_success: true,
        impact_metrics,
        recovery_time_ms: Some(recovery_time),
    })
}

/// Determine target component for a fault type
pub fn determine_target_component(fault: &FaultType) -> String {
    match fault {
        FaultType::NetworkPartition { .. } | FaultType::NetworkLatency { .. } => "network".to_string(),
        FaultType::ComponentCrash { component, .. } | FaultType::ComponentSlowdown { component, .. } => component.clone(),
        FaultType::MemoryExhaustion { .. } | FaultType::CpuExhaustion { .. } | FaultType::DiskExhaustion { .. } => "resource".to_string(),
        FaultType::DatabaseTimeout { .. } | FaultType::DatabaseCorruption { .. } => "database".to_string(),
        FaultType::AuthenticationFailure { .. } | FaultType::CertificateExpiry { .. } => "security".to_string(),
        FaultType::ByzantineBehavior { .. } => "network".to_string(),
    }
}

/// Get fault duration based on fault type
pub fn get_fault_duration(fault: &FaultType) -> u64 {
    match fault {
        FaultType::NetworkPartition { duration_ms, .. } => *duration_ms,
        _ => 5000, // Default 5 seconds
    }
}

/// Determine fault severity
pub fn determine_fault_severity(fault: &FaultType) -> FaultSeverity {
    match fault {
        FaultType::NetworkPartition { .. } => FaultSeverity::Critical,
        FaultType::ComponentCrash { .. } => FaultSeverity::High,
        FaultType::MemoryExhaustion { cause_oom: true, .. } => FaultSeverity::Critical,
        FaultType::ByzantineBehavior { .. } => FaultSeverity::High,
        _ => FaultSeverity::Medium,
    }
}

// Fault Injector Implementations

pub struct NetworkFaultInjector;

impl NetworkFaultInjector {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl FaultInjector for NetworkFaultInjector {
    async fn inject_fault(&self, fault: FaultType) -> BearDogResult<String> {
        match fault {
            FaultType::NetworkPartition { .. } => {
                info!("🌐 Simulating network partition");
                // Network partition simulation logic would go here
                Ok(Uuid::new_v4().to_string())
            }
            FaultType::NetworkLatency { latency_ms, .. } => {
                info!("🌐 Injecting {}ms network latency", latency_ms);
                // Network latency simulation logic would go here
                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for network injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> BearDogResult<()> {
        info!("🌐 Removing network fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> BearDogResult<bool> {
        Ok(false)
    }

    fn target_component(&self) -> String {
        "network".to_string()
    }
}

pub struct SecurityFaultInjector;

impl SecurityFaultInjector {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl FaultInjector for SecurityFaultInjector {
    async fn inject_fault(&self, fault: FaultType) -> BearDogResult<String> {
        match fault {
            FaultType::AuthenticationFailure { failure_rate } => {
                info!("🔐 Injecting authentication failures at {:.2}% rate", failure_rate * 100.0);
                Ok(Uuid::new_v4().to_string())
            }
            FaultType::CertificateExpiry { .. } => {
                info!("🔐 Simulating certificate expiry");
                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for security injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> BearDogResult<()> {
        info!("🔐 Removing security fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> BearDogResult<bool> {
        Ok(false)
    }

    fn target_component(&self) -> String {
        "security".to_string()
    }
}

pub struct DatabaseFaultInjector;

impl DatabaseFaultInjector {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl FaultInjector for DatabaseFaultInjector {
    async fn inject_fault(&self, fault: FaultType) -> BearDogResult<String> {
        match fault {
            FaultType::DatabaseTimeout { timeout_ms } => {
                info!("💾 Injecting database timeout of {}ms", timeout_ms);
                Ok(Uuid::new_v4().to_string())
            }
            FaultType::DatabaseCorruption { .. } => {
                info!("💾 Simulating database corruption");
                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for database injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> BearDogResult<()> {
        info!("💾 Removing database fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> BearDogResult<bool> {
        Ok(false)
    }

    fn target_component(&self) -> String {
        "database".to_string()
    }
}

pub struct ResourceFaultInjector;

impl ResourceFaultInjector {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl FaultInjector for ResourceFaultInjector {
    async fn inject_fault(&self, fault: FaultType) -> BearDogResult<String> {
        match fault {
            FaultType::MemoryExhaustion { memory_mb, .. } => {
                info!("💾 Injecting memory exhaustion: {}MB", memory_mb);
                // Memory exhaustion simulation would go here
                Ok(Uuid::new_v4().to_string())
            }
            FaultType::CpuExhaustion { cpu_percent, .. } => {
                info!("🔥 Injecting CPU exhaustion: {}%", cpu_percent);
                // CPU exhaustion simulation would go here
                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for resource injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> BearDogResult<()> {
        info!("🔧 Removing resource fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> BearDogResult<bool> {
        Ok(false)
    }

    fn target_component(&self) -> String {
        "resource".to_string()
    }
} 