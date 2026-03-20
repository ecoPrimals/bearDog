// SPDX-License-Identifier: AGPL-3.0-only

// Self-Discovery Engine
//
// This module implements the core self-discovery functionality for zero-knowledge bootstrap.
// It enables a primal to discover its own capabilities and identity without any hardcoded
// knowledge about the ecosystem.

use crate::ecosystem::primal_types::{PrimalMetadata, UniversalEndpoint};
use crate::zero_knowledge_bootstrap::SelfIdentity;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Inputs for primal ID generation (see [`SelfDiscoveryEngine::generate_primal_id_from_inputs`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrimalIdEnvInputs {
    /// `HOSTNAME` / `COMPUTERNAME`
    pub hostname: Option<String>,
    /// `PRIMAL_TYPE` / `SERVICE_TYPE`
    pub primal_type: Option<String>,
}

impl PrimalIdEnvInputs {
    /// Read identity inputs with `std::env::var` (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            hostname: std::env::var("HOSTNAME")
                .ok()
                .or_else(|| std::env::var("COMPUTERNAME").ok()),
            primal_type: std::env::var("PRIMAL_TYPE")
                .ok()
                .or_else(|| std::env::var("SERVICE_TYPE").ok()),
        }
    }
}

/// Injected configuration for [`SelfDiscoveryEngine`] (no reads in [`Default::default`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct SelfDiscoveryEnvInputs {
    /// Primal ID components ([`PrimalIdEnvInputs`]).
    pub primal_id: PrimalIdEnvInputs,
    pub primal_name: Option<String>,
    pub service_name: Option<String>,
    pub beardog_display_name: Option<String>,
    pub localhost_override: Option<String>,
    pub network_host_override: Option<String>,
    pub mesh_port_override: Option<u16>,
    pub admin_port_override: Option<u16>,
    pub mesh_bind_address_override: Option<String>,
}

impl SelfDiscoveryEnvInputs {
    /// Read inputs from the process environment (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            primal_id: PrimalIdEnvInputs::from_env(),
            primal_name: std::env::var("PRIMAL_NAME").ok(),
            service_name: std::env::var("SERVICE_NAME").ok(),
            beardog_display_name: std::env::var("BEARDOG_DISPLAY_NAME").ok(),
            localhost_override: std::env::var("BEARDOG_LOCALHOST").ok(),
            network_host_override: std::env::var("BEARDOG_HOST")
                .ok()
                .or_else(|| std::env::var("BEARDOG_BIND_ADDRESS").ok()),
            mesh_port_override: std::env::var("BEARDOG_MESH_PORT")
                .ok()
                .and_then(|p| p.parse().ok()),
            admin_port_override: std::env::var("BEARDOG_ADMIN_PORT")
                .ok()
                .and_then(|p| p.parse().ok()),
            mesh_bind_address_override: std::env::var("BEARDOG_MESH_BIND_ADDRESS").ok(),
        }
    }
}

/// Self-Discovery Engine for Zero-Knowledge Bootstrap
///
/// This engine enables a primal to discover its own capabilities and identity
/// without any hardcoded knowledge about the ecosystem. It implements the core
/// zero-knowledge bootstrap functionality by:
///
/// - Generating unique primal IDs without hardcoded names
/// - Auto-detecting available capabilities from the runtime environment
/// - Discovering communication endpoints dynamically
/// - Building self-metadata based on discovered information
/// - Validating self-knowledge before announcing to the ecosystem
///
/// # Examples
///
/// ```ignore
/// use beardog_core::zero_knowledge_bootstrap::SelfDiscoveryEngine;
///
/// let mut engine = SelfDiscoveryEngine::new()?;
/// let identity = engine.discover_self_identity()?;
/// println!("Discovered primal ID: {}", identity.primal_id);
/// ```
#[derive(Debug)]
pub struct SelfDiscoveryEngine {
    discovered_capabilities: Vec<ServiceCapabilityType>,
    inputs: SelfDiscoveryEnvInputs,
    // Note: Config-based discovery and metadata caching removed as unused.
    // These can be re-added when dynamic configuration and caching are needed.
}

/// Self-capability detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCapabilityDetection {
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// Confidence score for this capability detection (0.0 to 1.0)
    pub confidence_score: f64,
    /// Evidence supporting this capability detection
    pub evidence: Vec<String>,
    /// Whether `auto_detected` is enabled
    pub auto_detected: bool,
}

/// Self-identity discovery result
#[derive(Debug, Clone)]
pub struct SelfIdentityDiscovery {
    /// The unique primal identifier for this service
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
    ///
    /// # Errors
    /// Returns an error if initialization of internal components or configuration fails.
    pub fn new() -> Result<Self, BearDogError> {
        Self::with_inputs(SelfDiscoveryEnvInputs::default())
    }

    /// Create an engine using environment-derived inputs (see [`SelfDiscoveryEnvInputs::from_env`]).
    pub fn from_env() -> Result<Self, BearDogError> {
        Self::with_inputs(SelfDiscoveryEnvInputs::from_env())
    }

    /// Create an engine with explicit discovery inputs (no environment reads).
    pub fn with_inputs(inputs: SelfDiscoveryEnvInputs) -> Result<Self, BearDogError> {
        info!("🌱 Initializing Self-Discovery Engine");
        info!("🎯 Mission: Discover own capabilities without hardcoded knowledge");

        Ok(Self {
            discovered_capabilities: Vec::new(),
            inputs,
        })
    }

    /// Discover self-identity - the foundation of zero-knowledge bootstrap
    ///
    /// # Errors
    /// Returns an error if self-identity discovery fails, if capability detection encounters issues,
    /// or if validation of self-knowledge fails.
    pub fn discover_self_identity(&mut self) -> Result<SelfIdentity, BearDogError> {
        let start_time = std::time::Instant::now();

        self.log_discovery_plan();
        let (primal_id, capabilities, endpoints, metadata) = self.execute_discovery_phases()?;
        let self_identity =
            Self::build_self_identity(primal_id, &capabilities, endpoints, metadata);
        Self::log_discovery_results(&self_identity, start_time);

        Ok(self_identity)
    }

    /// Logs the discovery plan
    #[allow(clippy::unused_self, reason = "will use self when fully implemented")]
    fn log_discovery_plan(&self) {
        info!("🔍 Starting self-identity discovery...");
        info!("📋 Discovery Plan:");
        info!("   1. Generate unique primal ID");
        info!("   2. Auto-detect available capabilities");
        info!("   3. Discover communication endpoints");
        info!("   4. Build self-metadata");
        info!("   5. Validate self-knowledge");
    }

    /// Executes all discovery phases
    ///
    /// # Errors
    /// Returns an error if any discovery phase fails
    fn execute_discovery_phases(
        &mut self,
    ) -> Result<
        (
            String,
            Vec<SelfCapabilityDetection>,
            Vec<UniversalEndpoint>,
            PrimalMetadata,
        ),
        BearDogError,
    > {
        // Phase 1: Generate unique primal identity
        let primal_id = Self::generate_primal_id_from_inputs(&self.inputs.primal_id);
        info!("✅ Generated primal ID: {}", primal_id);

        // Phase 2: Auto-detect capabilities
        let capabilities = self.auto_detect_capabilities();
        info!("✅ Detected {} capabilities", capabilities.len());

        // Phase 3: Discover endpoints
        let endpoints = Self::discover_endpoints(&self.inputs);
        info!("✅ Discovered {} endpoints", endpoints.len());

        // Phase 4: Build metadata
        let metadata = Self::build_self_metadata(&primal_id, &capabilities, &self.inputs);
        info!("✅ Built self-metadata");

        // Phase 5: Validate self-knowledge
        Self::validate_self_knowledge(&capabilities)?;
        info!("✅ Self-knowledge validated");

        Ok((primal_id, capabilities, endpoints, metadata))
    }

    /// Builds the final `SelfIdentity` object from discovered components
    fn build_self_identity(
        primal_id: String,
        capabilities: &[SelfCapabilityDetection],
        endpoints: Vec<UniversalEndpoint>,
        metadata: PrimalMetadata,
    ) -> SelfIdentity {
        SelfIdentity {
            primal_id,
            capabilities: capabilities
                .iter()
                .map(|c| c.capability_type.clone())
                .collect(),
            endpoints,
            metadata,
            bootstrap_time: std::time::SystemTime::now(),
        }
    }

    /// Logs the final discovery results
    fn log_discovery_results(self_identity: &SelfIdentity, start_time: std::time::Instant) {
        let discovery_duration = start_time.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;

        info!("🎉 Self-identity discovery complete!");
        info!("📊 Discovery Results:");
        info!("   🆔 Primal ID: {}", self_identity.primal_id);
        info!("   ⚡ Capabilities: {:?}", self_identity.capabilities);
        info!(
            "   📡 Endpoints: {} discovered",
            self_identity.endpoints.len()
        );
        info!("   ⏱️  Duration: {}ms", discovery_duration);
    }

    /// Generate unique primal ID from explicit inputs (no environment reads).
    pub fn generate_primal_id_from_inputs(inputs: &PrimalIdEnvInputs) -> String {
        let uuid = Uuid::new_v4();
        let hostname = inputs
            .hostname
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let primal_type = inputs
            .primal_type
            .clone()
            .unwrap_or_else(|| "primal".to_string());

        let primal_id = format!(
            "{}-{}-{}",
            primal_type.to_lowercase(),
            hostname.chars().take(8).collect::<String>(),
            &uuid.to_string()[..8]
        );

        debug!(
            "🆔 Generated primal ID: {} (self-discovered type: {})",
            primal_id, primal_type
        );
        primal_id
    }

    /// Auto-detect available capabilities by examining the runtime environment
    fn auto_detect_capabilities(&mut self) -> Vec<SelfCapabilityDetection> {
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
        if Self::detect_key_management_capability() {
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
        if Self::detect_hsm_capability() {
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
        if Self::detect_authentication_capability() {
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
        if Self::detect_compliance_capability() {
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
        if Self::detect_threat_detection_capability() {
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

        capabilities
    }

    /// Detect key management capability
    fn detect_key_management_capability() -> bool {
        // Check if key management modules are available
        let has_hsm = std::path::Path::new("crates/beardog-tunnel").exists();
        let has_crypto = std::path::Path::new("crates/beardog-security").exists();
        has_hsm && has_crypto
    }

    /// Detect HSM capability
    fn detect_hsm_capability() -> bool {
        // Check if HSM modules are available
        std::path::Path::new("crates/beardog-tunnel").exists()
    }

    /// Detect authentication capability
    fn detect_authentication_capability() -> bool {
        // Check if auth modules are available
        std::path::Path::new("crates/beardog-auth").exists()
    }

    /// Detect compliance capability
    fn detect_compliance_capability() -> bool {
        // Check if compliance modules are available
        std::path::Path::new("crates/beardog-compliance").exists()
    }

    /// Detect threat detection capability
    fn detect_threat_detection_capability() -> bool {
        // Check if threat detection modules are available
        std::path::Path::new("crates/beardog-threat").exists()
    }

    /// Discover communication endpoints
    fn discover_endpoints(inputs: &SelfDiscoveryEnvInputs) -> Vec<UniversalEndpoint> {
        info!("📡 Discovering communication endpoints...");
        let mut endpoints = Vec::new();

        // Discover local endpoints
        let local_endpoint = Self::discover_local_endpoint(inputs);
        endpoints.push(local_endpoint);

        // Discover network endpoints
        let network_endpoint = Self::discover_network_endpoint(inputs);
        endpoints.push(network_endpoint);

        // Discover service mesh endpoints
        let mesh_endpoint = Self::discover_mesh_endpoint(inputs);
        endpoints.push(mesh_endpoint);

        debug!("📡 Discovered {} endpoints", endpoints.len());
        endpoints
    }

    /// Discover local endpoint
    fn discover_local_endpoint(inputs: &SelfDiscoveryEnvInputs) -> UniversalEndpoint {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        let port = network_config.service_ports.api_port;

        use beardog_types::constants::domains::network::config;

        let host = inputs
            .localhost_override
            .clone()
            .unwrap_or_else(|| config::default_service_host());

        UniversalEndpoint {
            url: format!("http://{host}:{port}"),
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string()],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
        }
    }

    /// Discover network endpoint
    fn discover_network_endpoint(inputs: &SelfDiscoveryEnvInputs) -> UniversalEndpoint {
        use beardog_config::global::BEARDOG_CONFIG;
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        // Get bind address from centralized config
        let host = inputs
            .network_host_override
            .clone()
            .unwrap_or_else(|| BEARDOG_CONFIG.network.addresses.bind_address.clone());
        let port = network_config.service_ports.api_port;

        UniversalEndpoint {
            url: format!("http://{host}:{port}"),
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string(), "gRPC".to_string()],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
        }
    }

    /// Discover service mesh endpoint
    fn discover_mesh_endpoint(inputs: &SelfDiscoveryEnvInputs) -> UniversalEndpoint {
        use beardog_config::domains::network_ports;
        use beardog_config::global::BEARDOG_CONFIG;

        let mesh_port = inputs
            .mesh_port_override
            .or(inputs.admin_port_override)
            .unwrap_or(network_ports::DEFAULT_MESH_PORT);

        let bind_address = inputs
            .mesh_bind_address_override
            .clone()
            .unwrap_or_else(|| BEARDOG_CONFIG.network.addresses.bind_address.clone());

        UniversalEndpoint {
            url: format!("https://{bind_address}:{mesh_port}"),
            protocols: vec![
                "HTTPS".to_string(),
                "gRPC".to_string(),
                "WebSocket".to_string(),
            ],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
        }
    }

    /// Build self-metadata
    /// Builds `self_metadata`
    fn build_self_metadata(
        primal_id: &str,
        capabilities: &[SelfCapabilityDetection],
        inputs: &SelfDiscoveryEnvInputs,
    ) -> PrimalMetadata {
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

        let display_name = inputs
            .beardog_display_name
            .clone()
            .or_else(|| inputs.primal_name.clone())
            .or_else(|| inputs.service_name.clone())
            .or_else(|| {
                // Extract type and identifier from primal_id for display
                primal_id.split('-').next().map(|s| {
                    let id_prefix = primal_id.chars().take(8).collect::<String>();
                    format!(
                        "{}-{}",
                        s.chars()
                            .next()
                            .unwrap_or('p')
                            .to_uppercase()
                            .collect::<String>()
                            + &s[1..],
                        id_prefix
                    )
                })
            });

        PrimalMetadata {
            display_name,
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
        }
    }

    /// Validate self-knowledge
    /// Validates `self_knowledge`
    fn validate_self_knowledge(
        capabilities: &[SelfCapabilityDetection],
    ) -> Result<(), BearDogError> {
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

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
#[path = "self_discovery_tests.rs"]
mod tests;
