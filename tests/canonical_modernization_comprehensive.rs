// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]

use beardog_errors::BearDogError;

#[tokio::test]
async fn test_canonical_modernization_basic() -> Result<(), BearDogError> {
    println!("Canonical modernization test running");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_error_system_basic() {
    let security_error = BearDogError::security("Test security".to_string());
    assert!(matches!(security_error, BearDogError::Security { .. }));

    let business_error = BearDogError::business("Test business".to_string());
    assert!(matches!(business_error, BearDogError::Business { .. }));

    let system_error = BearDogError::system("Test system".to_string());
    assert!(matches!(system_error, BearDogError::System { .. }));
}
