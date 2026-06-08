// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! Lineage, entropy, challenge, and device-enrollment genetic crypto routing.

use crate::btsp_provider::BeardogBtspProvider;
use crate::unix_socket_ipc::crypto_handlers_genetic::{
    handle_derive_device_seed, handle_derive_lineage_beacon_key, handle_derive_lineage_key,
    handle_generate_challenge, handle_generate_lineage_proof, handle_mix_entropy,
    handle_respond_to_challenge, handle_sign_lineage_certificate, handle_verify_challenge_response,
    handle_verify_lineage, handle_verify_lineage_certificate,
};
use std::sync::Arc;
use tracing::info;

/// # Errors
///
/// Returns an error if key derivation fails.
pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<Option<serde_json::Value>, super::super::HandlerError> {
    match method {
        "genetic.derive_lineage_key" => {
            info!("🧬 Genetic: derive_lineage_key (lineage-based key derivation)");
            Ok(Some(
                handle_derive_lineage_key(params.ok_or_else(|| {
                    "Parameters required for genetic.derive_lineage_key".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.derive_lineage_beacon_key" => {
            info!("🌑 Genetic: derive_lineage_beacon_key (TRUE Dark Forest beacon key)");
            Ok(Some(
                handle_derive_lineage_beacon_key(params.ok_or_else(|| {
                    "Parameters required for genetic.derive_lineage_beacon_key".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.mix_entropy" => {
            info!("🌱 Genetic: mix_entropy (three-tier entropy hierarchy)");
            Ok(Some(
                handle_mix_entropy(
                    params
                        .ok_or_else(|| "Parameters required for genetic.mix_entropy".to_string())?,
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_lineage" => {
            info!("🔍 Genetic: verify_lineage (family relationship verification)");
            let birdsong = btsp_provider.birdsong_manager();
            Ok(Some(
                handle_verify_lineage(
                    params.ok_or_else(|| {
                        "Parameters required for genetic.verify_lineage".to_string()
                    })?,
                    &birdsong,
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.generate_lineage_proof" => {
            info!("🔐 Genetic: generate_lineage_proof (proof generation)");
            let birdsong = btsp_provider.birdsong_manager();
            Ok(Some(
                handle_generate_lineage_proof(
                    params.ok_or_else(|| {
                        "Parameters required for genetic.generate_lineage_proof".to_string()
                    })?,
                    &birdsong,
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.generate_challenge" => {
            info!("🎲 Genetic: generate_challenge (challenge generation)");
            Ok(Some(
                handle_generate_challenge(params.ok_or_else(|| {
                    "Parameters required for genetic.generate_challenge".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.respond_to_challenge" => {
            info!("🔐 Genetic: respond_to_challenge (challenge response)");
            Ok(Some(
                handle_respond_to_challenge(params.ok_or_else(|| {
                    "Parameters required for genetic.respond_to_challenge".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_challenge_response" => {
            info!("🔍 Genetic: verify_challenge_response (response verification)");
            Ok(Some(
                handle_verify_challenge_response(params.ok_or_else(|| {
                    "Parameters required for genetic.verify_challenge_response".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.derive_device_seed" => {
            info!("🧬 Genetic: derive_device_seed (unique device derivation)");
            Ok(Some(
                handle_derive_device_seed(params.ok_or_else(|| {
                    "Parameters required for genetic.derive_device_seed".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.sign_lineage_certificate" => {
            info!("🧬 Genetic: sign_lineage_certificate (device enrollment)");
            Ok(Some(
                handle_sign_lineage_certificate(params.ok_or_else(|| {
                    "Parameters required for genetic.sign_lineage_certificate".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_lineage_certificate" => {
            info!("🔍 Genetic: verify_lineage_certificate (certificate verification)");
            Ok(Some(
                handle_verify_lineage_certificate(params.ok_or_else(|| {
                    "Parameters required for genetic.verify_lineage_certificate".to_string()
                })?)
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "lineage.list" => {
            info!("📋 Lineage: list (enumerate all lineage chains)");
            let birdsong = btsp_provider.birdsong_manager();
            let chains = birdsong.list_lineage_chains();
            Ok(Some(serde_json::json!({
                "chains": chains,
                "count": chains.len(),
            })))
        }

        "lineage.verify" => {
            info!("🔍 Lineage: verify (semantic → genetic.verify_lineage)");
            let birdsong = btsp_provider.birdsong_manager();
            Ok(Some(
                handle_verify_lineage(
                    params.ok_or_else(|| "Parameters required for lineage.verify".to_string())?,
                    &birdsong,
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "lineage.get" => {
            info!("📖 Lineage: get (retrieve chain by ID)");
            let chain_id = params
                .and_then(|p| p.get("chain_id"))
                .and_then(|v| v.as_str())
                .ok_or("Missing required parameter: chain_id")?;
            let birdsong = btsp_provider.birdsong_manager();
            match birdsong.get_lineage_chain(chain_id) {
                Some(chain) => Ok(serde_json::to_value(chain)
                    .map(Some)
                    .map_err(|e| format!("Serialize: {e}"))?),
                None => Ok(Some(
                    serde_json::json!({ "error": "chain_not_found", "chain_id": chain_id }),
                )),
            }
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::HsmManager;
    use base64::Engine;
    use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;
    use serde_json::json;
    use std::sync::Arc;

    fn seed32() -> String {
        base64::engine::general_purpose::STANDARD.encode([11u8; 32])
    }

    async fn test_btsp() -> Arc<BeardogBtspProvider> {
        let hsm = Arc::new(HsmManager::new());
        let genetics =
            Arc::new(EcosystemGeneticEngine::new().expect("EcosystemGeneticEngine::new"));
        Arc::new(
            BeardogBtspProvider::new_for_testing(hsm, genetics)
                .await
                .expect("BeardogBtspProvider::new_for_testing"),
        )
    }

    #[tokio::test]
    async fn genetic_route_unknown_method_returns_none() {
        let btsp = test_btsp().await;
        let out = route("genetic.unknown_rpc", None, &btsp)
            .await
            .expect("route");
        assert!(out.is_none());
    }

    #[tokio::test]
    async fn genetic_route_derive_lineage_key_requires_params() {
        let btsp = test_btsp().await;
        let err = route("genetic.derive_lineage_key", None, &btsp)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("Parameters required") || err.contains("derive_lineage_key"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_mix_entropy_requires_params() {
        let btsp = test_btsp().await;
        let err = route("genetic.mix_entropy", None, &btsp)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("mix_entropy") || err.contains("Parameters required"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_verify_lineage_requires_params() {
        let btsp = test_btsp().await;
        let err = route("genetic.verify_lineage", None, &btsp)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("verify_lineage") || err.contains("Parameters required"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_derive_lineage_beacon_key_empty_seed_rejected() {
        let btsp = test_btsp().await;
        let err = route("genetic.derive_lineage_beacon_key", Some(&json!({})), &btsp)
            .await
            .expect_err("empty seed should be rejected");
        assert!(
            err.contains("lineage_seed") || err.contains("required"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_mix_entropy_empty_tiers_uses_machine_entropy() {
        let btsp = test_btsp().await;
        let out = route("genetic.mix_entropy", Some(&json!({})), &btsp)
            .await
            .expect("route")
            .expect("some");
        assert!(out.get("entropy").is_some());
        assert!(out.get("quality_score").is_some());
    }

    #[tokio::test]
    async fn genetic_route_derive_lineage_key_success() {
        let btsp = test_btsp().await;
        let s = seed32();
        let params = json!({
            "our_family_id": "fam-a",
            "peer_family_id": "fam-b",
            "context": "unit-test",
            "lineage_seed": s,
        });
        let out = route("genetic.derive_lineage_key", Some(&params), &btsp)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(out["method"], "Blake3-Lineage-KDF");
    }

    #[tokio::test]
    async fn genetic_route_generate_then_verify_lineage_proof() {
        let btsp = test_btsp().await;
        let s = seed32();
        let proof_req = json!({
            "our_family_id": "fam-a",
            "peer_family_id": "fam-b",
            "lineage_seed": s,
        });
        let proof_val = route("genetic.generate_lineage_proof", Some(&proof_req), &btsp)
            .await
            .expect("route")
            .expect("some");
        let proof = proof_val["proof"].as_str().expect("proof");

        let verify = json!({
            "our_family_id": "fam-a",
            "peer_family_id": "fam-b",
            "lineage_proof": proof,
            "lineage_seed": s,
        });
        let v = route("genetic.verify_lineage", Some(&verify), &btsp)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(v["valid"], true);
    }

    #[tokio::test]
    async fn lineage_list_returns_empty_chains() {
        let btsp = test_btsp().await;
        let out = route("lineage.list", None, &btsp)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(out["count"], 0);
        assert!(out["chains"].as_array().expect("chains array").is_empty());
    }

    #[tokio::test]
    async fn lineage_verify_delegates_to_genetic() {
        let btsp = test_btsp().await;
        let s = seed32();
        let proof_req = json!({
            "our_family_id": "fam-a",
            "peer_family_id": "fam-b",
            "lineage_seed": s,
        });
        let proof_val = route("genetic.generate_lineage_proof", Some(&proof_req), &btsp)
            .await
            .expect("route")
            .expect("some");
        let proof = proof_val["proof"].as_str().expect("proof");

        let verify = json!({
            "our_family_id": "fam-a",
            "peer_family_id": "fam-b",
            "lineage_proof": proof,
            "lineage_seed": s,
        });
        let v = route("lineage.verify", Some(&verify), &btsp)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(v["valid"], true);
    }

    #[tokio::test]
    async fn lineage_get_missing_chain() {
        let btsp = test_btsp().await;
        let params = json!({ "chain_id": "nonexistent" });
        let out = route("lineage.get", Some(&params), &btsp)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(out["error"], "chain_not_found");
    }

    #[tokio::test]
    async fn genetic_route_generate_challenge_success() {
        let btsp = test_btsp().await;
        let challenge = route(
            "genetic.generate_challenge",
            Some(&json!({
                "challenger_node_id": "n1",
                "target_family_id": "fam-t",
            })),
            &btsp,
        )
        .await
        .expect("route")
        .expect("some");
        assert!(challenge["nonce"].as_str().expect("nonce").len() >= 32);
        assert!(!challenge["challenge_id"].as_str().expect("id").is_empty());
    }
}
