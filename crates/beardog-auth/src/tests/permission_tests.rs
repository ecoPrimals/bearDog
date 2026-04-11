// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::types::authorization::{CrossNodeAuthorization, ResourcePermission};
use beardog_errors::BearDogError;
use chrono::Utc;

#[tokio::test]
async fn test_permission_implies() -> Result<(), BearDogError> {
    let perm = ResourcePermission::Read;
    assert!(perm.implies(&ResourcePermission::Read), "Read implies Read");
    assert!(
        !perm.implies(&ResourcePermission::Write),
        "Read does not imply Write"
    );
    Ok(())
}

#[tokio::test]
async fn test_admin_permission_implies_all() -> Result<(), BearDogError> {
    let admin = ResourcePermission::Admin;
    assert!(admin.implies(&ResourcePermission::Read));
    assert!(admin.implies(&ResourcePermission::Write));
    assert!(admin.implies(&ResourcePermission::Execute));
    assert!(admin.implies(&ResourcePermission::Admin));
    Ok(())
}

#[tokio::test]
async fn test_authorization_has_permission() -> Result<(), BearDogError> {
    let now = Utc::now();
    let auth = CrossNodeAuthorization {
        request_id: "req-001".to_string(),
        requester_node_id: "node-a".to_string(),
        resource_owner_node_id: "node-b".to_string(),
        resource_id: "data/dataset-1".to_string(),
        permissions: vec![ResourcePermission::Read, ResourcePermission::Execute],
        conditions: vec![],
        created_at: now,
        expires_at: now + chrono::Duration::hours(1),
        signature: "sig".to_string(),
        is_active: true,
    };

    assert!(auth.is_valid());
    assert!(auth.has_permission(&ResourcePermission::Read));
    assert!(!auth.has_permission(&ResourcePermission::Write));
    Ok(())
}
