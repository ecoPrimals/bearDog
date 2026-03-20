// SPDX-License-Identifier: AGPL-3.0-only

//! # Canonical Provider Traits (DEPRECATED - LEGACY)
//!
//! ⚠️ **MIGRATION NOTICE**: These traits are **DEPRECATED** and maintained only for
//! backward compatibility during the migration period.
//!
//! ## Migration Path
//!
//! **DO NOT USE** these traits for new code. Use the following instead:
//!
//! ### Current (Recommended):
//! ```rust
//! use beardog_traits::unified::{
//!     BearDogProvider,
//!     SecurityProvider,
//!     CryptoProvider,
//!     HsmProvider,
//!     MonitoringProvider,
//! };
//! ```
//!
//! ### Future Target (Week 3-4):
//! ```rust
//! use beardog_types::canonical::providers_unified::{
//!     UnifiedProvider,
//!     UnifiedSecurityProvider,
//!     UnifiedHsmProvider,
//!     UnifiedMonitoringProvider,
//! };
//! ```
//!
//! ## Why Deprecated?
//!
//! The `canonical/` traits are being phased out as part of the BearDog v3.6
//! unification initiative. The trait system is consolidating from three parallel
//! hierarchies into a single, unified location in `beardog-types`.
//!
//! ## Timeline
//!
//! - **Current**: `canonical/` traits deprecated but functional
//! - **Week 3-4**: All consumers migrated to `unified/`
//! - **Future**: `canonical/` traits removed entirely
//!
//! ## Support
//!
//! These traits will be maintained for backward compatibility through v3.6.x
//! but will be removed in v3.7.0.

/// Connection status, provider info, and the legacy [`BaseProvider`] surface.
pub mod base;

/// Authentication- and policy-oriented canonical provider hooks.
pub mod security;

/// Hardware-backed key operations in the legacy trait layer.
pub mod hsm;

/// Encryption, signing, and digest helpers for canonical providers.
pub mod crypto;

/// Orchestration hooks for multi-step security workflows.
pub mod workflow;

/// Cache-oriented provider contracts retained for compatibility.
pub mod cache;

/// Metrics and health probes in the canonical naming scheme.
pub mod monitoring;

/// Persistence and query abstractions for canonical integrations.
pub mod database;

/// AI/ML provider hooks predating the unified module.
pub mod ai;

/// Catch-all “universal” provider capabilities.
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
