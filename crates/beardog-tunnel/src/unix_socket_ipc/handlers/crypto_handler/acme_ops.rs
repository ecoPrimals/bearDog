// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME Phase 2 crypto routing — songBird delegation surface.

use crate::unix_socket_ipc::handlers::crypto::{
    handle_build_csr, handle_ecdsa_p256_generate_signing_keypair, handle_jwk_thumbprint,
    handle_sign_jws_es256,
};
#[cfg(feature = "tls-x509")]
use crate::unix_socket_ipc::handlers::crypto::handle_parse_certificate;
use tracing::info;

/// # Errors
///
/// Returns an error if the underlying crypto operation fails.
pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, super::super::HandlerError> {
    match method {
        "crypto.ecdsa_p256_generate_signing_keypair" => {
            info!("Crypto: ecdsa_p256_generate_signing_keypair (ACME account/leaf key)");
            Ok(Some(
                handle_ecdsa_p256_generate_signing_keypair(params).await?,
            ))
        }

        "crypto.sign_jws_es256" => {
            info!("Crypto: sign_jws_es256 (ACME JWS request signing)");
            Ok(Some(handle_sign_jws_es256(params).await?))
        }

        "crypto.jwk_thumbprint" => {
            info!("Crypto: jwk_thumbprint (ACME HTTP-01 key authorization)");
            Ok(Some(handle_jwk_thumbprint(params).await?))
        }

        "x509.build_csr" => {
            info!("X509: build_csr (PKCS#10 for ACME certificate request)");
            Ok(Some(handle_build_csr(params).await?))
        }

        #[cfg(feature = "tls-x509")]
        "x509.parse_certificate" => {
            info!("X509: parse_certificate (certificate metadata extraction)");
            Ok(Some(handle_parse_certificate(params).await?))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use serde_json::json;

    #[tokio::test]
    async fn route_generate_signing_keypair() {
        let v = route("crypto.ecdsa_p256_generate_signing_keypair", None)
            .await
            .expect("route")
            .expect("keypair");
        assert_eq!(v["algorithm"], "ES256");
    }

    #[tokio::test]
    async fn route_build_csr() {
        let params = json!({
            "domains": ["route-test.example.com"],
        });
        let v = route("x509.build_csr", Some(&params))
            .await
            .expect("route")
            .expect("csr");
        assert!(v["csr_der"].as_str().is_some());
    }

    #[tokio::test]
    async fn route_unknown_returns_none() {
        assert!(
            route("acme.not_real", None)
                .await
                .expect("route")
                .is_none()
        );
    }
}
