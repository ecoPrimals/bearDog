// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// Number of threat_threshold
    pub threat_threshold: u8,

    /// Whether automated_response is enabled
    pub automated_response: bool,

    /// Number of max_alerts_per_minute
    pub max_alerts_per_minute: u32,

    /// Whether ml_enhancement is enabled
    pub ml_enhancement: bool,

    /// Collection of threat feeds
    pub threat_feeds: Vec<String>,

    /// Whether auto_quarantine is enabled
    pub auto_quarantine: bool,

    /// Collection of notification endpoints
    pub notification_endpoints: Vec<String>,

    /// Whether feature is enabled
    pub enabled: bool,

    /// The rules path value
    pub rules_path: String,

    /// Collection of monitor paths
    pub monitor_paths: Vec<String>,

    /// The alert threshold value
    pub alert_threshold: f64,

    /// Number of cache_size
    pub cache_size: usize,

    /// Number of monitoring_interval
    pub monitoring_interval: u64,
}
impl Default for ThreatDetectionConfig {
    fn default(true,
            threat_threshold: 70,
            automated_response: false,
            max_alerts_per_minute: 100,
            ml_enhancement: true,
            threat_feeds: Vec::new(false,
            notification_endpoints: Vec::new(true,
            rules_path: String::with_capacity(64),
            monitor_paths: Vec::new(0.8,
            cache_size: beardog_errors::process_env::var("BEARDOG_THREAT_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000), // 1000 entries default
            monitoring_interval: beardog_errors::process_env::var("BEARDOG_THREAT_MONITORING_INTERVAL")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(300), // 5 minutes default
        }
    }
}
impl ThreatDetectionConfig {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Is Valid operation.
    /// Checks if valid
    pub fn is_valid(&self) -> bool {
        self.threat_threshold <= 100
            && self.alert_threshold >= 0.0
            && self.alert_threshold <= 1.0
            && self.cache_size > 0
            && self.monitoring_interval > 0
    }

    /// Enable High Security Mode operation.
    pub fn enable_high_security_mode(&mut self) {
        self.threat_threshold = 50;
        self.automated_response = true;
        self.auto_quarantine = true;
        self.alert_threshold = 0.6;
        self.real_time_detection = true;
        self.ml_enhancement = true;
    }

    pub fn enable_performance_mode(&mut self) {
        self.threat_threshold = 85;
        self.alert_threshold = 0.9;
        self.monitoring_interval = 600;
        self.cache_size = 500;
        self.max_alerts_per_minute = 50;
    }
}
