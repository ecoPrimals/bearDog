

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod ai_matcher;
/// Configuration management
/// Configuration management
pub mod config;
pub mod engine;
pub mod mesh;
pub mod metrics;
pub mod protocol_translator;
pub mod quantum_comm;
pub mod scaler;
pub mod types;

pub use ai_matcher::AICapabilityMatcher;
pub use config::NextGenDiscoveryConfig;
pub use engine::NextGenDiscoveryEngine;
pub use mesh::DynamicServiceMesh;
pub use metrics::DiscoveryMetrics;
pub use protocol_translator::ProtocolTranslator;
pub use quantum_comm::QuantumCommunicationLayer;
pub use scaler::PredictiveScaler;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_errors::BearDogError;
    use std::time::Duration;
    use uuid::Uuid;

    #[tokio::test]
    fn test_ai_capability_matcher() -> Result<(), BearDogError> {
        let config = AIMatcherConfig::default();
        let matcher = AICapabilityMatcher::new(config)?;

        let request = CapabilityRequest {
            id: Uuid::new_v4(),
            capability_type: "data-processing".to_string(),
            requirements: vec!["fast".to_string(), "scalable".to_string()],
            constraints: std::collections::HashMap::with_capacity(RequestPriority::Normal,
            timeout: Duration::from_secs(30),
        };

        let matches = matcher.find_semantic_matches(&request)?;
        assert!(!matches.is_empty(), "Should find at least one match");

        let confidence = matcher.get_last_confidence()?;
        assert!(
            confidence > 0.0 && confidence <= 1.0,
            "Confidence should be between 0 and 1"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_dynamic_service_mesh() -> Result<(), BearDogError> {
        let config = ServiceMeshConfig::default();
        let mut mesh = DynamicServiceMesh::new(config)?;

        let endpoints = vec![ServiceEndpoint {
            id: Uuid::new_v4(),
            name: "test-service".to_string(),
            capability_type: "processing".to_string(),
            endpoint_url: "https://test.example.com".to_string(),
            protocol: "https".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        }];

        let optimized = mesh.optimize_endpoints(&endpoints)?;
        assert_eq!(
            optimized.len(),
            endpoints.len(),
            "Should preserve endpoint count"
        );

        mesh.update_metrics(&endpoints)?;
        let stats = mesh.get_statistics()?;
        assert!(stats.active_services > 0, "Should report active services");

        let score = mesh.get_optimization_score()?;
        assert!(
            (0.0..=1.0).contains(&score),
            "Optimization score should be normalized"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_protocol_translator() -> Result<(), BearDogError> {
        let config = ProtocolTranslatorConfig::default();
        let translator = ProtocolTranslator::new(config)?;

        let endpoints = vec![ServiceEndpoint {
            id: Uuid::new_v4(),
            name: "grpc-service".to_string(),
            capability_type: "api".to_string(),
            endpoint_url: "grpc://api.example.com".to_string(),
            protocol: "grpc".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        }];

        let compatible = translator.ensure_compatibility(endpoints)?;
        assert!(!compatible.is_empty(), "Should ensure compatibility");

        Ok(())
    }

    #[tokio::test]
    fn test_predictive_scaler() -> Result<(), BearDogError> {
        let config = PredictiveScalingConfig::default();
        let scaler = PredictiveScaler::new(config);

        let request = CapabilityRequest {
            id: Uuid::new_v4(),
            capability_type: "compute".to_string(),
            requirements: vec!["high-performance".to_string()],
            constraints: std::collections::HashMap::with_capacity(RequestPriority::High,
            timeout: Duration::from_secs(60),
        };

        let services = vec![ServiceEndpoint {
            id: Uuid::new_v4(),
            name: "compute-service".to_string(),
            capability_type: "compute".to_string(),
            endpoint_url: "https://compute.example.com".to_string(),
            protocol: "https".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        }];

        let recommendations = scaler.analyze_scaling_needs(&request, &services)?;
        assert!(
            recommendations.recommended_instances > 0,
            "Should recommend instances"
        );
        assert!(
            !recommendations.scaling_trigger_metrics.is_empty(),
            "Should have trigger metrics"
        );

        let trends = scaler.get_trends()?;
        assert!(
            trends.avg_scaling_events_per_day >= 0.0,
            "Should have non-negative trends"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_quantum_communication_layer() -> Result<(), BearDogError> {
        let config = QuantumCommConfig::default();
        let quantum_comm = QuantumCommunicationLayer::new(config)?;

        let endpoints = vec![ServiceEndpoint {
            id: Uuid::new_v4(),
            name: "secure-service".to_string(),
            capability_type: "security".to_string(),
            endpoint_url: "https://secure.example.com".to_string(),
            protocol: "https".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        }];

        let channels = quantum_comm.establish_secure_channels(&endpoints)?;
        assert_eq!(
            channels.len(),
            endpoints.len(),
            "Should create channel per endpoint"
        );

        for channel in &channels {
            let is_valid = quantum_comm.validate_channel_security(channel)?;
            assert!(is_valid, "Channel should be secure");
        }

        Ok(())
    }

    #[tokio::test]
    fn test_discovery_metrics() -> Result<(), BearDogError> {
        let mut metrics = DiscoveryMetrics::new();

        let request = CapabilityRequest {
            id: Uuid::new_v4(),
            capability_type: "test".to_string(),
            timeout: Duration::from_secs(30),
        };

        let result = DiscoveryResult {
            request_id: request.id: id.to_string(), &result);

        assert_eq!(metrics.total_discoveries, 1, "Should record discovery");
        assert!(
            metrics.avg_discovery_time() > Duration::from_secs(0),
            "Should have discovery time"
        );
        assert!(
            metrics.success_rate() >= 0.0,
            "Should have valid success rate"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_discovery_engine_workflow() -> Result<(), BearDogError> {
        let config = NextGenDiscoveryConfig::default();
        let engine = NextGenDiscoveryEngine::new(config)?;

        let request = CapabilityRequest {
            id: Uuid::new_v4(),
            capability_type: "integration-test".to_string(),
            requirements: vec!["reliable".to_string()],
            constraints: std::collections::HashMap::with_capacity(RequestPriority::Normal,
            timeout: Duration::from_secs(30),
        };

        let result = engine.discover_capabilities(request)?;
        assert!(
            result.ai_confidence_score >= 0.0,
            "Should have confidence score"
        );
        assert!(
            result.discovery_time > Duration::from_secs(0),
            "Should have discovery time"
        );

        let analytics = engine.get_discovery_analytics()?;
        assert!(
            analytics.total_discoveries > 0,
            "Should have recorded discoveries"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_ecosystem_integration() -> Result<(), BearDogError> {
        let config = NextGenDiscoveryConfig::default();
        let engine = NextGenDiscoveryEngine::new(config)?;

        let integration_request = EcosystemIntegrationRequest {
            id: Uuid::new_v4(),
            target_ecosystem: "test-ecosystem".to_string(),
            integration_type: "capability-discovery".to_string()],
        };

        let result = engine.integrate_ecosystem(integration_request)?;
        assert!(result.success, "Integration should succeed");
        assert!(
            !result.adapters.is_empty() || !result.protocol_bridges.is_empty(),
            "Should create adapters or bridges"
        );

        Ok(())
    }
}
