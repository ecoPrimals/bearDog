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


/// # EcoPrimal Interface Implementation - MIGRATED TO MODULAR STRUCTURE
///
/// **MIGRATION COMPLETE** - This file now delegates to the modular structure:
/// 
/// ## MODULAR STRUCTURE (eliminates large file):
/// - `primal_interface/mod.rs` - Module organization  
/// - `primal_interface/trait_impl.rs` - Core trait implementation (~200 lines)
/// - `primal_interface/ecosystem_integration.rs` - Ecosystem integrations (~400 lines)
/// - `primal_interface/hsm_management.rs` - HSM management (~400 lines)
/// - `primal_interface/api_endpoints.rs` - API endpoints (~400 lines)
/// **TOTAL**: 4 focused modules, each <400 lines (was 1 file with 1,216 lines)

// Re-export everything from the modular structure
pub use primal_interface::*;
