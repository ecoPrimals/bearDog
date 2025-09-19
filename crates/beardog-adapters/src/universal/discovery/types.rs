

use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug, Clone)]
    /// The capability type value
    pub capability_type: String,
    /// Collection of requirements
    pub requirements: Vec<String>,
    /// Mapping of constraints
    pub constraints: HashMap<String, String>,
    /// The priority value
    pub priority: RequestPriority,
    pub timeout: Duration,
}

impl CapabilityRequest {

/// Requirements Hash operation.
    pub fn requirements_hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.requirements.hash(&mut hasher);

        for (key, value) in &self.constraints {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish(Uuid,
    /// Collection of services
    pub services: Vec<ServiceEndpoint>,
    /// Optional scaling recommendations
    pub scaling_recommendations: Option<ScalingRecommendations>,
    /// Collection of secure channels
    pub secure_channels: Vec<SecureChannel>,
    pub discovery_time: Duration,
    pub ai_confidence_score: f64,
}

impl DiscoveryResult {

/// New operation.
    /// Creates a new instance
    pub fn new(request_id: Uuid) -> Self {
        Self {
            request_id,
            services: Vec::new(None,
            secure_channels: Vec::new(),
            discovery_time: Duration::from_secs(0.0,
        }
    }
}

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,
    /// The capability type value
    pub capability_type: String,
    /// The endpoint url value
    pub endpoint_url: String,
    /// The protocol value
    pub protocol: String,
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Collection of scaling trigger metrics
    pub scaling_trigger_metrics: Vec<String>,
    /// The estimated cost impact value
    pub estimated_cost_impact: f64,
    pub performance_predictions: PerformancePredictions,
}

#[derive(Debug, Clone)]
    /// The expected throughput rps value
    pub expected_throughput_rps: f64,
    /// The resource utilization value
    pub resource_utilization: ResourceUtilization,
}

#[derive(Debug, Clone)]
    /// The memory percent value
    pub memory_percent: f64,
    /// The network mbps value
    pub network_mbps: f64,
}

#[derive(Debug, Clone)]
    pub endpoint_id: Uuid,
    /// The encryption type value
    pub encryption_type: EncryptionType,
    /// The key exchange method value
    pub key_exchange_method: KeyExchangeMethod,
    /// The established at value
    pub established_at: SystemTime,
}

#[derive(Debug, Clone)]
    /// The source ecosystem value
    pub source_ecosystem: String,
    /// The target ecosystem value
    pub target_ecosystem: String,
    /// Collection of integration requirements
    pub integration_requirements: Vec<String>,
    pub performance_requirements: PerformanceRequirements,
}

#[derive(Debug, Clone)]
    /// Number of min_throughput_rps
    pub min_throughput_rps: u64,
    /// The availability percent value
    pub availability_percent: f64,
}

#[derive(Debug, Clone)]
    /// The ecosystem analysis value
    pub ecosystem_analysis: EcosystemAnalysis,
    /// Collection of adapters
    pub adapters: Vec<DynamicAdapter>,
    pub protocol_bridges: Vec<ProtocolBridge>,
    pub performance_model: PerformanceModel,
    /// Collection of secure channels
    pub secure_channels: Vec<SecureChannel>,
    pub integration_time: Duration,
    /// Whether success is enabled
    pub success: bool,
}

#[derive(Debug, Clone)]
    /// Collection of target capabilities
    pub target_capabilities: Vec<EcosystemCapability>,
    /// The compatibility matrix value
    pub compatibility_matrix: CompatibilityMatrix,
    /// Collection of incompatibilities
    pub incompatibilities: Vec<ProtocolIncompatibility>,
    /// The integration complexity value
    pub integration_complexity: IntegrationComplexity,
    /// The recommended approach value
    pub recommended_approach: IntegrationApproach,
}

#[derive(Debug, Clone)]
    /// The capability type value
    pub capability_type: String,
    /// The version value
    pub version: String,
    /// Collection of protocols
    pub protocols: Vec<String>,
    /// Collection of dependencies
    pub dependencies: Vec<String>,
}

#[derive(HashMap<String, HashMap<String, CompatibilityLevel>>,
}

#[derive(Debug, Clone)]
    /// The target protocol value
    pub target_protocol: String,
    /// The incompatibility type value
    pub incompatibility_type: IncompatibilityType,
    /// The severity value
    pub severity: IncompatibilitySeverity,
    pub performance_impact: f64,
    /// Optional workaround
    pub workaround: Option<String>,
}

#[derive(Debug, Clone)]
    /// The source capability value
    pub source_capability: String,
    /// The target capability value
    pub target_capability: String,
    /// The adapter type value
    pub adapter_type: AdapterType,
    pub transformation_rules: Vec<TransformationRule>,
}

#[derive(Debug, Clone)]
    pub transformation: TransformationType,
    /// The target path value
    pub target_path: String,
}

#[derive(Debug, Clone)]
    /// The source protocol value
    pub source_protocol: String,
    /// The target protocol value
    pub target_protocol: String,
    /// The translation overhead ms value
    pub translation_overhead_ms: f64,
}

#[derive(Debug, Clone)]
    /// Collection of scaling projections
    pub scaling_projections: Vec<ScalingProjection>,
}

#[derive(Debug, Clone)]
    pub predicted_performance: PerformancePredictions,
    /// The resource requirements value
    pub resource_requirements: ResourceUtilization,
}
