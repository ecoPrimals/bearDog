// SPDX-License-Identifier: AGPL-3.0-or-later

//! Multi-method discovery orchestration.

use super::{DiscoveryMethod, DiscoveryQuery, PrimalDiscovery};
use beardog_errors::BearDogError;
use std::collections::HashSet;

impl PrimalDiscovery {
    /// Try multiple discovery methods
    pub(crate) async fn discover_multi(
        &self,
        query: &DiscoveryQuery,
        methods: &[DiscoveryMethod],
    ) -> Result<Vec<super::DiscoveredPrimal>, BearDogError> {
        let mut all_discovered = Vec::new();

        for method in methods {
            let result = match method {
                DiscoveryMethod::Environment => self.discover_from_env(query),
                DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                    self.discover_from_upa(query, registry_addr).await
                }
                DiscoveryMethod::Mdns { service_type } => {
                    self.discover_from_mdns(query, service_type).await
                }
                DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(query, domain).await,
                DiscoveryMethod::Multi(_) => continue,
            };

            if let Ok(mut discovered) = result {
                all_discovered.append(&mut discovered);
            }
        }

        let mut seen = HashSet::new();
        all_discovered.retain(|p| seen.insert(p.name.clone()));

        Ok(all_discovered)
    }
}
