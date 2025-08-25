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


/// # Canonical Integration Configuration Types
///
/// **ELIMINATES INTEGRATION CONFIG FRAGMENTATION**
/// This module consolidates integration configurations from:
/// - `beardog-config/src/integration.rs` (WorkflowConfig, AdapterConfigs, etc.)
/// - `beardog-workflows/src/workflows/` (workflow configurations)
/// - Multiple external system integration configurations
/// ## Unified Integration Architecture
/// Split into focused sub-modules for maintainability:
/// - `workflows` - Workflow management configurations
/// - `adapters` - External system adapter configurations
/// - `external_systems` - External service integration configurations
/// - `monitoring` - Integration monitoring and audit configurations
/// - `policies` - Integration policies and governance
pub mod adapters;
pub mod external_systems;
pub mod monitoring;
pub mod policies;
pub mod workflows;

// Re-export main types for backwards compatibility
pub use adapters::{
    AdapterAuthMethod, AdapterConfig, AdapterConnectionPooling, AdapterPerformanceConfig,
    AdapterSecurityPolicy,
};
pub use external_systems::{
    ExternalSystemConfig, MessageQueuingConfig,
    ProtocolSupportConfig,
};


pub use monitoring::{IntegrationAuditConfig, IntegrationMonitoringConfig};
pub use policies::{IntegrationCircuitBreaker, IntegrationRateLimiting, IntegrationPoliciesConfig, NotificationRetryPolicy};
pub use workflows::{
    IntegrationWorkflowsConfig, WorkflowApprovalConfig, WorkflowExecutionConfig,
};


use serde::{Deserialize, Serialize};
/// **CANONICAL** Integration Configuration - Top-level integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IntegrationConfig {
    /// Adapter configuration
    pub adapters: AdapterConfig,
    /// External systems configuration
    pub external_systems: ExternalSystemConfig,
    /// Monitoring configuration
    pub monitoring: IntegrationMonitoringConfig,
    /// Policies configuration
    pub policies: IntegrationPoliciesConfig,
    /// Workflows configuration
    pub workflows: IntegrationWorkflowsConfig,
}

