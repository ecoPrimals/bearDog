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


/// # AI-First CLI Commands - Modular Architecture
///
/// **DECOMPOSED FROM LARGE FILE** - Was 900 lines, now modular structure
/// This module provides CLI commands optimized for AI and automation consumption.
/// - JSON output for all operations
/// - Batch processing capabilities
/// - Streaming support
/// - Comprehensive error codes
/// - No interactive prompts (automation-friendly)
/// ## Modular Structure (eliminates large file)
/// - `types` - Common CLI types and response structures (~80 lines)
/// - `commands` - Main command definitions and enums (~100 lines)
/// - `security` - Security operations and handlers (~150 lines)
/// - `genetics` - Genetics operations and handlers (~120 lines)
/// - `hsm` - HSM operations and handlers (~140 lines)
/// - `config` - Configuration operations and handlers (~100 lines)
/// - `handlers` - Command execution logic and processing (~200 lines)
/// **TOTAL**: 7 focused modules, each <200 lines (was 1 file with 900 lines)
// Module declarations
pub mod commands;
pub mod config;
pub mod genetics;
pub mod handlers;
pub mod hsm;
pub mod security;
pub mod types;

// Re-export the main types and interfaces
pub use handlers::*;
// Re-export the main AI command enum for backward compatibility
pub use commands::AiCommand;
