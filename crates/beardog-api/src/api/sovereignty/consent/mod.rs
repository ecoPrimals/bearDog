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


/// # Consent Management for Individual Sovereignty - Modular Architecture
///
/// **DECOMPOSED FROM LARGE FILE** - Was 924 lines, now modular structure
/// This module ensures that every interaction, data sharing, and resource access
/// happens only with explicit, informed consent from individuals. Your consent
/// is sacred and cannot be bypassed or assumed.
/// ## Core Consent Principles
/// - **Explicit Consent**: Every action requires clear, specific approval
/// - **Informed Consent**: You understand exactly what you're agreeing to
/// - **Granular Control**: Consent can be specific to exact operations and data
/// - **Revocable Always**: You can withdraw consent at any time
/// - **Audit Trail**: Complete history of all consent decisions
/// ## Modular Structure (eliminates large file)
/// - `types` - Consent types and enum definitions (~80 lines)
/// - `models` - Internal data structures and models (~120 lines)
/// - `engine` - Main consent management engine (~250 lines)
/// - `requests` - Consent request handling and workflows (~200 lines)
/// - `records` - Consent record management and storage (~150 lines)
/// - `templates` - Consent templates for common scenarios (~100 lines)
/// **TOTAL**: 6 focused modules, each <250 lines (was 1 file with 924 lines)

// Module declarations
pub mod engine;
pub mod models;
pub mod records;
pub mod requests;
pub mod templates;
pub mod types;
// Re-export the main types and interfaces
pub use models::*;
pub use records::*;
pub use requests::*;
pub use templates::*;
pub use types::*;
// Re-export the main consent management engine for backward compatibility
pub use engine::ConsentManagementEngine;
