// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Configuration types for threat detection
///
/// This module contains configuration structures and settings for the threat detection engine.
/// ## Features
/// - Threat detection configuration
/// - Real-time detection settings
/// - Automated response configuration
/// - Monitoring and alerting settings
/// - Machine learning integration settings
/// - Threat intelligence feed configuration
/// ## Example
/// ```rust
/// use beardog::threat::types::ThreatDetectionConfig;
/// let config = ThreatDetectionConfig {
///     real_time_detection: true,
///     threat_threshold: 70,
///     automated_response: true,
///     ml_enhancement: true,
///     auto_quarantine: true,
///     ..Default::default()
/// };
/// ```
use serde::{Deserialize, Serialize};

/// Configuration for threat detection engine
/// This structure defines all configuration options for the threat detection system,
/// including detection thresholds, automated responses, and integration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable real-time threat detection
    ///
    /// When enabled, the system continuously monitors for threats in real-time.
    /// Disabling this will switch to batch processing mode.
    pub real_time_detection: bool,
    /// Threat scoring threshold (0-100)
    /// Minimum threat score required to trigger an alert.
    /// Higher values reduce false positives but may miss lower-severity threats.
    pub threat_threshold: u8,
    /// Enable automated response
    /// When enabled, the system automatically responds to detected threats
    /// based on configured response actions.
    pub automated_response: bool,
    /// Maximum alerts per minute
    /// Rate limiting for alert generation to prevent alert flooding.
    /// Alerts exceeding this rate will be batched or suppressed.
    pub max_alerts_per_minute: u32,
    /// Enable machine learning enhancement
    /// When enabled, the system uses ML models to enhance threat detection
    /// accuracy and reduce false positives.
    pub ml_enhancement: bool,
    /// Threat intelligence feeds
    /// List of threat intelligence feed URLs to integrate with the system.
    /// These feeds provide indicators of compromise and threat actor information.
    pub threat_feeds: Vec<String>,
    /// Quarantine suspicious activity
    /// When enabled, suspicious activities are automatically quarantined
    /// pending further investigation.
    pub auto_quarantine: bool,
    /// Alert notification endpoints
    /// List of notification endpoints (URLs, email addresses, etc.) to send
    /// alerts to when threats are detected.
    pub notification_endpoints: Vec<String>,
    /// Enable threat detection
    /// Master switch for the entire threat detection system.
    /// When disabled, no threat detection activities will occur.
    pub enabled: bool,
    /// Path to detection rules
    /// File system path to the detection rules configuration file.
    /// Rules define how threats are identified and classified.
    pub rules_path: String,
    /// Paths to monitor for threats
    /// List of file system paths, network interfaces, or data sources
    /// to monitor for suspicious activity.
    pub monitor_paths: Vec<String>,
    /// Alert threshold for triggering actions
    /// Floating-point threshold (0.0 to 1.0) for triggering automated
    /// response actions. Higher values require more confidence.
    pub alert_threshold: f64,
    /// Cache size for threat data
    /// Maximum number of threat events to keep in memory cache
    /// for quick access and correlation.
    pub cache_size: usize,
    /// Monitoring interval in seconds
    /// How frequently the system performs monitoring checks.
    /// Lower values provide more responsive detection but use more resources.
    pub monitoring_interval: u64,
}
impl Default for ThreatDetectionConfig {}


    fn default() -> Self {
        Self {
            real_time_detection: true,
            threat_threshold: 70,
            automated_response: false,
            max_alerts_per_minute: 100,
            ml_enhancement: true,
            threat_feeds: vec![],
            auto_quarantine: false,
            notification_endpoints: vec![],
            enabled: true,
            rules_path: String::new(),
            monitor_paths: vec![],
            alert_threshold: 0.8,
            cache_size: 1000,
            monitoring_interval: 300,
        }
    }
impl ThreatDetectionConfig {
    /// Create a new threat detection configuration with default values
    /// # Returns
    /// A new `ThreatDetectionConfig` instance with sensible default values
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionConfig;
    /// let config = ThreatDetectionConfig::new();
    /// assert_eq!(config.threat_threshold, 70);
    /// assert!(config.real_time_detection);
    /// ```}


    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the configuration is valid
    /// Validates that all configuration parameters are within acceptable ranges
    /// and that required fields are properly set.
    /// 
    /// Returns `true` if the configuration is valid, `false` otherwise.
    /// 
    /// # Examples
    /// ```
    /// let config = ThreatDetectionConfig::default();
    /// assert!(config.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.threat_threshold <= 100
            && self.alert_threshold >= 0.0
            && self.alert_threshold <= 1.0
            && self.cache_size > 0
            && self.monitoring_interval > 0
    }

    /// Enable high-security mode
    /// Configures the system for maximum security with aggressive detection
    /// and automated response capabilities.
    /// let mut config = ThreatDetectionConfig::default();
    /// config.enable_high_security_mode();
    /// assert_eq!(config.threat_threshold, 50);
    /// assert!(config.auto_quarantine);
    pub fn enable_high_security_mode(&mut self) {
        self.threat_threshold = 50;
        self.automated_response = true;
        self.auto_quarantine = true;
        self.alert_threshold = 0.6;
        self.real_time_detection = true;
        self.ml_enhancement = true;
    }

    /// Enable performance mode
    /// Configures the system for better performance with reduced detection
    /// sensitivity and longer monitoring intervals.
    /// 
    /// # Examples
    /// ```
    /// let mut config = ThreatDetectionConfig::default();
    /// config.enable_performance_mode();
    /// assert_eq!(config.threat_threshold, 85);
    /// assert_eq!(config.monitoring_interval, 600);
    /// ```
    pub fn enable_performance_mode(&mut self) {
        self.threat_threshold = 85;
        self.alert_threshold = 0.9;
        self.monitoring_interval = 600;
        self.cache_size = 500;
        self.max_alerts_per_minute = 50;
    }
}
