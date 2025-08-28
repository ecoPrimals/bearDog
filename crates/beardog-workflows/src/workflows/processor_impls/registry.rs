

use beardog_errors::BearDogError;
use beardog_types::canonical::workflow::WorkflowType;
use super::core::WorkflowProcessor;
use super::{
    ComplianceAuditProcessor, ConfigChangeProcessor, EmergencyAccessProcessor,
    KeyDeletionProcessor, KeyRotationProcessor, PolicyChangeProcessor, SystemMaintenanceProcessor,
    UserProvisioningProcessor,
};
use std::collections::HashMap;

pub struct WorkflowProcessorRegistry {
    processors: HashMap<WorkflowType, Box<dyn WorkflowProvider>>,
}
impl WorkflowProcessorRegistry {

    #[must_use] pub fn new() -> Self {
        let mut processors: HashMap<WorkflowType, Box<dyn WorkflowProvider>> = HashMap::with_capacity(16);
        processors.insert(WorkflowType::KeyRotation, Box::new(KeyRotationProcessor));
        processors.insert(WorkflowType::KeyDeletion, Box::new(KeyDeletionProcessor));
        processors.insert(WorkflowType::PolicyChange, Box::new(PolicyChangeProcessor));
        processors.insert(
            WorkflowType::ConfigurationChange,
            Box::new(ConfigChangeProcessor));
        );
            WorkflowType::UserProvisioning,
            Box::new(UserProvisioningProcessor));
            WorkflowType::EmergencyAccess,
            Box::new(EmergencyAccessProcessor));
            WorkflowType::SystemMaintenance,
            Box::new(SystemMaintenanceProcessor));
            WorkflowType::ComplianceAudit,
            Box::new(ComplianceAuditProcessor));
        Self { processors }
    }

    #[must_use] pub fn get_processor(&self, workflow_type: &WorkflowType) -> Option<&dyn WorkflowProvider> {
        self.processors.get(workflow_type).map(|p| p.as_ref())

    pub fn register_processor(
        &mut self,
        workflow_type: WorkflowType,
        processor: Box<dyn WorkflowProvider>,
    ) {
        self.processors.insert(workflow_type, processor);

    #[must_use] pub fn list_processors(&self) -> Vec<(WorkflowType, &'static str)> {
        self.processors
            .iter()
            .map(|(wt, p)| (wt.clone(), p.get_processor_name()))
            .collect()
impl Default for WorkflowProcessorRegistry {}

    fn default() -> Self {
        Self::new()
