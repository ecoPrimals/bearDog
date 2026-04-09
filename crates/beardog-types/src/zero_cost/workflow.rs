// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::future::Future;
use std::marker::PhantomData;

use crate::zero_cost::types::{Workflow, WorkflowResult};

/// Zero-cost workflow engine with compile-time optimization
pub struct ZeroCostWorkflowEngine<P>
where
    P: WorkflowProcessorTrait,
{
    processor: P,
    config: WorkflowEngineConfig,
    _capabilities: PhantomData<P::SupportedWorkflows>,
}

/// Trait defining the interface for workflow processors with zero-cost abstractions
pub trait WorkflowProcessorTrait {
    /// Compile-time description of which workflow categories this processor handles
    type SupportedWorkflows: WorkflowTypes;
    /// Error type returned from validation and processing
    type Error: std::error::Error + Send + Sync + 'static;

    /// Process a workflow asynchronously and return the result
    /// Processes workflow
    fn process_workflow(
        &self,
        workflow: Workflow,
    ) -> impl Future<Output = Result<WorkflowResult, Self::Error>>;

    ///
    /// # Errors
    /// Returns an error if the workflow is invalid or incompatible with this processor
    /// Validates workflow
    fn validate_workflow(&self, workflow: &Workflow) -> Result<(), Self::Error>;

    /// Capability flags describing supported workflow families
    fn supported_types(&self) -> &Self::SupportedWorkflows;
}

/// Workflow type capabilities with compile-time constants
pub trait WorkflowTypes {
    /// Whether security workflows are supported
    const SECURITY_WORKFLOWS: bool;
    /// Whether key management workflows are supported
    const KEY_MANAGEMENT: bool;
    /// Whether user management workflows are supported
    const USER_MANAGEMENT: bool;
}

/// Workflow engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEngineConfig {
    /// Maximum number of concurrent workflow executions
    /// Number of `max_concurrent`
    pub max_concurrent: usize,
    /// Per-workflow wall-clock limit in milliseconds
    pub timeout_ms: u64,
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
}

impl<P> ZeroCostWorkflowEngine<P>
where
    P: WorkflowProcessorTrait,
{
    /// Create a new zero-cost workflow engine
    #[must_use]
    pub const fn new(processor: P, config: WorkflowEngineConfig) -> Self {
        Self {
            processor,
            config,
            _capabilities: PhantomData,
        }
    }

    /// Process a workflow with zero-cost delegation
    ///
    /// # Errors
    /// Returns an error if the workflow processing fails or is rejected by the processor
    /// Processes data
    pub async fn process(&self, workflow: Workflow) -> Result<WorkflowResult, P::Error> {
        self.processor.process_workflow(workflow).await
    }

    /// Validate a workflow
    ///
    /// # Errors
    /// Returns an error if the workflow is invalid or unsupported
    /// Validates input
    pub fn validate(&self, workflow: &Workflow) -> Result<(), P::Error> {
        self.processor.validate_workflow(workflow)
    }

    /// Get supported workflow types
    pub fn supported_types(&self) -> &P::SupportedWorkflows {
        self.processor.supported_types()
    }

    /// Get workflow engine configuration
    #[must_use]
    pub const fn config(&self) -> &WorkflowEngineConfig {
        &self.config
    }

    /// Get maximum concurrent workflows
    #[must_use]
    pub const fn max_concurrent(&self) -> usize {
        self.config.max_concurrent
    }

    /// Get timeout in milliseconds
    #[must_use]
    pub const fn timeout_ms(&self) -> u64 {
        self.config.timeout_ms
    }

    /// Get retry attempts
    #[must_use]
    pub const fn retry_attempts(&self) -> u32 {
        self.config.retry_attempts
    }
}

impl Default for WorkflowEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            timeout_ms: 30000,
            retry_attempts: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    // SPDX-License-Identifier: AGPL-3.0-or-later
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
    use super::*;
    use crate::zero_cost::types::WorkflowStep;

    #[tokio::test]
    async fn test_workflow_engine_config_default() {
        let config = WorkflowEngineConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.max_concurrent, 100);
        assert_eq!(config.timeout_ms, 30000);
        assert_eq!(config.retry_attempts, 3);
    }

    // Mock workflow processor for testing
    struct MockWorkflowProcessor;
    struct MockWorkflowTypes;

    impl WorkflowTypes for MockWorkflowTypes {
        /// Security Workflows
        const SECURITY_WORKFLOWS: bool = true;
        /// Key Management
        const KEY_MANAGEMENT: bool = true;
        /// User Management
        const USER_MANAGEMENT: bool = false;
    }

    impl WorkflowProcessorTrait for MockWorkflowProcessor {
        /// S
        type SupportedWorkflows = MockWorkflowTypes;
        /// E
        type Error = std::io::Error;

        /// Processes workflow
        async fn process_workflow(
            &self,
            workflow: Workflow,
        ) -> Result<WorkflowResult, Self::Error> {
            Ok(WorkflowResult {
                workflow_id: workflow.id,
                status: "completed ".to_string(),
                results: std::collections::HashMap::new(),
                completed_at: chrono::Utc::now(),
            })
        }

        fn supported_types(&self) -> &Self::SupportedWorkflows {
            &MockWorkflowTypes
        }

        fn validate_workflow(&self, _workflow: &Workflow) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_workflow_processing() -> Result<(), Box<dyn std::error::Error>> {
        let processor = MockWorkflowProcessor;
        let config = WorkflowEngineConfig::default();
        let engine = ZeroCostWorkflowEngine::new(processor, config);

        let workflow = Workflow {
            id: "test-workflow".to_string(),
            workflow_type: "security".to_string(),
            steps: vec![WorkflowStep {
                id: "step1".to_string(),
                action: "validate".to_string(),
                parameters: std::collections::HashMap::new(),
            }],
            metadata: std::collections::HashMap::new(),
        };

        let result = engine.process(workflow).await?;
        assert_eq!(result.status, "completed ");
        assert_eq!(result.workflow_id, "test-workflow");
        Ok(())
    }

    struct FailingWorkflowProcessor;

    impl WorkflowProcessorTrait for FailingWorkflowProcessor {
        type SupportedWorkflows = MockWorkflowTypes;
        type Error = std::io::Error;

        async fn process_workflow(
            &self,
            _workflow: Workflow,
        ) -> Result<WorkflowResult, Self::Error> {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "process rejected",
            ))
        }

        fn supported_types(&self) -> &Self::SupportedWorkflows {
            &MockWorkflowTypes
        }

        fn validate_workflow(&self, _workflow: &Workflow) -> Result<(), Self::Error> {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "validation failed",
            ))
        }
    }

    #[test]
    fn engine_accessors_reflect_config_and_supported_types() {
        let processor = MockWorkflowProcessor;
        let config = WorkflowEngineConfig {
            max_concurrent: 42,
            timeout_ms: 99,
            retry_attempts: 7,
        };
        let engine = ZeroCostWorkflowEngine::new(processor, config.clone());

        assert_eq!(engine.max_concurrent(), 42);
        assert_eq!(engine.timeout_ms(), 99);
        assert_eq!(engine.retry_attempts(), 7);
        assert_eq!(engine.config().max_concurrent, config.max_concurrent);
        assert!(
            MockWorkflowTypes::SECURITY_WORKFLOWS
                && MockWorkflowTypes::KEY_MANAGEMENT
                && !MockWorkflowTypes::USER_MANAGEMENT
        );
    }

    #[tokio::test]
    async fn validate_propagates_processor_error() {
        let engine =
            ZeroCostWorkflowEngine::new(FailingWorkflowProcessor, WorkflowEngineConfig::default());
        let workflow = Workflow {
            id: "w".to_string(),
            workflow_type: "security".to_string(),
            steps: vec![],
            metadata: std::collections::HashMap::new(),
        };
        let res = engine.validate(&workflow);
        assert!(matches!(
            &res,
            Err(e) if e.kind() == std::io::ErrorKind::InvalidData
        ));
    }

    #[tokio::test]
    async fn process_propagates_processor_error() {
        let engine =
            ZeroCostWorkflowEngine::new(FailingWorkflowProcessor, WorkflowEngineConfig::default());
        let workflow = Workflow {
            id: "w2".to_string(),
            workflow_type: "security".to_string(),
            steps: vec![],
            metadata: std::collections::HashMap::new(),
        };
        let res = engine.process(workflow).await;
        assert!(matches!(
            &res,
            Err(e) if e.kind() == std::io::ErrorKind::InvalidInput
        ));
    }
}
