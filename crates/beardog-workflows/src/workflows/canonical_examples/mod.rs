// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical workflow examples for the `BearDog` ecosystem
//!
//! This module provides reference implementations of workflow types,
//! repositories, processors, observers, and commands.

mod commands;
mod example;
mod observer;
mod processor;
mod repository;
mod types;

#[cfg(test)]
mod tests;

pub use commands::StartWorkflowCommand;
pub use example::run_comprehensive_example;
pub use observer::LoggingWorkflowObserver;
pub use processor::ExampleWorkflowProcessor;
pub use repository::InMemoryWorkflowRepository;
pub use types::{ExampleWorkflow, ExampleWorkflowId, ExampleWorkflowStatus, ProcessingContext};
