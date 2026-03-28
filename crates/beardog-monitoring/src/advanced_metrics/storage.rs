// SPDX-License-Identifier: AGPL-3.0-only

// Metrics Storage System

use super::types::{
    CustomMetric, EcosystemMetric, MetricsMetadata, PerformanceMetric, SecurityEventType,
    SecurityMetric,
};
use std::collections::HashMap;

/// Comprehensive metrics storage
#[derive(Debug, Default)]
pub struct MetricsStore {
    /// Latest snapshot per named performance metric (counter, gauge, histogram, etc.).
    pub performance: HashMap<String, PerformanceMetric>,
    /// Security event metrics
    /// Mapping of security
    pub security: HashMap<String, SecurityMetric>,
    /// Ecosystem interaction metrics
    /// Mapping of ecosystem
    pub ecosystem: HashMap<String, EcosystemMetric>,
    /// Custom application metrics
    /// Mapping of custom
    pub custom: HashMap<String, CustomMetric>,
    /// Metrics metadata
    /// The metadata value
    pub metadata: MetricsMetadata,
}

impl MetricsStore {
    /// Create a new metrics store
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the stored [`PerformanceMetric`] for `name`, if any.
    #[must_use]
    pub fn get_performance_metric(&self, name: &str) -> Option<&PerformanceMetric> {
        self.performance.get(name)
    }

    /// Inserts or replaces a performance metric and bumps aggregate metadata counts.
    pub fn store_performance_metric(&mut self, metric: PerformanceMetric) {
        self.performance.insert(metric.name.clone(), metric);
        self.metadata.total_metrics += 1;
    }

    /// Get security metric by event type
    /// Gets `security_metric`
    #[must_use]
    pub fn get_security_metric(&self, event_type: &SecurityEventType) -> Option<&SecurityMetric> {
        self.security.get(&format!("{event_type:?}"))
    }

    /// Store security metric
    pub fn store_security_metric(&mut self, key: String, metric: SecurityMetric) {
        self.security.insert(key, metric);
        self.metadata.total_metrics += 1;
    }

    /// Get ecosystem metric by service
    /// Gets `ecosystem_metric`
    #[must_use]
    pub fn get_ecosystem_metric(&self, service: &str) -> Option<&EcosystemMetric> {
        self.ecosystem.get(service)
    }

    /// Store ecosystem metric
    pub fn store_ecosystem_metric(&mut self, metric: EcosystemMetric) {
        self.ecosystem.insert(metric.service.clone(), metric);
        self.metadata.total_metrics += 1;
    }

    /// Get custom metric by name
    /// Gets `custom_metric`
    #[must_use]
    pub fn get_custom_metric(&self, name: &str) -> Option<&CustomMetric> {
        self.custom.get(name)
    }

    /// Store custom metric
    pub fn store_custom_metric(&mut self, metric: CustomMetric) {
        self.custom.insert(metric.name.clone(), metric);
        self.metadata.total_metrics += 1;
    }

    /// Get total metrics count
    #[must_use]
    pub const fn total_metrics(&self) -> u64 {
        self.metadata.total_metrics
    }
}
