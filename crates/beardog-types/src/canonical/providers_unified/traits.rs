// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified async provider traits (base, security/HSM, and auxiliary domains).

/// Core [`UnifiedProvider`] surface plus telemetry types.
pub mod base_traits;
/// Consolidated async provider hierarchy (`ConsolidatedProvider` and shared DTOs).
pub mod consolidated;
/// Storage, AI, and ancillary provider contracts.
pub mod other_traits;
/// Security, authn/z, and HSM-specific provider APIs.
pub mod security_traits;

// Re-export all traits and types for backward compatibility
pub use base_traits::{
    CustomMetric, HealthStatus, NetworkIoMetrics, ProviderCapability, ProviderHealth, ProviderInfo,
    ProviderMetrics, ProviderType, ResourceUsage, SystemMetrics, UnifiedProvider,
};
pub use other_traits::{
    UnifiedAiProvider, UnifiedMonitoringProvider, UnifiedNetworkProvider, UnifiedStorageProvider,
};
pub use security_traits::{
    AttestationResponse, AuthenticationRequest, AuthenticationResponse, AuthorizationRequest,
    AuthorizationResponse, BackupInfo, HsmDeviceInfo, KeyBackupSpec, KeyGenerationSpec, KeyInfo,
    KeyType, KeyUsage, SecurityContext, UnifiedHsmProvider, UnifiedSecurityProvider,
};
