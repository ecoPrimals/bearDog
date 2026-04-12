// SPDX-License-Identifier: AGPL-3.0-or-later

//! HKDF-based derivation for TLS and genetic lineage.

use crate::tarpc_types::{
    CryptoError, CryptoResult, LineageKey, LineageRequest, TlsSecrets, TlsSecretsRequest,
};

pub(super) async fn tls_derive_handshake_secrets(
    request: TlsSecretsRequest,
) -> CryptoResult<TlsSecrets> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hkdf = Hkdf::<Sha256>::new(None, &request.shared_secret);

    let mut client_secret = vec![0u8; 32];
    let mut server_secret = vec![0u8; 32];

    let client_info = format!(
        "tls13 c hs traffic {}",
        hex::encode(&request.transcript_hash)
    );
    let server_info = format!(
        "tls13 s hs traffic {}",
        hex::encode(&request.transcript_hash)
    );

    hkdf.expand(client_info.as_bytes(), &mut client_secret)
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("HKDF expand failed: {e}"),
        })?;

    hkdf.expand(server_info.as_bytes(), &mut server_secret)
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("HKDF expand failed: {e}"),
        })?;

    Ok(TlsSecrets {
        client_traffic_secret: client_secret,
        server_traffic_secret: server_secret,
    })
}

pub(super) async fn tls_derive_application_secrets(
    request: TlsSecretsRequest,
) -> CryptoResult<TlsSecrets> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hkdf = Hkdf::<Sha256>::new(None, &request.shared_secret);

    let mut client_secret = vec![0u8; 32];
    let mut server_secret = vec![0u8; 32];

    let client_info = format!(
        "tls13 c ap traffic {}",
        hex::encode(&request.transcript_hash)
    );
    let server_info = format!(
        "tls13 s ap traffic {}",
        hex::encode(&request.transcript_hash)
    );

    hkdf.expand(client_info.as_bytes(), &mut client_secret)
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("HKDF expand failed: {e}"),
        })?;

    hkdf.expand(server_info.as_bytes(), &mut server_secret)
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("HKDF expand failed: {e}"),
        })?;

    Ok(TlsSecrets {
        client_traffic_secret: client_secret,
        server_traffic_secret: server_secret,
    })
}

pub(super) async fn genetic_derive_lineage_key(
    request: LineageRequest,
) -> CryptoResult<LineageKey> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let info = format!(
        "beardog-lineage-{}-gen{}",
        request.context, request.generation
    );
    let hkdf = Hkdf::<Sha256>::new(None, &request.family_seed);

    let mut key = vec![0u8; 32];
    hkdf.expand(info.as_bytes(), &mut key)
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("Lineage derivation failed: {e}"),
        })?;

    Ok(LineageKey {
        key,
        generation: request.generation,
    })
}
