// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-centric workflow DTOs and the canonical [`WorkflowProvider`] trait.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Arbitrary JSON parameters passed when starting a workflow run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParams {
    /// The parameters value
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Outcome payload and coarse status string for a finished or partial run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// The result value
    pub result: serde_json::Value,
    /// Current status of the component
    pub status: String,
}

/// Summary row for [`WorkflowProvider::list_workflows`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInfo {
    /// Workflow definition id.
    pub id: String,
    /// Name of the item
    pub name: String,
    /// Current status of the component
    pub status: String,
}

/// One execution attempt of a workflow definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Unique run id.
    pub id: String,
    /// Parent definition id.
    pub workflow_id: String,
    /// Current status of the component
    pub status: String,
    /// Optional result
    pub result: Option<serde_json::Value>,
}

/// Declarative workflow template stored in the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Name of the item
    pub name: String,
    /// Collection of steps
    pub steps: Vec<serde_json::Value>,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

/// CRUD and execution API for durable workflows.
pub trait WorkflowProvider: BaseProvider {
    /// Execute Workflow operation.
    /// Executes workflow
    fn execute_workflow(
        &self,
        workflow_id: &str,
        params: WorkflowParams,
    ) -> impl std::future::Future<Output = Result<WorkflowResult, BearDogError>> + Send;

    /// Get workflow status
    /// Gets `workflow_status`
    fn get_workflow_status(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Best-effort cancellation of an in-flight run.
    fn cancel_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// List Workflows operation.
    fn list_workflows(
        filter: &str,
    ) -> impl std::future::Future<Output = Result<Vec<WorkflowInfo>, BearDogError>> + Send;

    /// Get workflow execution history
    /// Gets `workflow_executions`
    fn get_workflow_executions(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<WorkflowExecution>, BearDogError>> + Send;

    /// Create new workflow
    /// Creates workflow
    fn create_workflow(
        &self,
        definition: WorkflowDefinition,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Update workflow definition
    /// Updates workflow
    fn update_workflow(
        &self,
        workflow_id: &str,
        definition: WorkflowDefinition,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Delete workflow
    /// Removes workflow
    fn delete_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}
