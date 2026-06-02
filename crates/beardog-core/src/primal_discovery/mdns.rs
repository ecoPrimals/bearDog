// SPDX-License-Identifier: AGPL-3.0-or-later

//! mDNS and local DNS-SD discovery via [`crate::primal_discovery_mdns`].

use super::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[cfg(feature = "mdns")]
use crate::self_knowledge::SimpleCapability;

impl PrimalDiscovery {
    /// Discover from mDNS using the in-crate [`MdnsDiscoveryClient`].
    pub(crate) async fn discover_from_mdns(
        &self,
        query: &DiscoveryQuery,
        service_type: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 mDNS discovery for service: {}", service_type);

        #[cfg(feature = "mdns")]
        {
            use crate::primal_discovery_mdns::MdnsDiscoveryClient;

            let client = MdnsDiscoveryClient::new().with_timeout(query.timeout);
            let mdns_results = run_mdns_query(&client, query, service_type).await?;
            let mut discovered: Vec<DiscoveredPrimal> = mdns_results
                .into_iter()
                .filter_map(|p| mdns_primal_to_discovered(p).ok())
                .collect();

            if let Some(want_name) = &query.name {
                discovered.retain(|p| p.name.eq_ignore_ascii_case(want_name));
            }

            if !query.capabilities.is_empty() {
                discovered.retain(|primal| {
                    query
                        .capabilities
                        .iter()
                        .all(|req| primal.capabilities.contains(req))
                });
            }

            info!("mDNS discovery returned {} primals", discovered.len());
            return Ok(discovered);
        }

        #[cfg(not(feature = "mdns"))]
        {
            debug!(
                "mDNS feature not enabled (timeout {:?}). Enable with: cargo build --features mdns",
                query.timeout
            );
            Ok(Vec::new())
        }
    }

    /// Discover from DNS-SD.
    ///
    /// Local domains (`.local` / `local.`) use multicast mDNS via [`MdnsDiscoveryClient`].
    /// Unicast DNS-SD for other domains is not yet integrated (hickory-resolver suspended).
    pub(crate) async fn discover_from_dns_sd(
        &self,
        query: &DiscoveryQuery,
        domain: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 DNS-SD discovery in domain: {}", domain);

        if is_local_dns_sd_domain(domain) {
            let service_type = match query.name.as_deref() {
                Some(n) => format!("_{n}._tcp"),
                None => "_beardog._tcp".to_string(),
            };
            return self.discover_from_mdns(query, &service_type).await;
        }

        #[cfg(feature = "mdns")]
        info!(
            "Unicast DNS-SD for domain '{}' is not yet integrated; \
             use a .local domain, mDNS, or environment discovery",
            domain
        );

        #[cfg(not(feature = "mdns"))]
        debug!("DNS-SD/mDNS feature not enabled");

        Ok(Vec::new())
    }
}

#[cfg(feature = "mdns")]
async fn run_mdns_query(
    client: &crate::primal_discovery_mdns::MdnsDiscoveryClient,
    query: &DiscoveryQuery,
    service_type: &str,
) -> Result<Vec<crate::primal_discovery_mdns::MdnsDiscoveredPrimal>, BearDogError> {
    if query.capabilities.is_empty() {
        client.discover_on_service_type(service_type).await
    } else {
        let cap = simple_capability_label(&query.capabilities[0]);
        client.discover_by_capability_on(service_type, cap).await
    }
}

fn is_local_dns_sd_domain(domain: &str) -> bool {
    let d = domain.trim().trim_end_matches('.');
    d.is_empty() || d == "local" || d.to_ascii_lowercase().ends_with(".local")
}

#[cfg(feature = "mdns")]
fn simple_capability_label(cap: &SimpleCapability) -> &'static str {
    match cap {
        SimpleCapability::SecureTunneling => "SecureTunneling",
        SimpleCapability::GeneticLineage => "GeneticLineage",
        SimpleCapability::Cryptography => "Cryptography",
        SimpleCapability::HsmIntegration => "HsmIntegration",
        SimpleCapability::Discovery => "Discovery",
    }
}

#[cfg(feature = "mdns")]
fn mdns_primal_to_discovered(
    primal: crate::primal_discovery_mdns::MdnsDiscoveredPrimal,
) -> Result<DiscoveredPrimal, BearDogError> {
    use crate::self_knowledge::Endpoint;
    let name = primal
        .instance_name
        .split('.')
        .next()
        .unwrap_or(&primal.instance_name)
        .to_string();

    let mut endpoints = Vec::new();
    for addr in &primal.addresses {
        let url = format!("http://{addr}:{}", primal.port);
        if let Ok(ep) = Endpoint::parse(&url) {
            endpoints.push(ep);
        }
    }

    if endpoints.is_empty() {
        return Err(BearDogError::network(
            "mDNS service resolved without usable addresses".to_string(),
        ));
    }

    let capabilities = primal
        .capabilities
        .iter()
        .filter_map(|c| parse_mdns_capability(c))
        .collect();

    Ok(DiscoveredPrimal {
        name,
        endpoints,
        capabilities,
        trust_score: Some(0.7),
        discovered_at: std::time::SystemTime::now(),
    })
}

#[cfg(feature = "mdns")]
fn parse_mdns_capability(cap: &str) -> Option<SimpleCapability> {
    match cap.trim() {
        "SecureTunneling" => Some(SimpleCapability::SecureTunneling),
        "GeneticLineage" => Some(SimpleCapability::GeneticLineage),
        "Cryptography" => Some(SimpleCapability::Cryptography),
        "HsmIntegration" => Some(SimpleCapability::HsmIntegration),
        "Discovery" => Some(SimpleCapability::Discovery),
        _ => None,
    }
}
