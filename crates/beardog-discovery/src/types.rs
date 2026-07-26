// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core types for service discovery

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// A discovered service that provides one or more capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    /// Unique service ID
    pub id: String,

    /// Service type label from discovery (opaque; do not branch runtime logic on fixed names)
    pub service_type: String,

    /// Display name
    pub display_name: String,

    /// Service endpoint
    pub endpoint: ServiceEndpoint,

    /// Capabilities this service provides
    pub capabilities: Vec<Capability>,

    /// `QoS` metrics
    pub qos: QoSMetrics,

    /// Health status
    pub health: HealthStatus,

    /// When this service was discovered
    pub discovered_at: SystemTime,

    /// TTL for this discovery entry
    pub ttl_secs: u64,

    /// Discovery method used
    pub discovery_method: String,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Primary URL (e.g., `<https://orchestrator.internal:8080>`)
    pub primary_url: String,

    /// Fallback URLs
    pub fallback_urls: Vec<String>,

    /// Whether TLS is required
    pub use_tls: bool,

    /// Optional path prefix
    pub path_prefix: Option<String>,
}

impl ServiceEndpoint {
    /// Get the full URL with path
    #[must_use]
    pub fn full_url(&self, path: &str) -> String {
        let base = &self.primary_url;
        let prefix = self.path_prefix.as_deref().unwrap_or("");
        format!("{base}{prefix}{path}")
    }
}

/// A capability that a service provides
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capability {
    /// Capability type (e.g., "orchestration", "security", "compute")
    pub capability_type: String,

    /// Version
    pub version: String,

    /// Features provided
    pub features: Vec<String>,

    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Quality of Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSMetrics {
    /// Latency in milliseconds
    pub latency_ms: f64,

    /// Throughput (operations per second)
    pub throughput_ops_sec: f64,

    /// Availability (0.0 to 1.0)
    pub availability: f64,

    /// Reliability (0.0 to 1.0)
    pub reliability: f64,

    /// Last updated
    pub updated_at: SystemTime,
}

impl Default for QoSMetrics {
    fn default() -> Self {
        Self {
            latency_ms: 0.0,
            throughput_ops_sec: 0.0,
            availability: 1.0,
            reliability: 1.0,
            updated_at: SystemTime::now(),
        }
    }
}

impl QoSMetrics {
    /// Calculate weighted score based on weights
    #[must_use]
    pub fn calculate_score(&self, weights: &QoSWeights) -> f64 {
        // Normalize latency (lower is better, 0-100ms range)
        let latency_score = (100.0 - self.latency_ms.min(100.0)) / 100.0;

        // Normalize throughput (higher is better, 0-10000 ops/sec range)
        let throughput_score = (self.throughput_ops_sec.min(10000.0)) / 10000.0;

        // Availability and reliability already 0.0-1.0

        weights.reliability.mul_add(
            self.reliability,
            weights.availability.mul_add(
                self.availability,
                weights
                    .latency
                    .mul_add(latency_score, weights.throughput * throughput_score),
            ),
        )
    }
}

/// Relative weights used when ranking discovered services by [`QoSMetrics::calculate_score`].
///
/// Weights should reflect policy (e.g. favor low latency vs high availability). They are not
/// required to sum to `1.0`; the score is a weighted blend of normalized metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSWeights {
    /// Weight for latency score (lower observed latency yields a higher contribution).
    pub latency: f64,
    /// Weight for throughput score (higher ops/sec yields a higher contribution).
    pub throughput: f64,
    /// Weight for availability (0.0–1.0 scale).
    pub availability: f64,
    /// Weight for reliability (0.0–1.0 scale).
    pub reliability: f64,
}

impl Default for QoSWeights {
    fn default() -> Self {
        Self {
            latency: 0.4,
            throughput: 0.2,
            availability: 0.3,
            reliability: 0.1,
        }
    }
}

/// Health status of a service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but operational
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Health status unknown
    Unknown,
}

/// Information about this primal (self-knowledge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    /// Primal ID (e.g., "beardog")
    pub primal_id: String,

    /// Primal type (e.g., "security")
    pub primal_type: String,

    /// Version
    pub version: String,

    /// Display name
    pub display_name: String,

    /// Capabilities this primal provides
    pub capabilities: Vec<Capability>,

    /// Self endpoint
    pub endpoint: ServiceEndpoint,
}

/// Required capability specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredCapability {
    /// Capability type needed
    pub capability_type: String,

    /// Is this capability required or optional?
    pub required: bool,

    /// Minimum version required
    pub min_version: Option<String>,

    /// Required features
    pub features: Vec<String>,

    /// Fallback strategy if not available
    pub fallback: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_score_calculation() {
        let metrics = QoSMetrics {
            latency_ms: 10.0,
            throughput_ops_sec: 5000.0,
            availability: 0.99,
            reliability: 0.95,
            updated_at: SystemTime::now(),
        };

        let weights = QoSWeights::default();
        let score = metrics.calculate_score(&weights);

        // Score should be between 0.0 and 1.0
        assert!((0.0..=1.0).contains(&score));
        // With good metrics, score should be high
        assert!(score > 0.8);
    }

    #[test]
    fn test_endpoint_full_url() {
        let endpoint = ServiceEndpoint {
            primary_url: "https://example.com:8080".to_string(),
            fallback_urls: vec![],
            use_tls: true,
            path_prefix: Some("/api/v1".to_string()),
        };

        assert_eq!(
            endpoint.full_url("/health"),
            "https://example.com:8080/api/v1/health"
        );
    }
}
