

use crate::threat::types::analysis::events::SecurityEvent;
use crate::threat::types::engine::ml_models::*;
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Simple trait for universal compute adapter - dyn compatible
pub trait UniversalComputeAdapter: Send + Sync {
    /// Request compute from network (e.g., toadstool) - returns boxed future for dyn compatibility
    fn request_compute(&self, service: &str, request: &serde_json::Value) -> Pin<Box<dyn Future<Output = BearDogResult<serde_json::Value>> + Send + '_>>;
}

/// Lightweight ML prediction with smart capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlPrediction {
    pub threat_score: f64,
    pub confidence: f64,
    pub risk_level: RiskLevel,
    pub reasoning: Vec<String>,
    pub model_version: String,
    pub processing_time_ms: u64,
}

/// Smart risk assessment levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

/// Lightweight ML engine with smart threat detection
pub struct SmartThreatMLEngine {
    /// Local lightweight models for fast inference
    local_models: HashMap<String, MlModel>,
    /// Universal adapter for heavy compute (toadstool network) - trait object for flexibility
    universal_adapter: Option<Box<dyn UniversalComputeAdapter>>,
    /// Smart caching for performance
    prediction_cache: HashMap<String, MlPrediction>,
    /// Performance metrics
    local_predictions: u64,
    network_predictions: u64,
}

impl std::fmt::Debug for SmartThreatMLEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmartThreatMLEngine")
            .field("local_models", &self.local_models)
            .field("universal_adapter", &self.universal_adapter.is_some())
            .field("prediction_cache_size", &self.prediction_cache.len())
            .field("local_predictions", &self.local_predictions)
            .field("network_predictions", &self.network_predictions)
            .finish()
    }
}

impl SmartThreatMLEngine {
    pub fn new() -> Self {
        Self {
            local_models: HashMap::new(),
            universal_adapter: None,
            prediction_cache: HashMap::new(),
            local_predictions: 0,
            network_predictions: 0,
        }
    }

    /// Initialize with universal adapter for network compute
    pub fn with_universal_adapter(mut self, adapter: impl UniversalComputeAdapter + 'static) -> Self {
        self.universal_adapter = Some(Box::new(adapter));
        self
    }

    /// Smart threat prediction with local-first, network fallback
    pub async fn predict_threat_smart(&mut self, event: &SecurityEvent) -> BearDogResult<MlPrediction> {
        let cache_key = self.generate_cache_key(event);
        
        // Check cache first (zero-cost optimization)
        if let Some(cached) = self.prediction_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Try lightweight local prediction first
        if let Ok(prediction) = self.predict_local_lightweight(event).await {
            self.local_predictions += 1;
            self.prediction_cache.insert(cache_key, prediction.clone());
            return Ok(prediction);
        }

        // Fallback to universal adapter for heavy compute
        if let Some(adapter) = &self.universal_adapter {
            let prediction = self.predict_via_universal_adapter(event, adapter.as_ref()).await?;
            self.network_predictions += 1;
            self.prediction_cache.insert(cache_key, prediction.clone());
            return Ok(prediction);
        }

        // Final fallback to basic heuristics
        Ok(self.predict_basic_heuristics(event))
    }

    /// Lightweight local prediction for common threat patterns
    async fn predict_local_lightweight(&self, event: &SecurityEvent) -> BearDogResult<MlPrediction> {
        let start_time = std::time::Instant::now();
        
        // Smart pattern matching for common threats
        let threat_score = self.calculate_lightweight_score(event);
        let risk_level = self.score_to_risk_level(threat_score);
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(MlPrediction {
            threat_score,
            confidence: 0.75, // Moderate confidence for local predictions
            risk_level,
            reasoning: vec!["Local lightweight analysis".to_string()],
            model_version: "beardog-smart-v1.0".to_string(),
            processing_time_ms: processing_time,
        })
    }

    /// Heavy compute via universal adapter (toadstool network)
    async fn predict_via_universal_adapter(
        &self, 
        event: &SecurityEvent, 
        adapter: &dyn UniversalComputeAdapter
    ) -> BearDogResult<MlPrediction> {
        let start_time = std::time::Instant::now();
        
        // Serialize event for network compute
        let event_data = serde_json::to_string(event).map_err(|e| {
            beardog_errors::BearDogError::validation(format!("Failed to serialize event: {}", e))
        })?;
        
        // Request heavy ML computation via universal adapter
        let compute_request = serde_json::json!({
            "type": "threat_analysis",
            "data": event_data,
            "model": "advanced_threat_detection",
            "priority": "high"
        });

        // Send to toadstool network for advanced processing
        let response = adapter.request_compute("ml_threat_analysis", &compute_request).await?;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        // Parse network response
        let threat_score: f64 = response.get("threat_score")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5);
            
        let confidence: f64 = response.get("confidence")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.9);
            
        let reasoning: Vec<String> = response.get("reasoning")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_else(|| vec!["Network ML analysis".to_string()]);

        Ok(MlPrediction {
            threat_score,
            confidence,
            risk_level: self.score_to_risk_level(threat_score),
            reasoning,
            model_version: "toadstool-advanced-v2.0".to_string(),
            processing_time_ms: processing_time,
        })
    }

    /// Basic heuristic fallback for maximum reliability
    fn predict_basic_heuristics(&self, event: &SecurityEvent) -> MlPrediction {
        let threat_score = match event.event_type.as_str() {
            "login_failure" => 0.3,
            "malware_detected" => 0.9,
            "suspicious_network" => 0.6,
            "privilege_escalation" => 0.8,
            _ => 0.2,
        };

        MlPrediction {
            threat_score,
            confidence: 0.5, // Low confidence for heuristics
            risk_level: self.score_to_risk_level(threat_score),
            reasoning: vec!["Basic heuristic analysis".to_string()],
            model_version: "beardog-heuristics-v1.0".to_string(),
            processing_time_ms: 1, // Near-instant
        }
    }

    /// Smart lightweight scoring for common patterns
    fn calculate_lightweight_score(&self, event: &SecurityEvent) -> f64 {
        let mut score: f64 = 0.0;
        
        // Smart pattern detection
        if event.event_type.contains("malware") { score += 0.4; }
        if event.event_type.contains("attack") { score += 0.3; }
        if event.event_type.contains("suspicious") { score += 0.2; }
        if event.event_type.contains("failure") && event.event_type.contains("login") { score += 0.25; }
        
        // Source reputation analysis
        if let Some(source_ip) = &event.source_ip {
            if self.is_known_bad_ip(source_ip) { score += 0.3; }
            if self.is_tor_exit_node(source_ip) { score += 0.15; }
        }
        
        // Time-based analysis - use chrono::Timelike trait
        use chrono::Timelike;
        let hour = chrono::Utc::now().hour();
        if hour < 6 || hour > 22 { score += 0.1; } // Off-hours activity
        
        // Normalize to 0.0-1.0 range
        // Calculate final threat score with proper typing
        let final_score = score.min(1.0_f64).max(0.0_f64);
        final_score
    }

    fn score_to_risk_level(&self, score: f64) -> RiskLevel {
        match score {
            s if s >= 0.8 => RiskLevel::Critical,
            s if s >= 0.6 => RiskLevel::High,
            s if s >= 0.4 => RiskLevel::Medium,
            s if s >= 0.2 => RiskLevel::Low,
            _ => RiskLevel::Minimal,
        }
    }

    fn generate_cache_key(&self, event: &SecurityEvent) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        event.event_type.hash(&mut hasher);
        event.source_ip.hash(&mut hasher);
        // Use destination_ip instead of target_ip
        event.destination_ip.hash(&mut hasher);
        format!("threat_{}", hasher.finish())
    }

    // Smart IP reputation checks (lightweight)
    fn is_known_bad_ip(&self, ip: &str) -> bool {
        // Basic IP reputation check using simple heuristics
        // Check for known malicious patterns
        if ip.starts_with("192.168.") || ip.starts_with("10.") || ip.starts_with("172.") {
            // Private IP ranges are generally safe
            return false;
        }
        
        // Check for suspicious patterns (this is a simplified implementation)
        // In production, this would integrate with threat intelligence feeds
        let suspicious_patterns = [
            "0.0.0.0", "127.0.0.1", "255.255.255.255"
        ];
        
        suspicious_patterns.iter().any(|&pattern| ip == pattern)
    }

    fn is_tor_exit_node(&self, ip: &str) -> bool {
        // Basic Tor exit node detection using known patterns
        // In production, this would use a real-time Tor exit node list
        
        // Check for common Tor exit node IP patterns (simplified heuristics)
        // Real implementation would maintain an updated list from Tor directory authorities
        let known_tor_patterns = [
            // These are example patterns - real implementation would use actual data
            "95.211.", "176.10.", "198.98."
        ];
        
        known_tor_patterns.iter().any(|&pattern| ip.starts_with(pattern))
    }

    /// Performance metrics for monitoring
    pub fn get_performance_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("local_predictions".to_string(), self.local_predictions);
        stats.insert("network_predictions".to_string(), self.network_predictions);
        stats.insert("cache_size".to_string(), self.prediction_cache.len() as u64);
        stats
    }

    /// Clear cache for memory management
    pub fn clear_cache(&mut self) {
        self.prediction_cache.clear();
    }
}
