// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime introspection for `primal.announce` payloads.
//!
//! Method names and capability domains are derived from [`HandlerRegistry`]
//! plus gate-layer auth methods, not from static lists in `beardog-ipc`.

use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;

/// Meta/introspection domains excluded from biomeOS routing announcements.
const EXCLUDED_ANNOUNCE_DOMAINS: &[&str] = &[
    "capabilities",
    "capability",
    "health",
    "identity",
    "lifecycle",
    "rpc",
    "introspection",
];

/// Gate-layer `auth.*` methods (pre-dispatch) included in announce payloads.
const GATE_AUTH_ANNOUNCE_METHODS: &[&str] = &[
    "auth.check",
    "auth.mode",
    "auth.peer_info",
    "auth.issue_ionic",
    "auth.issue_session",
    "auth.verify_ionic",
    "auth.public_key",
    "auth.sign",
];

/// Collect dotted method names for `primal.announce` from a live registry.
pub async fn registered_announce_method_names(registry: &HandlerRegistry) -> Vec<String> {
    let mut methods: Vec<String> = registry
        .all_methods()
        .await
        .into_iter()
        .filter(|m| m.contains('.'))
        .filter(|m| !is_excluded_announce_method(m))
        .map(str::to_owned)
        .collect();

    for gate_method in GATE_AUTH_ANNOUNCE_METHODS {
        if !methods.iter().any(|m| m == gate_method) {
            methods.push((*gate_method).to_owned());
        }
    }

    methods.sort_unstable();
    methods.dedup();
    methods
}

/// Build announce method names using a fresh registry for the given identity.
///
/// Used at startup before the IPC server is fully wired (CLI path) or when
/// only identity is available.
pub async fn registered_announce_method_names_for_identity(
    identity: Arc<PrimalIdentity>,
) -> Vec<String> {
    let registry = HandlerRegistry::new(identity);
    registered_announce_method_names(&registry).await
}

/// Derive unique capability domain prefixes from an announce method list.
#[must_use]
pub fn announce_capability_domains(methods: &[String]) -> Vec<String> {
    let mut domains: Vec<String> = methods
        .iter()
        .filter_map(|m| m.split('.').next())
        .filter(|domain| !EXCLUDED_ANNOUNCE_DOMAINS.contains(domain))
        .map(str::to_owned)
        .collect();
    domains.sort_unstable();
    domains.dedup();
    domains
}

/// Default `cost_hints` for derived capability domains.
#[must_use]
pub fn default_cost_hints_for_capabilities(capabilities: &[String]) -> serde_json::Value {
    let mut hints = serde_json::Map::new();
    for cap in capabilities {
        let cost = match cap.as_str() {
            "crypto" => 5.0,
            "security" => 10.0,
            "auth" => 1.0,
            "btsp" => 3.0,
            _ => 5.0,
        };
        hints.insert(cap.clone(), serde_json::json!(cost));
    }
    serde_json::Value::Object(hints)
}

/// Default `latency_estimates` (milliseconds) for derived capability domains.
#[must_use]
pub fn default_latency_estimates_for_capabilities(capabilities: &[String]) -> serde_json::Value {
    let mut estimates = serde_json::Map::new();
    for cap in capabilities {
        let latency = match cap.as_str() {
            "crypto" => 2,
            "security" => 15,
            "auth" => 1,
            "btsp" => 5,
            _ => 5,
        };
        estimates.insert(cap.clone(), serde_json::json!(latency));
    }
    serde_json::Value::Object(estimates)
}

#[must_use]
fn is_excluded_announce_method(method: &str) -> bool {
    method
        .split('.')
        .next()
        .is_some_and(|domain| EXCLUDED_ANNOUNCE_DOMAINS.contains(&domain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registered_methods_include_crypto_and_security() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::new(identity);
        let methods = registered_announce_method_names(&registry).await;

        assert!(
            methods.len() >= 40,
            "expected at least 40 methods, got {}",
            methods.len()
        );
        assert!(methods.iter().any(|m| m == "crypto.sign_ed25519"));
        assert!(methods.iter().any(|m| m == "crypto.ionic_bond.propose"));
        assert!(methods.iter().any(|m| m == "security.evaluate"));
        assert!(methods.iter().any(|m| m == "auth.check"));
    }

    #[tokio::test]
    async fn registered_methods_are_dotted_canonical() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::new(identity);
        let methods = registered_announce_method_names(&registry).await;

        for method in &methods {
            assert!(
                method.contains('.'),
                "method {method} is not in dotted canonical form"
            );
        }
    }

    #[test]
    fn capability_domains_derived_from_methods() {
        let methods = vec![
            "crypto.sign_ed25519".to_owned(),
            "security.evaluate".to_owned(),
            "auth.check".to_owned(),
            "btsp.negotiate".to_owned(),
        ];
        let domains = announce_capability_domains(&methods);
        assert_eq!(
            domains,
            vec![
                "auth".to_owned(),
                "btsp".to_owned(),
                "crypto".to_owned(),
                "security".to_owned(),
            ]
        );
    }
}
