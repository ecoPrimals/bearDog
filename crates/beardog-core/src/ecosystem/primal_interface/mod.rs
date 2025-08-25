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


/// # EcoPrimal Interface Implementation - Modular Architecture
///
/// **DECOMPOSED FROM LARGE FILE** - Was 1,214 lines, now modular structure
/// This module implements the EcoPrimal trait for `BearDog`, enabling full
/// integration with the biomeOS ecosystem and Universal Primal Architecture.
/// ## Modular Structure (eliminates large file)
/// - `trait_impl` - Core EcoPrimal trait implementation (~200 lines)
/// - `ecosystem_integration` - ToadStool, Songbird, Squirrel integrations (~400 lines)
/// - `hsm_management` - HSM providers and management (~400 lines)
/// - `api_endpoints` - AI-first API endpoints and handlers (~400 lines)
/// **TOTAL**: 4 focused modules, each <400 lines (was 1 file with 1,214 lines)

// Module declarations
pub mod trait_impl;
pub mod ecosystem_integration;
pub mod hsm_management;
pub mod api_endpoints;
// Re-export the main implementation
pub use trait_impl::*;
// Re-export integration components
pub use ecosystem_integration::*;
pub use hsm_management::*;
pub use api_endpoints::*;
// Note: The main MemoryKeyManager is exported from trait_impl which contains
// the EcoPrimal implementation for `BearDog`Core 
