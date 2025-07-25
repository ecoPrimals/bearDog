//! External Primal Client
//!
//! Client manager for external primal network communication

use super::core_types::*;
use beardog_errors::BearDogResult;
use async_trait::async_trait;

/// External primal client manager for network communication
pub struct ExternalPrimalClient;

/// Trait for external primal connections
#[async_trait]
pub trait ExternalPrimalConnection: Send + Sync + std::fmt::Debug {
    async fn send_request(&self, request: Vec<u8>) -> BearDogResult<Vec<u8>>;
    async fn health_check(&self) -> BearDogResult<bool>;
}

/// Tarpc-based external primal connection
pub struct TarpcPrimalConnection;

// TODO: Extract complete external primal client implementation 