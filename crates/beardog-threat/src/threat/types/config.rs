

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {

    pub real_time_detection: bool,

    pub threat_threshold: u8,

    pub automated_response: bool,

    pub max_alerts_per_minute: u32,

    pub ml_enhancement: bool,

    pub threat_feeds: Vec<String>,

    pub auto_quarantine: bool,

    pub notification_endpoints: Vec<String>,

    pub enabled: bool,

    pub rules_path: String,

    pub monitor_paths: Vec<String>,

    pub alert_threshold: f64,

    pub cache_size: usize,

    pub monitoring_interval: u64,
}
impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            real_time_detection: true,
            threat_threshold: 70,
            automated_response: false,
            max_alerts_per_minute: 100,
            ml_enhancement: true,
            threat_feeds: Vec::new(),
            auto_quarantine: false,
            notification_endpoints: Vec::new(),
            enabled: true,
            rules_path: String::new(),
            monitor_paths: Vec::new(),
            alert_threshold: 0.8,
            cache_size: 1000,
            monitoring_interval: 300,
        }
    }
}
impl ThreatDetectionConfig {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.threat_threshold <= 100
            && self.alert_threshold >= 0.0
            && self.alert_threshold <= 1.0
            && self.cache_size > 0
            && self.monitoring_interval > 0
    }

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
