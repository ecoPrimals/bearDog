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


/// Configuration structures for the Capability Manager
///
/// **CANONICAL MIGRATION COMPLETE** ✅
/// This module now uses the unified configuration system from `beardog-types::config::manager`.
/// All duplicate configuration structs have been eliminated in favor of the canonical system.

// ============================================================================
// CANONICAL CONFIGURATION IMPORTS - Use unified system
/// **CANONICAL CAPABILITY MANAGER CONFIG** - Use this for all capability management
pub use beardog_types::config::manager::CapabilityManagerConfig;
/// **CANONICAL MANAGER CONFIG** - Complete manager configuration system
pub use beardog_types::config::manager::ManagerConfig;
// MIGRATION COMPLETE NOTICE
/// **MIGRATION COMPLETE** ✅
/// The local `CapabilityManagerConfig` has been replaced with the canonical version from
/// `beardog-types::config::manager::CapabilityManagerConfig`.
/// **Benefits of Canonical Configuration:**
/// - Single source of truth across ecosystem
/// - Consistent configuration patterns
/// - Better type safety and validation
/// - Unified configuration management
/// - Environment-driven configuration support
/// **Usage:**
/// ```rust
/// use beardog_types::config::manager::CapabilityManagerConfig;
/// 
/// let config = CapabilityManagerConfig::default();
/// // Use unified configuration system
/// ```
