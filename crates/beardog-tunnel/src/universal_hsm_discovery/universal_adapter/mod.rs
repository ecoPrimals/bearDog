//! Universal HSM Adapter
//!
//! This module provides a unified interface for interacting with different HSM types,
//! abstracting away the specific implementation details and providing consistent
//! operations across all supported HSM interfaces.

pub mod core_types;
pub mod external_primal_service;
pub mod capabilities;
pub mod distributed_operations;
pub mod service_discovery;
pub mod external_primal_client;
pub mod adapter_core;
pub mod operation_routing;
pub mod hsm_adapters;

// Re-export main components
pub use core_types::*;
pub use external_primal_service::*;
pub use capabilities::*;
pub use distributed_operations::*;
pub use service_discovery::*;
pub use external_primal_client::*;
pub use adapter_core::*;
pub use operation_routing::*;
pub use hsm_adapters::*; 