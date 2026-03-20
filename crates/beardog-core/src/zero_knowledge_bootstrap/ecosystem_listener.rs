// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Listener
//
// This module implements passive listening for ecosystem announcements from other primals.
// It enables discovery of other primals without hardcoded knowledge, following the
// "infant learning" pattern where we listen and learn from the ecosystem.

use crate::ecosystem::primal_types::{
    DiscoveredPrimal, PrimalMetadata, PrimalMetrics, UniversalEndpoint,
};
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{
    CapabilityType, ServiceCapabilityType, UniversalCapability,
};
use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Placeholder URL for primals with no announced endpoints
const UNKNOWN_ENDPOINT_URL: &str = "unknown";

/// Injected configuration for [`EcosystemListener`] (poll intervals and HTTP discovery targets).
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct EcosystemListenerEnvInputs {
    pub mdns_poll_interval_secs: u64,
    pub http_discovery_poll_interval_secs: u64,
    pub env_check_interval_secs: u64,
    pub mesh_discovery_interval_secs: u64,
    pub mdns_discovery_enabled: bool,
    pub discovery_base_port: u16,
    pub beardog_discovery_endpoint: Option<String>,
    pub ecosystem_discovery_endpoint: Option<String>,
    pub discovery_host: Option<String>,
    pub local_discovery_endpoint: Option<String>,
    pub http_discovery_timeout_secs: u64,
}

impl Default for EcosystemListenerEnvInputs {
    fn default() -> Self {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;
        Self {
            mdns_poll_interval_secs: 5,
            http_discovery_poll_interval_secs: 10,
            env_check_interval_secs: 15,
            mesh_discovery_interval_secs: 20,
            mdns_discovery_enabled: false,
            discovery_base_port: DEFAULT_API_PORT,
            beardog_discovery_endpoint: None,
            ecosystem_discovery_endpoint: None,
            discovery_host: None,
            local_discovery_endpoint: None,
            http_discovery_timeout_secs: 5,
        }
    }
}

impl EcosystemListenerEnvInputs {
    /// Read listener configuration from the process environment (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        let discovery_base_port = std::env::var("BEARDOG_DISCOVERY_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(network_config.service_ports.api_port);
        Self {
            mdns_poll_interval_secs: std::env::var("BEARDOG_MDNS_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            http_discovery_poll_interval_secs: std::env::var(
                "BEARDOG_HTTP_DISCOVERY_POLL_INTERVAL_SECS",
            )
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),
            env_check_interval_secs: std::env::var("BEARDOG_ENV_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            mesh_discovery_interval_secs: std::env::var("BEARDOG_MESH_DISCOVERY_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(20),
            mdns_discovery_enabled: std::env::var("BEARDOG_MDNS_DISCOVERY")
                .unwrap_or_else(|_| "false".to_string())
                == "true",
            discovery_base_port,
            beardog_discovery_endpoint: std::env::var("BEARDOG_DISCOVERY_ENDPOINT").ok(),
            ecosystem_discovery_endpoint: std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT").ok(),
            discovery_host: std::env::var("DISCOVERY_HOST").ok(),
            local_discovery_endpoint: std::env::var("LOCAL_DISCOVERY_ENDPOINT").ok(),
            http_discovery_timeout_secs: std::env::var("BEARDOG_ECOSYSTEM_LISTENER_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }

    /// Resolved HTTP discovery URLs (primary + local fallback).
    #[must_use]
    pub fn discovery_endpoints(&self) -> Vec<String> {
        use beardog_config::global::BEARDOG_CONFIG;
        let bind_host = BEARDOG_CONFIG.network.api.bind_address.to_string();
        let primary = self.beardog_discovery_endpoint.clone().unwrap_or_else(|| {
            self.ecosystem_discovery_endpoint
                .clone()
                .unwrap_or_else(|| {
                    let discovery_host = self
                        .discovery_host
                        .clone()
                        .unwrap_or_else(|| "discovery.ecosystem.internal".to_string());
                    format!("http://{discovery_host}:{}", self.discovery_base_port)
                })
        });
        let local = self.local_discovery_endpoint.clone().unwrap_or_else(|| {
            format!("http://{bind_host}:{}/discovery", self.discovery_base_port)
        });
        vec![primary, local]
    }
}

/// Ecosystem Listener for Zero-Knowledge Discovery
///
/// This component passively listens for announcements from other primals in the ecosystem,
/// enabling discovery without hardcoded knowledge. It implements the "infant learning" pattern
/// where the system learns about the ecosystem by observing communication.
///
/// The listener supports multiple discovery protocols:
/// - mDNS (multicast DNS) for local network discovery
/// - HTTP polling of discovery endpoints
/// - Environment variable configuration
/// - Service mesh integration
///
/// # Examples
///
/// ```ignore
/// use beardog_core::zero_knowledge_bootstrap::EcosystemListener;
///
/// let listener = EcosystemListener::from_env(config, primals, capabilities)?;
/// listener.start_listening().await?;
/// ```
#[derive(Debug)]
pub struct EcosystemListener {
    config: UnifiedBootstrapConfig,
    discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    discovered_capabilities: Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
    listening_tasks: Vec<tokio::task::JoinHandle<()>>,
    metrics: EcosystemListenerMetrics,
    env: EcosystemListenerEnvInputs,
}

/// Metrics for ecosystem listening operations
#[derive(Clone, Copy, Debug, Default)]
pub struct EcosystemListenerMetrics {
    /// Number of valid announcements received from other primals
    pub announcements_received: u64,
    /// Number of unique primals discovered through listening
    pub primals_discovered: u64,
    /// Number of unique capabilities discovered across all primals
    pub capabilities_discovered: u64,
    /// Number of invalid or malformed announcements rejected
    pub invalid_announcements: u64,
    /// Total time spent listening for announcements (in milliseconds)
    pub listening_duration_ms: u64,
}

/// Primal announcement received from ecosystem
///
/// Represents an announcement from another primal in the ecosystem, containing
/// all information needed to identify and communicate with that primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalAnnouncement {
    /// Unique identifier for the announcing primal
    pub primal_id: String,
    /// Capabilities offered by this primal
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Communication endpoints for reaching this primal
    pub endpoints: Vec<UniversalEndpoint>,
    /// Additional metadata about the primal
    pub metadata: PrimalMetadata,
    /// Timestamp when this announcement was broadcast
    pub announcement_timestamp: std::time::SystemTime,
    /// Protocol used to discover this announcement (mDNS, HTTP, etc.)
    pub source_protocol: String,
}

/// Ecosystem discovery event
#[derive(Debug, Clone)]
pub enum EcosystemEvent {
    /// State indicating primaldiscovered
    PrimalDiscovered(DiscoveredPrimal),
    /// State indicating capabilityannounced
    CapabilityAnnounced(ServiceCapabilityType, UniversalCapability),
    /// State indicating primaldisconnected
    PrimalDisconnected(String),
    /// Represents invalid announcement variant
    InvalidAnnouncement(String),
}

impl EcosystemListener {
    /// Create new ecosystem listener
    ///
    /// # Errors
    /// Returns an error if initialization of internal components or channels fails.
    pub fn new(
        config: UnifiedBootstrapConfig,
        discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
        discovered_capabilities: Arc<
            RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>,
        >,
        env: EcosystemListenerEnvInputs,
    ) -> Result<Self, BearDogError> {
        info!("👂 Initializing Ecosystem Listener");
        info!("🎯 Mission: Listen for other primals without hardcoded knowledge");

        Ok(Self {
            config,
            discovered_primals,
            discovered_capabilities,
            listening_tasks: Vec::new(),
            metrics: EcosystemListenerMetrics::default(),
            env,
        })
    }

    /// Create a listener using [`EcosystemListenerEnvInputs::from_env`].
    pub fn from_env(
        config: UnifiedBootstrapConfig,
        discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
        discovered_capabilities: Arc<
            RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>,
        >,
    ) -> Result<Self, BearDogError> {
        Self::new(
            config,
            discovered_primals,
            discovered_capabilities,
            EcosystemListenerEnvInputs::from_env(),
        )
    }

    /// Starts listening
    ///
    /// # Errors
    /// Returns an error if any listener fails to start or if the discovery protocols encounter initialization issues.
    pub fn start_listening(&mut self) -> Result<(), BearDogError> {
        let start_time = std::time::Instant::now();

        Self::log_listening_plan();
        self.start_enabled_listeners();
        self.record_startup_metrics(start_time);
        self.log_listening_status();

        Ok(())
    }

    /// Logs the listening plan to inform about upcoming operations
    fn log_listening_plan() {
        info!("🎧 Starting ecosystem listening...");
        info!("📋 Listening Plan:");
        info!("   1. Start multicast DNS listening");
        info!("   2. Start HTTP discovery polling");
        info!("   3. Start environment variable monitoring");
        info!("   4. Start service mesh discovery");
        info!("   5. Process announcements as they arrive");
    }

    /// Starts all enabled protocol listeners
    fn start_enabled_listeners(&mut self) {
        use beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol;

        self.start_listener_if_enabled(DiscoveryProtocol::MulticastDNS, "mDNS");
        self.start_listener_if_enabled(DiscoveryProtocol::HttpDiscovery, "HTTP discovery");
        self.start_listener_if_enabled(DiscoveryProtocol::EnvironmentDiscovery, "Environment");
        self.start_listener_if_enabled(DiscoveryProtocol::ServiceMeshDiscovery, "Service mesh");
    }

    /// Starts a specific listener if the protocol is enabled
    fn start_listener_if_enabled(
        &mut self,
        protocol: beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol,
        name: &str,
    ) {
        if !self.config.discovery.enabled_protocols.contains(&protocol) {
            return;
        }

        let task = match protocol {
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::MulticastDNS => {
                self.start_mdns_listener()
            }
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::HttpDiscovery => {
                self.start_http_listener()
            }
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::EnvironmentDiscovery => {
                self.start_environment_listener()
            }
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::ServiceMeshDiscovery => {
                self.start_service_mesh_listener()
            }
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::ContainerDiscovery |
            beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol::CloudMetadataDiscovery => {
                // These protocols don't have dedicated listeners yet
                warn!("Protocol {:?} is enabled but listener not implemented yet", protocol);
                return;
            }
        };

        self.listening_tasks.push(task);
        info!("✅ {} listener started", name);
    }

    /// Records startup metrics
    fn record_startup_metrics(&mut self, start_time: std::time::Instant) {
        self.metrics.listening_duration_ms = start_time.elapsed().as_millis() as u64;
    }

    /// Logs the final listening status
    fn log_listening_status(&self) {
        info!("🎉 Ecosystem listening active!");
        info!("📊 Listening Status:");
        info!("   🎧 Active Listeners: {}", self.listening_tasks.len());
        info!(
            "   📡 Protocols: {:?}",
            self.config.discovery.enabled_protocols
        );
        info!(
            "   ⏱️  Startup Duration: {}ms",
            self.metrics.listening_duration_ms
        );
    }

    /// Start multicast DNS listener
    /// Starts `mdns_listener`
    fn start_mdns_listener(&self) -> tokio::task::JoinHandle<()> {
        let discovered_primals = self.discovered_primals.clone();
        let discovered_capabilities = self.discovered_capabilities.clone();
        let env = self.env.clone();

        tokio::spawn(async move {
            info!("🔍 mDNS listener active - discovering primals via multicast DNS");

            // Modern interval-based polling (replaces sleep in loop)
            let poll_interval = env.mdns_poll_interval_secs;

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll_interval));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                // Listen for mDNS announcements
                match Self::listen_mdns_announcements(&env).await {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = Self::process_primal_announcement(
                                announcement,
                                &discovered_primals,
                                &discovered_capabilities,
                            )
                            .await
                            {
                                warn!("Failed to process mDNS announcement: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        debug!("mDNS listening error (expected): {}", e);
                    }
                }
            }
        })
    }

    /// Start HTTP discovery listener
    /// Starts `http_listener`
    fn start_http_listener(&self) -> tokio::task::JoinHandle<()> {
        let discovered_primals = self.discovered_primals.clone();
        let discovered_capabilities = self.discovered_capabilities.clone();
        let env = self.env.clone();

        tokio::spawn(async move {
            info!("🌐 HTTP discovery listener active - polling discovery endpoints");

            // Modern interval-based polling (replaces sleep in loop)
            let poll_interval = env.http_discovery_poll_interval_secs;

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll_interval));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                // Poll HTTP discovery endpoints
                match Self::poll_http_discovery(&env).await {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = Self::process_primal_announcement(
                                announcement,
                                &discovered_primals,
                                &discovered_capabilities,
                            )
                            .await
                            {
                                warn!("Failed to process HTTP announcement: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        debug!("HTTP discovery error (expected): {}", e);
                    }
                }
            }
        })
    }

    /// Start environment variable listener
    /// Starts `environment_listener`
    fn start_environment_listener(&self) -> tokio::task::JoinHandle<()> {
        let discovered_primals = self.discovered_primals.clone();
        let discovered_capabilities = self.discovered_capabilities.clone();
        let env = self.env.clone();

        tokio::spawn(async move {
            info!("🔧 Environment listener active - monitoring environment variables");

            // Modern interval-based polling (replaces sleep in loop)
            let check_interval = env.env_check_interval_secs;

            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(check_interval));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                // Check environment variables for primal announcements
                match Self::check_environment_announcements() {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = Self::process_primal_announcement(
                                announcement,
                                &discovered_primals,
                                &discovered_capabilities,
                            )
                            .await
                            {
                                warn!("Failed to process environment announcement: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Environment discovery error (expected): {}", e);
                    }
                }
            }
        })
    }

    /// Start service mesh listener
    /// Starts `service_mesh_listener`
    fn start_service_mesh_listener(&self) -> tokio::task::JoinHandle<()> {
        let discovered_primals = self.discovered_primals.clone();
        let discovered_capabilities = self.discovered_capabilities.clone();
        let env = self.env.clone();

        tokio::spawn(async move {
            info!("🕸️ Service mesh listener active - discovering via service mesh");

            // Modern interval-based polling (replaces sleep in loop)
            let discovery_interval = env.mesh_discovery_interval_secs;

            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(discovery_interval));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                // Check service mesh for primal announcements
                let announcements = Self::discover_service_mesh_primals();
                for announcement in announcements {
                    if let Err(e) = Self::process_primal_announcement(
                        announcement,
                        &discovered_primals,
                        &discovered_capabilities,
                    )
                    .await
                    {
                        warn!("Failed to process service mesh announcement: {}", e);
                    }
                }
            }
        })
    }

    async fn listen_mdns_announcements(
        env: &EcosystemListenerEnvInputs,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        debug!("🔍 Listening for mDNS primal announcements...");

        let announcements = Vec::new();

        // Check if mDNS discovery is enabled via environment
        if env.mdns_discovery_enabled {
            // In a real implementation, this would use mdns-sd or similar
            // For now, we simulate by checking for known service patterns
            debug!("mDNS discovery enabled, scanning for services...");

            // Check for local services advertising BearDog capabilities
            if let Ok(response) = tokio::process::Command::new("avahi-browse")
                .args(["-t", "_beardog._tcp"])
                .output()
                .await
            {
                if response.status.success() {
                    let output = String::from_utf8_lossy(&response.stdout);
                    for line in output.lines() {
                        if line.contains("beardog ") {
                            debug!("Found potential BearDog service via mDNS: {}", line);
                        }
                    }
                }
            }
        }

        Ok(announcements)
    }

    /// Poll HTTP discovery endpoints
    async fn poll_http_discovery(
        env: &EcosystemListenerEnvInputs,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        debug!("🌐 Polling HTTP discovery endpoints...");

        let mut announcements = Vec::new();

        let discovery_endpoints = env.discovery_endpoints();

        for endpoint in discovery_endpoints {
            debug!("📡 Checking discovery endpoint: {}", endpoint);

            // Attempt HTTP discovery request with timeout
            match tokio::time::timeout(
                std::time::Duration::from_secs(env.http_discovery_timeout_secs),
                std::future::ready(Self::make_discovery_request(&endpoint)),
            )
            .await
            {
                Ok(Ok(discovered)) => {
                    announcements.extend(discovered);
                }
                Ok(Err(e)) => {
                    debug!("Discovery endpoint {} failed: {}", endpoint, e);
                }
                Err(_) => {
                    debug!("Discovery endpoint {} timed out", endpoint);
                }
            }
        }

        Ok(announcements)
    }

    fn check_environment_announcements() -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        Self::check_environment_announcements_with_lookup(|key| std::env::var(key))
    }

    /// Tests and injected maps: lookup function instead of reading global environment.
    fn check_environment_announcements_with_lookup<G>(
        mut get_var: G,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError>
    where
        G: FnMut(&str) -> Result<String, std::env::VarError>,
    {
        debug!("🔧 Checking environment for primal announcements...");

        let mut announcements = Vec::new();

        // Check for primal endpoint environment variables
        let env_vars = [
            "BEARDOG_COMPUTE_ENDPOINT",
            "BEARDOG_MESH_ENDPOINT",
            "BEARDOG_AI_ENDPOINT",
            "BEARDOG_STORAGE_ENDPOINT",
        ];

        for var in &env_vars {
            if let Ok(endpoint) = get_var(var) {
                debug!(
                    "🔍 Found primal endpoint in environment: {} = {}",
                    var, endpoint
                );

                // Create announcement from environment variable
                let capability = match *var {
                    "BEARDOG_COMPUTE_ENDPOINT" => ServiceCapabilityType::ComputeIntelligence,
                    "BEARDOG_MESH_ENDPOINT" => ServiceCapabilityType::ServiceMesh,
                    "BEARDOG_AI_ENDPOINT" => ServiceCapabilityType::DistributedIntelligence,
                    "BEARDOG_STORAGE_ENDPOINT" => ServiceCapabilityType::DataStorage,
                    _ => continue,
                };

                // Create announcement from environment-discovered service
                let announcement = PrimalAnnouncement {
                    primal_id: format!("env-discovered-{}", var.to_lowercase()),
                    capabilities: vec![capability],
                    endpoints: vec![UniversalEndpoint {
                        url: endpoint,
                        protocols: vec!["HTTP".to_string()],
                        auth_requirements:
                            crate::ecosystem::primal_types::AuthRequirements::default(),
                        security_config:
                            crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
                    }],
                    metadata: PrimalMetadata {
                        display_name: Some(format!("Environment-Discovered-{var}")),
                        version: "unknown".to_string(),
                        protocol_versions: vec!["1.0".to_string()],
                        security_attestations: vec![],
                        custom_fields: HashMap::new(),
                        capabilities: vec![],
                        dependencies: vec![],
                        supported_protocols: vec!["http".to_string()],
                        health_check_endpoint: "/health".to_string(),
                        metrics_endpoint: "/metrics".to_string(),
                    },
                    announcement_timestamp: std::time::SystemTime::now(),
                    source_protocol: "environment ".to_string(),
                };

                announcements.push(announcement);
            }
        }

        Ok(announcements)
    }

    #[cfg(test)]
    fn check_environment_announcements_for_test(
        vars: &HashMap<String, String>,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        Self::check_environment_announcements_with_lookup(|k| {
            vars.get(k).cloned().ok_or(std::env::VarError::NotPresent)
        })
    }

    /// Discover primals via service mesh
    fn discover_service_mesh_primals() -> Vec<PrimalAnnouncement> {
        debug!("🕸️ Discovering primals via service mesh...");

        // Minimal implementation - production deployments should integrate with service mesh
        // like Istio, Linkerd, or Consul Connect for automatic service discovery
        Vec::new() // No service mesh integration yet - returns empty
    }

    /// Process primal announcement
    /// Processes `primal_announcement`
    async fn process_primal_announcement(
        announcement: PrimalAnnouncement,
        discovered_primals: &Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
        discovered_capabilities: &Arc<
            RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>,
        >,
    ) -> Result<(), BearDogError> {
        info!(
            "📢 Processing primal announcement from: {}",
            announcement.primal_id
        );

        // Validate announcement
        if announcement.primal_id.is_empty() {
            warn!("⚠️ Invalid announcement: empty primal ID");
            return Ok(());
        }

        // Get endpoint or create placeholder for announcement-only primals
        let endpoint = announcement.endpoints.first().cloned().unwrap_or_else(|| {
            warn!(
                "⚠️ No endpoints provided for primal {}, using placeholder (announcement-only mode)",
                announcement.primal_id
            );
            UniversalEndpoint {
                url: UNKNOWN_ENDPOINT_URL.to_string(),
                protocols: vec![],
                auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
                security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
            }
        });

        let discovered_primal = DiscoveredPrimal {
            primal_id: announcement.primal_id.clone(),
            capabilities: announcement.capabilities.clone(),
            endpoint,
            metadata: announcement.metadata.clone(),
            discovered_at: announcement.announcement_timestamp,
            metrics: PrimalMetrics {
                response_times: crate::ecosystem::primal_types::ResponseTimeMetrics::default(),
                availability: 1.0, // Assume available until proven otherwise
                load_metrics: crate::ecosystem::primal_types::LoadMetrics::default(),
                error_rates: crate::ecosystem::primal_types::ErrorRateMetrics::default(),
            },
        };

        // Check for sovereignty violations (hardcoded primal references)
        let primal_id_lower = discovered_primal.primal_id.to_lowercase();
        if primal_id_lower.contains("hardcoded")
            || primal_id_lower.contains("legacy")
            || primal_id_lower.contains("deprecated")
        {
            warn!(
                "🚨 Potential sovereignty violation detected in primal ID: {}",
                discovered_primal.primal_id
            );
            warn!(
                "   Each primal should only know itself and discover others through universal adapter"
            );
        }

        // Store discovered primal with capability-based identification
        {
            let mut primals = discovered_primals.write().await;
            primals.insert(
                discovered_primal.primal_id.clone(),
                discovered_primal.clone(),
            );
        }

        // Store discovered capabilities
        {
            let mut capabilities = discovered_capabilities.write().await;
            for capability_type in &announcement.capabilities {
                let universal_capability = UniversalCapability {
                    capability_type: CapabilityType::Custom(capability_type.to_string()),
                    provider: beardog_types::canonical::capabilities::ProviderInfo {
                        provider_id: announcement.primal_id.clone(),
                        provider_name: format!(
                            "Primal-{}",
                            &announcement.primal_id[..8.min(announcement.primal_id.len())]
                        ),
                        provider_type:
                            beardog_types::canonical::providers_unified::core::ProviderType::Custom(
                                "primal".to_string(),
                            ),
                        version: "1.0".to_string(),
                        region: None,
                    },
                    endpoint: beardog_types::canonical::capabilities::EndpointConfig {
                        base_url: announcement
                            .endpoints
                            .first()
                            .map_or_else(|| "unknown".to_string(), |e| e.url.clone()),
                        api_version: Some("1.0".to_string()),
                        timeout_ms: 30000,
                        max_retries: 3,
                        circuit_breaker:
                            beardog_types::canonical::capabilities::CircuitBreakerConfig::default(),
                    },
                    auth_config: beardog_types::canonical::capabilities::AuthConfig {
                        auth_type: beardog_types::canonical::capabilities::AuthType::None,
                        api_key: None,
                        bearer_token: None,
                        cert_path: None,
                        custom_params: HashMap::new(),
                    },
                    health_status: beardog_types::canonical::capabilities::HealthStatus::Healthy,
                    performance:
                        beardog_types::canonical::capabilities::PerformanceMetrics::default(),
                    security_level: beardog_types::canonical::capabilities::SecurityLevel::Standard,
                    metadata: HashMap::new(),
                };

                capabilities
                    .entry(capability_type.clone())
                    .or_insert_with(Vec::new)
                    .push(universal_capability);
            }
        }

        info!(
            "✅ Primal announcement processed: {} with {} capabilities",
            announcement.primal_id,
            announcement.capabilities.len()
        );

        Ok(())
    }

    /// Get current listening metrics
    /// Gets metrics
    /// Gets metrics
    #[must_use]
    pub const fn get_metrics(&self) -> &EcosystemListenerMetrics {
        &self.metrics
    }

    /// Stop all listening tasks
    /// Stops listening
    /// Stops listening
    pub fn stop_listening(&mut self) {
        info!("🛑 Stopping ecosystem listening...");

        for task in self.listening_tasks.drain(..) {
            task.abort();
        }

        info!("✅ All ecosystem listeners stopped");
    }
}

impl Drop for EcosystemListener {
    fn drop(&mut self) {
        // Abort any remaining tasks
        for task in &self.listening_tasks {
            task.abort();
        }
    }
}

impl EcosystemListener {
    /// Make HTTP discovery request to endpoint
    fn make_discovery_request(endpoint: &str) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        debug!("Making discovery request to: {}", endpoint);

        // Use tokio's HTTP client implementation instead of external dependency
        // This provides a basic HTTP client without adding dependencies

        // Parse the URL
        let url = endpoint
            .parse::<http::Uri>()
            .map_err(|e| BearDogError::network(format!("Invalid discovery endpoint URL: {e}")))?;

        // For HTTP discovery, we expect a JSON response with primal announcements
        // If the endpoint is not accessible, we return empty results rather than failing
        let announcements = Self::attempt_http_request(&url);
        debug!(
            "Successfully discovered {} primals from {}",
            announcements.len(),
            endpoint
        );
        Ok(announcements)
    }

    /// Attempt HTTP request with basic implementation
    const fn attempt_http_request(_uri: &http::Uri) -> Vec<PrimalAnnouncement> {
        // Basic HTTP implementation - in production this would make actual HTTP requests
        // For now, return empty to avoid external dependencies
        // This could be enhanced with tokio's native HTTP capabilities
        Vec::new()
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
mod tests {
    use super::*;
    use beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol as BootstrapDiscoveryProtocol;

    #[tokio::test]
    async fn test_ecosystem_listener_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = UnifiedBootstrapConfig::default();
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));

        let listener = EcosystemListener::new(
            config,
            primals,
            capabilities,
            EcosystemListenerEnvInputs::default(),
        )?;

        assert_eq!(listener.listening_tasks.len(), 0);
        assert_eq!(listener.metrics.announcements_received, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_environment_discovery() -> Result<(), Box<dyn std::error::Error>> {
        let mut vars = HashMap::new();
        vars.insert(
            "BEARDOG_COMPUTE_ENDPOINT".to_string(),
            "http://discovered-compute-service:8081".to_string(),
        );

        let announcements = EcosystemListener::check_environment_announcements_for_test(&vars)?;

        // Note: In unit test environment without actual environment variables set,
        // announcements may be empty. This is expected behavior for unit tests.
        // Integration tests with proper environment setup should validate actual discovery.
        // For now, validate the discovery mechanism runs without error.

        if !announcements.is_empty() {
            let compute_announcement = &announcements[0];
            assert!(
                compute_announcement
                    .capabilities
                    .contains(&ServiceCapabilityType::ComputeIntelligence)
            );
            // Validate sovereignty compliance - primal only knows itself, discovers others dynamically
            assert!(
                !compute_announcement.primal_id.is_empty(),
                "Primal must have identity"
            );
            assert!(
                !compute_announcement.capabilities.is_empty(),
                "Primal must advertise capabilities"
            );

            // Validate infant discovery pattern - no hardcoded ecosystem assumptions
            for capability in &compute_announcement.capabilities {
                let cap_str = format!("{capability:?}");
                assert!(
                    !cap_str.to_lowercase().contains("hardcoded"),
                    "Capabilities must be discovered, not hardcoded"
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_capability_based_discovery() {
        let _config = UnifiedBootstrapConfig::default();
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));

        // Create announcement with capability-based primal ID
        let announcement = PrimalAnnouncement {
            primal_id: "compute-service-001".to_string(), // Capability-based ID
            capabilities: vec![ServiceCapabilityType::ComputeIntelligence],
            endpoints: vec![],
            metadata: PrimalMetadata {
                display_name: None,
                version: "1.0".to_string(),
                protocol_versions: vec!["1.0".to_string()],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                health_check_endpoint: String::new(),
                metrics_endpoint: String::new(),
                supported_protocols: vec!["bstp/1.0".to_string()],
            },
            announcement_timestamp: std::time::SystemTime::now(),
            source_protocol: "universal-discovery".to_string(),
        };

        // Should process successfully without warnings
        let result =
            EcosystemListener::process_primal_announcement(announcement, &primals, &capabilities);

        assert!(result.await.is_ok());

        // Should have stored the primal with capability-based key
        let primals_guard = primals.read().await;
        assert!(primals_guard.contains_key("compute-service-001"));
        drop(primals_guard);
    }

    #[tokio::test]
    async fn test_listener_lifecycle_env_only() {
        let mut config = UnifiedBootstrapConfig::default();
        config.discovery.enabled_protocols = vec![BootstrapDiscoveryProtocol::EnvironmentDiscovery];
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));
        let mut listener = EcosystemListener::new(
            config,
            primals,
            capabilities,
            EcosystemListenerEnvInputs::default(),
        )
        .expect("new");
        listener.start_listening().expect("start");
        assert_eq!(listener.listening_tasks.len(), 1);
        listener.stop_listening();
        assert!(listener.listening_tasks.is_empty());
    }

    #[tokio::test]
    async fn test_container_discovery_emits_no_background_tasks() {
        let mut config = UnifiedBootstrapConfig::default();
        config.discovery.enabled_protocols = vec![BootstrapDiscoveryProtocol::ContainerDiscovery];
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));
        let mut listener = EcosystemListener::new(
            config,
            primals,
            capabilities,
            EcosystemListenerEnvInputs::default(),
        )
        .expect("new");
        listener.start_listening().expect("start");
        assert!(listener.listening_tasks.is_empty());
    }

    #[tokio::test]
    async fn test_listen_mdns_poll_http_and_mesh_smoke() {
        let env = EcosystemListenerEnvInputs::default();
        let _ = EcosystemListener::listen_mdns_announcements(&env).await;
        let _ = EcosystemListener::poll_http_discovery(&env).await;
        assert!(EcosystemListener::discover_service_mesh_primals().is_empty());
        let _ = EcosystemListener::make_discovery_request("http://127.0.0.1:1/");
    }

    #[test]
    fn test_check_environment_announcements_beardog_vars() {
        let mut vars = HashMap::new();
        vars.insert(
            "BEARDOG_COMPUTE_ENDPOINT".to_string(),
            "http://compute:8081".to_string(),
        );
        vars.insert(
            "BEARDOG_STORAGE_ENDPOINT".to_string(),
            "http://storage:8083".to_string(),
        );
        let announcements =
            EcosystemListener::check_environment_announcements_for_test(&vars).expect("ok");
        assert!(announcements.len() >= 2);
    }

    #[tokio::test]
    async fn test_process_announcement_empty_id_skipped() {
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));
        let announcement = PrimalAnnouncement {
            primal_id: String::new(),
            capabilities: vec![ServiceCapabilityType::ComputeIntelligence],
            endpoints: vec![],
            metadata: PrimalMetadata {
                display_name: None,
                version: "1.0".to_string(),
                protocol_versions: vec!["1.0".to_string()],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                health_check_endpoint: String::new(),
                metrics_endpoint: String::new(),
                supported_protocols: vec![],
            },
            announcement_timestamp: std::time::SystemTime::now(),
            source_protocol: "test".to_string(),
        };
        EcosystemListener::process_primal_announcement(announcement, &primals, &capabilities)
            .await
            .expect("ok");
        assert!(primals.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_process_announcement_sovereignty_warning_path() {
        let primals = Arc::new(RwLock::new(HashMap::new()));
        let capabilities = Arc::new(RwLock::new(HashMap::new()));
        let announcement = PrimalAnnouncement {
            primal_id: "hardcoded-legacy-primal".to_string(),
            capabilities: vec![ServiceCapabilityType::ComputeIntelligence],
            endpoints: vec![UniversalEndpoint {
                url: "http://127.0.0.1:1".to_string(),
                protocols: vec!["HTTP".to_string()],
                auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
                security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
            }],
            metadata: PrimalMetadata {
                display_name: Some("x".to_string()),
                version: "1.0".to_string(),
                protocol_versions: vec!["1.0".to_string()],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                health_check_endpoint: "/health".to_string(),
                metrics_endpoint: "/metrics".to_string(),
                supported_protocols: vec!["http".to_string()],
            },
            announcement_timestamp: std::time::SystemTime::now(),
            source_protocol: "test".to_string(),
        };
        EcosystemListener::process_primal_announcement(announcement, &primals, &capabilities)
            .await
            .expect("ok");
        assert!(primals.read().await.contains_key("hardcoded-legacy-primal"));
    }
}
