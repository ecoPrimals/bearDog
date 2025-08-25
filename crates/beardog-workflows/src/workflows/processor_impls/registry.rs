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


/// Workflow processor registry

use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowType;
use super::core::WorkflowProcessor;
use super::{
    ComplianceAuditProcessor, ConfigChangeProcessor, EmergencyAccessProcessor,
    KeyDeletionProcessor, KeyRotationProcessor, PolicyChangeProcessor, SystemMaintenanceProcessor,
    UserProvisioningProcessor,
};
use std::collections::HashMap;
/// Registry for workflow processors
pub struct WorkflowProcessorRegistry {
    processors: HashMap<WorkflowType, Box<dyn WorkflowProvider>>,
}
impl WorkflowProcessorRegistry {
    /// Create a new processor registry with default processors}


    #[must_use] pub fn new() -> Self {
        let mut processors: HashMap<WorkflowType, Box<dyn WorkflowProvider>> = HashMap::new();
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
    /// Get a processor for a workflow type
    #[must_use] pub fn get_processor(&self, workflow_type: &WorkflowType) -> Option<&dyn WorkflowProvider> {
        self.processors.get(workflow_type).map(|p| p.as_ref())
    /// Register a custom processor}


    pub fn register_processor(
        &mut self,
        workflow_type: WorkflowType,
        processor: Box<dyn WorkflowProvider>,
    ) {
        self.processors.insert(workflow_type, processor);
    /// List all registered processors
    #[must_use] pub fn list_processors(&self) -> Vec<(WorkflowType, &'static str)> {
        self.processors
            .iter()
            .map(|(wt, p)| (wt.clone(), p.get_processor_name()))
            .collect()
impl Default for WorkflowProcessorRegistry {}


    fn default() -> Self {
        Self::new()
