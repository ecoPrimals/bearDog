// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared helpers for [`super::super::method_gate`] tests.

use super::super::*;

pub const PRIMAL: &str = "beardog";
pub const NODE: &str = "test-node";

pub fn test_gate(mode: EnforcementMode) -> MethodGate {
    MethodGate::new(mode, PRIMAL, NODE)
}

pub fn issue_test_token(scopes: &[&str], ttl: i64) -> String {
    use crate::ionic_token::issue_ionic_token;
    use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
    let sk = derive_primal_signing_key(PRIMAL, NODE);
    let scope_strings: Vec<String> = scopes.iter().map(|s| (*s).to_owned()).collect();
    issue_ionic_token(&sk, "did:key:z6MkTest", "testuser", &scope_strings, ttl)
}
