// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod base;

pub mod security;

pub mod hsm;

pub mod crypto;

pub mod workflow;

pub mod cache;

pub mod monitoring;

pub mod database;

pub mod ai;

pub mod universal;

pub use ai::AiProvider;
pub use base::{
    BaseProvider, ConnectionStatus, PlatformProvider, ProviderInfo, ProviderMetrics, ServiceHealth,
};
pub use cache::{CacheProvider, EnhancedCacheProvider};
pub use crypto::CryptoProvider;
pub use database::DatabaseProvider;
pub use hsm::HsmProvider;
pub use monitoring::MonitoringProvider;
pub use security::SecurityProvider;
pub use universal::UniversalProvider;
pub use workflow::WorkflowProvider;
