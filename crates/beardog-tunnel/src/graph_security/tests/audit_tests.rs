// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for origin audit

use crate::graph_security::audit::audit_origin;

fn assert_collaboration_network_required(err: beardog_errors::BearDogError) {
    let message = err.to_string();
    assert!(
        message.contains("collaboration network") || message.contains("Not yet available"),
        "unexpected error: {message}"
    );
}

#[tokio::test]
async fn test_audit_origin_requires_live_collaboration_network() {
    let err = audit_origin(&"template-123".to_string())
        .await
        .expect_err("audit should fail without collaboration network");
    assert_collaboration_network_required(err);
}

#[tokio::test]
async fn test_audit_origin_unknown_template_requires_live_network() {
    let err = audit_origin(&"unknown-template".to_string())
        .await
        .expect_err("audit should fail without collaboration network");
    assert_collaboration_network_required(err);
}

#[tokio::test]
async fn test_audit_origin_popular_template_requires_live_network() {
    let err = audit_origin(&"popular-template".to_string())
        .await
        .expect_err("audit should fail without collaboration network");
    assert_collaboration_network_required(err);
}
