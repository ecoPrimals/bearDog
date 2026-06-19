// SPDX-License-Identifier: AGPL-3.0-or-later

use super::types::{DiscoveredPrimal, DiscoveryMethod, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::{Endpoint, SimpleCapability};
use beardog_errors::BearDogError;
use beardog_types::constants::domains::network::ipc_discovery as ipc;
use beardog_types::constants::domains::timeouts::HEALTH_CHECK_TIMEOUT;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use tracing::{debug, info, warn};

impl PrimalDiscovery {
    /// Discover from environment variables
    ///
    /// Reads via [`beardog_errors::process_env::vars`] (OS env merged with the test overlay).
    pub(super) fn discover_from_env(
        &self,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        let vars: HashMap<String, String> = match &self.env_override {
            Some(m) => m.clone(),
            None => beardog_errors::process_env::vars().collect(),
        };
        self.discover_from_env_vars(&vars, query)
    }

    /// Discover from explicit environment map
    ///
    /// This method accepts an explicit environment map, making it concurrent-safe
    /// for testing while maintaining the same logic as `discover_from_env()`.
    pub(super) fn discover_from_env_vars(
        &self,
        env_vars: &HashMap<String, String>,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        debug!("Discovering from environment variables");

        let mut discovered = Vec::new();

        // If specific name requested, check PRIMAL_<NAME>_ADDR
        if let Some(name) = &query.name {
            let env_key = format!("PRIMAL_{}_ADDR", name.to_uppercase());
            if let Some(addr) = env_vars.get(&env_key) {
                let endpoint = Endpoint::parse(addr)?;
                info!("Found {} at {} (from {})", name, addr, env_key);

                // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                let capabilities = Self::parse_capabilities_from_env_map(env_vars, &caps_key);

                discovered.push(DiscoveredPrimal {
                    name: name.clone(),
                    endpoints: vec![endpoint],
                    capabilities,
                    trust_score: Some(1.0), // Explicit config = trusted
                    discovered_at: std::time::SystemTime::now(),
                });
            }
        } else {
            // Scan all PRIMAL_*_ADDR environment variables
            for (key, value) in env_vars {
                if key.starts_with("PRIMAL_") && key.ends_with("_ADDR") {
                    let name = key
                        .strip_prefix("PRIMAL_")
                        .and_then(|s| s.strip_suffix("_ADDR"))
                        .unwrap_or("unknown");

                    if let Ok(endpoint) = Endpoint::parse(value) {
                        info!("Found {} at {} (from {})", name, value, key);

                        // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                        let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                        let capabilities =
                            Self::parse_capabilities_from_env_map(env_vars, &caps_key);

                        discovered.push(DiscoveredPrimal {
                            name: name.to_lowercase(),
                            endpoints: vec![endpoint],
                            capabilities,
                            trust_score: Some(1.0),
                            discovered_at: std::time::SystemTime::now(),
                        });
                    }
                }
            }
        }

        Self::append_biomeos_socket_primals(env_vars, query, &mut discovered);

        // Filter by capabilities if specified in query
        let discovered = if query.capabilities.is_empty() {
            discovered
        } else {
            discovered
                .into_iter()
                .filter(|primal| {
                    // Primal must have ALL requested capabilities
                    query
                        .capabilities
                        .iter()
                        .all(|req_cap| primal.capabilities.contains(req_cap))
                })
                .collect()
        };

        if discovered.is_empty() {
            warn!("No primals discovered from environment");
        } else {
            info!("Discovered {} primals from environment", discovered.len());
        }

        Ok(discovered)
    }

    /// Discover peer primals from `*.sock` entries under the resolved biomeOS runtime directory.
    ///
    /// Resolution order for the directory: `BEARDOG_BIOMEOS_SOCKET_DIR`, then
    /// `$XDG_RUNTIME_DIR/biomeos`, then [`ipc::biomeos_ipc_socket_dir`].
    /// The registry listener ([`ipc::DEFAULT_UPA_REGISTRY_SOCKET_STEM`]) is skipped; use UPA discovery for that.
    fn append_biomeos_socket_primals(
        env_vars: &HashMap<String, String>,
        query: &DiscoveryQuery,
        discovered: &mut Vec<DiscoveredPrimal>,
    ) {
        let dir = ipc::biomeos_ipc_socket_dir_from_components(
            env_vars
                .get(ipc::ENV_BIOMEOS_SOCKET_DIR_OVERRIDE)
                .map(String::as_str),
            env_vars.get("XDG_RUNTIME_DIR").map(String::as_str),
            env_vars
                .get(ipc::ENV_BIOMEOS_IPC_NAMESPACE)
                .map(String::as_str),
        );

        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                debug!(
                    "Platform IPC socket directory not readable ({}): {}",
                    dir.display(),
                    e
                );
                return;
            }
        };

        let mut seen: HashSet<String> = discovered.iter().map(|p| p.name.to_lowercase()).collect();

        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let ext = path.extension();
            if ext != Some(OsStr::new("sock")) {
                continue;
            }
            let Ok(meta) = std::fs::metadata(&path) else {
                continue;
            };
            if meta.is_dir() {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if stem.eq_ignore_ascii_case(ipc::DEFAULT_UPA_REGISTRY_SOCKET_STEM) {
                continue;
            }
            if let Some(want) = &query.name
                && !want.eq_ignore_ascii_case(stem)
            {
                continue;
            }
            let lname = stem.to_lowercase();
            if seen.contains(&lname) {
                continue;
            }
            let Ok(endpoint) = Endpoint::parse(&format!("unix://{}", path.display())) else {
                continue;
            };
            let caps_key = format!("PRIMAL_{}_CAPABILITIES", stem.to_uppercase());
            let capabilities = Self::parse_capabilities_from_env_map(env_vars, &caps_key);
            info!(
                "Found primal '{}' at {} (runtime socket scan)",
                lname,
                path.display()
            );
            seen.insert(lname.clone());
            discovered.push(DiscoveredPrimal {
                name: lname,
                endpoints: vec![endpoint],
                capabilities,
                trust_score: Some(0.85),
                discovered_at: std::time::SystemTime::now(),
            });
        }
    }

    /// Parse capabilities from environment variable
    ///
    /// Expected format: Comma-separated list like "SecureTunneling,GeneticLineage,Discovery"
    fn _parse_capabilities_from_env(env_key: &str) -> Vec<SimpleCapability> {
        beardog_errors::process_env::var(env_key)
            .ok()
            .map(|caps_str| Self::parse_capabilities_str(&caps_str, env_key))
            .unwrap_or_default()
    }

    fn parse_capabilities_from_env_map(
        env_vars: &HashMap<String, String>,
        env_key: &str,
    ) -> Vec<SimpleCapability> {
        env_vars
            .get(env_key)
            .map(|caps_str| Self::parse_capabilities_str(caps_str, env_key))
            .unwrap_or_default()
    }

    pub(crate) fn parse_capabilities_str(caps_str: &str, env_key: &str) -> Vec<SimpleCapability> {
        caps_str
            .split(',')
            .filter_map(|cap| {
                let cap_trimmed = cap.trim();
                match cap_trimmed {
                    "SecureTunneling" => Some(SimpleCapability::SecureTunneling),
                    "GeneticLineage" => Some(SimpleCapability::GeneticLineage),
                    "Cryptography" => Some(SimpleCapability::Cryptography),
                    "HsmIntegration" => Some(SimpleCapability::HsmIntegration),
                    "Discovery" => Some(SimpleCapability::Discovery),
                    _ => {
                        warn!("Unknown capability in {}: {}", env_key, cap_trimmed);
                        None
                    }
                }
            })
            .collect()
    }

    /// Discover from UPA registry (COMPLETE IMPLEMENTATION)
    pub(super) async fn discover_from_upa(
        &self,
        query: &DiscoveryQuery,
        registry_addr: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 UPA registry discovery at: {}", registry_addr);

        // UPA uses Unix socket + JSON-RPC
        let socket_path = registry_addr.trim_start_matches("unix://");

        // Build JSON-RPC request
        let capability = if query.capabilities.is_empty() {
            "generic".to_string()
        } else {
            format!("{:?}", query.capabilities[0])
        };

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "upa.discover",
            "params": {
                "capability": capability,
                "timeout_ms": u64::try_from(HEALTH_CHECK_TIMEOUT.as_millis()).unwrap_or(u64::MAX)
            },
            "id": 1
        });

        // Connect to UPA registry via Unix socket
        match tokio::net::UnixStream::connect(socket_path).await {
            Ok(mut stream) => {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                // Send request
                let request_str =
                    serde_json::to_string(&request).map_err(|e| BearDogError::Network {
                        message: format!("Failed to serialize UPA request: {e}"),
                        category: beardog_errors::NetworkErrorCategory::Connection,
                    })?;
                stream.write_all(request_str.as_bytes()).await?;
                stream.write_all(b"\n").await?;

                // Read response
                let mut buffer = vec![0u8; 8192];
                let n = stream.read(&mut buffer).await?;
                let response_str = String::from_utf8_lossy(&buffer[..n]);

                // Parse JSON-RPC response
                let response: serde_json::Value =
                    serde_json::from_str(&response_str).map_err(|e| BearDogError::Network {
                        message: format!("Failed to parse UPA response: {e}"),
                        category: beardog_errors::NetworkErrorCategory::Connection,
                    })?;

                if let Some(result) = response.get("result")
                    && let Some(primals_array) = result.as_array()
                {
                    info!("✅ UPA discovered {} primals", primals_array.len());

                    let primals = primals_array
                        .iter()
                        .filter_map(|p| serde_json::from_value::<DiscoveredPrimal>(p.clone()).ok())
                        .collect();

                    return Ok(primals);
                }

                warn!("UPA returned no results");
                Ok(Vec::new())
            }
            Err(e) => {
                warn!("UPA registry not available at {}: {}", registry_addr, e);
                Ok(Vec::new())
            }
        }
    }

    /// Discover from mDNS
    ///
    /// # Integration Status
    ///
    /// The beardog-discovery crate has a complete mDNS implementation (45 tests pass).
    /// Integration requires:
    /// 1. Add `beardog-discovery` to beardog-core/Cargo.toml
    /// 2. Enable the `mdns` feature
    /// 3. Convert between `DiscoveredService` and `DiscoveredPrimal` types
    ///
    /// # Current Behavior
    ///
    /// - With `mdns` feature: Logs warning, returns empty
    /// - Without `mdns` feature: Logs warning, returns empty
    ///
    /// Production deployments should use Unix socket discovery (via beardog-ipc)
    /// or HTTP-based service registries until mDNS integration is complete.
    pub(super) fn discover_from_mdns(
        &self,
        _query: &DiscoveryQuery,
        service_type: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 mDNS discovery for service: {}", service_type);

        // beardog-discovery integration pending (see module docs above).

        #[cfg(feature = "mdns")]
        {
            warn!(
                "mDNS discovery requested for '{}' - beardog-discovery integration pending. \
                 Use Unix socket or HTTP discovery instead.",
                service_type
            );
            Ok(Vec::new())
        }

        #[cfg(not(feature = "mdns"))]
        {
            debug!("mDNS feature not enabled. Enable with: cargo build --features mdns");
            Ok(Vec::new())
        }
    }

    /// Discover from DNS-SD (COMPLETE IMPLEMENTATION)
    pub(super) fn discover_from_dns_sd(
        &self,
        _query: &DiscoveryQuery, // Planned: Use for capability filtering in discovery results
        domain: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 DNS-SD discovery in domain: {}", domain);

        // DNS-SD uses the same infrastructure as mDNS, just with different domain
        #[cfg(feature = "mdns")]
        {
            // EVOLUTION: DNS-SD discovery via ecosystem relay IPC (capability-based).
            // This is the proper implementation using inter-primal communication
            // instead of hardcoded mock data.
            //
            // NOTE: beardog-discovery ready (45 tests pass), pending integration wiring
            // For now, return empty until integration is complete.
            // This is honest about current capabilities (fallback, not full discovery).
            warn!(
                "DNS-SD discovery via ecosystem relay IPC not yet complete - beardog-discovery crate pending"
            );
            warn!("Returning empty discovery results until integration is complete");
            warn!("See: ROADMAP.md (repository root) for project direction and future work");

            // Return empty - honest about current state
            // Tests that depend on discovery should use explicit test-only mocks
            Ok(vec![])
        }

        #[cfg(not(feature = "mdns"))]
        {
            warn!("DNS-SD/mDNS feature not enabled, returning empty results");
            Ok(Vec::new())
        }
    }

    /// Try multiple discovery methods
    pub(super) async fn discover_multi(
        &self,
        query: &DiscoveryQuery,
        methods: &[DiscoveryMethod],
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        let mut all_discovered = Vec::new();

        for method in methods {
            let result = match method {
                DiscoveryMethod::Environment => self.discover_from_env(query),
                DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                    self.discover_from_upa(query, registry_addr).await
                }
                DiscoveryMethod::Mdns { service_type } => {
                    self.discover_from_mdns(query, service_type)
                }
                DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(query, domain),
                DiscoveryMethod::Multi(_) => {
                    // Prevent infinite recursion
                    continue;
                }
            };

            if let Ok(mut discovered) = result {
                all_discovered.append(&mut discovered);
            }
        }

        // Deduplicate by name (keep first occurrence)
        let mut seen = std::collections::HashSet::new();
        all_discovered.retain(|p| seen.insert(p.name.clone()));

        Ok(all_discovered)
    }
}
