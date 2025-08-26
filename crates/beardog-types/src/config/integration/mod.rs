

pub mod adapters;
pub mod external_systems;
pub mod monitoring;
pub mod policies;
pub mod workflows;

pub use adapters::{
    AdapterAuthMethod, AdapterConfig, AdapterConnectionPooling, AdapterPerformanceConfig,
    AdapterSecurityPolicy,
};
pub use external_systems::{
    ExternalSystemConfig, MessageQueuingConfig,
    ProtocolSupportConfig,
};

pub use monitoring::{IntegrationAuditConfig, IntegrationMonitoringConfig};
pub use policies::{IntegrationCircuitBreaker, IntegrationRateLimiting, IntegrationPoliciesConfig, NotificationRetryPolicy};
pub use workflows::{
    IntegrationWorkflowsConfig, WorkflowApprovalConfig, WorkflowExecutionConfig,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IntegrationConfig {

    pub adapters: AdapterConfig,

    pub external_systems: ExternalSystemConfig,

    pub monitoring: IntegrationMonitoringConfig,

    pub policies: IntegrationPoliciesConfig,

    pub workflows: IntegrationWorkflowsConfig,
}

