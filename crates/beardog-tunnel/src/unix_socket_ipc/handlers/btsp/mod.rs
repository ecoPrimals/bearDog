// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP (`BearDog` Tunnel Security Protocol) handlers - UNIFIED
//!
//! Provides secure communication for both:
//! - **Internal Mode**: P2P mesh tunneling via genetic lineage (primals)
//! - **External Mode**: HTTPS communication via certificate trust (APIs)
//!
//! # Architecture
//!
//! BTSP Unified consolidates two communication patterns into a single API:
//! - Trust mode (genetic lineage vs. certificate) is the fundamental difference
//! - Same crypto foundation (X25519, ChaCha20-Poly1305, Ed25519) for both
//! - Backward compatible with existing BTSP internal mode calls

mod contact;
mod negotiation;
mod peer;
mod session;
mod tunnel;

use super::{HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use std::sync::Arc;

/// Handler for BTSP Unified methods
///
/// # Core Operations (6 methods, internal mode)
///
/// - `btsp.contact.exchange` (semantic) / `btsp.contact_exchange` — Exchange contact info via genetic lineage
/// - `btsp.tunnel.establish` / `btsp.tunnel_establish` — **UNIFIED**: Establish secure tunnel (internal OR external)
/// - `btsp.tunnel.encrypt` / `btsp.tunnel_encrypt` — Encrypt data through tunnel
/// - `btsp.tunnel.decrypt` / `btsp.tunnel_decrypt` — Decrypt data from tunnel
/// - `btsp.tunnel.status` / `btsp.tunnel_status` — Get tunnel status
/// - `btsp.tunnel.close` / `btsp.tunnel_close` — Close tunnel gracefully
///
/// # Server Surface (5 methods, handshake-as-a-service for other primals)
///
/// - `btsp.server.create_session` — Generate ephemeral keys + challenge for a calling primal
/// - `btsp.server.verify` — Verify a client's challenge response, derive session keys
/// - `btsp.server.export_keys` — Export wrapped session keys for the relay path
/// - `btsp.server.negotiate` — Re-negotiate cipher suite for an active session
/// - `btsp.server.status` — Report session store health and active session count
///
/// # Backward Compatibility
///
/// All existing BTSP calls work unchanged. Legacy `btsp.session.*` aliases
/// are preserved and route to the new `btsp.server.*` implementations.
pub struct BtspHandler {
    /// Persistent server-side session store for handshake-as-a-service.
    session_store: crate::btsp_handshake::BtspSessionStore,
}

impl MethodHandler for BtspHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Contact exchange (semantic `domain.operation` first; path-like = backward compat)
            "btsp.contact.exchange",
            "beardog./btsp/contact/exchange",
            "btsp.contact_exchange",
            "btsp.contact/exchange",
            // Tunnel establishment (UNIFIED - supports internal + external)
            "btsp.tunnel.establish",
            "beardog./btsp/tunnel/establish",
            "btsp.tunnel_establish",
            "btsp.tunnel/establish",
            // Tunnel encryption
            "btsp.tunnel.encrypt",
            "beardog./btsp/tunnel/encrypt",
            "btsp.tunnel_encrypt",
            "btsp.tunnel/encrypt",
            // Tunnel decryption
            "btsp.tunnel.decrypt",
            "beardog./btsp/tunnel/decrypt",
            "btsp.tunnel_decrypt",
            "btsp.tunnel/decrypt",
            // Tunnel status
            "btsp.tunnel.status",
            "beardog./btsp/tunnel/status",
            "btsp.tunnel_status",
            "btsp.tunnel/status",
            // Tunnel close
            "btsp.tunnel.close",
            "beardog./btsp/tunnel/close",
            "btsp.tunnel_close",
            "btsp.tunnel/close",
            // Unified BTSP methods (Phase 2+)
            "btsp.configure_tls",
            "btsp.verify_peer",
            "btsp.trust.seed",
            "btsp.tunnel_send_http",
            // Server surface: handshake-as-a-service for other primals
            // (canonical `btsp.server.*` namespace per primalSpring gap synthesis)
            "btsp.server.create_session",
            "btsp.server.verify",
            "btsp.server.export_keys",
            "btsp.server.negotiate",
            "btsp.server.status",
            // Phase 3: encrypted post-handshake channel negotiation
            "btsp.negotiate",
            // Legacy aliases (backward compat with pre-server-surface callers)
            "btsp.session.create",
            "btsp.session.verify",
            "btsp.session.negotiate",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        // Match on method (semantic `btsp.*` dot names, legacy underscores, path-like beardog.*)
        if method == "btsp.contact.exchange"
            || method.ends_with("contact_exchange")
            || method.contains("/contact/exchange")
        {
            self.handle_contact_exchange(params, btsp_provider).await
        } else if method == "btsp.tunnel.establish"
            || method.ends_with("tunnel_establish")
            || method.contains("/tunnel/establish")
        {
            self.handle_tunnel_establish(params, btsp_provider).await
        } else if method == "btsp.tunnel.encrypt"
            || method.ends_with("tunnel_encrypt")
            || method.contains("/tunnel/encrypt")
        {
            self.handle_tunnel_encrypt(params, btsp_provider).await
        } else if method == "btsp.tunnel.decrypt"
            || method.ends_with("tunnel_decrypt")
            || method.contains("/tunnel/decrypt")
        {
            self.handle_tunnel_decrypt(params, btsp_provider).await
        } else if method == "btsp.tunnel.status"
            || method.ends_with("tunnel_status")
            || method.contains("/tunnel/status")
        {
            self.handle_tunnel_status(params, btsp_provider).await
        } else if method == "btsp.tunnel.close"
            || method.ends_with("tunnel_close")
            || method.contains("/tunnel/close")
        {
            self.handle_tunnel_close(params, btsp_provider).await
        } else if method == "btsp.configure_tls" {
            self.handle_configure_tls(params, btsp_provider).await
        } else if method == "btsp.verify_peer" {
            self.handle_verify_peer(params, btsp_provider).await
        } else if method == "btsp.trust.seed" {
            self.handle_trust_seed(params, btsp_provider).await
        } else if method == "btsp.tunnel_send_http" {
            self.handle_tunnel_send_http(params, btsp_provider).await
        } else if method == "btsp.server.create_session" || method == "btsp.session.create" {
            self.handle_server_create_session(params).await
        } else if method == "btsp.server.verify" || method == "btsp.session.verify" {
            self.handle_server_verify(params).await
        } else if method == "btsp.server.export_keys" {
            self.handle_server_export_keys(params).await
        } else if method == "btsp.server.negotiate" || method == "btsp.session.negotiate" {
            self.handle_server_negotiate(params).await
        } else if method == "btsp.negotiate" {
            self.handle_phase3_negotiate(params).await
        } else if method == "btsp.server.status" {
            self.handle_server_status().await
        } else {
            Err(format!("Unknown BTSP method: {method}").into())
        }
    }
}

impl Default for BtspHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl BtspHandler {
    /// Create a new handler with a fresh session store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            session_store: crate::btsp_handshake::BtspSessionStore::new(),
        }
    }
}

#[cfg(test)]
#[path = "../btsp_tests.rs"]
mod tests;
