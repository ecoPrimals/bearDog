// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pre-dispatch capability gate for JSON-RPC methods (JH-0).
//!
//! Every incoming RPC call passes through [`MethodGate::check`] *before*
//! reaching `HandlerRegistry::route`. The gate classifies methods into
//! [`MethodAccessLevel::Public`] (health probes, identity, capability
//! advertisement — always allowed) and [`MethodAccessLevel::Protected`]
//! (require a valid capability token once enforcement is activated).
//!
//! Two enforcement modes:
//! - **Permissive** (default): protected methods are logged but allowed,
//!   preserving backward compatibility during ecosystem rollout.
//! - **Enforced**: protected methods without a valid token are rejected
//!   with `PERMISSION_DENIED` (-32001).
//!
//! Caller identity is extracted from `SO_PEERCRED` on Unix sockets once
//! the Rust API stabilizes. Until then, the gate operates on bearer tokens
//! and connection origin.
//!
//! Implements the ecosystem standard defined in
//! `primalSpring/wateringHole/METHOD_GATE_STANDARD.md`.

use crate::ionic_token::{IonicTokenPayload, TokenError, scope_covers_method, verify_ionic_token};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_verifying_key;
use crate::unix_socket_ipc::types::JsonRpcError;
use ed25519_dalek::VerifyingKey;

/// Access level for a JSON-RPC method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodAccessLevel {
    /// Health probes, identity, capability advertisement — always allowed.
    Public,
    /// Requires a valid capability token when enforcement is active.
    Protected,
}

/// Prefix patterns that are always public.
const PUBLIC_METHOD_PREFIXES: &[&str] = &["health."];

/// Exact method names that are always public.
const PUBLIC_METHODS: &[&str] = &[
    "identity.get",
    "identity.create",
    "capabilities.list",
    "capability.list",
    "lifecycle.status",
    "auth.check",
    "auth.mode",
    "auth.peer_info",
    "auth.issue_ionic",
    "auth.verify_ionic",
];

/// Classify a method string into its access level.
#[must_use]
pub fn classify_method(method: &str) -> MethodAccessLevel {
    if PUBLIC_METHODS.contains(&method) {
        return MethodAccessLevel::Public;
    }
    for prefix in PUBLIC_METHOD_PREFIXES {
        if method.starts_with(prefix) {
            return MethodAccessLevel::Public;
        }
    }
    MethodAccessLevel::Protected
}

/// Peer credentials extracted from `SO_PEERCRED` on Unix sockets.
///
/// Uses only the stable subset of `std::os::unix::net::UCred`:
/// `uid` (stable since 1.75) and `pid` (stable `Option<i32>`).
/// GID is deferred until `peer_credentials_unix_socket` stabilizes.
#[derive(Debug, Clone)]
pub struct PeerCredentials {
    /// Process ID of the caller (if available).
    pub pid: Option<u32>,
    /// User ID of the caller.
    pub uid: u32,
}

/// Identity and authorization context for an incoming RPC call.
#[derive(Debug, Clone)]
pub struct CallerContext {
    /// Optional bearer / capability token sent in the request.
    pub bearer_token: Option<String>,
    /// Peer credentials from `SO_PEERCRED` (Unix socket only).
    pub peer: Option<PeerCredentials>,
    /// Where the connection came from.
    pub origin: ConnectionOrigin,
    /// Validated claims from a successfully verified ionic token.
    /// Populated by [`MethodGate::check`] when the bearer token passes
    /// full cryptographic + expiry verification.
    pub validated_claims: Option<IonicTokenPayload>,
}

/// How the caller connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionOrigin {
    /// Local Unix domain socket.
    Unix,
    /// TCP loopback (`127.0.0.1` / `::1`).
    Loopback,
    /// Remote TCP connection.
    Remote,
}

impl CallerContext {
    /// Create a caller context for a Unix domain socket connection.
    ///
    /// Peer credentials (`SO_PEERCRED`) are not extracted here because
    /// `std::os::unix::net::UnixStream::peer_cred()` is still behind the
    /// unstable `peer_credentials_unix_socket` feature gate and the crate
    /// uses `#![forbid(unsafe_code)]`. Once the API stabilizes (or a safe
    /// wrapper like `rustix` is adopted), this method will populate
    /// `PeerCredentials` automatically.
    #[must_use]
    pub const fn from_unix() -> Self {
        Self {
            bearer_token: None,
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        }
    }

    /// Build a caller context for loopback TCP with no peer credentials.
    #[must_use]
    pub const fn loopback() -> Self {
        Self {
            bearer_token: None,
            peer: None,
            origin: ConnectionOrigin::Loopback,
            validated_claims: None,
        }
    }

    /// Build a caller context for a remote TCP connection.
    #[must_use]
    pub const fn remote() -> Self {
        Self {
            bearer_token: None,
            peer: None,
            origin: ConnectionOrigin::Remote,
            validated_claims: None,
        }
    }
}

/// Enforcement mode for the method gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementMode {
    /// Log violations but allow all calls (backward-compatible default).
    Permissive,
    /// Reject unauthenticated calls to protected methods.
    Enforced,
}

impl EnforcementMode {
    /// Resolve from `BEARDOG_AUTH_MODE` env var.
    /// Defaults to `Permissive` if unset or unrecognized.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var("BEARDOG_AUTH_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "enforced" | "enforce" | "strict" => Self::Enforced,
            _ => Self::Permissive,
        }
    }

    /// Human-readable label for diagnostics and `auth.mode` responses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permissive => "permissive",
            Self::Enforced => "enforced",
        }
    }
}

/// Pre-dispatch gate that checks caller authorization before method execution.
#[derive(Debug)]
pub struct MethodGate {
    mode: EnforcementMode,
    /// Ed25519 verifying key of the issuing primal (for token signature checks).
    verifying_key: VerifyingKey,
    /// Primal name (for token issuance in dispatch handlers).
    primal_name: String,
    /// Node ID (for token issuance in dispatch handlers).
    node_id: String,
}

impl MethodGate {
    /// Create a gate with explicit configuration.
    #[must_use]
    pub fn new(mode: EnforcementMode, primal_name: &str, node_id: &str) -> Self {
        Self {
            mode,
            verifying_key: derive_primal_verifying_key(primal_name, node_id),
            primal_name: primal_name.to_owned(),
            node_id: node_id.to_owned(),
        }
    }

    /// Create a gate from the environment (`BEARDOG_AUTH_MODE`) and primal identity.
    #[must_use]
    pub fn from_env(primal_name: &str, node_id: &str) -> Self {
        Self::new(EnforcementMode::from_env(), primal_name, node_id)
    }

    /// Current enforcement mode.
    #[must_use]
    pub const fn mode(&self) -> EnforcementMode {
        self.mode
    }

    /// The primal's verifying key (for external verification or handlers).
    #[must_use]
    pub const fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Primal name.
    #[must_use]
    pub fn primal_name(&self) -> &str {
        &self.primal_name
    }

    /// Node ID.
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Pre-dispatch authorization check.
    ///
    /// For public methods, always returns `Ok(())`.
    ///
    /// For protected methods, performs full ionic token verification:
    /// decode -> Ed25519 signature check -> expiry check -> scope check.
    /// On success, populates `caller.validated_claims`.
    ///
    /// # Errors
    ///
    /// Returns `JsonRpcError` with `UNAUTHORIZED` (-32000) for invalid/expired
    /// tokens, or `PERMISSION_DENIED` (-32001) for insufficient scope or
    /// missing tokens in `Enforced` mode.
    pub fn check(&self, method: &str, caller: &mut CallerContext) -> Result<(), JsonRpcError> {
        let level = classify_method(method);

        if level == MethodAccessLevel::Public {
            return Ok(());
        }

        if let Some(ref token_str) = caller.bearer_token {
            match verify_ionic_token(token_str, &self.verifying_key) {
                Ok(payload) => {
                    if !scope_covers_method(&payload.scope, method) {
                        match self.mode {
                            EnforcementMode::Permissive => {
                                tracing::warn!(
                                    method,
                                    scope = ?payload.scope,
                                    "method gate: token scope insufficient (permissive — allowing)"
                                );
                            }
                            EnforcementMode::Enforced => {
                                return Err(JsonRpcError::permission_denied(method));
                            }
                        }
                    }
                    caller.validated_claims = Some(payload);
                    return Ok(());
                }
                Err(e) => match self.mode {
                    EnforcementMode::Permissive => {
                        tracing::warn!(
                            method,
                            error = %e,
                            "method gate: token verification failed (permissive — allowing)"
                        );
                        return Ok(());
                    }
                    EnforcementMode::Enforced => {
                        let code = match e {
                            TokenError::Expired { .. }
                            | TokenError::InvalidSignature
                            | TokenError::Malformed(_)
                            | TokenError::UnsupportedFormat(_) => JsonRpcError::UNAUTHORIZED,
                        };
                        return Err(JsonRpcError {
                            code,
                            message: format!("token verification failed: {e}"),
                            data: Some(serde_json::json!({"method": method})),
                        });
                    }
                },
            }
        }

        // No token at all
        match self.mode {
            EnforcementMode::Permissive => {
                tracing::warn!(
                    method,
                    caller_uid = caller.peer.as_ref().map(|p| p.uid),
                    caller_pid = caller.peer.as_ref().and_then(|p| p.pid),
                    origin = ?caller.origin,
                    "method gate: unauthenticated call to protected method (permissive — allowing)"
                );
                Ok(())
            }
            EnforcementMode::Enforced => {
                tracing::warn!(
                    method,
                    caller_uid = caller.peer.as_ref().map(|p| p.uid),
                    caller_pid = caller.peer.as_ref().and_then(|p| p.pid),
                    origin = ?caller.origin,
                    "method gate: REJECTED unauthenticated call to protected method"
                );
                Err(JsonRpcError::permission_denied(method))
            }
        }
    }
}

// ── Inline auth method handlers ────────────────────────────────────────
//
// These are handled at the dispatch layer (not in HandlerRegistry) because
// they need CallerContext and MethodGate state.

/// Handle `auth.check` — is the caller authenticated?
#[must_use]
pub fn handle_auth_check(caller: &CallerContext) -> serde_json::Value {
    let authenticated = caller.bearer_token.is_some();
    let mut result = serde_json::json!({
        "authenticated": authenticated,
        "origin": format!("{:?}", caller.origin).to_lowercase(),
        "has_peer_credentials": caller.peer.is_some(),
    });
    if let Some(ref claims) = caller.validated_claims {
        result["claims"] = serde_json::json!({
            "iss": claims.iss,
            "sub": claims.sub,
            "scope": claims.scope,
            "exp": claims.exp,
        });
    }
    result
}

/// Handle `auth.mode` — current enforcement mode.
#[must_use]
pub fn handle_auth_mode(gate: &MethodGate) -> serde_json::Value {
    serde_json::json!({
        "mode": gate.mode().as_str(),
        "env_var": "BEARDOG_AUTH_MODE",
    })
}

/// Handle `auth.peer_info` — peer credential introspection.
#[must_use]
pub fn handle_auth_peer_info(caller: &CallerContext) -> serde_json::Value {
    match &caller.peer {
        Some(creds) => serde_json::json!({
            "available": true,
            "uid": creds.uid,
            "pid": creds.pid,
            "origin": format!("{:?}", caller.origin).to_lowercase(),
        }),
        None => serde_json::json!({
            "available": false,
            "reason": "peer credentials not yet extracted (API unstable or non-Unix transport)",
            "origin": format!("{:?}", caller.origin).to_lowercase(),
        }),
    }
}

/// Check if a method is handled at the gate layer (auth introspection + ionic token lifecycle).
#[must_use]
pub fn is_gate_handled_method(method: &str) -> bool {
    matches!(
        method,
        "auth.check"
            | "auth.mode"
            | "auth.peer_info"
            | "auth.issue_ionic"
            | "auth.verify_ionic"
            | "identity.create"
    )
}

/// Dispatch a gate-handled method. Returns `Some(result)` if handled, `None`
/// if the method is not a gate-handled method.
#[must_use]
pub fn dispatch_auth_method(
    method: &str,
    gate: &MethodGate,
    caller: &CallerContext,
    params: Option<&serde_json::Value>,
) -> Option<serde_json::Value> {
    use crate::ionic_token_handlers::{
        handle_auth_issue_ionic, handle_auth_verify_ionic, handle_identity_create,
    };
    match method {
        "auth.check" => Some(handle_auth_check(caller)),
        "auth.mode" => Some(handle_auth_mode(gate)),
        "auth.peer_info" => Some(handle_auth_peer_info(caller)),
        "identity.create" => Some(handle_identity_create()),
        "auth.issue_ionic" => Some(handle_auth_issue_ionic(
            gate.primal_name(),
            gate.node_id(),
            params,
        )),
        "auth.verify_ionic" => Some(handle_auth_verify_ionic(gate.verifying_key(), params)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMAL: &str = "beardog";
    const NODE: &str = "test-node";

    fn test_gate(mode: EnforcementMode) -> MethodGate {
        MethodGate::new(mode, PRIMAL, NODE)
    }

    fn issue_test_token(scopes: &[&str], ttl: i64) -> String {
        use crate::ionic_token::issue_ionic_token;
        use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
        let sk = derive_primal_signing_key(PRIMAL, NODE);
        let scope_strings: Vec<String> = scopes.iter().map(|s| (*s).to_owned()).collect();
        issue_ionic_token(&sk, "did:key:z6MkTest", "testuser", &scope_strings, ttl)
    }

    // ── classify_method ──

    #[test]
    fn health_methods_are_public() {
        assert_eq!(classify_method("health.check"), MethodAccessLevel::Public);
        assert_eq!(
            classify_method("health.liveness"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("health.readiness"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn identity_get_is_public() {
        assert_eq!(classify_method("identity.get"), MethodAccessLevel::Public);
    }

    #[test]
    fn identity_create_is_public() {
        assert_eq!(
            classify_method("identity.create"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn capabilities_list_is_public() {
        assert_eq!(
            classify_method("capabilities.list"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("capability.list"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn auth_introspection_is_public() {
        assert_eq!(classify_method("auth.check"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.mode"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.peer_info"), MethodAccessLevel::Public);
    }

    #[test]
    fn ionic_auth_methods_are_public() {
        assert_eq!(
            classify_method("auth.issue_ionic"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("auth.verify_ionic"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn lifecycle_status_is_public() {
        assert_eq!(
            classify_method("lifecycle.status"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn crypto_methods_are_protected() {
        assert_eq!(
            classify_method("crypto.sign_ed25519"),
            MethodAccessLevel::Protected
        );
        assert_eq!(
            classify_method("crypto.sign_contract"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn ionic_bond_methods_are_protected() {
        assert_eq!(
            classify_method("crypto.ionic_bond.propose"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn empty_method_is_protected() {
        assert_eq!(classify_method(""), MethodAccessLevel::Protected);
    }

    #[test]
    fn legacy_identity_aliases_are_protected() {
        assert_eq!(classify_method("identity"), MethodAccessLevel::Protected);
        assert_eq!(classify_method("whoami"), MethodAccessLevel::Protected);
    }

    // ── CallerContext ──

    #[test]
    fn unix_context_has_no_peer() {
        let ctx = CallerContext::from_unix();
        assert!(ctx.peer.is_none());
        assert!(ctx.bearer_token.is_none());
        assert!(ctx.validated_claims.is_none());
        assert_eq!(ctx.origin, ConnectionOrigin::Unix);
    }

    #[test]
    fn loopback_context() {
        let ctx = CallerContext::loopback();
        assert_eq!(ctx.origin, ConnectionOrigin::Loopback);
    }

    #[test]
    fn remote_context() {
        let ctx = CallerContext::remote();
        assert_eq!(ctx.origin, ConnectionOrigin::Remote);
    }

    // ── EnforcementMode ──

    #[test]
    fn enforcement_mode_as_str() {
        assert_eq!(EnforcementMode::Permissive.as_str(), "permissive");
        assert_eq!(EnforcementMode::Enforced.as_str(), "enforced");
    }

    // ── MethodGate::check ──

    #[test]
    fn public_method_always_passes() {
        let gate = test_gate(EnforcementMode::Enforced);
        let mut caller = CallerContext::loopback();
        assert!(gate.check("health.check", &mut caller).is_ok());
        assert!(gate.check("identity.get", &mut caller).is_ok());
        assert!(gate.check("identity.create", &mut caller).is_ok());
        assert!(gate.check("capabilities.list", &mut caller).is_ok());
        assert!(gate.check("auth.check", &mut caller).is_ok());
        assert!(gate.check("auth.issue_ionic", &mut caller).is_ok());
        assert!(gate.check("auth.verify_ionic", &mut caller).is_ok());
    }

    #[test]
    fn protected_method_passes_in_permissive_mode() {
        let gate = test_gate(EnforcementMode::Permissive);
        let mut caller = CallerContext::loopback();
        assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
    }

    #[test]
    fn protected_method_rejected_in_enforced_mode_without_token() {
        let gate = test_gate(EnforcementMode::Enforced);
        let mut caller = CallerContext::loopback();
        let result = gate.check("crypto.sign_ed25519", &mut caller);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, JsonRpcError::PERMISSION_DENIED);
        assert!(err.message.contains("crypto.sign_ed25519"));
    }

    #[test]
    fn protected_method_passes_with_valid_ionic_token() {
        let gate = test_gate(EnforcementMode::Enforced);
        let token = issue_test_token(&["crypto.*"], 3600);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
        assert!(caller.validated_claims.is_some());
        assert_eq!(caller.validated_claims.as_ref().unwrap().sub, "testuser");
    }

    #[test]
    fn insufficient_scope_rejected_in_enforced_mode() {
        let gate = test_gate(EnforcementMode::Enforced);
        let token = issue_test_token(&["health.*"], 3600);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        let result = gate.check("crypto.sign_ed25519", &mut caller);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, JsonRpcError::PERMISSION_DENIED);
    }

    #[test]
    fn insufficient_scope_allowed_in_permissive_mode() {
        let gate = test_gate(EnforcementMode::Permissive);
        let token = issue_test_token(&["health.*"], 3600);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
        assert!(caller.validated_claims.is_some());
    }

    #[test]
    fn expired_token_rejected_in_enforced_mode() {
        let gate = test_gate(EnforcementMode::Enforced);
        let token = issue_test_token(&["*"], -10);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        let result = gate.check("crypto.sign_ed25519", &mut caller);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, JsonRpcError::UNAUTHORIZED);
    }

    #[test]
    fn expired_token_allowed_in_permissive_mode() {
        let gate = test_gate(EnforcementMode::Permissive);
        let token = issue_test_token(&["*"], -10);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
    }

    #[test]
    fn bogus_token_rejected_in_enforced_mode() {
        let gate = test_gate(EnforcementMode::Enforced);
        let mut caller = CallerContext {
            bearer_token: Some("not-a-real-token".to_owned()),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        let result = gate.check("crypto.sign_ed25519", &mut caller);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, JsonRpcError::UNAUTHORIZED);
    }

    #[test]
    fn gate_error_includes_method_in_data() {
        let gate = test_gate(EnforcementMode::Enforced);
        let mut caller = CallerContext::loopback();
        let err = gate.check("crypto.blake3_hash", &mut caller).unwrap_err();
        let method_in_data = err
            .data
            .as_ref()
            .and_then(|d| d.get("method"))
            .and_then(serde_json::Value::as_str);
        assert_eq!(method_in_data, Some("crypto.blake3_hash"));
    }

    // ── auth method handlers ──

    #[test]
    fn auth_check_unauthenticated() {
        let caller = CallerContext::loopback();
        let result = handle_auth_check(&caller);
        assert_eq!(result["authenticated"], false);
        assert_eq!(result["origin"], "loopback");
        assert!(result.get("claims").is_none());
    }

    #[test]
    fn auth_check_authenticated_with_claims() {
        let gate = test_gate(EnforcementMode::Permissive);
        let token = issue_test_token(&["*"], 3600);
        let mut caller = CallerContext {
            bearer_token: Some(token),
            peer: None,
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        // Run through gate to populate validated_claims
        gate.check("crypto.sign", &mut caller).unwrap();
        let result = handle_auth_check(&caller);
        assert_eq!(result["authenticated"], true);
        assert_eq!(result["claims"]["sub"], "testuser");
    }

    #[test]
    fn auth_mode_response() {
        let gate = test_gate(EnforcementMode::Permissive);
        let result = handle_auth_mode(&gate);
        assert_eq!(result["mode"], "permissive");
        assert_eq!(result["env_var"], "BEARDOG_AUTH_MODE");
    }

    #[test]
    fn auth_peer_info_no_creds() {
        let caller = CallerContext::loopback();
        let result = handle_auth_peer_info(&caller);
        assert_eq!(result["available"], false);
    }

    #[test]
    fn auth_peer_info_with_creds() {
        let caller = CallerContext {
            bearer_token: None,
            peer: Some(PeerCredentials {
                pid: Some(1234),
                uid: 1000,
            }),
            origin: ConnectionOrigin::Unix,
            validated_claims: None,
        };
        let result = handle_auth_peer_info(&caller);
        assert_eq!(result["available"], true);
        assert_eq!(result["uid"], 1000);
        assert_eq!(result["pid"], 1234);
    }

    // ── dispatch_auth_method ──

    #[test]
    fn dispatch_routes_auth_methods() {
        let gate = test_gate(EnforcementMode::Permissive);
        let caller = CallerContext::loopback();
        assert!(dispatch_auth_method("auth.check", &gate, &caller, None).is_some());
        assert!(dispatch_auth_method("auth.mode", &gate, &caller, None).is_some());
        assert!(dispatch_auth_method("auth.peer_info", &gate, &caller, None).is_some());
    }

    #[test]
    fn dispatch_routes_ionic_methods() {
        let gate = test_gate(EnforcementMode::Permissive);
        let caller = CallerContext::loopback();
        assert!(dispatch_auth_method("identity.create", &gate, &caller, None).is_some());

        let issue_params = serde_json::json!({"subject": "alice"});
        assert!(
            dispatch_auth_method("auth.issue_ionic", &gate, &caller, Some(&issue_params)).is_some()
        );

        let token_result =
            dispatch_auth_method("auth.issue_ionic", &gate, &caller, Some(&issue_params)).unwrap();
        let verify_params = serde_json::json!({"token": token_result["token"]});
        assert!(
            dispatch_auth_method("auth.verify_ionic", &gate, &caller, Some(&verify_params))
                .is_some()
        );
    }

    #[test]
    fn dispatch_returns_none_for_non_auth() {
        let gate = test_gate(EnforcementMode::Permissive);
        let caller = CallerContext::loopback();
        assert!(dispatch_auth_method("crypto.sign_ed25519", &gate, &caller, None).is_none());
    }

    #[test]
    fn is_gate_handled_method_correct() {
        assert!(is_gate_handled_method("auth.check"));
        assert!(is_gate_handled_method("auth.mode"));
        assert!(is_gate_handled_method("auth.peer_info"));
        assert!(is_gate_handled_method("auth.issue_ionic"));
        assert!(is_gate_handled_method("auth.verify_ionic"));
        assert!(is_gate_handled_method("identity.create"));
        assert!(!is_gate_handled_method("crypto.sign"));
    }
}
