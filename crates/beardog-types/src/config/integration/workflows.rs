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


/// # Integration Workflows Configuration - Canonical
///
/// **UNIFIED INTEGRATION WORKFLOWS CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Integration Workflows Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationWorkflowsConfig {
    pub enabled: bool,
    pub workflow_timeout: Duration,
    pub max_concurrent_workflows: u32,
    pub approval: WorkflowApprovalConfig,
}

impl Default for IntegrationWorkflowsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workflow_timeout: Duration::from_secs(3600),
            max_concurrent_workflows: 10,
            approval: WorkflowApprovalConfig::default(),
        }
    }
}

/// **CANONICAL** Workflow Approval Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowApprovalConfig {
    pub required: bool,
    pub approval_timeout: Duration,
    pub auto_approve_threshold: u32,
}

impl Default for WorkflowApprovalConfig {
    fn default() -> Self {
        Self {
            required: false,
            approval_timeout: Duration::from_secs(86400), // 24 hours
            auto_approve_threshold: 1000,
        }
    }
}

/// **CANONICAL** Workflow Execution Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionConfig {
    pub max_execution_time: Duration,
    pub retry_attempts: u32,
    pub parallel_execution: bool,
}

impl Default for WorkflowExecutionConfig {
    fn default() -> Self {
        Self {
            max_execution_time: Duration::from_secs(1800), // 30 minutes
            retry_attempts: 3,
            parallel_execution: true,
        }
    }
}
