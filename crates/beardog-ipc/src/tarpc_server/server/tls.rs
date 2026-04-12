// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS handshake signing helper (delegates to Ed25519).

use crate::tarpc_types::{CryptoResult, SignRequest, SignResponse, TlsSignRequest};

use super::signatures;

pub(super) async fn tls_sign_handshake(request: TlsSignRequest) -> CryptoResult<SignResponse> {
    signatures::sign_ed25519(SignRequest {
        data: request.transcript_hash,
        private_key: request.private_key,
    })
    .await
}
