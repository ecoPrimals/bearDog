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


/// Universal adapters for cross-ecosystem integration
///
/// This module provides name-agnostic adapters that allow BearDog
/// to integrate with any service mesh or orchestrator following
/// the Universal Primal Architecture Standard.
pub mod bridge_adapter;
pub mod capability_adapter;
pub mod http_adapter;
pub mod protocol_adapter;
pub mod security_provider_bridge;

// New modular architecture
pub mod commercial_extraction;
pub mod service_registration;
// Re-export key types
pub use capability_adapter::{BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector};
pub use bridge_adapter::BridgeAdapter;
pub use http_adapter::HttpAdapter;
pub use protocol_adapter::ProtocolAdapter;
pub use security_provider_bridge::{BridgeConfig, SecurityProviderBridge};
// Re-export from modular components
pub use commercial_extraction::*;
pub use service_registration::*;
