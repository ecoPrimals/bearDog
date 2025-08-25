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


/// Modular workflow processors
///
/// This module contains all workflow processors organized by functionality.
/// Previously this was a single large file, now split for better maintainability.

pub mod core;
pub mod key_management;
pub mod policy;
pub mod registry;
pub mod system;
pub mod user_management;
// Re-export the trait and core types
pub use core::{WorkflowProcessingResult, WorkflowProcessor};
// Re-export all processors
pub use key_management::{KeyDeletionProcessor, KeyRotationProcessor};
pub use policy::{ConfigChangeProcessor, PolicyChangeProcessor};
pub use registry::WorkflowProcessorRegistry;
pub use system::{ComplianceAuditProcessor, SystemMaintenanceProcessor};
pub use user_management::{EmergencyAccessProcessor, UserProvisioningProcessor};
