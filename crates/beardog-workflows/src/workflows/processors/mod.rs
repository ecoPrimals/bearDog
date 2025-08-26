

pub mod core;
pub mod key_management;
pub mod policy;
pub mod registry;
pub mod security;
pub mod system;

pub use crate::workflows::canonical::{
    AuditAction, Workflow, WorkflowAuditEntry, WorkflowExecutionStatus, WorkflowMetrics,
    WorkflowProcessingResult, WorkflowStatus,
};
pub use core::WorkflowProcessor;

pub use key_management::KeyManagementProcessor;
pub use policy::PolicyProcessor;
pub use registry::RegistryProcessor;
pub use security::SecurityProcessor;
pub use system::SystemProcessor;
