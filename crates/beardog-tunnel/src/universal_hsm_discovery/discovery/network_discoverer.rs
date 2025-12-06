//! Network HSM Discoverer
//!
//! Provides discovery functionality for network-accessible HSMs
//!
//! This module discovers HSMs that are accessible over the network, including:
//! - Network HSMs with HTTP/HTTPS APIs
//! - Hardware HSMs with network interfaces
//! - Cloud HSM endpoints (when configured)
//! - mDNS/DNS-SD advertised services
//! - Configured static endpoints

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tracing::{debug, info, warn};

/// Network HSM discoverer configuration
#[derive(Debug, Clone)]
pub struct NetworkDiscoveryConfig {
    /// Timeout for network probes
    pub probe_timeout: Duration,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable static endpoint scanning
    pub enable_static_scan: bool,
    /// List of known HSM endpoints to probe
    pub known_endpoints: Vec<String>,
    /// Port ranges to scan
    pub scan_ports: Vec<u16>,
}

impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        use std::env;
        
        // Load from environment with fallback to defaults
        let probe_timeout_secs = env::var("BEARDOG_HSM_PROBE_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);
        
        let enable_mdns = env::var("BEARDOG_HSM_ENABLE_MDNS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(true);
        
        let enable_static_scan = env::var("BEARDOG_HSM_ENABLE_STATIC_SCAN")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(false); // Changed to false for security (was true)
        
        // Load known endpoints from env (comma-separated)
        let known_endpoints = env::var("BEARDOG_HSM_KNOWN_ENDPOINTS")
            .ok()
            .map(|s| s.split(',').map(|e| e.trim().to_string()).collect())
            .unwrap_or_default();
        
        // ✅ MIGRATED: Load scan ports from env (comma-separated) or use defaults
        // Common HSM ports: HTTPS (443), Alt HTTPS (8443), custom HSM ports (9000, 9443)
        let scan_ports = env::var("BEARDOG_HSM_SCAN_PORTS")
            .ok()
            .and_then(|s| {
                s.split(',')
                    .map(|p| p.trim().parse().ok())
                    .collect::<Option<Vec<u16>>>()
            })
            .unwrap_or_else(|| {
                use beardog_config::global::BEARDOG_CONFIG;
                vec![
                    443,  // Standard HTTPS (industry standard)
                    8443, // Alt HTTPS (industry standard)
                    9000, // Custom HSM port (configurable via BEARDOG_HSM_SCAN_PORTS)
                    BEARDOG_CONFIG.network.discovery.port, // Discovery port from config
                ]
            });
        
        Self {
            probe_timeout: Duration::from_secs(probe_timeout_secs),
            enable_mdns,
            enable_static_scan,
            known_endpoints,
            scan_ports,
        }
    }
}

/// Network HSM discoverer
#[derive(Debug, Clone)]
pub struct NetworkDiscoverer {
    /// Discovery configuration
    config: NetworkDiscoveryConfig,
}

impl NetworkDiscoverer {
    /// Create new network discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            config: NetworkDiscoveryConfig::default(),
        })
    }

    /// Create new network discoverer with custom configuration
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn with_config(config: NetworkDiscoveryConfig) -> Result<Self, BearDogError> {
        Ok(Self { config })
    }

    /// Discover network HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering network-accessible HSMs");
        
        let mut discovered = Vec::new();

        // Discover known endpoints
        if !self.config.known_endpoints.is_empty() {
            debug!("Probing {} known endpoints", self.config.known_endpoints.len());
            discovered.extend(self.discover_known_endpoints().await?);
        }

        // Discover via mDNS/DNS-SD
        if self.config.enable_mdns {
            debug!("Performing mDNS discovery");
            discovered.extend(self.discover_via_mdns().await?);
        }

        // Discover via static network scanning (if enabled)
        if self.config.enable_static_scan {
            debug!("Performing static network scan");
            discovered.extend(self.discover_via_static_scan().await?);
        }

        info!("✅ Network discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Discover HSMs at known endpoints
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_known_endpoints(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let mut discovered = Vec::new();

        for endpoint in &self.config.known_endpoints {
            debug!("Probing known endpoint: {}", endpoint);
            
            if let Some(hsm) = self.probe_endpoint(endpoint).await? {
                info!("✓ Found HSM at {}", endpoint);
                discovered.push(hsm);
            }
        }

        Ok(discovered)
    }

    /// Discover HSMs via mDNS/DNS-SD
    ///
    /// # Architecture Note
    /// Per "Discover, Don't Implement" principle, this delegates to any primal
    /// with `ServiceDiscovery` capability for network protocol implementation
    /// (mDNS, DNS-SD, Consul, etcd, etc.).
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_via_mdns(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Delegating mDNS/DNS-SD discovery to ecosystem via capability discovery");

        // ✅ ARCHITECTURAL PATTERN: Capability-based discovery
        //
        // Instead of implementing mDNS directly, we use UniversalPrimalAdapter
        // which handles all network protocols (mDNS, DNS-SD, Consul, etcd, etc.)
        // through capability-based discovery (no hardcoded primal names).
        //
        // Benefits:
        // - No protocol duplication
        // - Universal adapter handles network complexity
        // - BearDog focuses on HSM abstraction
        // - Capability-based (discovers ANY primal with ServiceDiscovery capability)
        // - Primal-agnostic: works with any ecosystem member providing this capability
        //
        // See: crates/beardog-adapters/src/universal/primal_capability_adapter.rs
        //
        // PHASE-2: Wire to UniversalPrimalAdapter::discover_network_primals()
        // For now, return empty to maintain interface compatibility

        debug!("mDNS discovery will be available via UniversalPrimalAdapter");
        Ok(Vec::new())
    }

    /// Discover HSMs via static network scanning
    ///
    /// # Security Note
    /// Network scanning is security-sensitive and must respect network policies.
    /// This is opt-in only and should be explicitly enabled by configuration.
    ///
    /// # Errors
    /// Returns an error if network scan fails
    async fn discover_via_static_scan(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Static network scan requested");

        // ⚠️ SECURITY: Network scanning disabled by default
        //
        // Reasons:
        // 1. Security: Port scanning can be flagged as hostile by IDS/IPS
        // 2. Ethics: Network scanning without permission is invasive
        // 3. Performance: Can cause network congestion
        // 4. Reliability: Blocked by many firewalls
        //
        // Alternatives:
        // - Use known endpoints (self.discover_known_endpoints())
        // - Use mDNS/DNS-SD (service advertisement, not scanning)
        // - Use ecosystem service registry via capability discovery
        //
        // If network scanning is required, it should:
        // - Be explicitly enabled in config
        // - Respect network policy constraints
        // - Implement proper rate limiting
        // - Log all scan activity for audit
        
        warn!("Static network scanning disabled by default for security/ethics compliance");
        warn!("Use known endpoints or mDNS discovery instead");
        
        Ok(Vec::new())
    }

    /// Probe a specific endpoint for HSM capabilities
    ///
    /// # Errors
    /// Returns an error if probe fails
    async fn probe_endpoint(&self, endpoint: &str) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing endpoint: {}", endpoint);

        // Parse endpoint
        let (host, port, protocol) = self.parse_endpoint(endpoint)?;

        // Build full URL for health check
        let health_url = format!("{}://{}:{}/health", protocol, host, port);
        let info_url = format!("{}://{}:{}/info", protocol, host, port);

        // Attempt to connect to the endpoint with timeout
        let client = reqwest::Client::builder()
            .timeout(self.config.probe_timeout)
            .danger_accept_invalid_certs(false) // Security: validate certs
            .build()
            .map_err(|e| BearDogError::internal(format!("Failed to create HTTP client: {e}")))?;

        // Try health endpoint first
        match tokio::time::timeout(
            self.config.probe_timeout,
            client.get(&health_url).send()
        ).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    debug!("✓ Health check successful for {}", endpoint);
                    
                    // Try to get detailed info from /info endpoint
                    match tokio::time::timeout(
                        self.config.probe_timeout,
                        client.get(&info_url).send()
                    ).await {
                        Ok(Ok(info_response)) if info_response.status().is_success() => {
                            // Try to parse HSM info from response
                            if let Ok(text) = info_response.text().await {
                                debug!("HSM info response: {}", text);
                                // Could parse JSON here for detailed capabilities
                            }
                        }
                        _ => {
                            debug!("Info endpoint not available for {}", endpoint);
                        }
                    }
                    
                    return Ok(Some(self.create_network_hsm(endpoint.to_string(), host, port, protocol)));
                } else {
                    debug!("Health check failed with status: {}", response.status());
                }
            }
            Ok(Err(e)) => {
                debug!("Connection failed to {}: {}", endpoint, e);
            }
            Err(_) => {
                debug!("Connection timeout to {}", endpoint);
            }
        }

        // If health check failed, try a simple TCP connection test
        match tokio::time::timeout(
            self.config.probe_timeout,
            tokio::net::TcpStream::connect(format!("{}:{}", host, port))
        ).await {
            Ok(Ok(_stream)) => {
                info!("✓ TCP connection successful to {} (no HTTP health endpoint)", endpoint);
                // Endpoint is reachable but doesn't have standard health endpoint
                // Still create HSM entry if it's in known endpoints
                if self.is_endpoint_configured(endpoint) {
                    return Ok(Some(self.create_network_hsm(endpoint.to_string(), host, port, protocol)));
                }
            }
            Ok(Err(e)) => {
                debug!("TCP connection failed to {}: {}", endpoint, e);
            }
            Err(_) => {
                debug!("TCP connection timeout to {}", endpoint);
            }
        }

        Ok(None)
    }

    /// Parse endpoint string into components
    ///
    /// # Errors
    /// Returns an error if endpoint format is invalid
    fn parse_endpoint(&self, endpoint: &str) -> Result<(String, u16, String), BearDogError> {
        // Simple parsing - in production would use url crate
        if endpoint.starts_with("https://") {
            let host = endpoint.strip_prefix("https://")
                .and_then(|s| s.split(':').next())
                .unwrap_or(endpoint);
            let port = endpoint.split(':').nth(2)
                .and_then(|p| p.parse().ok())
                .unwrap_or(443);
            Ok((host.to_string(), port, "https".to_string()))
        } else if endpoint.starts_with("http://") {
            let host = endpoint.strip_prefix("http://")
                .and_then(|s| s.split(':').next())
                .unwrap_or(endpoint);
            let port = endpoint.split(':').nth(2)
                .and_then(|p| p.parse().ok())
                .unwrap_or(80);
            Ok((host.to_string(), port, "http".to_string()))
        } else {
            // Assume hostname:port or just hostname
            let parts: Vec<&str> = endpoint.split(':').collect();
            let host = parts[0].to_string();
            let port = parts.get(1)
                .and_then(|p| p.parse().ok())
                .unwrap_or(443);
            Ok((host, port, "https".to_string()))
        }
    }

    /// Check if endpoint is in known configuration
    fn is_endpoint_configured(&self, endpoint: &str) -> bool {
        self.config.known_endpoints.iter().any(|e| e == endpoint)
    }

    /// Create a DiscoveredHsm for a network endpoint
    fn create_network_hsm(&self, name: String, host: String, port: u16, protocol: String) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("network-hsm-{}", name.replace("://", "-").replace(':', "-")),
            hsm_type: HsmType::Hardware, // Network HSMs are typically hardware
            endpoint: HsmEndpoint {
                host: host.clone(),
                port: Some(port),
                protocol: protocol.clone(),
                secure: protocol == "https",
            },
            capabilities: self.create_network_hsm_capabilities(),
            assigned_tier: HsmTier::Tier1, // Network HSMs are typically high-grade (Tier 1)
            supports_human_entropy: false, // Network HSMs typically don't support human entropy
            health_status: HsmHealthStatus::Healthy, // Assume healthy until probed
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create capabilities for network HSM
    fn create_network_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec![
                    "AES-128".to_string(),
                    "AES-192".to_string(),
                    "AES-256".to_string(),
                    "3DES".to_string(),
                ],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-3072".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "RSA-PKCS1".to_string(),
                    "ECDSA".to_string(),
                ],
                hashing: vec![
                    "SHA-1".to_string(),
                    "SHA-256".to_string(),
                    "SHA-384".to_string(),
                    "SHA-512".to_string(),
                ],
                key_agreement: vec!["ECDH".to_string(), "DH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384, 521],
                aes: vec![128, 192, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(3), // Network HSMs typically FIPS 140-2 Level 3
                common_criteria_eal: Some(4),
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 50000, // Network HSMs are high-performance
                typical_latency_ms: 20.0, // Network latency included
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: true,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: true,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false, // Most current HSMs don't support this yet
                multi_party_computation: true,
                threshold_cryptography: true,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Add known endpoint to configuration
    pub fn add_known_endpoint(&mut self, endpoint: String) {
        if !self.config.known_endpoints.contains(&endpoint) {
            self.config.known_endpoints.push(endpoint);
        }
    }

    /// Get current configuration
    pub fn config(&self) -> &NetworkDiscoveryConfig {
        &self.config
    }
}

impl Default for NetworkDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            config: NetworkDiscoveryConfig::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = NetworkDiscoverer::new();
        assert!(discoverer.is_ok());
    }

    #[test]
    fn test_discoverer_with_config() {
        let config = NetworkDiscoveryConfig {
            probe_timeout: Duration::from_secs(10),
            enable_mdns: false,
            enable_static_scan: false,
            known_endpoints: vec!["https://hsm.example.com".to_string()],
            scan_ports: vec![443],
        };
        
        let discoverer = NetworkDiscoverer::with_config(config);
        assert!(discoverer.is_ok());
        
        let disc = discoverer?;
        assert_eq!(disc.config.known_endpoints.len(), 1);
        assert!(!disc.config.enable_mdns);
    }

    #[tokio::test]
    async fn test_network_discovery_empty() {
        let discoverer = NetworkDiscoverer::new()?;
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        let hsms = result?;
        // With no configured endpoints, should find nothing
        assert_eq!(hsms.len(), 0);
    }

    #[tokio::test]
    async fn test_network_discovery_with_endpoints() {
        let config = NetworkDiscoveryConfig {
            known_endpoints: vec![
                "https://hsm1.example.com".to_string(),
                "https://hsm2.example.com:8443".to_string(),
            ],
            enable_mdns: false,
            enable_static_scan: false,
            ..Default::default()
        };
        
        let discoverer = NetworkDiscoverer::with_config(config)?;
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        let hsms = result?;
        assert_eq!(hsms.len(), 2);
    }

    #[test]
    fn test_endpoint_parsing() {
        let discoverer = NetworkDiscoverer::new()?;
        
        // Test HTTPS URL
        let (host, port, protocol) = discoverer.parse_endpoint("https://hsm.example.com")?;
        assert_eq!(host, "hsm.example.com");
        assert_eq!(port, 443);
        assert_eq!(protocol, "https");
        
        // Test HTTPS with port
        let (host, port, protocol) = discoverer.parse_endpoint("https://hsm.example.com:8443")?;
        assert_eq!(host, "hsm.example.com");
        assert_eq!(port, 8443);
        assert_eq!(protocol, "https");
        
        // Test hostname only
        let (host, port, protocol) = discoverer.parse_endpoint("hsm.example.com")?;
        assert_eq!(host, "hsm.example.com");
        assert_eq!(port, 443);
        assert_eq!(protocol, "https");
        
        // Test hostname with port
        let (host, port, protocol) = discoverer.parse_endpoint("hsm.example.com:9443")?;
        assert_eq!(host, "hsm.example.com");
        assert_eq!(port, 9443);
        assert_eq!(protocol, "https");
    }

    #[test]
    fn test_network_hsm_capabilities() {
        let discoverer = NetworkDiscoverer::new()?;
        let caps = discoverer.create_network_hsm_capabilities();
        
        // Verify network HSM capabilities
        assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
        assert_eq!(caps.security.fips_140_2_level, Some(3));
        assert!(caps.compliance.fips_140_2);
        assert!(caps.compliance.pci_dss);
        assert!(caps.api_support.pkcs11);
        assert!(caps.api_support.kmip);
        assert!(caps.performance.max_operations_per_second >= 50000);
        assert!(caps.advanced_features.multi_party_computation);
    }

    #[test]
    fn test_add_known_endpoint() {
        let mut discoverer = NetworkDiscoverer::new()?;
        assert_eq!(discoverer.config().known_endpoints.len(), 0);
        
        discoverer.add_known_endpoint("https://hsm1.example.com".to_string());
        assert_eq!(discoverer.config().known_endpoints.len(), 1);
        
        // Adding duplicate should not increase count
        discoverer.add_known_endpoint("https://hsm1.example.com".to_string());
        assert_eq!(discoverer.config().known_endpoints.len(), 1);
        
        discoverer.add_known_endpoint("https://hsm2.example.com".to_string());
        assert_eq!(discoverer.config().known_endpoints.len(), 2);
    }
}



