
 /// Core functionality
 /// Core functionality

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod core;
pub mod key_management;
pub mod policy;
pub mod registry;
pub mod system;
pub mod user_management;

pub use core::{WorkflowProcessingResult, WorkflowProcessor};

pub use key_management::{KeyDeletionProcessor, KeyRotationProcessor};
pub use policy::{ConfigChangeProcessor, PolicyChangeProcessor};
pub use registry::WorkflowProcessorRegistry;
pub use system::{ComplianceAuditProcessor, SystemMaintenanceProcessor};
pub use user_management::{EmergencyAccessProcessor, UserProvisioningProcessor};
