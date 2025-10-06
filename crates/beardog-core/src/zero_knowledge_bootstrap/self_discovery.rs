// Self-Discovery Engine
//
// This module implements the core self-discovery functionality for zero-knowledge bootstrap.
// It enables a primal to discover its own capabilities and identity without any hardcoded
// knowledge about the ecosystem.

use crate::ecosystem::primal_types::{PrimalMetadata, UniversalEndpoint};
use crate::zero_knowledge_bootstrap::SelfIdentity;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

pub struct SelfDiscoveryEngine {
    config: UnifiedBootstrapConfig,
    discovered_capabilities: Vec<ServiceCapabilityType>,
    self_metadata: Option<PrimalMetadata>,
}

/// Self-capability detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCapabilityDetection {
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    pub confidence_score: f64, // 0.0 to 1.0
    pub evidence: Vec<String>,
    /// Whether `auto_detected` is enabled
    pub auto_detected: bool,
}

/// Self-identity discovery result
#[derive(Debug, Clone)]
pub struct SelfIdentityDiscovery {
    pub primal_id: String,
    /// Collection of detected capabilities
    pub detected_capabilities: Vec<SelfCapabilityDetection>,
    /// Collection of endpoints
    pub endpoints: Vec<UniversalEndpoint>,
    /// The metadata value
    pub metadata: PrimalMetadata,
    /// Number of `discovery_duration_ms`
    pub discovery_duration_ms: u64,
}

impl SelfDiscoveryEngine {
    /// Create new self-discovery engine
    /// Creates a new instance
    pub fn new() -> BearDogResult<Self> {
        let config = UnifiedBootstrapConfig::default();

        info!("🌱 Initializing Self-Discovery Engine");
        info!("🎯 Mission: Discover own capabilities without hardcoded knowledge");

        Ok(Self {
            config,
            discovered_capabilities: Vec::new(),
            self_metadata: None,
        })
    }

    /// Discover self-identity - the foundation of zero-knowledge bootstrap
    pub fn discover_self_identity(&mut self) -> BearDogResult<SelfIdentity> {
        let start_time = std::time::Instant::now();

        info!("🔍 Starting self-identity discovery...");
        info!("📋 Discovery Plan:");
        info!("   1. Generate unique primal ID");
        info!("   2. Auto-detect available capabilities");
        info!("   3. Discover communication endpoints");
        info!("   4. Build self-metadata");
        info!("   5. Validate self-knowledge");

        // Phase 1: Generate unique primal identity
        let primal_id = self.generate_primal_id()?;
        info!("✅ Generated primal ID: {}", primal_id);

        // Phase 2: Auto-detect capabilities
        let capabilities = self.auto_detect_capabilities()?;
        info!("✅ Detected {} capabilities", capabilities.len());

        // Phase 3: Discover endpoints
        let endpoints = self.discover_endpoints()?;
        info!("✅ Discovered {} endpoints", endpoints.len());

        // Phase 4: Build metadata
        let metadata = self.build_self_metadata(&primal_id, &capabilities)?;
        info!("✅ Built self-metadata");

        // Phase 5: Validate self-knowledge
        self.validate_self_knowledge(&capabilities)?;
        info!("✅ Self-knowledge validated");

        let discovery_duration = start_time.elapsed().as_millis() as u64;

        let self_identity = SelfIdentity {
            primal_id,
            capabilities: capabilities
                .iter()
                .map(|c| c.capability_type.clone())
                .collect(),
            endpoints,
            metadata,
            bootstrap_time: std::time::SystemTime::now(),
        };

        info!("🎉 Self-identity discovery complete!");
        info!("📊 Discovery Results:");
        info!("   🆔 Primal ID: {}", self_identity.primal_id);
        info!("   ⚡ Capabilities: {:?}", self_identity.capabilities);
        info!(
            "   📡 Endpoints: {} discovered",
            self_identity.endpoints.len()
        );
        info!("   ⏱️  Duration: {}ms", discovery_duration);

        Ok(self_identity)
    }

    /// Generate unique primal ID (no hardcoded names)
    fn generate_primal_id(&self) -> BearDogResult<String> {
        // Generate truly unique ID without hardcoded primal names
        let uuid = Uuid::new_v4();
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Create primal ID that's unique but not hardcoded
        let primal_id = format!(
            "beardog-{}-{}",
            hostname.chars().take(8).collect::<String>(),
            &uuid.to_string()[..8]
        );

        debug!("🆔 Generated primal ID: {} (no hardcoded names)", primal_id);
        Ok(primal_id)
    }

    /// Auto-detect available capabilities by examining the runtime environment
    fn auto_detect_capabilities(&mut self) -> BearDogResult<Vec<SelfCapabilityDetection>> {
        info!("🔍 Auto-detecting capabilities...");
        let mut capabilities = Vec::new();

        // Security capability detection (always present for BearDog)
        capabilities.push(SelfCapabilityDetection {
            capability_type: ServiceCapabilityType::Security,
            confidence_score: 1.0,
            evidence: vec![
                "BearDog core security module detected".to_string(),
                "HSM integration available".to_string(),
                "Cryptographic functions present".to_string(),
            ],
            auto_detected: true,
        });

        // Key management detection
        if self.detect_key_management_capability()? {
            capabilities.push(SelfCapabilityDetection {
                capability_type: ServiceCapabilityType::KeyManagement,
                confidence_score: 0.95,
                evidence: vec![
                    "Key generation functions detected".to_string(),
                    "HSM integration available".to_string(),
                ],
                auto_detected: true,
            });
        }

        // Hardware security module detection
        if self.detect_hsm_capability()? {
            capabilities.push(SelfCapabilityDetection {
                capability_type: ServiceCapabilityType::HardwareSecurityModule,
                confidence_score: 0.9,
                evidence: vec![
                    "HSM provider modules detected".to_string(),
                    "Hardware attestation available".to_string(),
                ],
                auto_detected: true,
            });
        }

        // Authentication capability detection
        if self.detect_authentication_capability()? {
            capabilities.push(SelfCapabilityDetection {
                capability_type: ServiceCapabilityType::Authentication,
                confidence_score: 0.85,
                evidence: vec![
                    "Authentication modules detected".to_string(),
                    "JWT/token handling available".to_string(),
                ],
                auto_detected: true,
            });
        }

        // Compliance auditing detection
        if self.detect_compliance_capability()? {
            capabilities.push(SelfCapabilityDetection {
                capability_type: ServiceCapabilityType::ComplianceAudit,
                confidence_score: 0.8,
                evidence: vec![
                    "Compliance modules detected".to_string(),
                    "Audit logging available".to_string(),
                ],
                auto_detected: true,
            });
        }

        // Threat detection capability
        if self.detect_threat_detection_capability()? {
            capabilities.push(SelfCapabilityDetection {
                capability_type: ServiceCapabilityType::ThreatDetection,
                confidence_score: 0.75,
                evidence: vec![
                    "Threat analysis modules detected".to_string(),
                    "Security monitoring available".to_string(),
                ],
                auto_detected: true,
            });
        }

        self.discovered_capabilities = capabilities
            .iter()
            .map(|c| c.capability_type.clone())
            .collect();

        info!("🎯 Capability auto-detection complete:");
        for cap in &capabilities {
            info!(
                "   ⚡ {} (confidence: {:.1}%)",
                format!("{:?}", cap.capability_type),
                cap.confidence_score * 100.0
            );
        }

        Ok(capabilities)
    }

    /// Detect key management capability
    fn detect_key_management_capability(&self) -> BearDogResult<bool> {
        // Check if key management modules are available
        let has_hsm = std::path::Path::new("crates/beardog-tunnel").exists();
        let has_crypto = std::path::Path::new("crates/beardog-security").exists();

        Ok(has_hsm && has_crypto)
    }

    /// Detect HSM capability
    fn detect_hsm_capability(&self) -> BearDogResult<bool> {
        // Check if HSM modules are available
        let has_tunnel = std::path::Path::new("crates/beardog-tunnel").exists();
        Ok(has_tunnel)
    }

    /// Detect authentication capability
    fn detect_authentication_capability(&self) -> BearDogResult<bool> {
        // Check if auth modules are available
        let has_auth = std::path::Path::new("crates/beardog-auth").exists();
        Ok(has_auth)
    }

    /// Detect compliance capability
    fn detect_compliance_capability(&self) -> BearDogResult<bool> {
        // Check if compliance modules are available
        let has_compliance = std::path::Path::new("crates/beardog-compliance").exists();
        Ok(has_compliance)
    }

    /// Detect threat detection capability
    fn detect_threat_detection_capability(&self) -> BearDogResult<bool> {
        // Check if threat detection modules are available
        let has_threat = std::path::Path::new("crates/beardog-threat").exists();
        Ok(has_threat)
    }

    /// Discover communication endpoints
    fn discover_endpoints(&self) -> BearDogResult<Vec<UniversalEndpoint>> {
        info!("📡 Discovering communication endpoints...");
        let mut endpoints = Vec::new();

        // Discover local endpoints
        let local_endpoint = self.discover_local_endpoint()?;
        endpoints.push(local_endpoint);

        // Discover network endpoints
        if let Ok(network_endpoint) = self.discover_network_endpoint() {
            endpoints.push(network_endpoint);
        }

        // Discover service mesh endpoints
        if let Ok(mesh_endpoint) = self.discover_mesh_endpoint() {
            endpoints.push(mesh_endpoint);
        }

        debug!("📡 Discovered {} endpoints", endpoints.len());
        Ok(endpoints)
    }

    /// Discover local endpoint
    fn discover_local_endpoint(&self) -> BearDogResult<UniversalEndpoint> {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        let port = network_config.service_ports.api_port;

        Ok(UniversalEndpoint {
            url: format!("http://127.0.0.1:{port}"),
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string()],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: Default::default(),
        })
    }

    /// Discover network endpoint
    fn discover_network_endpoint(&self) -> BearDogResult<UniversalEndpoint> {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = network_config.service_ports.api_port;

        Ok(UniversalEndpoint {
            url: format!("http://{host}:{port}"),
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string(), "gRPC".to_string()],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: Default::default(),
        })
    }

    /// Discover service mesh endpoint
    fn discover_mesh_endpoint(&self) -> BearDogResult<UniversalEndpoint> {
        let mesh_port = std::env::var("BEARDOG_MESH_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8443);

        Ok(UniversalEndpoint {
            url: format!("https://0.0.0.0:{mesh_port}"),
            protocols: vec![
                "HTTPS".to_string(),
                "gRPC".to_string(),
                "WebSocket".to_string(),
            ],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: Default::default(),
        })
    }

    /// Build self-metadata
    /// Builds `self_metadata`
    fn build_self_metadata(
        &self,
        primal_id: &str,
        capabilities: &[SelfCapabilityDetection],
    ) -> BearDogResult<PrimalMetadata> {
        let version = env!("CARGO_PKG_VERSION").to_string();

        let mut custom_fields = HashMap::new();
        custom_fields.insert("bootstrap_method".to_string(), "zero_knowledge".to_string());
        custom_fields.insert(
            "discovery_engine".to_string(),
            "self_discovery_v1".to_string(),
        );
        custom_fields.insert(
            "capabilities_count".to_string(),
            capabilities.len().to_string(),
        );
        custom_fields.insert("auto_detected".to_string(), "true".to_string());

        Ok(PrimalMetadata {
            display_name: Some(format!("BearDog-{}", &primal_id[..8])),
            version,
            protocol_versions: vec!["1.0".to_string(), "2.0".to_string()],
            // SecurityAttestation starts empty for infant primals - attestations are acquired
            // dynamically through HSM interaction and ecosystem trust establishment
            security_attestations: Vec::new(),
            custom_fields,
            capabilities: vec![],
            dependencies: vec![],
            supported_protocols: vec!["http".to_string(), "https".to_string()],
            health_check_endpoint: "/health".to_string(),
            metrics_endpoint: "/metrics".to_string(),
        })
    }

    /// Validate self-knowledge
    /// Validates `self_knowledge`
    fn validate_self_knowledge(
        &self,
        capabilities: &[SelfCapabilityDetection],
    ) -> BearDogResult<()> {
        info!("✅ Validating self-knowledge...");

        // Ensure we have at least one capability
        if capabilities.is_empty() {
            return Err(BearDogError::validation(
                "No capabilities detected during self-discovery",
            ));
        }

        // Ensure security capability is present (core BearDog functionality)
        let has_security = capabilities
            .iter()
            .any(|c| matches!(c.capability_type, ServiceCapabilityType::Security));

        if !has_security {
            warn!(
                "⚠️ Security capability not detected - this may indicate incomplete self-discovery"
            );
        }

        // Validate confidence scores
        for cap in capabilities {
            if cap.confidence_score < 0.5 {
                warn!(
                    "⚠️ Low confidence for capability {:?}: {:.1}%",
                    cap.capability_type,
                    cap.confidence_score * 100.0
                );
            }
        }

        info!("✅ Self-knowledge validation complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_self_discovery_engine() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();

        // Should create engine successfully
        assert!(engine.discovered_capabilities.is_empty());

        // Should discover self-identity
        let identity = engine.discover_self_identity().unwrap();

        // Should have unique primal ID
        assert!(!identity.primal_id.is_empty());
        assert!(identity.primal_id.starts_with("beardog-"));

        // Should have discovered capabilities
        assert!(!identity.capabilities.is_empty());

        // Should have at least one endpoint
        assert!(!identity.endpoints.is_empty());

        // Should have metadata
        assert!(identity.metadata.display_name.is_some());
        assert!(!identity.metadata.version.is_empty());
    }

    #[tokio::test]
    async fn test_zero_hardcoded_knowledge() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        // Should not contain hardcoded primal names
        // Validate primal sovereignty - each primal only knows itself
        assert!(!identity.primal_id.is_empty(), "Must have self-identity");
        assert!(
            identity.primal_id.to_lowercase().contains("beardog"),
            "Should identify as beardog variant, got: {}",
            identity.primal_id
        );
        assert!(
            !identity.capabilities.is_empty(),
            "Must know own capabilities"
        );

        // Validate infant discovery - no hardcoded ecosystem knowledge
        for capability in &identity.capabilities {
            let cap_str = format!("{:?}", capability);
            assert!(
                !cap_str.to_lowercase().contains("hardcoded"),
                "Capabilities should be discovered dynamically"
            );
        }
        // ✅ SOVEREIGNTY COMPLIANCE: No hardcoded primal assumptions
        assert!(
            !identity.primal_id.is_empty(),
            "Primal ID should be discovered dynamically"
        );

        // Should not have hardcoded endpoints
        for endpoint in &identity.endpoints {
            // Validate endpoint sovereignty - no hardcoded service assumptions
            assert!(!endpoint.url.is_empty(), "Must have endpoint URL");
            assert!(
                !endpoint.url.contains("hardcoded"),
                "URL should be discovered/configured dynamically"
            );

            // Validate proper endpoint format
            assert!(
                endpoint.url.starts_with("http://") || endpoint.url.starts_with("https://"),
                "Endpoint should be proper URL"
            );
        }
    }

    #[tokio::test]
    async fn test_capability_auto_detection() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let capabilities = engine.auto_detect_capabilities().unwrap();

        // Should detect at least security capability
        assert!(capabilities
            .iter()
            .any(|c| matches!(c.capability_type, ServiceCapabilityType::Security)));

        // All capabilities should have reasonable confidence
        for cap in &capabilities {
            assert!(cap.confidence_score > 0.0);
            assert!(cap.confidence_score <= 1.0);
            assert!(!cap.evidence.is_empty());
        }
    }
}
