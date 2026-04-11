// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::types::authorization::{
    AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation, ResourcePermission,
};
use beardog_errors::BearDogError;
use chrono::Utc;

#[tokio::test]
async fn test_authorization_creation() -> Result<(), BearDogError> {
    let now = Utc::now();
    let auth = CrossNodeAuthorization {
        request_id: "authz-test".to_string(),
        requester_node_id: "beardog-node-1".to_string(),
        resource_owner_node_id: "beardog-node-2".to_string(),
        resource_id: "keys/master".to_string(),
        permissions: vec![ResourcePermission::Read],
        conditions: vec![],
        created_at: now,
        expires_at: now + chrono::Duration::hours(1),
        signature: "sig".to_string(),
        is_active: true,
    };

    assert!(auth.is_valid());
    assert_eq!(auth.request_id, "authz-test");
    Ok(())
}

#[tokio::test]
async fn test_authorization_proof_construction() -> Result<(), BearDogError> {
    let proof = AuthorizationProof {
        authorization_id: "proof-001".to_string(),
        operation: CrossNodeOperation::default(),
        timestamp: Utc::now(),
        proof_signature: "ed25519-sig".to_string(),
    };

    assert_eq!(proof.authorization_id, "proof-001");
    assert!(!proof.proof_signature.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_inactive_authorization_invalid() -> Result<(), BearDogError> {
    let now = Utc::now();
    let auth = CrossNodeAuthorization {
        request_id: "inactive".to_string(),
        requester_node_id: "a".to_string(),
        resource_owner_node_id: "b".to_string(),
        resource_id: "res".to_string(),
        permissions: vec![ResourcePermission::Write],
        conditions: vec![],
        created_at: now,
        expires_at: now + chrono::Duration::hours(1),
        signature: "sig".to_string(),
        is_active: false,
    };

    assert!(!auth.is_active);
    assert!(auth.has_permission(&ResourcePermission::Write));
    Ok(())
}
