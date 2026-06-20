// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build a signed attestation for Neural API registration using the primal's
/// unified Ed25519 identity key.
pub(super) fn build_neural_attestation(primal_name: &str) -> serde_json::Value {
    use beardog_tunnel::unix_socket_ipc::handlers::primal_signing::{
        canonical_announcement_message, sign_with_primal_identity,
    };

    let node_id = beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral(None);
    let version = env!("CARGO_PKG_VERSION");
    let methods: Vec<String> = Vec::new();
    let message = canonical_announcement_message(primal_name, version, &methods);
    let (signature, public_key) = sign_with_primal_identity(primal_name, &node_id, &message);

    serde_json::json!({
        "schema_version": 2,
        "algorithm": "ed25519",
        "public_key": public_key,
        "signature": signature,
        "signed_fields": ["primal", "version"],
    })
}
