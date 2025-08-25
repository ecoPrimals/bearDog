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


/// Configuration types for node registry
///
/// This module contains all configuration-related types for the node registry,
/// organized into focused sub-modules for better maintainability.
/// # Sub-modules
/// - `registry`: Main registry configuration (`RegistryConfig`)
/// - `bootstrap`: Bootstrap node configuration (`BootstrapNodeConfig`)
/// - `federation`: Federation configuration (`FederationConfig`)
/// - `phonebook`: Phonebook service configuration (`PhonebookConfig`)
/// - `p2p`: Peer-to-peer networking configuration (`P2PConfig`)
/// # Example
/// ```rust
/// use beardog::node_registry::types::config::{RegistryConfig, FederationConfig};
/// let registry_config = RegistryConfig::new(
///     "my-registry".to_string(),
///     "My Registry".to_string()
/// )
/// .with_port(9090)
/// .with_max_nodes(5000);
/// let federation_config = FederationConfig::new()
///     .with_enabled(true)
///     .with_heartbeat_interval(30);
/// ```

pub mod registry;
pub mod bootstrap;
pub mod federation;
pub mod phonebook;
pub mod p2p;
// Re-export all configuration types for backward compatibility
pub use registry::RegistryConfig;
pub use bootstrap::BootstrapNodeConfig;
pub use federation::FederationConfig;
pub use phonebook::PhonebookConfig;
pub use p2p::P2PConfig; 
