// SPDX-License-Identifier: AGPL-3.0-only

//! Lineage, entropy, challenge, and device-enrollment genetic crypto routing.

use crate::unix_socket_ipc::crypto_handlers_genetic::{
    handle_derive_device_seed, handle_derive_lineage_beacon_key, handle_derive_lineage_key,
    handle_generate_challenge, handle_generate_lineage_proof, handle_mix_entropy,
    handle_respond_to_challenge, handle_sign_lineage_certificate, handle_verify_challenge_response,
    handle_verify_lineage, handle_verify_lineage_certificate,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "genetic.derive_lineage_key" => {
            info!("🧬 Genetic: derive_lineage_key (lineage-based key derivation)");
            Ok(Some(
                handle_derive_lineage_key(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_lineage_key".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.derive_lineage_beacon_key" => {
            info!("🌑 Genetic: derive_lineage_beacon_key (TRUE Dark Forest beacon key)");
            Ok(Some(
                handle_derive_lineage_beacon_key(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_lineage_beacon_key".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.mix_entropy" => {
            info!("🌱 Genetic: mix_entropy (three-tier entropy hierarchy)");
            Ok(Some(
                handle_mix_entropy(
                    params
                        .ok_or_else(|| "Parameters required for genetic.mix_entropy".to_string())?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_lineage" => {
            info!("🔍 Genetic: verify_lineage (family relationship verification)");
            Ok(Some(
                handle_verify_lineage(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_lineage".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.generate_lineage_proof" => {
            info!("🔐 Genetic: generate_lineage_proof (proof generation)");
            Ok(Some(
                handle_generate_lineage_proof(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.generate_lineage_proof".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.generate_challenge" => {
            info!("🎲 Genetic: generate_challenge (challenge generation)");
            Ok(Some(
                handle_generate_challenge(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.generate_challenge".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.respond_to_challenge" => {
            info!("🔐 Genetic: respond_to_challenge (challenge response)");
            Ok(Some(
                handle_respond_to_challenge(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.respond_to_challenge".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_challenge_response" => {
            info!("🔍 Genetic: verify_challenge_response (response verification)");
            Ok(Some(
                handle_verify_challenge_response(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_challenge_response".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.derive_device_seed" => {
            info!("🧬 Genetic: derive_device_seed (unique device derivation)");
            Ok(Some(
                handle_derive_device_seed(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_device_seed".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.sign_lineage_certificate" => {
            info!("🧬 Genetic: sign_lineage_certificate (device enrollment)");
            Ok(Some(
                handle_sign_lineage_certificate(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.sign_lineage_certificate".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        "genetic.verify_lineage_certificate" => {
            info!("🔍 Genetic: verify_lineage_certificate (certificate verification)");
            Ok(Some(
                handle_verify_lineage_certificate(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_lineage_certificate".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())?,
            ))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod genetic_route_tests {
    use super::route;

    #[tokio::test]
    async fn genetic_route_unknown_method_returns_none() {
        let out = route("genetic.unknown_rpc", None).await.expect("route");
        assert!(out.is_none());
    }

    #[tokio::test]
    async fn genetic_route_derive_lineage_key_requires_params() {
        let err = route("genetic.derive_lineage_key", None)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("Parameters required") || err.contains("derive_lineage_key"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_mix_entropy_requires_params() {
        let err = route("genetic.mix_entropy", None)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("mix_entropy") || err.contains("Parameters required"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn genetic_route_verify_lineage_requires_params() {
        let err = route("genetic.verify_lineage", None)
            .await
            .expect_err("expected parameter error");
        assert!(
            err.contains("verify_lineage") || err.contains("Parameters required"),
            "{err}"
        );
    }
}
