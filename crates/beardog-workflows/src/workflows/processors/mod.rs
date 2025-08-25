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


/// Workflow processor implementations
///
/// This module contains various processor implementations for different workflow types.
// Core processor trait
pub mod core;
pub mod key_management;
pub mod policy;
pub mod registry;
pub mod security;
pub mod system;

// Re-export core traits and canonical types
pub use crate::workflows::canonical::{
    AuditAction, Workflow, WorkflowAuditEntry, WorkflowExecutionStatus, WorkflowMetrics,
    WorkflowProcessingResult, WorkflowStatus,
};
pub use core::WorkflowProcessor;
// Re-export processor implementations
pub use key_management::KeyManagementProcessor;
pub use policy::PolicyProcessor;
pub use registry::RegistryProcessor;
pub use security::SecurityProcessor;
pub use system::SystemProcessor;
