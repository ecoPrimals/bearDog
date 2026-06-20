// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys::resolve_primal_name;
use tracing::{info, warn};

/// Best-effort registration with the ecosystem's IPC registry (non-fatal).
///
/// Attempts to connect to whatever orchestrator provides the registry capability
/// and register `BearDog`'s capabilities. The caller never knows which primal
/// provides the registry — capability-based discovery handles routing.
///
/// Failure is logged and swallowed per PRIMAL IPC Protocol v3.1:
/// registration SHOULD be attempted but MUST NOT prevent standalone operation.
pub(super) async fn attempt_orchestrator_registration(_socket_path: &str, _tcp_addr: Option<&str>) {
    use beardog_ipc::{Capability, OrchestratorRegistryClient};

    let Ok(client) = OrchestratorRegistryClient::connect().await else {
        info!("no IPC registry socket found (standalone operation)");
        return;
    };

    let capabilities = vec![Capability::Crypto, Capability::BTSP, Capability::Ed25519];

    if let Err(e) = client.register(&resolve_primal_name(), capabilities).await {
        warn!(error = %e, "IPC registry registration failed (non-fatal)");
    } else {
        info!("registered with ecosystem IPC registry");
    }
}
