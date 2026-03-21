// SPDX-License-Identifier: AGPL-3.0-only

//! Dispatches JSON-RPC crypto methods to domain-specific route modules.

use crate::btsp_provider::BeardogBtspProvider;
use std::sync::Arc;

use super::{
    aliases_and_beardog, genetic, hashing, kex_aead, password_kdf, signatures, tls_ops, tls12_dot,
};

pub async fn dispatch(
    method: &str,
    params: Option<&serde_json::Value>,
    _btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value, String> {
    if let Some(v) = signatures::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = kex_aead::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = hashing::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = password_kdf::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = tls_ops::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = tls12_dot::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = genetic::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = aliases_and_beardog::route(method, params).await? {
        return Ok(v);
    }

    Err(format!("Unknown crypto method: {method}"))
}
