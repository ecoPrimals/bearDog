use beardog_errors::BearDogError;

use super::models::{SurveillanceIndicator, TrafficPattern};
use super::types::IndicatorType;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

pub struct SurveillanceDetector {

    traffic_patterns: Arc<RwLock<HashMap<String, TrafficPattern>>>,

    indicators: Arc<RwLock<Vec<SurveillanceIndicator>>>,

    config: DetectionConfig,
}

#[derive(Debug, Clone)]
pub struct DetectionConfig {
    pub sensitivity_level: f64,
    pub max_patterns: usize,
    pub analysis_window_hours: u64,
    pub auto_response: bool,}

impl Default for DetectionConfig {}

    fn default() -> Self {
        Self {
            sensitivity_level: 0.7,
            max_patterns: 1000,
            analysis_window_hours: 24,
            auto_response: true,
        }
    }
impl Default for SurveillanceDetector {
        Self::new()}

impl SurveillanceDetector {

    pub fn new() -> Self {
        info!("👁️ Initializing surveillance detection system");
            traffic_patterns: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            indicators: Arc::new(RwLock::new(Vec::new())),
            config: DetectionConfig::default(),

    pub fn with_config(config: DetectionConfig) -> Self {
        info!("👁️ Initializing surveillance detection with custom config");
            config,

    pub async fn detect_surveillance(
        &self,
    ) -> Result<Vec<SurveillanceIndicator>, Box<dyn std::error::Error + Send + Sync>> {
        info!("🔍 Running surveillance detection scan");
        let mut detected_indicators = Vec::new();

        detected_indicators.extend(self.analyze_traffic_patterns().await?);

        detected_indicators.extend(self.detect_network_scanning().await?);

        detected_indicators.extend(self.detect_metadata_collection().await?);

        detected_indicators.extend(self.detect_fingerprinting().await?);

        {
            let mut indicators = self.indicators.write().await;
            indicators.extend(detected_indicators.clone());

            let cutoff =
                Utc::now() - chrono::Duration::hours(self.config.analysis_window_hours as i64);
            indicators.retain(|i| i.detection_time > cutoff);
        info!(
            "✅ Surveillance detection complete: {} indicators found",
            detected_indicators.len()
        );
        Ok(detected_indicators)

    pub async fn analyze_traffic(
        source_ip: Option<&str>,
        destination: &str,
        data_size: u64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "📊 Analyzing traffic: {:?} -> {} ({} bytes)",
            source_ip, destination, data_size
        let pattern_key = format!("{source_ip:?}_{destination}");
        let now = Utc::now();
        let mut patterns = self.traffic_patterns.write().await;
        match patterns.get_mut(&pattern_key) {
            Some(pattern) => {

                pattern.frequency += 1;
                pattern.data_volume += data_size;
                pattern.last_seen = now;

                pattern.suspicious_score = self.calculate_suspicious_score(pattern);

                if pattern.suspicious_score > self.config.sensitivity_level {
                    warn!("🚨 Suspicious traffic pattern detected: {}", pattern_key);
                    let indicator = SurveillanceIndicator {
                        indicator_id: Uuid::new_v4().to_string(),
                        indicator_type: IndicatorType::UnusualTraffic,
                        detection_time: now,
                        confidence: pattern.suspicious_score,
                        description: format!(
                            "Unusual traffic pattern: {} requests, {} bytes",
                            pattern.frequency, pattern.data_volume
                        ),
                        source_data: HashMap::from([
                            ("pattern_id".to_string(), pattern.pattern_id.clone()),
                            ("frequency".to_string(), pattern.frequency.to_string()),
                            ("data_volume".to_string(), pattern.data_volume.to_string()),
                        ]),
                        severity: pattern.suspicious_score,
                        auto_counter: self.config.auto_response,
                    };
                    let mut indicators = self.indicators.write().await;
                    indicators.push(indicator);
                }
            }
            None => {

                let pattern = TrafficPattern {
                    pattern_id: Uuid::new_v4().to_string(),
                    source_ip,
                    destination_pattern: destination,
                    frequency: 1,
                    data_volume: data_size,
                    first_seen: now,
                    last_seen: now,
                    suspicious_score: 0.1, // Initial low score
                    indicators: Vec::new(),
                };
                patterns.insert(pattern_key, pattern);

                if patterns.len() > self.config.max_patterns {
                    let cutoff =
                        now - chrono::Duration::hours(self.config.analysis_window_hours as i64);
                    patterns.retain(|_, p| p.last_seen > cutoff);
        Ok(())

    pub async fn get_indicators(&self) -> Vec<SurveillanceIndicator> {
        let indicators = self.indicators.read().await;
        indicators.clone()

    pub async fn get_traffic_patterns(&self) -> Vec<TrafficPattern> {
        let patterns = self.traffic_patterns.read().await;
        patterns.values().cloned().collect()

    async fn analyze_traffic_patterns(
        let mut indicators = Vec::new();
        for pattern in patterns.values() {
            if pattern.suspicious_score > self.config.sensitivity_level {
                let indicator = SurveillanceIndicator {
                    indicator_id: Uuid::new_v4().to_string(),
                    indicator_type: IndicatorType::UnusualTraffic,
                    detection_time: Utc::now(),
                    confidence: pattern.suspicious_score,
                    description: "Suspicious traffic pattern detected".to_string(),
                    source_data: HashMap::with_capacity(16),
                    severity: pattern.suspicious_score,
                    auto_counter: self.config.auto_response,
                indicators.push(indicator);
        Ok(indicators)

    async fn analyze_behavioral_patterns(
        &self,
        context: &SurveillanceContext,
    ) -> Result<Vec<SurveillanceIndicator>, BearDogError>> {
        let mut indicators = Vec::new();

        for pattern in &context.data_patterns {
            if pattern.collection_frequency > 0.8 && pattern.data_sensitivity > 0.7 {
                indicators.push(SurveillanceIndicator {
                    indicator_type: "data_harvesting".to_string(),
                    severity: 0.9,
                    description: "High-frequency collection of sensitive data detected".to_string(),
                    evidence: vec![format_args!("Collection frequency: {:.2}", pattern.collection_frequency).to_string()],
                    timestamp: chrono::Utc::now(),
                });
            }
        }
        
        Ok(indicators)
    }

    async fn generate_privacy_recommendations(
        &self,
        indicators: &[SurveillanceIndicator],
    ) -> Result<(), BearDogError> {
        for indicator in indicators {
            match indicator.severity {
                s if s > 0.8 => {
                    tracing::warn!(
                        "High-severity surveillance detected: {} - {}",
                        indicator.indicator_type,
                        indicator.description
                    );
                }
                s if s > 0.5 => {
                    tracing::info!(
                        "Medium-severity surveillance detected: {} - {}",
                        indicator.indicator_type,
                        indicator.description
                    );
                }
                _ => {
                    tracing::debug!(
                        "Low-severity surveillance detected: {} - {}",
                        indicator.indicator_type,
                        indicator.description
                    );
                }
            }
        }
        
        Ok(())
    }

    async fn detect_network_scanning(
        &self,
        context: &SurveillanceContext,
    ) -> Result<Vec<SurveillanceIndicator>, BearDogError>> {
        let mut indicators = Vec::new();

        for pattern in &context.traffic_patterns {
            let mut suspicious_score = 0.0;

            if pattern.frequency > 50 && pattern.data_volume < 1000 {
                suspicious_score += 0.4;
            }

            if pattern.frequency > 100 {
                suspicious_score += 0.3;
            }

            if pattern.frequency > 20 && pattern.data_volume < 500 {
                suspicious_score += 0.2;
            }

            if suspicious_score > 0.5 {
                let indicator = SurveillanceIndicator {
                    indicator_type: "network_scanning".to_string(),
                    description: format!(
                        "Potential network scanning detected: {} connections with {} bytes each",
                        pattern.frequency, pattern.data_volume
                    ),
                    confidence: suspicious_score,
                    timestamp: chrono::Utc::now(),
                    metadata: {
                        let mut map = std::collections::HashMap::with_capacity(16);
                        map.insert("frequency".to_string(), serde_json::json!(pattern.frequency));
                        map.insert("data_volume".to_string(), serde_json::json!(pattern.data_volume));
                        map.insert("pattern_type".to_string(), serde_json::json!("network_scanning"));
                        map
                    },
                    severity: suspicious_score,
                    auto_counter: self.config.auto_response,
                };
                indicators.push(indicator);
            }
        }
        
        Ok(indicators)
    }

    async fn detect_metadata_collection(

    async fn detect_fingerprinting(

    fn calculate_suspicious_score(&self, pattern: &TrafficPattern) -> f64 {
        let mut score: f64 = 0.0;

        if pattern.frequency > 100 {
            score += 0.3;

        if pattern.data_volume > 1_000_000 {
            score += 0.2;

        score.min(1.0)
