// SPDX-License-Identifier: AGPL-3.0-or-later

//! PKCS#11 HSM discoverer — orchestrates library search, token enumeration, and HSM records.

use super::types::Pkcs11TokenInfo;
use crate::universal_hsm_discovery::{DiscoveredHsm, HsmEndpoint, HsmType, UniversalHsmDiscovery};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::{debug, info};

/// PKCS#11 HSM discoverer
#[derive(Debug, Clone)]
pub struct Pkcs11Discoverer {
    /// Additional library paths to check
    pub(crate) custom_library_paths: Vec<PathBuf>,
    /// Whether to scan system library directories
    enable_system_scan: bool,
}

impl Pkcs11Discoverer {
    /// Create new PKCS#11 discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            custom_library_paths: Vec::new(),
            enable_system_scan: true,
        })
    }

    /// Add custom library path to scan
    pub fn add_library_path(&mut self, path: PathBuf) {
        self.custom_library_paths.push(path);
    }

    /// Discover PKCS#11 HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering PKCS#11 HSMs");

        let mut discovered = Vec::new();
        let mut found_libraries = HashSet::new();

        // Discover PKCS#11 libraries
        let library_paths = self.find_pkcs11_libraries().await?;
        debug!("Found {} potential PKCS#11 libraries", library_paths.len());

        for lib_path in library_paths {
            // Skip duplicates
            if !found_libraries.insert(lib_path.clone()) {
                continue;
            }

            // Try to enumerate tokens from this library
            match self.enumerate_tokens(&lib_path).await {
                Ok(tokens) => {
                    for token_info in tokens {
                        if let Some(hsm) = self.create_hsm_from_token(&token_info) {
                            info!("✓ Found PKCS#11 HSM: {}", hsm.name);
                            discovered.push(hsm);
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to enumerate tokens from {:?}: {}", lib_path, e);
                }
            }
        }

        info!("✅ PKCS#11 discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Create DiscoveredHsm from PKCS#11 token info
    fn create_hsm_from_token(&self, token_info: &Pkcs11TokenInfo) -> Option<DiscoveredHsm> {
        let now = Utc::now();

        // Determine HSM type and tier based on manufacturer
        let (hsm_type, tier, capabilities) = self.classify_token(token_info);

        Some(DiscoveredHsm {
            name: format!(
                "pkcs11-{}-{}",
                token_info
                    .manufacturer_id
                    .to_lowercase()
                    .replace(' ', "-"),
                token_info.slot_id
            ),
            hsm_type,
            endpoint: HsmEndpoint {
                host: token_info.library_path.to_string_lossy().to_string(),
                port: Some(token_info.slot_id as u16),
                protocol: "pkcs11".to_string(),
                secure: true,
            },
            capabilities,
            assigned_tier: tier,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        })
    }
}

impl Default for Pkcs11Discoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            custom_library_paths: Vec::new(),
            enable_system_scan: true,
        })
    }
}
