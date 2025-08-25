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


/// # Anti-Surveillance Privacy Protection - Modular Architecture
///
/// **DECOMPOSED FROM LARGE FILE** - Was 930 lines, now modular structure
/// This module implements comprehensive privacy protection mechanisms designed
/// to shield individuals from surveillance by corporations, governments, and
/// malicious actors. Your privacy is a fundamental human right.
/// ## Core Privacy Principles
/// - **Privacy by Design**: Everything defaults to maximum privacy
/// - **Data Minimization**: Collect only what's absolutely necessary
/// - **Individual Control**: You decide what to share and when
/// - **Surveillance Detection**: Actively detect and counter surveillance attempts
/// - **Anonymous by Default**: Protect identity unless explicitly disclosed
/// ## Modular Structure (eliminates large file)
/// - `types` - Privacy protection and vulnerability type definitions (~80 lines)
/// - `models` - Internal data structures and models (~100 lines)
/// - `engine` - Main privacy protection engine implementation (~300 lines)
/// - `surveillance` - Surveillance detection and monitoring (~200 lines)
/// - `audit` - Privacy audit trails and compliance (~150 lines)
/// **TOTAL**: 5 focused modules, each <300 lines (was 1 file with 930 lines)

// Module declarations
pub mod audit;
pub mod engine;
pub mod models;
pub mod surveillance;
pub mod types;
// Re-export the main types and interfaces
pub use audit::*;
pub use models::*;
pub use surveillance::*;
pub use types::*;
// Re-export the main privacy protection engine for backward compatibility
pub use engine::PrivacyProtectionEngine;
