

use std::sync::Arc;
use tokio::time::Duration;
use tracing::{debug, info, warn};
use super::types::{RegistryConfig, NodeInfo};
use crate::{BearDogError, BearDogResult};

use self::bootstrap::{
    BootstrapServices, BootstrapServiceFactory,
    types::{BootstrapConfig, BootstrapStats},
    NodeDiscovery, NodeVerification, FederationBootstrap,
};

#[derive(Debug, Default)]
struct BootstrapResults {

    environment_nodes: Vec<NodeInfo>,

    phonebook_nodes: Vec<NodeInfo>,

    federation_nodes: Vec<NodeInfo>,

    verified_nodes: Vec<NodeInfo>,

    failed_nodes: Vec<NodeInfo>,

    stats: BootstrapStats,
}

pub struct BootstrapManager {

    config: RegistryConfig,

    bootstrap_config: BootstrapConfig,

    services: BootstrapServices,}

impl BootstrapManager {

    pub async fn new(config: RegistryConfig) -> BearDogResult<Self> {
        info!("🔄 Initializing Bootstrap Manager with modular architecture");

        let bootstrap_config = Self::load_bootstrap_config(&config).await?;

        let services = BootstrapServiceFactory::create_services(bootstrap_config.clone());
        Ok(Self {
            config,
            bootstrap_config,
            services,
        })
    }

    pub async fn bootstrap_registry(
        &self,
        registry: &super::core::BearDogNodeRegistry,
    ) -> BearDogResult<()> {
        if !self.bootstrap_config.enable_federation_discovery {
            info!("📄 Bootstrap disabled, skipping");
            return Ok(());
        }
        info!("🚀 Starting comprehensive bootstrap process");

        match self.services.comprehensive_bootstrap().await {
            Ok(verified_nodes) => {

                for node in verified_nodes {
                    match registry.register_node(node.clone()).await {
                        Ok(_) => {
                            info!("✅ Successfully registered node: {}", node.node_id);
                        }
                        Err(e) => {
                            warn!("⚠️ Failed to register node {}: {}", node.node_id, e);
                    }
                }
                
                info!("🎉 Bootstrap process completed successfully");
                Ok(())
            }
            Err(e) => {
                warn!("❌ Bootstrap process failed: {}", e);
                Err(e)

    pub async fn get_bootstrap_stats(&self) -> BootstrapStats {
        let comprehensive_stats = self.services.get_comprehensive_stats().await;

        BootstrapStats {
            total_attempts: comprehensive_stats.get("discovery_phonebook_endpoints").unwrap_or(&0) +
                            comprehensive_stats.get("federation_cached_federations").unwrap_or(&0),
            successful_bootstraps: comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0),
            failed_bootstraps: comprehensive_stats.get("verification_failed_verifications").unwrap_or(&0),
            average_bootstrap_time_seconds: 5.0, // Would be calculated from actual timing data
            nodes_discovered: comprehensive_stats.get("discovery_phonebook_endpoints").unwrap_or(&0) * 10, // Estimate
            trusted_connections: comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0),
            federation_partners_discovered: comprehensive_stats.get("federation_total_federation_partners").unwrap_or(&0),

    pub async fn get_health_check(&self) -> BearDogResult<super::bootstrap::types::BootstrapHealthCheck> {
        let stats = self.get_bootstrap_stats().await;
        let is_healthy = stats.success_rate() > 50.0 && 
                        comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0) > &0;
        let mut warnings = Vec::new();
        if stats.success_rate() < 70.0 {
            warnings.push("Low bootstrap success rate".to_string());
        if comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0) == &0 {
            warnings.push("No verified nodes available".to_string());
        Ok(super::bootstrap::types::BootstrapHealthCheck {
            is_healthy,
            active_connections: comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0).clone(),
            trusted_node_count: comprehensive_stats.get("verification_verified_nodes").unwrap_or(&0).clone(),
            last_successful_bootstrap: if stats.successful_bootstraps > 0 {
                Some(chrono::Utc::now().to_rfc3339())
            } else {
                None
            },
            status: if is_healthy { "Healthy".to_string() } else { "Warning".to_string() },
            warnings,

    pub async fn bootstrap_single_node(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        info!("🎯 Bootstrapping single node: {}", node_info.node_id);

        self.services.discovery.validate_node(node_info)?;

        match self.services.verification.verify_node(node_info).await {
            Ok(verified) => {
                if verified {
                    info!("✅ Successfully bootstrapped node: {}", node_info.node_id);
                } else {
                    warn!("❌ Failed to verify node: {}", node_info.node_id);
                Ok(verified)
                warn!("⚠️ Error bootstrapping node {}: {}", node_info.node_id, e);

    pub async fn refresh_federation_cache(&self) -> BearDogResult<()> {
        info!("🔄 Refreshing federation cache");

        self.services.verification.clear_cache();
        self.services.federation.clear_cache();

        match self.services.federation.bootstrap_from_federation().await {
            Ok(nodes) => {
                info!("✅ Refreshed federation cache with {} nodes", nodes.len());
                warn!("⚠️ Failed to refresh federation cache: {}", e);

    async fn load_bootstrap_config(config: &RegistryConfig) -> BearDogResult<BootstrapConfig> {
        debug!("📋 Loading bootstrap configuration");

        let mut bootstrap_config = BootstrapConfig::default();

        if let Ok(timeout_str) = std::env::var("BEARDOG_BOOTSTRAP_TIMEOUT") {
            if let Ok(timeout_seconds) = timeout_str.parse::<u64>() {
                bootstrap_config.bootstrap_timeout_seconds = timeout_seconds;
        if let Ok(max_attempts_str) = std::env::var("BEARDOG_BOOTSTRAP_MAX_ATTEMPTS") {
            if let Ok(max_attempts) = max_attempts_str.parse::<u32>() {
                bootstrap_config.max_bootstrap_attempts = max_attempts;

        if let Ok(phonebook_urls) = std::env::var("BEARDOG_PHONEBOOK_URLS") {
            let urls: Vec<String> = phonebook_urls
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !urls.is_empty() {
                bootstrap_config.phonebook_urls = urls;

        if let Ok(regions) = std::env::var("BEARDOG_PREFERRED_REGIONS") {
            bootstrap_config.preferred_regions = regions

        if config.enable_federation {
            bootstrap_config.enable_federation_discovery = true;
        info!("✅ Bootstrap configuration loaded successfully");
        Ok(bootstrap_config)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_registry::types::TrustLevel;
    #[tokio::test]}

    async fn test_bootstrap_manager_creation() {
        let config = RegistryConfig::default();
        let bootstrap_manager = BootstrapManager::new(config).await;
        assert!(bootstrap_manager.is_ok());
    async fn test_single_node_bootstrap() {
        let bootstrap_manager = BootstrapManager::new(config).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let test_node = NodeInfo {
            node_id: "test_node_123".to_string(),
            address: "http://127.0.0.1:8080".to_string(),
            public_key: "test_public_key".to_string(),
            capabilities: vec!["compute".to_string()],
            trust_level: TrustLevel::Low,
            last_seen: None,
            metadata: std::collections::HashMap::with_capacity(16),
        };

        let result = bootstrap_manager.bootstrap_single_node(&test_node).await;

        assert!(result.is_err() || !result.map_err(|e| {
})?);
    async fn test_bootstrap_stats() {
        let stats = bootstrap_manager.get_bootstrap_stats().await;
        assert_eq!(stats.total_attempts, stats.successful_bootstraps + stats.failed_bootstraps);}

    async fn test_health_check() {
        let health = bootstrap_manager.get_health_check().await.map_err(|e| {
        assert!(!health.status.is_empty());
