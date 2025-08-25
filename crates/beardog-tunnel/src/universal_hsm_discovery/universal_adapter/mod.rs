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


/// Universal HSM Adapter
///
/// This module provides a unified interface for interacting with different HSM types,
/// abstracting away the specific implementation details and providing consistent
/// operations across all supported HSM interfaces.

pub mod adapter_core;
pub mod capabilities;
pub mod core_types;
pub mod distributed_operations;
pub mod external_primal_client;
pub mod external_primal_service;
pub mod hsm_adapters;
pub mod operation_routing;
pub mod service_discovery;
// Re-export main components
pub use adapter_core::*;
pub use capabilities::*;
pub use core_types::*;
pub use distributed_operations::*;
pub use external_primal_client::*;
pub use external_primal_service::*;
pub use hsm_adapters::*;
pub use operation_routing::*;
pub use service_discovery::*;
