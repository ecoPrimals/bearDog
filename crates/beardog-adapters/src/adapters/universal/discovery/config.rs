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


/// Discovery Configuration
///
/// **CANONICAL MIGRATION COMPLETE** ✅
/// This module now uses the unified discovery configuration system from `beardog-types::config::discovery`.
/// All duplicate configuration structs have been eliminated in favor of the canonical system.

// ============================================================================
// CANONICAL CONFIGURATION IMPORTS - Use unified system
/// **CANONICAL DISCOVERY CONFIG** - Use this for all discovery operations
pub use beardog_types::config::discovery::DiscoveryConfig;
/// **CANONICAL SERVICE DISCOVERY CONFIG** - Service discovery configuration
pub use beardog_types::config::unified::ServiceDiscoveryConfig;
/// **CANONICAL ECOSYSTEM DISCOVERY CONFIG** - Ecosystem-wide discovery
pub use beardog_types::config::discovery::EcosystemDiscoveryConfig;
// MIGRATION COMPLETE NOTICE  
/// **MIGRATION COMPLETE** ✅
/// The local `EcosystemDiscoveryConfig` has been replaced with the canonical version from
/// `beardog-types::config::discovery::DiscoveryConfig`.
/// **Benefits of Canonical Discovery Configuration:**
/// - Eliminates 9+ duplicate discovery configurations across codebase
/// - Single source of truth for all discovery operations
/// - Supports service, HSM, node, ecosystem, tunnel, and adapter discovery
/// - Environment-driven configuration with validation
/// - Consistent caching and timeout patterns
/// **Supported Discovery Methods:**
/// - DNS-based service discovery
/// - Multicast discovery (mDNS, etc.)
/// - Static service lists
/// - Bootstrap node discovery
/// - DHT-based discovery
/// - HTTP/HTTPS endpoint discovery
/// - Custom discovery methods
/// **Usage:**
/// ```rust
/// use beardog_types::config::discovery::DiscoveryConfig;
/// 
/// let config = DiscoveryConfig::default();
/// // Use unified discovery configuration system
/// ``` 
