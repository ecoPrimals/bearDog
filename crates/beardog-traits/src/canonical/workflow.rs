// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
// async_trait no longer needed - using native fn
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Temporary type definitions for canonical workflow traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParams {
    /// The parameters value
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// The result value
    pub result: serde_json::Value,
    /// Current status of the component
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub id: String,
    /// Name of the item
    pub name: String,
    /// Current status of the component
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    /// Current status of the component
    pub status: String,
    /// Optional result
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Name of the item
    pub name: String,
    /// Collection of steps
    pub steps: Vec<serde_json::Value>,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

pub trait WorkflowProvider: BaseProvider {
    /// Execute Workflow operation.
    /// Executes workflow
    fn execute_workflow(
        &self,
        workflow_id: &str,
        params: WorkflowParams,
    ) -> impl std::future::Future<Output = Result<WorkflowResult, BearDogError>> + Send;

    /// Get workflow status
    /// Gets workflow_status
    fn get_workflow_status(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Cancel workflow
    fn cancel_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// List Workflows operation.
    fn list_workflows(
        filter: &str,
    ) -> impl std::future::Future<Output = Result<Vec<WorkflowInfo>, BearDogError>> + Send;

    /// Get workflow execution history
    /// Gets workflow_executions
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
