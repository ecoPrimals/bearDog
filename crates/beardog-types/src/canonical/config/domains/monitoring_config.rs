//! # Monitoring Configuration Domain
//!
//! This module contains all monitoring-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// **CONSOLIDATED MONITORING CONFIGURATION** - Unifies all monitoring configs
///
/// Consolidates: `MetricsConfig`, `AnalysisConfig`, `HealthCheckConfig`, `AnomalyConfig`,
/// `TrendConfig`, `SecuritySentinelConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    
    /// Metrics collection configuration
    pub metrics: MetricsCollectionConfig,
    
    /// Analysis and processing configuration
    pub analysis: AnalysisProcessingConfig,
    
    /// Health monitoring configuration
    pub health: HealthMonitoringConfig,
    
    /// Anomaly detection configuration
    pub anomaly: AnomalyDetectionConfig,
    
    /// Trend analysis configuration
    pub trends: TrendAnalysisConfig,
    
    /// Security monitoring configuration
    pub security: SecurityMonitoringConfig,
    
    /// Export and integration configuration
    pub export: ExportIntegrationConfig,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricsCollectionConfig {
    /// Collection interval
    pub interval: Duration,
    
    /// Metrics to collect
    pub metrics: Vec<String>,
    
    /// Collection strategy
    pub strategy: String,
    
    /// Buffer size for metrics
    pub buffer_size: usize,
    
    /// Retention period
    pub retention_period: Duration,
}

/// Analysis and processing configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisProcessingConfig {
    /// Enable real-time analysis
    pub real_time: bool,
    
    /// Analysis algorithms
    pub algorithms: Vec<String>,
    
    /// Processing window size
    pub window_size: Duration,
    
    /// Statistical methods
    pub statistical_methods: Vec<String>,
    
    /// Machine learning models
    pub ml_models: Vec<String>,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    
    /// Health check interval
    pub check_interval: Duration,
    
    /// Health endpoints
    pub endpoints: Vec<String>,
    
    /// Health thresholds
    pub thresholds: HashMap<String, f64>,
    
    /// Recovery actions
    pub recovery_actions: HashMap<String, String>,
}

/// Anomaly detection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnomalyDetectionConfig {
    /// Enable anomaly detection
    pub enabled: bool,
    
    /// Detection algorithms
    pub algorithms: Vec<String>,
    
    /// Sensitivity level
    pub sensitivity: f64,
    
    /// Training window
    pub training_window: Duration,
    
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// Trend analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrendAnalysisConfig {
    /// Enable trend analysis
    pub enabled: bool,
    
    /// Analysis period
    pub analysis_period: Duration,
    
    /// Trend indicators
    pub indicators: Vec<String>,
    
    /// Forecasting models
    pub forecasting_models: Vec<String>,
    
    /// Prediction horizon
    pub prediction_horizon: Duration,
}

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityMonitoringConfig {
    /// Enable security monitoring
    pub enabled: bool,
    
    /// Security event types to monitor
    pub event_types: Vec<String>,
    
    /// Threat detection rules
    pub threat_rules: Vec<String>,
    
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    
    /// Response actions
    pub response_actions: HashMap<String, String>,
}

/// Export and integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportIntegrationConfig {
    /// Export destinations
    pub destinations: Vec<ExportDestination>,
    
    /// Export format
    pub format: String,
    
    /// Export interval
    pub interval: Duration,
    
    /// Compression settings
    pub compression: CompressionConfig,
}

/// Export destination configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportDestination {
    /// Destination type
    pub destination_type: String,
    
    /// Connection URL
    pub url: String,
    
    /// Authentication
    pub auth: HashMap<String, String>,
    
    /// Export filters
    pub filters: Vec<String>,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,
    
    /// Compression algorithm
    pub algorithm: String,
    
    /// Compression level
    pub level: u8,
}

/// Basic monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    
    /// Metrics collection interval
    pub metrics_interval: Duration,
    
    /// Alert configurations
    pub alerts: Vec<AlertConfig>,
    
    /// Dashboard configuration
    pub dashboard: DashboardConfig,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertConfig {
    /// Alert name
    pub name: String,
    
    /// Alert condition
    pub condition: String,
    
    /// Alert threshold
    pub threshold: f64,
    
    /// Alert severity
    pub severity: String,
    
    /// Notification channels
    pub channels: Vec<String>,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardConfig {
    /// Enable dashboard
    pub enabled: bool,
    
    /// Refresh interval
    pub refresh_interval: Duration,
    
    /// Metrics to display
    pub metrics: Vec<String>,
    
    /// Dashboard layout
    pub layout: String,
}

// Default implementations
impl Default for ConsolidatedMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: MetricsCollectionConfig::default(),
            analysis: AnalysisProcessingConfig::default(),
            health: HealthMonitoringConfig::default(),
            anomaly: AnomalyDetectionConfig::default(),
            trends: TrendAnalysisConfig::default(),
            security: SecurityMonitoringConfig::default(),
            export: ExportIntegrationConfig::default(),
        }
    }
}

impl Default for MetricsCollectionConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(60),
            metrics: vec!["cpu".to_string(), "memory".to_string()],
            strategy: "push".to_string(),
            buffer_size: 1000,
            retention_period: Duration::from_secs(86400 * 7), // 7 days
        }
    }
}

impl Default for AnalysisProcessingConfig {
    fn default() -> Self {
        Self {
            real_time: true,
            algorithms: vec!["moving_average".to_string(), "exponential_smoothing".to_string()],
            window_size: Duration::from_secs(300), // 5 minutes
            statistical_methods: vec!["mean".to_string(), "stddev".to_string()],
            ml_models: vec!["linear_regression".to_string()],
        }
    }
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            endpoints: vec!["/health".to_string(), "/ready".to_string()],
            thresholds: {
                let mut thresholds = HashMap::new();
                thresholds.insert("response_time_ms".to_string(), 1000.0);
                thresholds.insert("error_rate".to_string(), 0.05);
                thresholds
            },
            recovery_actions: HashMap::new(),
        }
    }
}

impl Default for AnomalyDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec!["isolation_forest".to_string(), "statistical_outlier".to_string()],
            sensitivity: 0.8,
            training_window: Duration::from_secs(86400), // 1 day
            alert_thresholds: {
                let mut thresholds = HashMap::new();
                thresholds.insert("anomaly_score".to_string(), 0.9);
                thresholds
            },
        }
    }
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_period: Duration::from_secs(86400), // 1 day
            indicators: vec!["cpu_trend".to_string(), "memory_trend".to_string()],
            forecasting_models: vec!["arima".to_string(), "linear_trend".to_string()],
            prediction_horizon: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            event_types: vec![
                "authentication_failure".to_string(),
                "unauthorized_access".to_string(),
                "suspicious_activity".to_string(),
            ],
            threat_rules: vec![
                "failed_login_attempts > 5".to_string(),
                "unusual_traffic_pattern".to_string(),
            ],
            alert_thresholds: {
                let mut thresholds = HashMap::new();
                thresholds.insert("threat_score".to_string(), 0.8);
                thresholds
            },
            response_actions: {
                let mut actions = HashMap::new();
                actions.insert("high_threat".to_string(), "block_ip".to_string());
                actions
            },
        }
    }
}

impl Default for ExportIntegrationConfig {
    fn default() -> Self {
        Self {
            destinations: vec![],
            format: "json".to_string(),
            interval: Duration::from_secs(300), // 5 minutes
            compression: CompressionConfig::default(),
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "gzip".to_string(),
            level: 6,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            alerts: vec![],
            dashboard: DashboardConfig::default(),
        }
    }
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            name: "default_alert".to_string(),
            condition: "cpu > 80%".to_string(),
            threshold: 80.0,
            severity: "warning".to_string(),
            channels: vec!["email".to_string()],
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh_interval: Duration::from_secs(30),
            metrics: vec!["cpu".to_string(), "memory".to_string()],
            layout: "grid".to_string(),
        }
    }
}

// BearDogConfig implementation for ConsolidatedMonitoringConfig
impl BearDogConfig for ConsolidatedMonitoringConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            if self.metrics.interval.as_secs() == 0 {
                return Err(BearDogError::validation("Metrics collection interval cannot be zero"));
            }
            
            if self.metrics.buffer_size == 0 {
                return Err(BearDogError::validation("Metrics buffer size cannot be zero"));
            }
            
            if self.health.enabled && self.health.check_interval.as_secs() == 0 {
                return Err(BearDogError::validation("Health check interval cannot be zero"));
            }
            
            if self.anomaly.enabled && (self.anomaly.sensitivity < 0.0 || self.anomaly.sensitivity > 1.0) {
                return Err(BearDogError::validation("Anomaly detection sensitivity must be between 0.0 and 1.0"));
            }
        }
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        Ok(Self {
            enabled: other.enabled,
            metrics: if other.enabled { 
                other.metrics.clone() 
            } else { 
                self.metrics.clone() 
            },
            analysis: if other.enabled { 
                other.analysis.clone() 
            } else { 
                self.analysis.clone() 
            },
            health: if other.health.enabled { 
                other.health.clone() 
            } else { 
                self.health.clone() 
            },
            anomaly: if other.anomaly.enabled { 
                other.anomaly.clone() 
            } else { 
                self.anomaly.clone() 
            },
            trends: if other.trends.enabled { 
                other.trends.clone() 
            } else { 
                self.trends.clone() 
            },
            security: if other.security.enabled { 
                other.security.clone() 
            } else { 
                self.security.clone() 
            },
            export: other.export.clone(),
        })
    }
    
    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();
        
        if let Ok(enabled) = std::env::var("BEARDOG_MONITORING_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(true);
        }
        
        if let Ok(interval) = std::env::var("BEARDOG_MONITORING_INTERVAL") {
            if let Ok(secs) = interval.parse::<u64>() {
                config.metrics.interval = Duration::from_secs(secs);
            }
        }
        
        if let Ok(buffer_size) = std::env::var("BEARDOG_MONITORING_BUFFER_SIZE") {
            config.metrics.buffer_size = buffer_size.parse().unwrap_or(1000);
        }
        
        config.validate()?;
        Ok(config)
    }
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize monitoring config to TOML: {e}")))
    }
    
    fn domain() -> &'static str {
        "monitoring"
    }
} 