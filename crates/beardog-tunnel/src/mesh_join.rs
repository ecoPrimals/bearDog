// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mesh join orchestration — reciprocal trust exchange over BTSP.
//!
//! After two gates complete a BTSP TCP handshake (proving shared `FAMILY_SEED`),
//! [`mesh_join`] orchestrates the key exchange: it sends our identity to the peer
//! and registers the peer's returned identity locally. Both sides end up with
//! each other in their [`TrustedIssuerRegistry`], enabling cross-gate ionic token
//! verification without manual env var seeding.

use std::borrow::Cow;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use beardog_errors::BearDogError;
use ed25519_dalek::VerifyingKey;
use serde_json::{Value, json};
use tracing::info;

use crate::tcp_ipc::client::BtspConnection;
use crate::trusted_issuer_registry::{
    TrustMethod, TrustedIssuerRegistry, did_from_verifying_key,
};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;

/// Result of a successful mesh join.
#[derive(Debug, Clone)]
pub struct MeshJoinResult {
    /// The remote gate's DID.
    pub remote_did: String,
    /// The remote gate's node/gate ID (if returned).
    pub remote_gate_id: Option<String>,
    /// Whether the remote newly registered us (vs already had us).
    pub we_were_registered: bool,
    /// Whether we newly registered the remote locally.
    pub peer_newly_registered: bool,
}

/// Perform a reciprocal trust exchange over an established BTSP connection.
///
/// This is the primary entry point for mesh join. After calling this, both
/// gates will have each other in their `TrustedIssuerRegistry` and can
/// verify each other's ionic tokens.
///
/// # Flow
///
/// 1. Derive our local signing key from `(primal_name, node_id)`
/// 2. Call `auth.exchange_trust` on the peer with our public key
/// 3. Parse the peer's response (their public key, DID, `gate_id`)
/// 4. Register the peer's key in our local registry
///
/// # Errors
///
/// Returns an error if:
/// - The RPC call fails (network, auth)
/// - The peer returns an error response
/// - The peer's public key is invalid
/// - Local registration fails (DID mismatch — should not happen with honest peers)
pub async fn mesh_join(
    conn: &mut BtspConnection,
    registry: &TrustedIssuerRegistry,
    primal_name: &str,
    node_id: &str,
    family_id: &str,
) -> Result<MeshJoinResult, BearDogError> {
    let local_sk = derive_primal_signing_key(primal_name, node_id);
    let local_vk = local_sk.verifying_key();
    let local_pk_b64 = BASE64_STANDARD.encode(local_vk.as_bytes());
    let local_did = did_from_verifying_key(&local_vk);

    info!(
        local_did = %local_did,
        session_id = %conn.session_id(),
        "mesh_join: sending auth.exchange_trust"
    );

    let response = conn
        .call(
            Cow::Borrowed("auth.exchange_trust"),
            Some(json!({
                "public_key": local_pk_b64,
                "did": local_did,
                "gate_id": node_id,
                "family_id": family_id,
            })),
        )
        .await?;

    if let Some(err) = response.get("error").and_then(Value::as_str) {
        return Err(BearDogError::system(format!(
            "peer rejected auth.exchange_trust: {err}"
        )));
    }

    let we_were_registered = response
        .get("registered")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let remote_pk_b64 = response
        .get("local_public_key")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            BearDogError::system("peer response missing local_public_key".to_string())
        })?;

    let remote_did = response
        .get("local_did")
        .and_then(Value::as_str)
        .ok_or_else(|| BearDogError::system("peer response missing local_did".to_string()))?;

    let remote_gate_id = response
        .get("local_gate_id")
        .and_then(Value::as_str)
        .map(String::from);

    let remote_pk_bytes = BASE64_STANDARD.decode(remote_pk_b64).map_err(|e| {
        BearDogError::system(format!("peer public_key invalid base64: {e}"))
    })?;

    let arr: [u8; 32] = remote_pk_bytes.try_into().map_err(|_| {
        BearDogError::system("peer public_key must be 32 bytes".to_string())
    })?;

    let remote_vk = VerifyingKey::from_bytes(&arr).map_err(|e| {
        BearDogError::system(format!("peer public_key invalid Ed25519: {e}"))
    })?;

    let peer_newly_registered = registry
        .register(
            remote_did,
            remote_vk,
            remote_gate_id.clone(),
            Some(family_id.to_owned()),
            TrustMethod::FamilySeed,
        )
        .map_err(|e| BearDogError::system(format!("failed to register peer: {e}")))?;

    info!(
        remote_did = %remote_did,
        remote_gate_id = ?remote_gate_id,
        peer_newly_registered,
        we_were_registered,
        "mesh_join: trust exchange complete"
    );

    Ok(MeshJoinResult {
        remote_did: remote_did.to_owned(),
        remote_gate_id,
        we_were_registered,
        peer_newly_registered,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth_event_bus::AuthEventBus;
    use crate::btsp_handshake::framing::{read_frame, write_frame};
    use crate::btsp_handshake::perform_server_handshake;
    use crate::btsp_handshake::FamilySeed;
    use crate::method_gate::{CallerContext, ConnectionOrigin};
    use crate::tcp_ipc::client::BtspTcpClient;
    use crate::trust_handlers::handle_auth_exchange_trust;
    use tokio::net::TcpListener;

    /// Full mesh_join E2E: real TCP, BTSP handshake, trust exchange, bidirectional
    /// registry population. Proves two gates can join the mesh via one call.
    #[tokio::test]
    async fn mesh_join_e2e_over_tcp() {
        let seed_bytes = b"mesh-join-test-family-seed-32b!!";
        let family_seed = FamilySeed::new(seed_bytes.to_vec());

        let server_registry = TrustedIssuerRegistry::new();
        let client_registry = TrustedIssuerRegistry::new();

        let server_primal = "beardog";
        let server_node_id = "eastgate";
        let client_primal = "beardog";
        let client_node_id = "flockgate";
        let family_id = "family_1";

        let event_bus = AuthEventBus::new(128);

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server_seed = family_seed.clone();
        let server_reg = server_registry.clone();
        let server_handle = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.expect("accept");
            let mut session =
                perform_server_handshake(&mut stream, &server_seed)
                    .await
                    .expect("server handshake");

            let frame = read_frame(&mut stream).await.expect("read frame");
            let plain = session.decrypt_frame(&frame).expect("decrypt");
            let request: Value = serde_json::from_slice(&plain).expect("parse");

            assert_eq!(request["method"], "auth.exchange_trust");

            let caller = CallerContext {
                bearer_token: None,
                peer: None,
                origin: ConnectionOrigin::Remote,
                validated_claims: None,
                btsp_family_verified: true,
                peer_id: None,
            };

            let result = handle_auth_exchange_trust(
                &server_reg,
                &event_bus,
                server_primal,
                server_node_id,
                &caller,
                request.get("params"),
            );

            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": request["id"],
            });
            let enc = session
                .encrypt_frame(response.to_string().as_bytes())
                .expect("encrypt");
            write_frame(&mut stream, &enc).await.expect("write");

            server_reg
        });

        let client_reg = client_registry.clone();
        let client_handle = tokio::spawn(async move {
            let client = BtspTcpClient::new(addr, family_seed);
            let mut conn = client.connect_and_handshake().await.expect("connect");

            let result = mesh_join(
                &mut conn,
                &client_reg,
                client_primal,
                client_node_id,
                family_id,
            )
            .await
            .expect("mesh_join");

            conn.close().await.ok();
            (result, client_reg)
        });

        let (server_result, client_result) = tokio::join!(server_handle, client_handle);
        let final_server_registry = server_result.expect("join server");
        let (join_result, final_client_registry) = client_result.expect("join client");

        // Client registered the server's key
        assert!(join_result.peer_newly_registered);
        assert!(join_result.we_were_registered);
        assert_eq!(join_result.remote_gate_id.as_deref(), Some(server_node_id));
        assert!(join_result.remote_did.starts_with("did:key:z6Mk"));

        // Server registry has the client's key
        assert_eq!(final_server_registry.len(), 1);
        // Client registry has the server's key
        assert_eq!(final_client_registry.len(), 1);

        // Verify the DIDs are correct (different node_ids → different keys)
        let server_did = {
            let sk = derive_primal_signing_key(server_primal, server_node_id);
            did_from_verifying_key(&sk.verifying_key())
        };
        let client_did = {
            let sk = derive_primal_signing_key(client_primal, client_node_id);
            did_from_verifying_key(&sk.verifying_key())
        };
        assert_ne!(server_did, client_did);

        // Server has client's DID, client has server's DID
        assert!(final_server_registry.get(&client_did).is_some());
        assert!(final_client_registry.get(&server_did).is_some());
    }
}
