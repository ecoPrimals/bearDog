//! HSM Adapters
//!
//! Specific HSM adapter implementations

use super::core_types::*;
use beardog_errors::BearDogResult;
use async_trait::async_trait;

/// HSM adapter trait
#[async_trait]
pub trait HsmAdapter: Send + Sync + std::fmt::Debug {
    async fn connect(&mut self) -> BearDogResult<()>;
    async fn disconnect(&mut self) -> BearDogResult<()>;
    async fn execute_operation(&self, operation: UniversalOperation) -> BearDogResult<OperationResult>;
    async fn health_check(&self) -> BearDogResult<HealthStatus>;
}

pub struct Pkcs11Adapter;
pub struct AndroidStrongBoxAdapter;
pub struct BearDogNativeAdapter;

// TODO: Extract complete HSM adapter implementations 