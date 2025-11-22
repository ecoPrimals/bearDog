// Helper functions for capability discovery and management
//
// This module contains utility functions used by the Universal Capability Adapter
// to keep the main implementation focused and under 1000 lines.

use crate::ecosystem::primal_types::{DiscoveredPrimal, UniversalEndpoint};
use crate::universal::types::*;
use beardog_errors::BearDogError;
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
    ) -> Result<Vec<UniversalCapability>> {
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
    ) -> Result<Vec<RankedCapabilityProvider>> {
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
    ) -> Result<Vec<RankedCapabilityProvider>> {
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
    ) -> Result<bool> {
        // Check if provider meets minimum security level
        let provider_security_level = provider.metadata
            .get("security_level")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);

        if provider_security_level < requirements.min_security_level {
            debug!(
                "Provider {} security level {} below requirement {}",
                provider.provider_id, provider_security_level, requirements.min_security_level
            );
            return Ok(false);
        }

        // Check encryption requirement
        if requirements.require_encryption {
            let supports_encryption = provider.metadata
                .get("supports_encryption")
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(false);
            
            if !supports_encryption {
                debug!(
                    "Provider {} does not support required encryption",
                    provider.provider_id
                );
                return Ok(false);
            }
        }

        // Check authentication requirement
        if requirements.require_authentication {
            let supports_auth = provider.metadata
                .get("supports_authentication")
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(false);
            
            if !supports_auth {
                debug!(
                    "Provider {} does not support required authentication",
                    provider.provider_id
                );
                return Ok(false);
            }
        }

        Ok(true)
    }


    fn meets_availability_requirements(
        provider: &UniversalCapability,
        requirements: &AvailabilityRequirements,
    ) -> Result<bool> {
        // Check uptime percentage
        let uptime = provider.metadata
            .get("uptime_percentage")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(99.9); // Default to high availability

        if uptime < requirements.min_uptime_percentage {
            debug!(
                "Provider {} uptime {}% below requirement {}%",
                provider.provider_id, uptime, requirements.min_uptime_percentage
            );
            return Ok(false);
        }

        // Check response time
        let response_time = provider.metadata
            .get("avg_response_time_ms")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(100); // Default to 100ms

        if response_time > requirements.max_response_time_ms {
            debug!(
                "Provider {} response time {}ms exceeds max {}ms",
                provider.provider_id, response_time, requirements.max_response_time_ms
            );
            return Ok(false);
        }

        Ok(true)
    }


    fn meets_performance_requirements(
        provider: &UniversalCapability,
        requirements: &PerformanceMetrics,
    ) -> Result<bool> {
        // Check throughput if specified
        if requirements.throughput > 0.0 {
            let provider_throughput = provider.metadata
                .get("throughput_ops_per_sec")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(1000.0);

            if provider_throughput < requirements.throughput {
                debug!(
                    "Provider {} throughput {} below requirement {}",
                    provider.provider_id, provider_throughput, requirements.throughput
                );
                return Ok(false);
            }
        }

        // Check error rate if specified
        if requirements.error_rate > 0.0 {
            let provider_error_rate = provider.metadata
                .get("error_rate_percentage")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.1);

            if provider_error_rate > requirements.error_rate {
                debug!(
                    "Provider {} error rate {}% exceeds max {}%",
                    provider.provider_id, provider_error_rate, requirements.error_rate
                );
                return Ok(false);
            }
        }

        // Check latency if specified
        if requirements.latency_ms > 0.0 {
            let provider_latency = provider.metadata
                .get("avg_latency_ms")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(50.0);

            if provider_latency > requirements.latency_ms {
                debug!(
                    "Provider {} latency {}ms exceeds max {}ms",
                    provider.provider_id, provider_latency, requirements.latency_ms
                );
                return Ok(false);
            }
        }

        Ok(true)
    }


    fn calculate_provider_rank(
        provider: &UniversalCapability,
        preferences: &Option<CapabilityPreferences>,
    ) -> Result<f64> {
        let mut rank = 0.0;

        // Base rank from provider health status (0.0-0.3)
        let health_score = match provider.metadata.get("health_status") {
            Some(status) if status == "healthy" => 0.3,
            Some(status) if status == "degraded" => 0.15,
            _ => 0.1,
        };
        rank += health_score;

        // Performance score (0.0-0.3)
        let response_time = provider.metadata
            .get("avg_response_time_ms")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(100.0);
        let perf_score = if response_time < 50.0 {
            0.3
        } else if response_time < 100.0 {
            0.2
        } else if response_time < 500.0 {
            0.1
        } else {
            0.05
        };
        rank += perf_score;

        // Availability score (0.0-0.2)
        let uptime = provider.metadata
            .get("uptime_percentage")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(99.0);
        let avail_score = if uptime >= 99.9 {
            0.2
        } else if uptime >= 99.0 {
            0.15
        } else if uptime >= 95.0 {
            0.1
        } else {
            0.05
        };
        rank += avail_score;

        // Apply preferences if provided
        if let Some(prefs) = preferences {
            // Preferred provider bonus (0.0-0.3)
            if prefs.preferred_providers.contains(&provider.provider_id) {
                rank += 0.3;
            }
            
            // Cost optimization scoring (0.0-0.2)
            if prefs.cost_optimization {
                let cost = provider.metadata
                    .get("cost_per_operation")
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or(1.0);
                let cost_score = if cost < 0.1 {
                    0.2
                } else if cost < 0.5 {
                    0.1
                } else if cost < 1.0 {
                    0.05
                } else {
                    0.0
                };
                rank += cost_score;
            }
        }

        // Ensure rank is between 0 and 1
        Ok(rank.min(1.0).max(0.0))
    }


    fn estimate_provider_performance(
        provider: &UniversalCapability,
    ) -> Result<PerformanceEstimate> {
        // Extract performance metrics from provider metadata
        let latency = provider.metadata
            .get("avg_latency_ms")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(50.0);

        let throughput = provider.metadata
            .get("throughput_ops_per_sec")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(1000.0);

        // Calculate confidence based on data availability
        let mut confidence = 0.5; // Base confidence

        // Increase confidence if we have actual metrics
        if provider.metadata.contains_key("avg_latency_ms") {
            confidence += 0.2;
        }
        if provider.metadata.contains_key("throughput_ops_per_sec") {
            confidence += 0.2;
        }
        if provider.metadata.contains_key("uptime_percentage") {
            confidence += 0.1;
        }

        Ok(PerformanceEstimate {
            estimated_latency_ms: latency,
            estimated_throughput: throughput,
            confidence_level: confidence.min(1.0),
        })
    }

    /// Validates provider_health
    /// 
    /// Returns true for non-empty provider IDs (safe default) - full health check in Phase 2
    fn validate_provider_health(provider_id: &str) -> Result<bool> {
        debug!("🏥 Validating health for provider: {}", provider_id);
        
        // Basic validation: ensure provider_id is not empty
        if provider_id.is_empty() {
            return Ok(false);
        }

        // PHASE-2(Health): Implement comprehensive provider health check
        // 
        // Implementation Requirements:
        // 1. Check if provider is registered and active in registry
        // 2. Perform lightweight health check (ping/status endpoint)
        // 3. Check recent error rates from metrics
        // 4. Verify network connectivity
        // 5. Check resource usage (CPU, memory, connections)
        // 6. Verify last heartbeat timestamp
        // 
        // Integration Points:
        // - beardog-monitoring: query HealthMetrics
        // - Provider registry: check registration status
        // - HTTP client: call /health endpoint if available
        // 
        // Return: true if healthy, false if degraded/unhealthy
        Ok(true)
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
    ) -> Result<CapabilityConnection> {
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
    pub fn update_connection_health(connection: &mut CapabilityConnection) -> Result<()> {
        connection.last_health_check = std::time::SystemTime::now();

        // Perform health check based on connection metrics
        let is_healthy = connection.metrics.error_count == 0 
            && connection.metrics.average_latency_ms < 1000.0
            && connection.metrics.responses_received > 0;

        connection.health_status = if is_healthy {
            HealthStatus::Healthy
        } else if connection.metrics.error_count < 5 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };

        debug!(
            "💓 Updated health for connection: {} - status: {:?}",
            connection.connection_id, connection.health_status
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
