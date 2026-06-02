// SPDX-License-Identifier: AGPL-3.0-or-later

//! Environment-based and runtime socket discovery.

use super::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::{Endpoint, SimpleCapability};
use beardog_errors::BearDogError;
use beardog_types::constants::domains::network::ipc_discovery as ipc;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use tracing::{debug, info, warn};

impl PrimalDiscovery {
    /// Discover from environment variables
    ///
    /// Reads via [`beardog_errors::process_env::vars`] (OS env merged with the test overlay).
    pub(crate) fn discover_from_env(
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
    pub(crate) fn discover_from_env_vars(
        &self,
        env_vars: &HashMap<String, String>,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        debug!("Discovering from environment variables");

        let mut discovered = Vec::new();

        if let Some(name) = &query.name {
            let env_key = format!("PRIMAL_{}_ADDR", name.to_uppercase());
            if let Some(addr) = env_vars.get(&env_key) {
                let endpoint = Endpoint::parse(addr)?;
                info!("Found {} at {} (from {})", name, addr, env_key);

                let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                let capabilities = parse_capabilities_from_env_map(env_vars, &caps_key);

                discovered.push(DiscoveredPrimal {
                    name: name.clone(),
                    endpoints: vec![endpoint],
                    capabilities,
                    trust_score: Some(1.0),
                    discovered_at: std::time::SystemTime::now(),
                });
            }
        } else {
            for (key, value) in env_vars {
                if key.starts_with("PRIMAL_") && key.ends_with("_ADDR") {
                    let name = key
                        .strip_prefix("PRIMAL_")
                        .and_then(|s| s.strip_suffix("_ADDR"))
                        .unwrap_or("unknown");

                    if let Ok(endpoint) = Endpoint::parse(value) {
                        info!("Found {} at {} (from {})", name, value, key);

                        let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                        let capabilities = parse_capabilities_from_env_map(env_vars, &caps_key);

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

        append_biomeos_socket_primals(env_vars, query, &mut discovered);

        let discovered = if query.capabilities.is_empty() {
            discovered
        } else {
            discovered
                .into_iter()
                .filter(|primal| {
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
}

/// Discover peer primals from `*.sock` entries under the resolved biomeOS runtime directory.
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
        let capabilities = parse_capabilities_from_env_map(env_vars, &caps_key);
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

fn parse_capabilities_from_env_map(
    env_vars: &HashMap<String, String>,
    env_key: &str,
) -> Vec<SimpleCapability> {
    env_vars
        .get(env_key)
        .map(|caps_str| parse_capabilities_str(caps_str, env_key))
        .unwrap_or_default()
}

/// Parse capabilities from environment variable
///
/// Expected format: Comma-separated list like "SecureTunneling,GeneticLineage,Discovery"
pub fn parse_capabilities_str(caps_str: &str, env_key: &str) -> Vec<SimpleCapability> {
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
