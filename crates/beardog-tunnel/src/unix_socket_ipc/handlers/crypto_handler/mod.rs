// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crypto RPC handler: routes JSON-RPC methods to domain route modules (`router`, `signatures`, etc.).

use crate::btsp_provider::BeardogBtspProvider;
use crate::unix_socket_ipc::handlers::{HandlerResult, MethodHandler};
use std::sync::Arc;

pub(crate) mod aliases_and_beardog;
mod genetic;
mod hashing;
mod kex_aead;
mod method_list;
mod password_kdf;
pub(crate) mod purpose_key;
mod router;
mod signatures;
mod tls12_dot;
mod tls_ops;

/// Crypto RPC handler — routes all cryptographic JSON-RPC operations.
pub struct CryptoHandler;

impl MethodHandler for CryptoHandler {
    fn methods(&self) -> Vec<&'static str> {
        method_list::crypto_method_names()
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        router::dispatch(method, params, btsp_provider).await
    }
}

#[cfg(test)]
#[path = "../crypto_handler_tests.rs"]
mod tests;
