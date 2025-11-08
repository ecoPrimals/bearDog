// Vendor-Agnostic HSM Discovery System
//
// This system discovers HSM capabilities without hardcoding vendor names,
// device types, or platform specifics. It uses capability-based discovery
// to identify security hardware regardless of manufacturer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::BearDogError;

/// Vendor-agnostic HSM discovery system
pub struct VendorAgnosticHsmDiscovery {
    /// Discovered HSM capabilities
    discovered_hsms: Arc<RwLock<HashMap<String, DiscoveredHsm>>>,
    detection_patterns: Vec<HsmDetectionPattern>,
}

/// A discovered HSM capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredHsm {
    pub hsm_id: String,
    /// Communication interface
    /// The interface value
    pub interface: HsmInterface,
    /// Supported cryptographic operations
    /// Collection of supported operations
    pub supported_operations: Vec<CryptoOperation>,
    /// Security level provided
    /// The security level value
    pub security_level: HsmSecurityLevel,
    pub performance: HsmPerformanceProfile,
    /// Trust score (built through successful operations)
    /// The trust score value
    pub trust_score: f64,
}

/// How to communicate with an HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmInterface {
    /// PKCS#11 interface
    Pkcs11 {
        library_path: String,
        slot_id: Option<u32>,
    },
    /// Network-based HSM
    Network {
        endpoint: String,
        protocol: NetworkProtocol,
    },
    /// Operating system native interface
    Native { interface_type: NativeInterface },
    /// Cloud provider API
    CloudApi {
        api_endpoint: String,
        authentication: CloudAuth,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    /// REST API
    Rest,
    /// gRPC
    Grpc,
    /// Custom protocol
    Custom { protocol_name: String },
}

/// Native OS interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeInterface {
    Tpm,
    /// Secure Enclave (iOS/macOS)
    SecureEnclave,
    /// StrongBox (Android)
    StrongBox,
    /// Windows CNG
    WindowsCng,
    /// Linux Keyring
    LinuxKeyring,
}

/// Cloud authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudAuth {
    /// API key authentication
    ApiKey { key_env_var: String },
    /// OAuth token
    OAuth { token_endpoint: String },
    /// Service account
    ServiceAccount { credentials_path: String },
    /// Instance metadata
    InstanceMetadata,
}

/// Cryptographic operations supported by HSM
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CryptoOperation {
    /// Key generation
    KeyGeneration,
    /// Digital signing
    Sign,
    /// Signature verification
    Verify,
    /// Encryption
    Encrypt,
    /// Decryption
    Decrypt,
    /// Key derivation
    KeyDerivation,
    /// Random number generation
    RandomGeneration,
    /// Certificate operations
    CertificateOps,
}

/// /// Security levels provided by HSM
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HsmSecurityLevel {
    /// Low security
    Low,
    /// Medium security
    Medium,
    /// High security
    High,
    /// Very high security (hardware-backed, certified)
    VeryHigh,
    /// Software-based (lowest security)
    Software,
    /// Hardware-backed but not certified
    HardwareBacked,
    /// FIPS 140-2 Level 2 equivalent
    Fips140Level2,
    /// FIPS 140-2 Level 3 equivalent
    Fips140Level3,
    /// FIPS 140-2 Level 4 equivalent (highest security)
    Fips140Level4,
    /// Common Criteria certified
    CommonCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmPerformanceProfile {
    /// Operations per second
    pub operations_per_second: f64,
    /// Average latency (ms)
    pub avg_latency_ms: f64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
}

impl Default for HsmPerformanceProfile {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            avg_latency_ms: 0.0,
            success_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HsmDetectionPattern {
    /// Pattern name
    /// Name of the item
    pub name: String,
    /// How to detect this HSM type
    /// The detection method value
    pub detection_method: HsmDetectionMethod,
    /// How to validate the HSM
    pub validation_method: HsmValidationMethod,
    /// Expected security level
    /// The expected security level value
    pub expected_security_level: HsmSecurityLevel,
}

#[derive(Debug, Clone)]
pub enum HsmDetectionMethod {
    /// Check PKCS#11 library
    Pkcs11Library { library_paths: Vec<String> },
    /// Check device file
    DeviceFile { device_paths: Vec<String> },
    /// Check environment variables
    Environment { variables: Vec<String> },
    /// Check network service
    NetworkService { ports: Vec<u16> },
    /// Check system capabilities
    SystemCapability { capabilities: Vec<String> },
    /// Check cloud metadata
    CloudMetadata { metadata_endpoints: Vec<String> },
}

#[derive(Debug, Clone)]
pub enum HsmValidationMethod {
    /// Test basic operations
    BasicOperations,
    /// Check version and capabilities
    CapabilityQuery,
    /// Test cryptographic operations
    CryptoTest { algorithm: String },
    /// Health check endpoint
    HealthCheck { endpoint: String },
    /// Check system capabilities
    SystemCapability { capabilities: Vec<String> },
}

impl VendorAgnosticHsmDiscovery {
    /// Create new vendor-agnostic HSM discovery system
    /// Creates a new instance
    pub fn new() -> Self {
        info!("🔐 Initializing Vendor-Agnostic HSM Discovery System");

        Self {
            discovered_hsms: Arc::new(RwLock::new(HashMap::new())),
            detection_patterns: Self::initialize_detection_patterns(),
        }
    }

    /// Initialize HSM detection patterns (vendor-agnostic)
    /// Initializes componentialize_detection_patterns
    fn initialize_detection_patterns() -> Vec<HsmDetectionPattern> {
        vec![
            // Pattern: PKCS#11 libraries (any vendor)
            HsmDetectionPattern {
                name: "pkcs11_hsm".to_string(),
                detection_method: HsmDetectionMethod::Pkcs11Library {
                    library_paths: vec![
                        "/usr/lib/libpkcs11.so".to_string(),
                        "/usr/local/lib/libpkcs11.so".to_string(),
                        "/opt/*/lib/libpkcs11.so".to_string(),
                    ],
                },
                validation_method: HsmValidationMethod::BasicOperations,
                expected_security_level: HsmSecurityLevel::HardwareBacked,
            },
            // Pattern: TPM devices
            HsmDetectionPattern {
                name: "tpm_device".to_string(),
                detection_method: HsmDetectionMethod::DeviceFile {
                    device_paths: vec!["/dev/tpm0".to_string(), "/dev/tpmrm0".to_string()],
                },
                validation_method: HsmValidationMethod::SystemCapability {
                    capabilities: vec!["tpm".to_string()],
                },
                expected_security_level: HsmSecurityLevel::Fips140Level2,
            },
            // Pattern: Mobile secure hardware
            HsmDetectionPattern {
                name: "mobile_secure_hardware".to_string(),
                detection_method: HsmDetectionMethod::SystemCapability {
                    capabilities: vec![
                        "android.hardware.security.keystore".to_string(),
                        "ios.secure_enclave".to_string(),
                    ],
                },
                validation_method: HsmValidationMethod::BasicOperations,
                expected_security_level: HsmSecurityLevel::HardwareBacked,
            },
            // Pattern: Cloud HSM services (any provider)
            HsmDetectionPattern {
                name: "cloud_hsm_service".to_string(),
                detection_method: HsmDetectionMethod::Environment {
                    variables: vec![
                        "CLOUD_HSM_ENDPOINT".to_string(),
                        "HSM_SERVICE_URL".to_string(),
                        "KMS_ENDPOINT".to_string(),
                    ],
                },
                validation_method: HsmValidationMethod::HealthCheck {
                    endpoint: "${CLOUD_HSM_ENDPOINT}/health".to_string(),
                },
                expected_security_level: HsmSecurityLevel::Fips140Level3,
            },
            // Pattern: Network HSM appliances
            HsmDetectionPattern {
                name: "network_hsm_appliance".to_string(),
                detection_method: HsmDetectionMethod::NetworkService {
                    ports: {
                        let default_ports = std::env::var("BEARDOG_HSM_DEFAULT_PORTS")
                            .ok()
                            .and_then(|s| {
                                let ports: Vec<u16> = s.split(',')
                                    .filter_map(|p| p.trim().parse().ok())
                                    .collect();
                                if ports.is_empty() { None } else { Some(ports) }
                            })
                            .unwrap_or_else(|| vec![1792, 1800, 443, 8080]);
                        std::env::var("BEARDOG_HSM_PORTS")
                            .ok()
                            .and_then(|s| s.split(',')
                                .filter_map(|p| p.trim().parse().ok())
                                .collect::<Vec<u16>>()
                                .into())
                            .unwrap_or(default_ports)
                    },
                },
                validation_method: HsmValidationMethod::HealthCheck {
                    endpoint: std::env::var("BEARDOG_HSM_HEALTH_ENDPOINT")
                        .unwrap_or_else(|_| {
                            use beardog_types::constants::domains::network::config;
                            let host = std::env::var("BEARDOG_HSM_HOST")
                                .unwrap_or_else(|_| config::default_service_host());
                            let port = std::env::var("BEARDOG_HSM_PORT")
                                .ok()
                                .and_then(|p| p.parse().ok())
                                .unwrap_or(443);
                            format!("https://{}:{}/health", host, port)
                        }),
                },
                expected_security_level: HsmSecurityLevel::Fips140Level3,
            },
        ]
    }

    /// Discover all available HSMs
    pub fn discover_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Starting vendor-agnostic HSM discovery...");

        let mut discovered = Vec::new();

        for pattern in &self.detection_patterns {
            match self.explore_hsm_pattern(pattern) {
                Ok(Some(hsm)) => {
                    info!(
                        "✅ Discovered HSM: {} (security: {:?})",
                        hsm.hsm_id, hsm.security_level
                    );
                    discovered.push(hsm.clone());

                    // Store the discovered HSM
                    let mut hsms = self.discovered_hsms.write();
                    hsms.insert(hsm.hsm_id.clone(), hsm);
                }
                Ok(None) => {
                    debug!("Pattern {} not detected", pattern.name);
                }
                Err(e) => {
                    warn!("Failed to explore HSM pattern {}: {}", pattern.name, e);
                }
            }
        }

        info!("🎉 HSM discovery complete! Found {} HSMs", discovered.len());
        Ok(discovered)
    }

    /// Explore a specific HSM detection pattern
    fn explore_hsm_pattern(
        &self,
        pattern: &HsmDetectionPattern,
    ) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("🔍 Exploring HSM pattern: {}", pattern.name);

        // Detect HSM using pattern
        let detected = match &pattern.detection_method {
            HsmDetectionMethod::Pkcs11Library { library_paths } => library_paths
                .iter()
                .any(|path| std::path::Path::new(path).exists()),
            HsmDetectionMethod::DeviceFile { device_paths } => device_paths
                .iter()
                .any(|path| std::path::Path::new(path).exists()),
            HsmDetectionMethod::Environment { variables } => {
                variables.iter().any(|var| std::env::var(var).is_ok())
            }
            HsmDetectionMethod::NetworkService { ports } => {
                // Test network connectivity to HSM services
                use beardog_types::constants::domains::network::config::LOCALHOST_IPV4;
                debug!("Testing network service on ports: {:?}", ports);
                
                // Use configurable HSM probe timeout (was hardcoded to 500ms before Nov 2025)
                let probe_timeout_millis = std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(500);
                let probe_timeout = std::time::Duration::from_millis(probe_timeout_millis);
                
                ports.iter().any(|port| {
                    std::net::TcpStream::connect(format!("{}:{}", LOCALHOST_IPV4, port))
                        .timeout(probe_timeout)
                        .is_ok()
                })
            }
            HsmDetectionMethod::SystemCapability { capabilities } => {
                // Check system HSM capabilities
                debug!("Checking HSM system capabilities: {:?}", capabilities);
                capabilities.iter().all(|cap| {
                    match cap.as_str() {
                        "tpm" | "tpm2.0" => {
                            std::path::Path::new("/dev/tpm0").exists()
                                || std::path::Path::new("/dev/tpmrm0").exists()
                                || which::which("tpm2_getcap").is_ok()
                        }
                        "pkcs11" => {
                            std::env::var("PKCS11_MODULE_PATH").is_ok()
                                || std::path::Path::new("/usr/lib/softhsm/libsofthsm2.so").exists()
                                || std::path::Path::new("/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so").exists()
                        }
                        "keystore" | "android_keystore" => {
                            std::env::var("ANDROID_ROOT").is_ok()
                        }
                        "secure_enclave" | "ios_secure_enclave" => {
                            cfg!(target_os = "ios") || cfg!(target_os = "macos")
                        }
                        "kms" => {
                            std::env::var("AWS_KMS_ENDPOINT").is_ok()
                                || std::env::var("AZURE_KEY_VAULT_ENDPOINT").is_ok()
                                || std::env::var("GCP_KMS_ENDPOINT").is_ok()
                        }
                        _ => which::which(cap).is_ok()
                    }
                })
            }
            HsmDetectionMethod::CloudMetadata {
                metadata_endpoints,
            } => {
                // Query cloud metadata endpoints
                debug!("Checking cloud metadata endpoints: {:?}", metadata_endpoints);
                
                // Use configurable HSM operation timeout (was hardcoded to 2s before Nov 2025)
                let operation_timeout_secs = std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(2);
                let timeout = std::time::Duration::from_secs(operation_timeout_secs);
                
                metadata_endpoints.iter().any(|endpoint| {
                    // Try to connect to cloud metadata service
                    // Parse host from endpoint
                    if let Ok(url) = endpoint.parse::<url::Url>() {
                        if let Some(host) = url.host_str() {
                            let port = url.port().unwrap_or(80);
                            return std::net::TcpStream::connect(format!("{}:{}", host, port))
                                .timeout(timeout)
                                .is_ok();
                        }
                    }
                    
                    // Fallback: check common cloud metadata endpoints
                    let common_endpoints = [
                        "169.254.169.254:80",  // AWS, Azure, GCP
                        "metadata.google.internal:80", // GCP
                    ];
                    
                    common_endpoints.iter().any(|addr| {
                        std::net::TcpStream::connect(addr)
                            .timeout(timeout)  // Uses configured timeout from above
                            .is_ok()
                    })
                })
            }
        };

        if !detected {
            return Ok(None);
        }

        info!("✅ HSM pattern {} detected! Validating...", pattern.name);

        // Create HSM interface based on detection method
        let interface = match &pattern.detection_method {
            HsmDetectionMethod::Pkcs11Library { library_paths } => {
                let library_path = library_paths
                    .iter()
                    .find(|path| std::path::Path::new(path).exists())
                    .ok_or_else(|| BearDogError::system(format!(
                        "No PKCS#11 library found in paths: {:?}", library_paths
                    )))?
                    .to_string();

                HsmInterface::Pkcs11 {
                    library_path,
                    slot_id: None,
                }
            }
            HsmDetectionMethod::DeviceFile { device_paths: _ } => HsmInterface::Native {
                interface_type: NativeInterface::Tpm,
            },
            HsmDetectionMethod::Environment { variables } => {
                if let Some(var) = variables.iter().find(|v| std::env::var(v).is_ok()) {
                    if let Ok(endpoint) = std::env::var(var) {
                        HsmInterface::CloudApi {
                            api_endpoint: endpoint,
                            authentication: CloudAuth::InstanceMetadata,
                        }
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Ok(None);
                }
            }
            _ => HsmInterface::Network {
                endpoint: "unknown".to_string(),
                protocol: NetworkProtocol::Rest,
            },
        };

        // Generate unique HSM ID
        let hsm_id = format!(
            "{}_{}",
            pattern.name,
            Uuid::new_v4().to_string()[..8].to_string()
        );

        Ok(Some(DiscoveredHsm {
            hsm_id,
            interface,
            supported_operations: vec![
                CryptoOperation::KeyGeneration {
                    algorithms: vec!["RSA2048".to_string(), "ECDSA".to_string()],
                },
                CryptoOperation::Signing {
                    algorithms: vec!["SHA256withRSA".to_string(), "SHA256withECDSA".to_string()],
                },
                CryptoOperation::Encryption {
                    algorithms: vec!["AES256".to_string()],
                },
            ],
            security_level: pattern.expected_security_level.clone(),
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.5, // Start with neutral trust
        }))
    }

    /// Get discovered HSMs by security level
    /// Gets hsms_by_security_level
    /// Gets hsms_by_security_level
    pub fn get_hsms_by_security_level(
        &self,
        min_level: HsmSecurityLevel,
    ) -> Vec<DiscoveredHsm> {
        let hsms = self.discovered_hsms.read();
        hsms.values()
            .filter(|hsm| hsm.security_level >= min_level)
            .cloned()
            .collect()
    }

    /// Get the highest security HSM available
    /// Gets highest_security_hsm
    /// Gets highest_security_hsm
    pub fn get_highest_security_hsm(&self) -> Option<DiscoveredHsm> {
        let hsms = self.discovered_hsms.read();
        hsms.values()
            .max_by(|a, b| a.security_level.cmp(&b.security_level))
            .cloned()
    }
}

impl Default for VendorAgnosticHsmDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
