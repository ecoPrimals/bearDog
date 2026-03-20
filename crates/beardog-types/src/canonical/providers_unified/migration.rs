// SPDX-License-Identifier: AGPL-3.0-only

// Provider Migration Utilities
//
// This module provides migration functions to transition from legacy provider
// configurations to the new canonical unified structure.

use super::CanonicalProviderConfig;
use super::core::ProviderType as CoreProviderType;
use beardog_errors::BearDogError;

/// Migrate from legacy provider config
pub fn migrate_from_legacy() -> Result<CanonicalProviderConfig, BearDogError> {
    // Create a default configuration with sensible production settings
    let mut config = CanonicalProviderConfig::default();

    // Set reasonable defaults for production migration
    config.core.enabled = true;
    config.core.name = "migrated-provider".to_string();
    config.core.version = "1.0.0".to_string();
    config.core.priority = 100;

    // Enable essential features
    config.health.enabled = true;
    config.health.check_interval = std::time::Duration::from_secs(30);

    // Configuration is valid by construction
    Ok(config)
}

/// Migrate from legacy provider config with specific settings
pub fn migrate_from_legacy_with_settings(
    name: &str,
    provider_type: CoreProviderType,
    _endpoint: &str,
) -> Result<CanonicalProviderConfig, BearDogError> {
    let mut config = migrate_from_legacy()?;

    // Apply specific settings
    config.core.name = name.to_string();
    config.core.provider_type = provider_type;

    // Configuration is valid by construction
    Ok(config)
}

#[must_use]
/// Heuristic: empty provider name or disabled health checks imply legacy layouts.
pub fn needs_migration(config: &CanonicalProviderConfig) -> bool {
    // Check for legacy patterns that indicate migration is needed
    config.core.name.is_empty() || !config.health.enabled
}

/// **TYPE MIGRATION MAPPINGS**
///
/// to their unified equivalents.
// Re-export unified types with legacy names for compatibility
pub use super::traits::{
    AuthenticationRequest as AuthenticationCredentials,
    AuthenticationResponse as AuthenticationResult, AuthorizationResponse as AuthorizationResult,
    ProviderCapability, ProviderHealth as ProviderHealthStatus, ProviderInfo,
};

// Legacy service types - map to unified equivalents
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Legacy `ServiceHealth` type - use `ProviderHealth` instead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Is Healthy
    /// Whether `is_healthy` is enabled
    pub is_healthy: bool,
    /// Last Check
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// Response Time Ms
    pub response_time_ms: f64,
    /// Error Message
    /// Optional error message
    pub error_message: Option<String>,
}

impl From<ServiceHealth> for super::traits::ProviderHealth {
    fn from(service_health: ServiceHealth) -> Self {
        use std::collections::HashMap;
        use std::time::SystemTime;

        #[allow(clippy::cast_sign_loss)]
        let timestamp = SystemTime::UNIX_EPOCH
            + std::time::Duration::from_secs(service_health.last_check.timestamp().max(0) as u64);

        let status = if service_health.is_healthy {
            super::traits::HealthStatus::Healthy
        } else {
            super::traits::HealthStatus::Unhealthy
        };

        let mut details = HashMap::new();
        details.insert(
            "response_time_ms".to_string(),
            service_health.response_time_ms.to_string(),
        );

        let resource_usage = super::traits::ResourceUsage {
            cpu_percent: 0.0,    // Not available in legacy type
            memory_bytes: 0,     // Not available in legacy type
            memory_percent: 0.0, // Not available in legacy type
            network_io: super::traits::NetworkIoMetrics {
                bytes_sent: 0,
                bytes_received: 0,
                packets_sent: 0,
                packets_received: 0,
            },
            disk_io: HashMap::new(),
        };

        Self {
            status,
            timestamp,
            details,
            resource_usage,
            last_error: if service_health.is_healthy {
                None
            } else {
                service_health.error_message
            },
        }
    }
}

/// Legacy `CacheStats` type - use `ProviderMetrics` instead  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Hit Count
    /// Number of hit
    pub hit_count: u64,
    /// Miss Count
    /// Number of miss
    pub miss_count: u64,
    /// Size
    /// Number of size
    pub size: u64,
    /// Eviction Count
    /// Number of eviction
    pub eviction_count: u64,
}

impl From<CacheStats> for super::traits::ProviderMetrics {
    fn from(cache_stats: CacheStats) -> Self {
        let total_requests = cache_stats.hit_count + cache_stats.miss_count;
        let hit_rate = if total_requests > 0 {
            #[allow(clippy::cast_precision_loss)]
            let hit_count_f64 = cache_stats.hit_count as f64;
            #[allow(clippy::cast_precision_loss)]
            let total_f64 = total_requests as f64;
            hit_count_f64 / total_f64
        } else {
            0.0
        };

        let mut custom_metrics = HashMap::new();
        #[allow(clippy::cast_precision_loss)]
        custom_metrics.insert("hit_count".to_string(), cache_stats.hit_count as f64);
        #[allow(clippy::cast_precision_loss)]
        custom_metrics.insert("miss_count".to_string(), cache_stats.miss_count as f64);
        #[allow(clippy::cast_precision_loss)]
        custom_metrics.insert("cache_size".to_string(), cache_stats.size as f64);
        #[allow(clippy::cast_precision_loss)]
        custom_metrics.insert(
            "eviction_count".to_string(),
            cache_stats.eviction_count as f64,
        );
        custom_metrics.insert("hit_rate".to_string(), hit_rate);

        let mut performance = HashMap::new();
        performance.insert("hit_rate".to_string(), hit_rate);
        performance.insert("error_rate".to_string(), 1.0 - hit_rate);

        let custom_metrics_vec = custom_metrics
            .into_iter()
            .map(|(name, value)| super::traits::CustomMetric {
                name: name.clone(),
                value,
                unit: "count".to_string(),
                description: format!("Cache metric: {name}"),
                tags: HashMap::new(),
            })
            .collect();

        let system_metrics = super::traits::SystemMetrics {
            uptime_seconds: 0,
            total_requests,
            successful_requests: cache_stats.hit_count,
            failed_requests: cache_stats.miss_count,
            avg_response_time_ms: 0.0,
            active_connections: 0,
            error_rate: 1.0 - hit_rate,
        };

        Self {
            timestamp: std::time::SystemTime::now(),
            performance,
            custom_metrics: custom_metrics_vec,
            system_metrics,
        }
    }
}

// Migration helpers removed Nov 11, 2025 - Zero active usage confirmed
// Migration is complete - all consumers use unified traits directly
