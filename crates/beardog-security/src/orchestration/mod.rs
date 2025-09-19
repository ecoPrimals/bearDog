

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod compliance_orchestration;
pub mod crypto_coordination;
pub mod security_orchestration;

pub use compliance_orchestration::ComplianceOrchestrator;
pub use crypto_coordination::CryptoCoordinator;
pub use security_orchestration::{SecurityOrchestrationConfig, SecurityOrchestrator};

pub trait SecurityOrchestrationOps: Send + Sync {


    fn orchestrate_security_workflow(&str,
    ) -> impl std::future::Future<Output = Result<SecurityWorkflowResult, crate::BearDogSecurityError>>
           + Send;


    fn check_security_health(&str,
    ) -> impl std::future::Future<Output = Result<SecurityHealthStatus, crate::BearDogSecurityError>>
           + Send;
}

#[derive(Debug, Clone)]
    /// Current status of the component
    pub status: SecurityWorkflowStatus,

    /// Collection of operations completed
    pub operations_completed: Vec<String>,


    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]

pub enum SecurityWorkflowStatus {


    /// Operation in progress
    Pending,


    /// Operation in progress
    InProgress,


    /// Successful completion state
    Completed,

    /// Error or failure state
    Failed(String),
}

#[derive(Debug, Clone, PartialEq)]

pub enum SecurityHealthStatus {


    /// Represents secure variant
    Secure,


    /// Currently warning
    Warning,


    /// Represents critical variant
    Critical,


    /// Unknown or undefined state
    Unknown,
}
