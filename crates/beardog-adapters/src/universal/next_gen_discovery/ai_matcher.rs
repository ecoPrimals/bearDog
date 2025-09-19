

use super::types::{
    AIInsights, AIMatcherConfig, CapabilityRequest, EcosystemAnalysis, EcosystemIntegrationRequest,
    ServiceEndpoint,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug, Clone)]
    semantic_cache: HashMap<String, CachedSemanticMatch>,
    learning_data: Vec<LearningRecord>,
}

#[derive(Debug, Clone)]
    cached_at: SystemTime,
    confidence_score: f64,
}

#[derive(Debug, Clone)]
    success_rate: f64,
    confidence: f64,
    timestamp: SystemTime,
}

impl AICapabilityMatcher {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: AIMatcherConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            semantic_cache: HashMap::with_capacity(16),
            learning_data: Vec::new(&CapabilityRequest,
    ) -> Result<Vec<ServiceEndpoint>, BearDogError> {

        let cache_key = self.generate_cache_key(request);
        if let Some(cached) = self.semantic_cache.get(&cache_key) {
            if self.is_cache_valid(cached) {
                return Ok(cached.matches);
            }
        }

        let candidate_services = self.discover_candidate_services()?;
        let mut matches = Vec::new(&EcosystemIntegrationRequest,
    ) -> Result<EcosystemAnalysis, BearDogError> {
        use super::types::{
            CompatibilityMatrix, IncompatibilitySeverity, IncompatibilityType, IntegrationApproach,
            IntegrationComplexity, ProtocolIncompatibility,
        };

        let capabilities = self
            .discover_ecosystem_capabilities(&request.target_ecosystem)
            ?;

        let mut compatibility_matrix = CompatibilityMatrix::new();
        compatibility_matrix.add_score("overall".to_string(), 0.8); // Default good compatibility

        let incompatibilities = vec![ProtocolIncompatibility {
            source_protocol: "http".to_string(),
            target_protocol: "grpc".to_string(),
            description: "Protocol translation required".to_string(),
            source_pattern: "REST API".to_string(),
            target_pattern: "gRPC Service".to_string(0.1,
        }];

        Ok(&request.target_ecosystem,
            capabilities,
            incompatibilities,
            compatibility_matrix,
            integration_complexity: IntegrationComplexity::Moderate,
            recommended_approach: IntegrationApproach::AdapterPattern,
        })
    }

/// Get Insights operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets insights
    /// Gets insights
    pub fn get_insights(&self) -> Result<AIInsights, BearDogError> {
        let total_matches = self.learning_data.len() as u64;
        let avg_confidence = if !self.learning_data.is_empty() {
            self.learning_data.iter().map(|r| r.confidence).sum::<f64>()
                / self.learning_data.len(avg_confidence,
            learning_progress,
            top_patterns,
            anomaly_detection_rate: 0.05, // 5% anomaly detection rate
        })
    }

/// Get Last Confidence operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets last_confidence
    /// Gets last_confidence
    pub fn get_last_confidence(&self) -> Result<f64, BearDogError> {
        Ok(self
            .learning_data
            .last()
            .map(|r| r.confidence)
            .unwrap_or(0.8))
    }


    fn discover_candidate_services(&self) -> Result<Vec<ServiceEndpoint>, BearDogError> {
        use beardog_types::canonical::HealthStatus;

        Ok(vec![
            ServiceEndpoint {
                id: Uuid::new_v4(),
                name: "ai-processing-service".to_string(),
                capability_type: "data-processing".to_string(),
                endpoint_url: "https://ai-service.beardog.internal".to_string(),
                protocol: "https".to_string(),
                metadata: HashMap::with_capacity(16),
            },
            ServiceEndpoint {
                id: Uuid::new_v4(),
                name: "auth-service".to_string(),
                capability_type: "authentication".to_string(),
                endpoint_url: "https://auth.beardog.internal".to_string(),
                protocol: "https".to_string(),
    ) -> Result<f64, BearDogError> {

        let similarity = if request_type == service_type {
            1.0
        } else if request_type.contains(&CapabilityRequest,
        mut matches: Vec<ServiceEndpoint>,
    ) -> Result<Vec<ServiceEndpoint>, BearDogError> {

        matches.sort_by(|a, b| {
            use beardog_types::canonical::HealthStatus;
            match (&a.health_status, &b.health_status) {
                (HealthStatus::Healthy, HealthStatus::Healthy) => a.name.cmp(&b.name),
                (HealthStatus::Healthy, _) => std::cmp::Ordering::Less,
                (_, HealthStatus::Healthy) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });

        match request.priority {
            super::types::RequestPriority::Critical => Ok(matches), // Return all for critical
            super::types::RequestPriority::High => Ok(matches.into_iter().take(10).collect()),
            super::types::RequestPriority::Normal => Ok(matches.into_iter().take(5).collect()),
            super::types::RequestPriority::Low => Ok(&str,
    ) -> Result<Vec<super::types::EcosystemCapability>, BearDogError> {

        Ok(vec![super::types::EcosystemCapability {
            name: format!("{ecosystem}-core"),
            version: "1.0.0".to_string(),
            protocol: "https".to_string()],
            metadata: HashMap::with_capacity(16),
        }])
    }


    fn generate_cache_key(&self, request: &CapabilityRequest) -> String {
        format!(
            "{}:{}",
            request.capability_type,
            request.requirements_hash()
        )
    }

    /// Checks if cache valid
    fn is_cache_valid(&self, cached: &CachedSemanticMatch) -> bool {
        cached
            .cached_at
            .elapsed()
            .unwrap_or(Duration::from_secs(u64::MAX))
            < self.config.cache_ttl
    }


    fn extract_top_patterns(HashMap<String, usize> = HashMap::with_capacity(16);

        for record in &self.learning_data {
            *pattern_counts
                .entry(record.request_type)
                .or_insert(0) += 1;
        }

        let mut patterns: Vec<_> = pattern_counts.into_iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));

        patterns
            .into_iter()
            .take(5)
            .map(|(pattern, _)| pattern)
            .collect()
    }


    fn calculate_learning_progress(&self) -> f64 {
        if self.learning_data.is_empty() {
            0.0
        } else {

            let recent_success_rate = self
                .learning_data
                .iter()
                .rev()
                .take(10)
                .map(|r| r.success_rate)
                .sum::<f64>()
                / 10.0_f64.min(self.learning_data.len() as f64);

            (recent_success_rate * 0.7) + (self.learning_data.len() as f64 / 1000.0 * 0.3).min(0.3)
        }
    }
}
