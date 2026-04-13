// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[tokio::test]
async fn test_security_handler_methods() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);
    let methods = handler.methods();

    assert!(methods.contains(&"security.evaluate"));
    assert!(methods.contains(&"trust.evaluate"));
    assert!(methods.contains(&"security.lineage"));
    assert!(methods.contains(&"trust.lineage"));
    assert!(methods.contains(&"birdsong.encrypt"));
    assert!(methods.contains(&"birdsong.decrypt"));
    assert!(methods.contains(&"security.generate_jwt_secret"));
    assert!(methods.contains(&"security.jwt_secret"));
    assert!(methods.contains(&"beardog.generate_jwt_secret"));
    assert!(methods.contains(&"beardog.jwt_secret"));

    let i_bird = methods
        .iter()
        .position(|&m| m == "birdsong.encrypt")
        .expect("birdsong.encrypt");
    let i_old_bird = methods
        .iter()
        .position(|&m| m == "beardog.birdsong.encrypt")
        .expect("beardog.birdsong.encrypt");
    assert!(i_bird < i_old_bird);

    let i_sec = methods
        .iter()
        .position(|&m| m == "security.generate_jwt_secret")
        .expect("security.generate_jwt_secret");
    let i_bd_jwt = methods
        .iter()
        .position(|&m| m == "beardog.generate_jwt_secret")
        .expect("beardog.generate_jwt_secret");
    assert!(i_sec < i_bd_jwt);
}

#[tokio::test]
async fn test_trust_evaluation_same_family() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);

    beardog_errors::process_env::set_var("FAMILY_ID", "test-family");
    beardog_errors::process_env::set_var("NODE_ID", "test-node");

    let params = serde_json::json!({
        "peer_id": "peer-123",
        "peer_family": "test-family"
    });

    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let result = handler
        .handle("security.evaluate", Some(&params), &btsp_provider)
        .await;

    assert!(result.is_ok());
    let response = result.expect("trust evaluation should succeed in test");

    assert_eq!(response["decision"], "auto_accept");
    assert_eq!(response["trust_level"], 1);
    assert_eq!(response["trust_level_name"], "limited");
    assert_eq!(response["reason"], "same_genetic_family");
}

#[tokio::test]
async fn test_trust_evaluation_different_family() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);

    beardog_errors::process_env::set_var("FAMILY_ID", "our-family");

    let params = serde_json::json!({
        "peer_id": "peer-456",
        "peer_family": "other-family"
    });

    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let result = handler
        .handle("trust.evaluate", Some(&params), &btsp_provider)
        .await;

    assert!(result.is_ok());
    let response = result.expect("trust evaluation should succeed in test");

    assert_eq!(response["decision"], "reject");
    assert_eq!(response["trust_level"], 0);
    assert_eq!(response["reason"], "different_family");
}

#[tokio::test]
async fn test_lineage_information() {
    struct CleanupGuard;
    impl Drop for CleanupGuard {
        fn drop(&mut self) {
            beardog_errors::process_env::remove_var("PRIMAL_NAME");
        }
    }
    beardog_errors::process_env::set_var("PRIMAL_NAME", "beardog");
    let _guard = CleanupGuard;

    let identity = Arc::new(PrimalIdentity::for_test("lineage-family", "lineage-node"));
    let handler = SecurityHandler::new(identity);

    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let result = handler
        .handle("security.lineage", None, &btsp_provider)
        .await;

    assert!(result.is_ok());
    let response = result.expect("lineage info should succeed in test");

    assert_eq!(response["primal"], "beardog");
    assert_eq!(response["family"], "lineage-family");
    assert_eq!(response["node"], "lineage-node");
    assert!(response["encryption_tag"].is_string());
}

#[tokio::test]
async fn test_jwt_secret_generation() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);

    let params = serde_json::json!({
        "purpose": "testing",
        "strength": "high"
    });

    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let result = handler
        .handle(
            "security.generate_jwt_secret",
            Some(&params),
            &btsp_provider,
        )
        .await;

    assert!(result.is_ok());
    let response = result.expect("JWT secret generation should succeed in test");

    assert!(response["secret"].is_string());
    assert_eq!(response["purpose"], "testing");
    assert_eq!(response["strength"], "high");
    assert_eq!(response["byte_length"], 64);
    assert!(
        response["encoded_length"]
            .as_u64()
            .expect("encoded_length should be number")
            >= 88
    );

    let secret = response["secret"]
        .as_str()
        .expect("secret should be string");
    assert!(
        base64::engine::general_purpose::STANDARD
            .decode(secret)
            .is_ok()
    );
}

#[tokio::test]
async fn test_jwt_secret_read_alias_matches_generate() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);
    let params = serde_json::json!({ "strength": "low" });
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let via_read = handler
        .handle("security.jwt_secret", Some(&params), &btsp_provider)
        .await
        .expect("security.jwt_secret");
    let via_legacy = handler
        .handle("beardog.jwt_secret", Some(&params), &btsp_provider)
        .await
        .expect("beardog.jwt_secret");
    assert_eq!(via_read["byte_length"], via_legacy["byte_length"]);
    assert_eq!(via_read["strength"], via_legacy["strength"]);
}

#[tokio::test]
async fn test_birdsong_encrypt_semantic_alias_requires_params() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let e1 = handler
        .handle("birdsong.encrypt", None, &btsp_provider)
        .await
        .expect_err("missing params");
    let e2 = handler
        .handle("beardog.birdsong.encrypt", None, &btsp_provider)
        .await
        .expect_err("missing params");
    assert_eq!(e1, e2);
}

#[tokio::test]
async fn test_jwt_secret_different_strengths() {
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({"strength": "high"});
    let result = handler
        .handle(
            "security.generate_jwt_secret",
            Some(&params),
            &btsp_provider,
        )
        .await
        .expect("JWT high strength should succeed");
    assert_eq!(result["byte_length"], 64);

    let params = serde_json::json!({"strength": "medium"});
    let result = handler
        .handle(
            "security.generate_jwt_secret",
            Some(&params),
            &btsp_provider,
        )
        .await
        .expect("JWT medium strength should succeed");
    assert_eq!(result["byte_length"], 48);

    let params = serde_json::json!({"strength": "low"});
    let result = handler
        .handle(
            "security.generate_jwt_secret",
            Some(&params),
            &btsp_provider,
        )
        .await
        .expect("JWT low strength should succeed");
    assert_eq!(result["byte_length"], 32);
}

#[tokio::test]
async fn test_issue_and_verify_consent_roundtrip() {
    let identity = Arc::new(PrimalIdentity::for_test("consent-family", "consent-node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let issue_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read:genome"
    });
    let issued = handler
        .handle(
            "security.issue_consent_token",
            Some(&issue_params),
            &btsp_provider,
        )
        .await
        .expect("issue_consent_token should succeed");

    assert_eq!(issued["owner_id"], "alice");
    assert_eq!(issued["scope"], "vault:read:genome");
    assert!(issued["token"].is_string());
    assert!(issued["issued_at"].is_string());

    let verify_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read:genome",
        "token": issued["token"]
    });
    let verified = handler
        .handle(
            "security.verify_consent",
            Some(&verify_params),
            &btsp_provider,
        )
        .await
        .expect("verify_consent should succeed");

    assert_eq!(verified["valid"], true);
    assert_eq!(verified["owner_id"], "alice");
    assert_eq!(verified["scope"], "vault:read:genome");
}

#[tokio::test]
async fn test_verify_consent_rejects_bad_token() {
    let identity = Arc::new(PrimalIdentity::for_test("consent-family", "consent-node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let bad_token = base64::engine::general_purpose::STANDARD.encode(b"not-a-real-hmac");
    let params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read:genome",
        "token": bad_token
    });
    let result = handler
        .handle("security.verify_consent", Some(&params), &btsp_provider)
        .await
        .expect("verify_consent should return ok with valid=false");

    assert_eq!(result["valid"], false);
}

#[tokio::test]
async fn test_verify_consent_rejects_wrong_scope() {
    let identity = Arc::new(PrimalIdentity::for_test("consent-family", "consent-node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let issue_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read:genome"
    });
    let issued = handler
        .handle(
            "security.issue_consent_token",
            Some(&issue_params),
            &btsp_provider,
        )
        .await
        .expect("issue");

    let verify_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:write:genome",
        "token": issued["token"]
    });
    let verified = handler
        .handle(
            "security.verify_consent",
            Some(&verify_params),
            &btsp_provider,
        )
        .await
        .expect("verify");

    assert_eq!(verified["valid"], false);
}

#[tokio::test]
async fn test_verify_consent_different_family_rejects() {
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let handler_a = SecurityHandler::new(Arc::new(PrimalIdentity::for_test("family-a", "n")));
    let handler_b = SecurityHandler::new(Arc::new(PrimalIdentity::for_test("family-b", "n")));

    let issue_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read"
    });
    let issued = handler_a
        .handle(
            "security.issue_consent_token",
            Some(&issue_params),
            &btsp_provider,
        )
        .await
        .expect("issue by family-a");

    let verify_params = serde_json::json!({
        "owner_id": "alice",
        "scope": "vault:read",
        "token": issued["token"]
    });
    let verified = handler_b
        .handle(
            "security.verify_consent",
            Some(&verify_params),
            &btsp_provider,
        )
        .await
        .expect("verify by family-b");

    assert_eq!(
        verified["valid"], false,
        "cross-family token must be rejected"
    );
}

#[tokio::test]
async fn test_verify_consent_missing_params() {
    let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let handler = SecurityHandler::new(identity);
    let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let r = handler
        .handle("security.verify_consent", None, &btsp_provider)
        .await;
    assert!(r.is_err());

    let partial = serde_json::json!({"owner_id": "alice"});
    let r = handler
        .handle("security.verify_consent", Some(&partial), &btsp_provider)
        .await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_security_handler_includes_consent_methods() {
    let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let handler = SecurityHandler::new(identity);
    let methods = handler.methods();
    assert!(methods.contains(&"security.verify_consent"));
    assert!(methods.contains(&"security.issue_consent_token"));
}
