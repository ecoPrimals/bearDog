// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Primal Authority (UPA) registry discovery.

use super::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use beardog_errors::BearDogError;
use beardog_types::constants::domains::timeouts::HEALTH_CHECK_TIMEOUT;
use tracing::{info, warn};

impl PrimalDiscovery {
    /// Discover from UPA registry
    pub(crate) async fn discover_from_upa(
        &self,
        query: &DiscoveryQuery,
        registry_addr: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 UPA registry discovery at: {}", registry_addr);

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

        let socket_path = registry_addr.trim_start_matches("unix://");

        #[cfg(unix)]
        let connect_result = tokio::net::UnixStream::connect(socket_path).await;
        #[cfg(not(unix))]
        let connect_result: Result<tokio::net::TcpStream, std::io::Error> = Err(
            std::io::Error::new(std::io::ErrorKind::Unsupported, "UDS not available on this platform")
        );

        match connect_result {
            Ok(mut stream) => {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                let request_str =
                    serde_json::to_string(&request).map_err(|e| BearDogError::Network {
                        message: format!("Failed to serialize UPA request: {e}"),
                        category: beardog_errors::NetworkErrorCategory::Connection,
                    })?;
                stream.write_all(request_str.as_bytes()).await?;
                stream.write_all(b"\n").await?;

                let mut buffer = vec![0u8; beardog_types::constants::domains::buffers::UDP_PACKET_SIZE];
                let n = stream.read(&mut buffer).await?;
                let response_str = String::from_utf8_lossy(&buffer[..n]);

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
}
