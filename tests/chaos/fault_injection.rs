use beardog_errors::BearDogError;


use super::models::*;
use super::ChaosTestFramework;
use beardog::{{BearDogError, BearDogError}};
use std::time::Instant;
use tracing::{error, info};
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait FaultInjector: Send + Sync {

    async fn inject_fault(&self, fault: FaultType) -> Result<String, BearDogError>;

    async fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError>;

    async fn is_fault_active(&self, fault_id: &str) -> Result<bool, BearDogError>;

    fn target_component(&self) -> String;
}

pub async fn inject_and_monitor_fault(framework: &mut ChaosTestFramework, fault: FaultType) -> Result<FaultResult, BearDogError> {
    let fault_id = Uuid::new_v4().to_string();
    let component = determine_target_component(&fault);
    
    info!("💥 Injecting fault: {:?} on component: {}", fault, component);
    
    let start_time = Instant::now();

    let injector = framework.fault_injectors.get(&component)
        .ok_or_else(|| BearDogError::internal(format_args!("No fault injector for component: {}", component).to_string()))?;
    
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

    let impact_metrics = framework.measure_fault_impact().await?;

    let active_fault = ActiveFault {
        id: fault_id.clone(),
        fault_type: fault.clone(),
        start_time,
        duration: std::time::Duration::from_millis(get_fault_duration(&fault)),
        target_component: component.clone(),
        severity: determine_fault_severity(&fault),
    };
    
    framework.chaos_controller.track_active_fault(active_fault).await;

    let fault_duration = get_fault_duration(&fault);
    tokio::time::sleep(std::time::Duration::from_millis(fault_duration)).await;

    let _ = injector.remove_fault(&fault_id).await;

    let recovery_time = framework.wait_for_recovery(&component).await?;

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

pub fn get_fault_duration(fault: &FaultType) -> u64 {
    match fault {
        FaultType::NetworkPartition { duration_ms, .. } => *duration_ms,
        _ => 5000, // Default 5 seconds
    }
}

pub fn determine_fault_severity(fault: &FaultType) -> FaultSeverity {
    match fault {
        FaultType::NetworkPartition { .. } => FaultSeverity::Critical,
        FaultType::ComponentCrash { .. } => FaultSeverity::High,
        FaultType::MemoryExhaustion { cause_oom: true, .. } => FaultSeverity::Critical,
        FaultType::ByzantineBehavior { .. } => FaultSeverity::High,
        _ => FaultSeverity::Medium,
    }
}

pub struct NetworkFaultInjector;

impl NetworkFaultInjector {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl FaultInjector for NetworkFaultInjector {
    async fn inject_fault(&self, fault: FaultType) -> Result<String, BearDogError> {
        match fault {
            FaultType::NetworkPartition { .. } => {
                info!("🌐 Simulating network partition");

                Ok(Uuid::new_v4().to_string())
            }
            FaultType::NetworkLatency { latency_ms, .. } => {
                info!("🌐 Injecting {}ms network latency", latency_ms);

                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for network injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError> {
        info!("🌐 Removing network fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> Result<bool, BearDogError> {
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
    async fn inject_fault(&self, fault: FaultType) -> Result<String, BearDogError> {
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

    async fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError> {
        info!("🔐 Removing security fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> Result<bool, BearDogError> {
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
    async fn inject_fault(&self, fault: FaultType) -> Result<String, BearDogError> {
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

    async fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError> {
        info!("💾 Removing database fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> Result<bool, BearDogError> {
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
    async fn inject_fault(&self, fault: FaultType) -> Result<String, BearDogError> {
        match fault {
            FaultType::MemoryExhaustion { memory_mb, .. } => {
                info!("💾 Injecting memory exhaustion: {}MB", memory_mb);

                Ok(Uuid::new_v4().to_string())
            }
            FaultType::CpuExhaustion { cpu_percent, .. } => {
                info!("🔥 Injecting CPU exhaustion: {}%", cpu_percent);

                Ok(Uuid::new_v4().to_string())
            }
            _ => Err(BearDogError::internal("Unsupported fault type for resource injector")),
        }
    }

    async fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError> {
        info!("🔧 Removing resource fault: {}", fault_id);
        Ok(())
    }

    async fn is_fault_active(&self, _fault_id: &str) -> Result<bool, BearDogError> {
        Ok(false)
    }

    fn target_component(&self) -> String {
        "resource".to_string()
    }
} 