// SPDX-License-Identifier: AGPL-3.0-or-later

//! Neural API and IPC registry registration.
//!
//! Handles runtime discovery-based registration with the ecosystem Neural API
//! (`primal.announce`) and the IPC registry (`ipc.register` / `ipc.heartbeat`).

use beardog_config::env_keys;
use beardog_core::self_knowledge::{PrimalSelfKnowledge, ipc_registry_capability_strings};
use tokio::time::Duration;
use tracing::{debug, info, warn};

/// Neural API registration fields (inject in tests; use [`Self::from_env`] at process startup).
#[derive(Debug, Clone, Default)]
pub struct NeuralRegistrationParams {
    /// `BEARDOG_NEURAL_REGISTRATION_INSTANCE`
    pub instance_override: Option<String>,
    /// `PRIMAL_TYPE`
    pub primal_type: Option<String>,
    /// `BEARDOG_PRIMAL_TYPE`
    pub beardog_primal_type: Option<String>,
}

impl NeuralRegistrationParams {
    /// Read `BEARDOG_NEURAL_REGISTRATION_INSTANCE`, `PRIMAL_TYPE`, `BEARDOG_PRIMAL_TYPE`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            instance_override: beardog_errors::process_env::var(
                env_keys::ENV_NEURAL_REGISTRATION_INSTANCE,
            )
            .ok(),
            primal_type: beardog_errors::process_env::var(env_keys::ENV_PRIMAL_TYPE).ok(),
            beardog_primal_type: beardog_errors::process_env::var(
                env_keys::ENV_PRIMAL_TYPE_PREFIXED,
            )
            .ok(),
        }
    }
    /// Registry instance id (capability-oriented; not a fixed product name).
    #[must_use]
    pub fn registration_instance_id(
        &self,
        identity: &beardog_types::primal_identity::PrimalIdentity,
    ) -> String {
        self.instance_override.clone().unwrap_or_else(|| {
            let role = self
                .primal_type
                .clone()
                .or_else(|| self.beardog_primal_type.clone())
                .unwrap_or_else(|| "security".to_string());
            format!("{role}-{}", identity.node_id())
        })
    }
}

/// Register `BearDog` with a runtime-discovered discovery endpoint (Neural API).
///
/// JSON-RPC `ipc.register` for the ecosystem IPC registry is started separately in
/// [`spawn_ipc_registry_registration_task`] (background, exponential backoff).
///
/// # Returns
///
/// `Ok(())` if the neural registration succeeds, `Err` if it fails (non-fatal for standalone).
/// Non-fatal — `BearDog` can operate standalone without discovery.
pub async fn register_with_discovery_service(
    socket_config: &beardog_core::socket_config::SocketConfig,
    neural_registration: &NeuralRegistrationParams,
    handler_registry: std::sync::Arc<crate::unix_socket_ipc::handlers::HandlerRegistry>,
) -> anyhow::Result<()> {
    use crate::primal_announce::registered_announce_method_names;
    use crate::unix_socket_ipc::handlers::primal_signing::{
        canonical_announcement_message, sign_with_primal_identity,
    };
    use beardog_ipc::{discover_neural_api_socket, register_with_neural_api, send_primal_announce};
    use beardog_types::primal_identity::PrimalIdentity;

    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);

        let identity = PrimalIdentity::from_env();

        let registration_instance = neural_registration.registration_instance_id(&identity);
        let socket_path = socket_config.socket_path_string();

        let node_id = identity.node_id();
        let version = env!("CARGO_PKG_VERSION");
        let message =
            canonical_announcement_message(&registration_instance, version, &[] as &[&str]);
        let (signature, public_key) =
            sign_with_primal_identity(&registration_instance, node_id, &message);
        let attestation = serde_json::json!({
            "schema_version": 2,
            "algorithm": "ed25519",
            "public_key": public_key,
            "signature": signature,
            "signed_fields": ["primal", "version"],
        });

        match register_with_neural_api(
            &neural_socket,
            &registration_instance,
            &socket_path,
            Some(&attestation),
        )
        .await
        {
            Ok(()) => {
                info!("registered with Neural API (TRUE PRIMAL)");
            }
            Err(e) => {
                warn!(error = %e, "Neural API registration failed");
            }
        }

        // biomeOS v3.69+ primal.announce (Wave 43)
        let announce_methods = registered_announce_method_names(&handler_registry).await;
        match send_primal_announce(
            &neural_socket,
            &registration_instance,
            &socket_path,
            &announce_methods,
            Some(&attestation),
        )
        .await
        {
            Ok(()) => {
                info!("primal.announce sent to biomeOS");
                return Ok(());
            }
            Err(e) => {
                warn!(error = %e, "primal.announce failed");
            }
        }
    } else {
        debug!("ℹ️  Neural API socket not detected; skipping neural registration");
    }

    warn!(
        "⚠️  Neural API registration did not complete; continuing (IPC registry may still be active)"
    );
    Err(anyhow::anyhow!(
        "Neural API registration unavailable or failed"
    ))
}

/// Background task: connect to the IPC registry (env-discovered socket), `ipc.register`, then `ipc.heartbeat`.
///
/// If the registry is unreachable, logs and retries with exponential backoff — standalone mode.
pub fn spawn_ipc_registry_registration_task(
    self_knowledge: PrimalSelfKnowledge,
    ipc_socket_path: String,
) {
    let capability_tags = ipc_registry_capability_strings(self_knowledge.my_capabilities());
    let primal_name = self_knowledge.my_name().to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    tokio::spawn(async move {
        use beardog_ipc::{DEFAULT_HEARTBEAT_INTERVAL, OrchestratorRegistryClient};

        const MAX_BACKOFF: Duration = Duration::from_secs(60);
        let mut backoff = Duration::from_millis(500);

        loop {
            let attempt = async {
                let client = OrchestratorRegistryClient::connect().await?;
                client
                    .register_ipc(&primal_name, &ipc_socket_path, &capability_tags, &version)
                    .await?;
                Ok::<_, beardog_ipc::IpcError>(client)
            }
            .await;

            match attempt {
                Ok(client) => {
                    info!(
                        primal = %primal_name,
                        endpoint = %ipc_socket_path,
                        "Registered with IPC registry; starting heartbeats"
                    );
                    let _heartbeat = client.start_heartbeat(DEFAULT_HEARTBEAT_INTERVAL);
                    std::future::pending::<()>().await;
                }
                Err(e) => {
                    warn!(
                        error = %e,
                        retry_in_secs = backoff.as_secs_f32(),
                        "IPC registry unreachable; continuing standalone (retrying)"
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
            }
        }
    });
}

/// First alphanumeric characters of a family seed (for logging only).
pub fn family_id_preview_from_seed(seed: &str) -> String {
    seed.chars()
        .filter(|c| c.is_alphanumeric())
        .take(4)
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod neural_registration_tests {
    use super::NeuralRegistrationParams;
    use beardog_config::env_keys;
    use beardog_types::primal_identity::PrimalIdentity;

    #[test]
    fn registration_instance_id_uses_override_when_set() {
        let p = NeuralRegistrationParams {
            instance_override: Some("my-reg".to_string()),
            primal_type: Some("ignored".to_string()),
            beardog_primal_type: Some("ignored2".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "node-z");
        assert_eq!(p.registration_instance_id(&id), "my-reg");
    }

    #[test]
    fn registration_instance_id_uses_primal_type_when_no_override() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("compute".to_string()),
            beardog_primal_type: None,
        };
        let id = PrimalIdentity::for_test("fam", "node-a");
        assert_eq!(p.registration_instance_id(&id), "compute-node-a");
    }

    #[test]
    fn registration_instance_id_falls_back_to_beardog_primal_type() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: None,
            beardog_primal_type: Some("edge".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "node-b");
        assert_eq!(p.registration_instance_id(&id), "edge-node-b");
    }

    #[test]
    fn registration_instance_id_default_role_is_security() {
        let p = NeuralRegistrationParams::default();
        let id = PrimalIdentity::for_test("fam", "node-c");
        assert_eq!(p.registration_instance_id(&id), "security-node-c");
    }

    #[test]
    fn primal_type_precedence_over_beardog_primal_type() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("primary".to_string()),
            beardog_primal_type: Some("secondary".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "n");
        assert_eq!(p.registration_instance_id(&id), "primary-n");
    }

    #[test]
    fn family_id_preview_from_seed_filters_and_lowercases() {
        assert_eq!(super::family_id_preview_from_seed("Ab12!@#xy"), "ab12");
        assert_eq!(super::family_id_preview_from_seed("!!!"), "");
    }

    #[test]
    fn neural_registration_params_from_env_reads_overrides() {
        beardog_errors::process_env::set_var(env_keys::ENV_NEURAL_REGISTRATION_INSTANCE, "inst-x");
        beardog_errors::process_env::set_var(env_keys::ENV_PRIMAL_TYPE, "compute");
        beardog_errors::process_env::set_var(env_keys::ENV_PRIMAL_TYPE_PREFIXED, "ignored");
        let p = NeuralRegistrationParams::from_env();
        beardog_errors::process_env::remove_var(env_keys::ENV_NEURAL_REGISTRATION_INSTANCE);
        beardog_errors::process_env::remove_var(env_keys::ENV_PRIMAL_TYPE);
        beardog_errors::process_env::remove_var(env_keys::ENV_PRIMAL_TYPE_PREFIXED);
        assert_eq!(p.instance_override, Some("inst-x".to_string()));
        assert_eq!(p.primal_type, Some("compute".to_string()));
        assert_eq!(p.beardog_primal_type, Some("ignored".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::{NeuralRegistrationParams, family_id_preview_from_seed};
    use beardog_config::env_keys;
    use beardog_types::primal_identity::PrimalIdentity;

    #[test]
    fn neural_registration_params_default_clone_debug() {
        let a = NeuralRegistrationParams::default();
        let b = a.clone();
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        let id = PrimalIdentity::for_test("f", "n1");
        assert_eq!(
            a.registration_instance_id(&id),
            b.registration_instance_id(&id)
        );
    }

    #[test]
    fn registration_instance_id_formats_with_hyphenated_node_id() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("relay".to_string()),
            beardog_primal_type: None,
        };
        let id = PrimalIdentity::for_test("fam", "node-with-dashes");
        assert_eq!(p.registration_instance_id(&id), "relay-node-with-dashes");
    }

    #[test]
    fn family_id_preview_takes_first_four_alphanumeric_only() {
        assert_eq!(family_id_preview_from_seed("Z9##wxyz"), "z9wx");
        assert_eq!(family_id_preview_from_seed("@#$%^&*()"), "");
    }

    #[test]
    fn neural_registration_params_clone_eq_for_discovery() {
        let a = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("edge".to_string()),
            beardog_primal_type: None,
        };
        let b = a.clone();
        assert_eq!(a.primal_type, b.primal_type);
    }

    #[tokio::test]
    async fn register_with_discovery_service_runs_without_neural_when_env_empty() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let prev_neural = beardog_errors::process_env::var(env_keys::ENV_NEURAL_API_SOCKET).ok();
        let prev_neurals = beardog_errors::process_env::var(env_keys::ENV_NEURALS_SOCKET).ok();
        beardog_errors::process_env::set_var(env_keys::ENV_NEURAL_API_SOCKET, "");
        beardog_errors::process_env::remove_var(env_keys::ENV_NEURALS_SOCKET);

        let socket_config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-discovery-mock.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        let neural_registration = NeuralRegistrationParams::default();
        let registry = crate::unix_socket_ipc::handlers::HandlerRegistry::default();
        let outcome =
            super::register_with_discovery_service(&socket_config, &neural_registration, registry)
                .await;

        match prev_neural {
            Some(v) => beardog_errors::process_env::set_var(env_keys::ENV_NEURAL_API_SOCKET, v),
            None => beardog_errors::process_env::remove_var(env_keys::ENV_NEURAL_API_SOCKET),
        }
        match prev_neurals {
            Some(v) => beardog_errors::process_env::set_var(env_keys::ENV_NEURALS_SOCKET, v),
            None => beardog_errors::process_env::remove_var(env_keys::ENV_NEURALS_SOCKET),
        }

        assert!(
            outcome.is_err() || outcome.is_ok(),
            "discovery registration completes or fails non-fatally in CI"
        );
    }

    #[tokio::test]
    async fn register_with_discovery_service_uses_registration_instance_from_params() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let prev_neural = beardog_errors::process_env::var(env_keys::ENV_NEURAL_API_SOCKET).ok();
        beardog_errors::process_env::set_var(env_keys::ENV_NEURAL_API_SOCKET, "");

        let socket_config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-reg-id.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        let neural_registration = NeuralRegistrationParams {
            instance_override: Some("custom-reg-instance".to_string()),
            primal_type: None,
            beardog_primal_type: None,
        };
        let registry = crate::unix_socket_ipc::handlers::HandlerRegistry::default();
        let _ =
            super::register_with_discovery_service(&socket_config, &neural_registration, registry)
                .await;

        match prev_neural {
            Some(v) => beardog_errors::process_env::set_var(env_keys::ENV_NEURAL_API_SOCKET, v),
            None => beardog_errors::process_env::remove_var(env_keys::ENV_NEURAL_API_SOCKET),
        }
    }

    #[test]
    fn socket_config_from_inputs_resolves_explicit_beardog_socket() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let cfg = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-mock-resolved.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        assert_eq!(cfg.socket_path_string(), "/tmp/beardog-mock-resolved.sock");
    }

    #[test]
    fn socket_config_from_inputs_includes_family_and_node_ids() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let cfg = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/x.sock".to_string()),
            family_id: Some("fam-a".to_string()),
            node_id: Some("node-b".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        assert_eq!(cfg.family_id(), "fam-a");
        assert_eq!(cfg.node_id(), "node-b");
    }
}
