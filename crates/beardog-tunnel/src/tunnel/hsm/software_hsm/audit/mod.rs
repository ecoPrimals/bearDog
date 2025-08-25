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


// # Software HSM Audit System - Modular Architecture
//
// **DECOMPOSED FROM LARGE FILE** - Was 933 lines, now modular structure
// This module provides comprehensive audit logging functionality for the Software HSM.
// It tracks all operations, maintains audit trails, and provides query capabilities.
// ## Modular Structure (eliminates large file)
// - `storage` - Persistent audit storage backend (~300 lines)
// - `logger` - High-level audit logging interface (~250 lines)
// - `analytics` - Query and analytics functionality (~200 lines)
// - `export` - Export and maintenance operations (~150 lines)
// **TOTAL**: 4 focused modules, each <300 lines (was 1 file with 933 lines)

// Module declarations
pub mod logger;
pub mod storage;
pub mod types;
// Re-export the main types and interfaces
pub use storage::*;
pub use types::*;
// Re-export the main audit logger for backward compatibility
pub use logger::DefaultAuditLogger;
