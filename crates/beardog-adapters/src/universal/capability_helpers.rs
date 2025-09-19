// Helper functions for capability discovery and management
//
// This module contains utility functions used by the Universal Capability Adapter
// to keep the main implementation focused and under 1000 lines.

use crate::ecosystem::primal_types::{DiscoveredPrimal, UniversalEndpoint};
use crate::universal::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{
    HealthStatus, PerformanceMetrics, ServiceCapabilityType, UniversalCapability,
};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

impl CapabilityDiscoveryRequest {
    /// Create a new capability discovery request
    /// Creates a new instance
    pub fn new(capability_type: ServiceCapabilityType) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            capability_type,
            requirements: CapabilityRequirements::default(),
            preferences: None,
        }
    }

    /// Set security requirements
    /// Creates instance with security requirements
    pub fn with_security_requirements(mut self, requirements: SecurityRequirements) -> Self {
        self.requirements.security = requirements;
        self
    }

    /// Set availability requirements
    /// Creates instance with availability requirements
    pub fn with_availability_requirements(
        mut self,
        requirements: AvailabilityRequirements,
    ) -> Self {
        self.requirements.availability = requirements;
        self
    }

    pub fn with_performance_requirements(mut self, requirements: PerformanceMetrics) -> Self {
        self.requirements.performance = requirements;
        self
    }

    /// Set capability preferences
    /// Creates instance with preferences
    pub fn with_preferences(mut self, preferences: CapabilityPreferences) -> Self {
        self.preferences = Some(preferences);
        self
    }
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            security: SecurityRequirements::default(),
            availability: AvailabilityRequirements::default(),
            performance: PerformanceMetrics::default(),
        }
    }
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: 3,
            require_encryption: true,
            require_authentication: true,
        }
    }
}

impl Default for AvailabilityRequirements {
    fn default() -> Self {
        Self {
            min_uptime_percentage: 99.0,
            max_response_time_ms: 1000,
        }
    }
}

/// Provider filtering and ranking utilities
pub struct ProviderUtils;

impl ProviderUtils {
    /// Filter providers based on requirements
    pub fn filter_providers(
        providers: &[UniversalCapability],
        requirements: &CapabilityRequirements,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        let mut filtered = Vec::new();

        for provider in providers {
            if Self::meets_security_requirements(provider, &requirements.security)? {
                if Self::meets_availability_requirements(provider, &requirements.availability)
                    ?
                {
                    if Self::meets_performance_requirements(provider, &requirements.performance)
                        ?
                    {
                        filtered.push(provider.clone());
                    }
                }
            }
        }

        info!(
            "🔍 Filtered {} providers from {} candidates",
            filtered.len(),
            providers.len()
        );
        Ok(filtered)
    }

    pub fn rank_providers(
        providers: &[UniversalCapability],
        preferences: &Option<CapabilityPreferences>,
    ) -> BearDogResult<Vec<RankedCapabilityProvider>> {
        let mut ranked = Vec::new();

        for provider in providers {
            let rank = Self::calculate_provider_rank(provider, preferences)?;
            let performance_estimate = Self::estimate_provider_performance(provider)?;

            ranked.push(RankedCapabilityProvider {
                provider_id: provider.provider_id.clone(),
                capability_type: provider.capability_type.clone(),
                rank,
                performance_estimate,
            });
        }

        // Sort by rank (highest first)
        ranked.sort_by(|a, b| {
            b.rank
                .partial_cmp(&a.rank)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        info!("📊 Ranked {} providers", ranked.len());
        Ok(ranked)
    }

    /// Validate that providers are actually available
    /// Validates providers
    /// Validates providers
    pub fn validate_providers(
        providers: &[RankedCapabilityProvider],
    ) -> BearDogResult<Vec<RankedCapabilityProvider>> {
        let mut validated = Vec::new();

        for provider in providers {
            if Self::validate_provider_health(&provider.provider_id)? {
                validated.push(provider.clone());
            } else {
                warn!(
                    "⚠️ Provider {} failed health validation",
                    provider.provider_id
                );
            }
        }

        info!("✅ Validated {} providers", validated.len());
        Ok(validated)
    }

    // Private helper methods
    fn meets_security_requirements(
        provider: &UniversalCapability,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<bool> {
        // Implementation would check provider's security capabilities
        Ok(true) // Placeholder
    }


    fn meets_availability_requirements(
        provider: &UniversalCapability,
        requirements: &AvailabilityRequirements,
    ) -> BearDogResult<bool> {
        // Implementation would check provider's availability metrics
        Ok(true) // Placeholder
    }


    fn meets_performance_requirements(
        provider: &UniversalCapability,
        requirements: &PerformanceMetrics,
    ) -> BearDogResult<bool> {
        // Implementation would check provider's performance capabilities
        Ok(true) // Placeholder
    }


    fn calculate_provider_rank(
        provider: &UniversalCapability,
        preferences: &Option<CapabilityPreferences>,
    ) -> BearDogResult<f64> {
        let mut rank = 0.0;

        // Base rank from provider metrics
        rank += 0.5; // Base score

        // Apply preferences if provided
        if let Some(prefs) = preferences {
            if prefs.preferred_providers.contains(&provider.provider_id) {
                rank += 0.3;
            }
            if prefs.cost_optimization {
                rank += 0.1; // Placeholder for cost scoring
            }
        }

        Ok(rank.min(1.0))
    }


    fn estimate_provider_performance(
        provider: &UniversalCapability,
    ) -> BearDogResult<PerformanceEstimate> {
        Ok(PerformanceEstimate {
            estimated_latency_ms: 50.0,
            estimated_throughput: 1000.0,
            confidence_level: 0.8,
        })
    }

    /// Validates provider_health
    fn validate_provider_health(provider_id: &str) -> BearDogResult<bool> {
        debug!("🏥 Validating health for provider: {}", provider_id);
        // Implementation would perform actual health check
        Ok(true) // Placeholder
    }
}

/// Connection management utilities
pub struct ConnectionUtils;

impl ConnectionUtils {
    /// Create a new capability connection
    /// Creates connection
    /// Creates connection
    pub fn create_connection(
        provider_id: String,
        capability_type: ServiceCapabilityType,
        endpoint: UniversalEndpoint,
    ) -> BearDogResult<CapabilityConnection> {
        let connection_id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now();

        info!("🔗 Creating connection to provider: {}", provider_id);

        Ok(CapabilityConnection {
            provider_id,
            capability_type,
            endpoint,
            connection_id,
            established_at: now,
            last_health_check: now,
            health_status: HealthStatus::Healthy,
            metrics: ConnectionMetrics::default(),
        })
    }

    /// Update connection health status
    /// Updates connection_health
    /// Updates connection_health
    pub fn update_connection_health(connection: &mut CapabilityConnection) -> BearDogResult<()> {
        connection.last_health_check = std::time::SystemTime::now();

        // Perform health check (placeholder)
        connection.health_status = HealthStatus::Healthy;

        debug!(
            "💓 Updated health for connection: {}",
            connection.connection_id
        );
        Ok(())
    }

    /// Calculate connection metrics
    pub fn calculate_connection_metrics(connection: &CapabilityConnection) -> ConnectionMetrics {
        let mut metrics = connection.metrics.clone();

        // Update average latency calculation
        if metrics.responses_received > 0 {
            metrics.average_latency_ms = metrics.average_latency_ms * 0.9 + 50.0 * 0.1;
            // Rolling average
        }

        metrics
    }
}
