//! PKCS#11 HSM Discoverer
//!
//! Discovers PKCS#11 compatible HSMs by scanning for common library paths

use super::super::*;
use beardog_errors::BearDogResult;
use std::path::Path;
use std::time::Duration;
use tracing::{debug, info};

/// PKCS#11 HSM discoverer
#[derive(Debug)]
pub struct Pkcs11Discoverer;

impl Pkcs11Discoverer {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }

    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Discovering PKCS#11 HSMs");

        let mut hsms = Vec::new();

        // Common PKCS#11 library paths to check
        let library_paths = self.get_common_pkcs11_paths();

        for path in library_paths {
            if Path::new(&path).exists() {
                debug!("Found PKCS#11 library: {}", path);

                let hsm = DiscoveredHsm {
                    hsm_id: format!(
                        "pkcs11-{}",
                        Path::new(&path).file_stem().unwrap().to_string_lossy()
                    ),
                    vendor: self.detect_vendor_from_path(&path),
                    model: "PKCS#11 HSM".to_string(),
                    version: "Unknown".to_string(),
                    interface_type: HsmInterfaceType::Pkcs11 {
                        library_path: path.clone(),
                    },
                    connection_info: HsmConnectionInfo {
                        connection_type: ConnectionType::Local,
                        authentication: AuthenticationMethod::Pin {
                            pin: "".to_string(),
                        },
                        endpoint: None,
                        port: None,
                        timeout: config.timeout,
                        retry_policy: RetryPolicy {
                            max_retries: 3,
                            base_delay: Duration::from_millis(100),
                            max_delay: Duration::from_secs(5),
                            backoff_multiplier: 2.0,
                        },
                        ssl_config: None,
                    },
                    capabilities: HsmCapabilities::default(), // Will be filled by capability detector
                    assigned_tier: HsmTier::BasicHardware,
                    supports_human_entropy: false,
                    health_status: HsmHealthStatus::Unknown,
                    discovered_at: chrono::Utc::now(),
                    last_health_check: chrono::Utc::now(),
                    integration_status: IntegrationStatus::Discovered,
                };

                hsms.push(hsm);
            }
        }

        info!("Found {} PKCS#11 HSMs", hsms.len());
        Ok(hsms)
    }

    fn get_common_pkcs11_paths(&self) -> Vec<String> {
        vec![
            // Linux paths
            "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so".to_string(),
            "/usr/local/lib/softhsm/libsofthsm2.so".to_string(),
            "/opt/nfast/toolkits/pkcs11/libcknfast.so".to_string(),
            "/usr/lib/opencryptoki/libopencryptoki.so".to_string(),
            // Windows paths
            "C:\\Program Files\\SoftHSM2\\lib\\softhsm2-x64.dll".to_string(),
            "C:\\Windows\\System32\\eTPKCS11.dll".to_string(),
            "C:\\Windows\\System32\\dkck201.dll".to_string(),
            // macOS paths
            "/usr/local/lib/softhsm/libsofthsm2.so".to_string(),
            "/opt/homebrew/lib/softhsm/libsofthsm2.so".to_string(),
            "/Library/Application Support/Yubico/libykcs11.dylib".to_string(),
        ]
    }

    fn detect_vendor_from_path(&self, path: &str) -> String {
        let path_lower = path.to_lowercase();

        if path_lower.contains("softhsm") {
            "SoftHSM".to_string()
        } else if path_lower.contains("nfast") {
            "Entrust nShield".to_string()
        } else if path_lower.contains("opencryptoki") {
            "OpenCryptoki".to_string()
        } else if path_lower.contains("etoken") {
            "SafeNet eToken".to_string()
        } else if path_lower.contains("yubico") {
            "Yubico".to_string()
        } else {
            "Unknown".to_string()
        }
    }
} 