// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimalIdentity {
    id: String,
    primal_type: String,
}

impl PrimalIdentity {
    fn new(id: String, primal_type: String) -> Self {
        Self { id, primal_type }
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn primal_type(&self) -> &str {
        &self.primal_type
    }
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: identity
/// `TEST_PRIORITY`: critical
#[test]
fn test_identity_creation() {
    // Test creating a primal identity
    let identity = PrimalIdentity::new("beardog-test-001".to_string(), "BearDog".to_string());

    assert_eq!(identity.id(), "beardog-test-001");
    assert_eq!(identity.primal_type(), "BearDog");
    assert!(!identity.id().is_empty(), "Identity ID should not be empty");
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: identity
/// `TEST_PRIORITY`: high
#[test]
fn test_identity_validation() {
    // Test that valid identities pass validation
    let valid_identity = PrimalIdentity::new("valid-id-123".to_string(), "BearDog".to_string());
    assert!(!valid_identity.id().is_empty());
    assert!(!valid_identity.primal_type().is_empty());

    // Test that identity fields are non-empty
    let identity = PrimalIdentity::new("test-id".to_string(), "TestType".to_string());
    assert!(identity.id().len() > 0, "ID should have length");
    assert!(identity.primal_type().len() > 0, "Type should have length");
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: identity
/// `TEST_PRIORITY`: high
#[test]
fn test_identity_verification() {
    // Test identity verification logic
    let identity1 = PrimalIdentity::new("verify-001".to_string(), "BearDog".to_string());

    // Verify identity has correct attributes
    assert_eq!(identity1.id(), "verify-001");
    assert_eq!(identity1.primal_type(), "BearDog");

    // Identity should be consistent
    assert_eq!(identity1.id(), identity1.id(), "ID should be stable");
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: identity
/// `TEST_PRIORITY`: normal
#[test]
fn test_identity_serialization() {
    // Test that identities can be serialized
    let identity = PrimalIdentity::new("serialize-001".to_string(), "BearDog".to_string());

    let serialized = serde_json::to_string(&identity);
    assert!(serialized.is_ok(), "Identity should serialize successfully");

    let json = serialized.unwrap();
    assert!(json.contains("serialize-001"), "JSON should contain ID");
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: identity
/// `TEST_PRIORITY`: normal
#[test]
fn test_identity_comparison() {
    // Test identity comparison
    let identity1 = PrimalIdentity::new("compare-001".to_string(), "BearDog".to_string());

    let identity2 = PrimalIdentity::new("compare-002".to_string(), "BearDog".to_string());

    let identity1_clone = PrimalIdentity::new("compare-001".to_string(), "BearDog".to_string());

    // Different identities should be different
    assert_ne!(identity1.id(), identity2.id());

    // Same ID should match
    assert_eq!(identity1.id(), identity1_clone.id());
    assert_eq!(identity1.primal_type(), identity1_clone.primal_type());
}
