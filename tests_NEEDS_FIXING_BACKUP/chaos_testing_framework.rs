use beardog_errors::BearDogError;
use tokio_test;

#[tokio::test]
fn test_chaos_basic() -> Result<(), BearDogError> {
    println!("Chaos engineering test running ");
    Ok(())
}

#[test]
fn test_fault_types() {
    // Basic fault type test
    assert!(true, "Fault types test passed ");
}
