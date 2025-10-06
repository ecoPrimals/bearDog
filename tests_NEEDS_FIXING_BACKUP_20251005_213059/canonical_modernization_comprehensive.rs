use beardog_errors::BearDogError;
use tokio_test;

#[tokio::test]
async fn test_canonical_modernization_basic() -> Result<(), BearDogError> {
    println!("Canonical modernization test running");
    Ok(())
}

#[test]
fn test_error_system_basic() {
    let security_error = BearDogError::security("Test security");
    assert!(matches!(security_error, BearDogError::Security { .. }));

    let business_error = BearDogError::business("Test business");
    assert!(matches!(business_error, BearDogError::Business { .. }));

    let system_error = BearDogError::system("Test system");
    assert!(matches!(system_error, BearDogError::System { .. }));
}
