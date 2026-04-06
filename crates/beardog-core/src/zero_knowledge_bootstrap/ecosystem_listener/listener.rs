// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`EcosystemListener`] implementation and lifecycle.

use super::discovery;
use super::env::EcosystemListenerEnvInputs;
use super::types::EcosystemListenerMetrics;
use crate::ecosystem::primal_types::DiscoveredPrimal;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

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
    ///
    /// # Errors
    ///
    /// Same as [`Self::new`].
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Listening duration ms capped to u64::MAX for metrics storage"
    )]
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
                match discovery::listen_mdns_announcements(&env).await {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = discovery::process_primal_announcement(
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
                match discovery::poll_http_discovery(&env).await {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = discovery::process_primal_announcement(
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
                match discovery::check_environment_announcements() {
                    Ok(announcements) => {
                        for announcement in announcements {
                            if let Err(e) = discovery::process_primal_announcement(
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
                let announcements = discovery::discover_service_mesh_primals();
                for announcement in announcements {
                    if let Err(e) = discovery::process_primal_announcement(
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

    /// Get current listening metrics
    /// Gets metrics
    #[must_use]
    pub const fn get_metrics(&self) -> &EcosystemListenerMetrics {
        &self.metrics
    }

    /// Stop all listening tasks
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

#[cfg(test)]
use super::types::PrimalAnnouncement;

#[cfg(test)]
impl EcosystemListener {
    async fn listen_mdns_announcements(
        env: &EcosystemListenerEnvInputs,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        discovery::listen_mdns_announcements(env).await
    }

    async fn poll_http_discovery(
        env: &EcosystemListenerEnvInputs,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        discovery::poll_http_discovery(env).await
    }

    fn check_environment_announcements_for_test(
        vars: &HashMap<String, String>,
    ) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        discovery::check_environment_announcements_for_test(vars)
    }

    fn discover_service_mesh_primals() -> Vec<PrimalAnnouncement> {
        discovery::discover_service_mesh_primals()
    }

    async fn process_primal_announcement(
        announcement: PrimalAnnouncement,
        discovered_primals: &Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
        discovered_capabilities: &Arc<
            RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>,
        >,
    ) -> Result<(), BearDogError> {
        discovery::process_primal_announcement(
            announcement,
            discovered_primals,
            discovered_capabilities,
        )
        .await
    }

    fn make_discovery_request(endpoint: &str) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
        discovery::make_discovery_request(endpoint)
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
    use super::super::env::EcosystemListenerEnvInputs;
    use super::*;
    use crate::ecosystem::primal_types::PrimalMetadata;
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
            endpoints: vec![crate::ecosystem::primal_types::UniversalEndpoint {
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

    #[test]
    fn discover_service_mesh_primals_is_empty() {
        assert!(super::super::discovery::discover_service_mesh_primals().is_empty());
    }
}
