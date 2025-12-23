// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{
    ConfigurationChangeProcessor, KeyDeletionProcessor, KeyRotationProcessor,
    PolicyChangeProcessor, SecurityScanProcessor,
};
use beardog_errors::BearDogError;
use beardog_types::workflow::{WorkflowRequest, WorkflowResponse, WorkflowType};
use std::collections::HashMap;

/// Zero-cost workflow processor using enum dispatch instead of trait objects
#[derive(Debug, Clone)]
pub enum WorkflowProcessor {
    /// Represents key rotation variant
    KeyRotation(KeyRotationProcessor),
    /// Represents key deletion variant
    KeyDeletion(KeyDeletionProcessor),
    /// Represents policy change variant
    PolicyChange(PolicyChangeProcessor),
    /// Represents configuration change variant
    ConfigurationChange(ConfigurationChangeProcessor),
    /// Represents security scan variant
    SecurityScan(SecurityScanProcessor),
}

impl WorkflowProcessor {
    /// Process a workflow request with zero-cost dispatch
    /// Processes data
    /// Processes data
    pub fn process(
        &self,
        request: &WorkflowRequest,
    ) -> Result<WorkflowResponse, BearDogError> {
        match self {
            Self::KeyRotation(processor) => processor.process(request),
            Self::KeyDeletion(processor) => processor.process(request),
            Self::PolicyChange(processor) => processor.process(request),
            Self::ConfigurationChange(processor) => processor.process(request),
            Self::SecurityScan(processor) => processor.process(request),
        }
    }
}

/// Zero-cost workflow processor registry using const generic dispatch
pub struct WorkflowProcessorRegistry {
    processors: HashMap<WorkflowType, WorkflowProcessor>,
}

impl WorkflowProcessorRegistry {
    /// Create a new registry with zero-cost processor dispatch
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        let mut processors = HashMap::with_capacity(16);

        processors.insert(
            WorkflowType::KeyRotation,
            WorkflowProcessor::KeyRotation(KeyRotationProcessor::new()),
        );
        processors.insert(
            WorkflowType::KeyDeletion,
            WorkflowProcessor::KeyDeletion(KeyDeletionProcessor::new()),
        );
        processors.insert(
            WorkflowType::PolicyChange,
            WorkflowProcessor::PolicyChange(PolicyChangeProcessor::new()),
        );
        processors.insert(
            WorkflowType::ConfigurationChange,
            WorkflowProcessor::ConfigurationChange(ConfigurationChangeProcessor::new()),
        );
        processors.insert(
            WorkflowType::SecurityScan,
            WorkflowProcessor::SecurityScan(SecurityScanProcessor::new()),
        );

        Self { processors }
    }

    /// Process workflow with compile-time dispatch
    /// Processes workflow
    /// Processes workflow
    pub fn process_workflow(
        &self,
        workflow_type: WorkflowType,
        request: &WorkflowRequest,
    ) -> Result<WorkflowResponse, BearDogError> {
        match self.processors.get(&workflow_type) {
            Some(processor) => processor.process(request),
            None => Err(BearDogError::validation(format!(
                "No processor registered for workflow type: {:?}",
                workflow_type
            ))),
        }
    }

    /// Get processor count
    /// Processes dataor_count
    /// Processes dataor_count
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }
}

impl Default for WorkflowProcessorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
