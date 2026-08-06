// SPDX-License-Identifier: AGPL-3.0-or-later

//! tarpc binary RPC service (G64 Cephalization).
//!
//! Exposes a `.tarpc.sock` sibling socket alongside the primary JSON-RPC
//! `.sock`. Hot-path crypto operations use bincode framing — no JSON
//! serde roundtrip, no string encoding overhead.
//!
//! ## Socket Convention
//!
//! ```text
//! beardog-family123.sock          ← JSON-RPC  (always present, bootstrap + diagnostic)
//! beardog-family123.tarpc.sock    ← tarpc      (optional, high-perf intra-gate)
//! ```
//!
//! ## Module Structure
//!
//! - [`types`] — wire types, service trait, `TARPC_METHOD_COUNT`
//! - [`server`] — `BearDogRpcServer` implementation + tests

mod server;
mod types;

pub use server::BearDogRpcServer;
pub use types::*;

use std::sync::Arc;

use beardog_types::primal_identity::PrimalIdentity;
use tracing::{info, warn};

/// Derives the tarpc socket path from a JSON-RPC socket path.
///
/// Follows the ecosystem convention from `biomeos-primal-sdk::tarpc_transport`:
/// `beardog.sock` → `beardog.tarpc.sock`.
#[must_use]
pub fn tarpc_socket_path(jsonrpc_socket: &str) -> String {
    if let Some(base) = jsonrpc_socket.strip_suffix(".sock") {
        format!("{base}.tarpc.sock")
    } else {
        format!("{jsonrpc_socket}.tarpc")
    }
}

/// Spawns the tarpc listener on the `.tarpc.sock` sibling socket.
///
/// This is fire-and-forget: if the listener fails to bind, it logs a warning
/// and returns without blocking the JSON-RPC server.
///
/// # Errors
///
/// Returns `Err` if the Unix socket cannot be bound (e.g. path too long,
/// permissions). The caller should treat this as non-fatal.
pub async fn spawn_tarpc_listener(
    jsonrpc_socket_path: &str,
    identity: Arc<PrimalIdentity>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use futures::StreamExt;
    use tarpc::server::{BaseChannel, Channel};

    let tarpc_path = tarpc_socket_path(jsonrpc_socket_path);

    if std::path::Path::new(&tarpc_path).exists() {
        std::fs::remove_file(&tarpc_path).ok();
    }

    let incoming = tarpc::serde_transport::unix::listen(
        &tarpc_path,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;
    info!(path = %tarpc_path, method_count = TARPC_METHOD_COUNT, "tarpc listener bound");

    let server = BearDogRpcServer::new(identity);

    tokio::spawn(async move {
        futures::pin_mut!(incoming);
        while let Some(result) = incoming.next().await {
            match result {
                Ok(transport) => {
                    let channel = BaseChannel::with_defaults(transport);
                    let handler = server.clone();
                    tokio::spawn(channel.execute(handler.serve()).for_each(|resp| async {
                        tokio::spawn(resp);
                    }));
                }
                Err(e) => {
                    warn!(error = %e, "tarpc accept failed");
                }
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarpc_socket_path_convention() {
        assert_eq!(
            tarpc_socket_path("/tmp/beardog.sock"),
            "/tmp/beardog.tarpc.sock"
        );
        assert_eq!(
            tarpc_socket_path("/run/user/1000/biomeos/beardog-family123.sock"),
            "/run/user/1000/biomeos/beardog-family123.tarpc.sock"
        );
    }

    #[test]
    fn tarpc_socket_path_no_sock_suffix() {
        assert_eq!(tarpc_socket_path("/tmp/beardog"), "/tmp/beardog.tarpc");
    }
}
