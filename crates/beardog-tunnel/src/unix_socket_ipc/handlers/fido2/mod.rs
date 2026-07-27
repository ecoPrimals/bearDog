// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2/CTAP2 IPC handler — hardware-attested authentication and credential
//! management via USB security keys.
//!
//! Exposes `BearDog`'s CTAP2 infrastructure as JSON-RPC methods so downstream
//! primals (lithoSpore, etc.) can request hardware-attested signatures without
//! embedding HID or CTAP2 crate dependencies.
//!
//! # Methods
//!
//! - `beardog.fido2.discover` — enumerate connected FIDO2 devices
//! - `beardog.fido2.register` — create a credential (CTAP2 `MakeCredential`)
//! - `beardog.fido2.authenticate` — get an assertion (CTAP2 `GetAssertion`)
//! - `beardog.fido2.entropy` — single-tap hardware entropy harvest
//! - `beardog.fido2.ceremony` — multi-tap entropy ceremony (Tier 3)
//!
//! # Feature Gate
//!
//! Real CTAP2 operations require the `ctap2` feature. Without it, discovery
//! returns an empty list and credential operations return a clear capability
//! error guiding the caller to enable the feature.
//!
//! # Security Model
//!
//! - Physical presence is enforced by the authenticator (user touch)
//! - Private keys never leave the security key
//! - `BearDog` relays CTAP2 frames, never holds credential secrets

mod attest_enrollment;
mod authenticate;
mod ceremony;
mod discover;
mod entropy;
#[cfg(feature = "ctap2")]
mod helpers;
mod register;

#[cfg(test)]
mod tests;

use super::{HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use serde_json::Value;
use std::sync::Arc;

/// FIDO2/CTAP2 handler for hardware-attested authentication.
#[derive(Default)]
pub struct Fido2Handler;

impl Fido2Handler {
    /// Create a new FIDO2 handler.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl MethodHandler for Fido2Handler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "beardog.fido2.discover",
            "beardog.fido2.register",
            "beardog.fido2.authenticate",
            "beardog.fido2.entropy",
            "beardog.fido2.ceremony",
            "beardog.fido2.attest_enrollment",
            "fido2.verify_attestation",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match method {
            "beardog.fido2.discover" => discover::handle_fido2_discover(params).await,
            "beardog.fido2.register" => register::handle_fido2_register(params).await,
            "beardog.fido2.authenticate" => authenticate::handle_fido2_authenticate(params).await,
            "beardog.fido2.entropy" => entropy::handle_fido2_entropy(params).await,
            "beardog.fido2.ceremony" => ceremony::handle_fido2_ceremony(params).await,
            "beardog.fido2.attest_enrollment" => {
                attest_enrollment::handle_fido2_attest_enrollment(params).await
            }
            "fido2.verify_attestation" => {
                attest_enrollment::handle_fido2_verify_attestation(params).await
            }
            _ => Err(format!("Unknown FIDO2 method: {method}").into()),
        }
    }
}
