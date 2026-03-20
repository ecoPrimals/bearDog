// SPDX-License-Identifier: AGPL-3.0-only

//! Workflow orchestration surface: traits, reference implementations, and shared types.
//!
//! The [`canonical_traits`] module defines repository, processor, observer, and service
//! contracts. [`canonical_examples`] provides in-memory and logging implementations suitable
//! for tests and demos. [`types`] holds cross-cutting DTOs and execution metadata.

pub mod canonical_examples;
/// Core workflow traits: identity, persistence, step execution, and lifecycle observation.
///
/// Defines repository, processor, observer, and service contracts (see submodule items).
pub mod canonical_traits;

/// Request/response payloads, configuration mirrors, and rich enums for execution and approval flows.
pub mod types;

pub use canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};

pub use canonical_examples::{
    ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus, InMemoryWorkflowRepository,
    LoggingWorkflowObserver, ProcessingContext,
};

pub use types::{WorkflowRequest, WorkflowResponse};
