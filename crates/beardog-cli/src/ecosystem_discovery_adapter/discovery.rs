// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`beardog_core::ecosystem_integration::PrimalDiscoveryService`] implementation.

use super::EcosystemDiscoveryAdapter;
use beardog_core::ecosystem_integration::PrimalDiscoveryService;
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{UniversalCapabilityType, UniversalServiceDescriptor};
use tracing::info;

/// Implementation of `PrimalDiscoveryService` trait using `EcosystemListener`
///
/// Modern async/await patterns with proper error handling via `?` operator
impl PrimalDiscoveryService for EcosystemDiscoveryAdapter {
    async fn discover_by_capability(
        &self,
        capability: UniversalCapabilityType,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        info!("🔍 Discovering primals with capability: {:?}", capability);

        // Ensure listener is running and trigger discovery
        self.trigger_discovery().await?;

        // Query discovered primals from shared state
        let primals = self.discovered_primals.read().await;

        // Filter primals by capability (zero-copy iteration)
        let matching_descriptors: Vec<UniversalServiceDescriptor> = primals
            .values()
            .filter(|primal| Self::primal_has_capability(primal, &capability))
            .map(Self::primal_to_descriptor)
            .collect();

        info!(
            "✅ Found {} primal(s) with requested capability",
            matching_descriptors.len()
        );

        Ok(matching_descriptors)
    }

    async fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        info!(
            "📤 Sending JSON-RPC to discovered peer (capability-routed service_id={})",
            service.service_id
        );

        let proto = service.endpoint.protocol.to_lowercase();
        if proto == "http" || proto == "https" {
            return Err(BearDogError::network(format!(
                "IPC-first cross-primal messaging requires unix:// or ipc:// (or socket-directory peer key); \
                     refused HTTP endpoint for service {}",
                service.service_id
            )));
        }

        let mut client = match proto.as_str() {
            "unix" | "ipc" => {
                let path = service
                    .endpoint
                    .path
                    .as_ref()
                    .filter(|p| p.starts_with('/'))
                    .ok_or_else(|| {
                        BearDogError::invalid_input(
                            "unix/ipc endpoint requires an absolute socket path in endpoint.path",
                        )
                    })?;
                beardog_tower_atomic::Client::connect_unix_path(
                    std::path::Path::new(path),
                    &service.service_id,
                )
                .await
                .map_err(Self::tower_atomic_error)?
            }
            "tcp" | "tcp-ipc" => {
                return Err(BearDogError::network(format!(
                    "TCP IPC to {} is not supported in this adapter; use unix:// socket path or biomeos socket-directory discovery",
                    service.service_id
                )));
            }
            _ => beardog_tower_atomic::Client::connect(&service.service_id)
                .await
                .map_err(Self::tower_atomic_error)?,
        };

        let (method, params) = Self::jsonrpc_method_and_params(&payload);
        client
            .call(&method, params)
            .await
            .map_err(Self::tower_atomic_error)
    }
}
