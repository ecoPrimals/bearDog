// Modular Advanced Metrics System
//
// This module provides a comprehensive, modular metrics collection and analysis system
// that replaces the large monolithic advanced_metrics.rs file with focused, maintainable components.
//
// ## Architecture
//
// The metrics system is organized into logical domains:
// - **Core**: Main metrics system and storage
// - **Performance**: Performance analysis and monitoring
// - **Security**: Security event tracking and analysis
// - **Ecosystem**: Cross-system interaction metrics
// - **Analytics**: Statistical analysis and reporting
// - **Export**: Metrics export and integration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

// Domain-specific metrics modules
pub mod analytics;
/// Core functionality
/// Core functionality
pub mod core;
pub mod ecosystem;
pub mod export;
pub mod performance;
pub mod security;

pub use core::{MetricsCore, MetricsCoreConfig};

// Re-export key types
pub use analytics::*;
// Removed unused import: pub use core::*;
pub use ecosystem::*;
pub use export::*;
pub use performance::*;
pub use security::*;

/// **UNIFIED ADVANCED METRICS SYSTEM**
///
/// all metrics collection, analysis, and reporting functionality.
#[derive(Debug)]
pub struct UnifiedMetricsSystem {
    /// Core metrics storage and management
    core: Arc<MetricsCore>,

    performance: Arc<PerformanceEngine>,

    /// Security metrics collector
    security: Arc<SecurityMetricsEngine>,

    /// Ecosystem interaction monitor
    ecosystem: Arc<EcosystemMonitor>,

    /// Analytics and reporting engine
    analytics: Arc<AnalyticsEngine>,

    /// Export and integration system
    export: Arc<ExportEngine>,

    /// Real-time event broadcaster
    broadcaster: broadcast::Sender<MetricEvent>,

    /// System configuration
    #[allow(dead_code)] // Config reserved for future metrics features
    config: UnifiedMetricsConfig,
}

impl UnifiedMetricsSystem {
    /// Create a new unified metrics system
    ///
    /// # Errors
    /// Returns an error if the unified metrics system cannot be initialized
    /// Creates a new instance
    pub fn new(config: UnifiedMetricsConfig) -> Result<Self, BearDogError> {
        let (broadcaster, _) = broadcast::channel(config.broadcast_buffer_size);

        let core = Arc::new(MetricsCore::new(config.core.clone())?);
        let performance = Arc::new(PerformanceEngine::new(config.performance.clone())?);
        let security = Arc::new(SecurityMetricsEngine::new(config.security.clone())?);
        let ecosystem = Arc::new(EcosystemMonitor::new(config.ecosystem.clone())?);
        let analytics = Arc::new(AnalyticsEngine::new(config.analytics.clone())?);
        let export = Arc::new(ExportEngine::new(config.export.clone())?);

        Ok(Self {
            core,
            performance,
            security,
            ecosystem,
            analytics,
            export,
            broadcaster,
            config,
        })
    }

    /// Start the unified metrics system
    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        // Start all subsystems
        self.core.start()?;
        self.performance.start()?;
        self.security.start()?;
        self.ecosystem.start()?;
        self.analytics.start()?;
        self.export.start()?;

        tracing::info!("Unified metrics system started successfully");
        Ok(())
    }

    /// Record a metric event
    pub fn record_event(&self, event: MetricEvent) -> Result<(), BearDogError> {
        // Route event to appropriate subsystem
        match &event.category {
            MetricCategory::Performance => self.performance.record_event(&event)?,
            MetricCategory::Security => self.security.record_event(&event)?,
            MetricCategory::Ecosystem => self.ecosystem.record_event(&event)?,
            MetricCategory::Custom => {
                // Custom metrics are handled by the core system
                tracing::debug!("Recording custom metric: {}", event.name);
            }
        }

        // Broadcast event for real-time monitoring
        if self.broadcaster.send(event).is_err() {
            tracing::warn!("Failed to broadcast metric event - no active subscribers");
        }

        Ok(())
    }

    /// Get comprehensive system metrics
    ///
    /// # Errors
    /// Returns an error if system metrics cannot be collected
    /// Gets system_metrics
    /// Gets system_metrics
    pub fn get_system_metrics(&self) -> Result<SystemMetrics, BearDogError> {
        let performance_metrics = self.performance.get_metrics()?;
        let security_metrics = self.security.get_metrics()?;
        let ecosystem_metrics = self.ecosystem.get_metrics()?;
        let analytics_summary = self.analytics.get_summary()?;

        Ok(SystemMetrics {
            performance: performance_metrics,
            security: security_metrics,
            ecosystem: ecosystem_metrics,
            analytics: analytics_summary,
            timestamp: std::time::SystemTime::now(),
        })
    }
}

/// Unified metrics configuration
#[derive(Debug, Clone)]
pub struct UnifiedMetricsConfig {
    /// Core system configuration
    /// The core value
    pub core: MetricsCoreConfig,

    pub performance: PerformanceConfig,

    /// Security metrics configuration
    /// The security value
    pub security: SecurityMetricsConfig,

    /// Ecosystem monitoring configuration
    /// The ecosystem value
    pub ecosystem: EcosystemConfig,

    /// Analytics configuration
    /// The analytics value
    pub analytics: AnalyticsConfig,

    /// Export configuration
    /// The export value
    pub export: ExportConfig,

    /// Broadcast buffer size
    /// Number of broadcast_buffer_size
    pub broadcast_buffer_size: usize,
}

impl Default for UnifiedMetricsConfig {
    fn default() -> Self {
        Self {
            core: MetricsCoreConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityMetricsConfig::default(),
            ecosystem: EcosystemConfig::default(),
            analytics: AnalyticsConfig::default(),
            export: ExportConfig::default(),
            broadcast_buffer_size: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricEvent {
    /// Event category
    /// The category value
    pub category: MetricCategory,

    /// Event name
    /// Name of the item
    pub name: String,

    /// Event value
    /// The value value
    pub value: MetricValue,

    /// Event labels/tags
    /// The labels value
    pub labels: std::collections::HashMap<String, String>,

    /// Event timestamp
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub enum MetricCategory {
    Performance,
    /// Represents security variant
    Security,
    /// Represents ecosystem variant
    Ecosystem,
    /// Represents custom variant
    Custom,
}

/// Metric value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Represents counter variant
    Counter(u64),
    /// Represents gauge variant
    Gauge(f64),
    /// Represents histogram variant
    Histogram(Vec<f64>),
    /// Represents summary variant
    Summary {
        sum: f64,
        count: u64,
    },
}

/// Comprehensive system metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub performance: PerformanceMetrics,
    /// The security value
    pub security: SecurityMetrics,
    /// The ecosystem value
    pub ecosystem: EcosystemMetrics,
    /// The analytics value
    pub analytics: AnalyticsSummary,
    pub timestamp: std::time::SystemTime,
}
