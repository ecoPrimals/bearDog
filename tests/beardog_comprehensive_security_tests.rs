use beardog_errors::BearDogError;
use std::collections::HashMap;
use tokio_test;

#[tokio::test]
fn test_basic_security() -> Result<(), BearDogError> {
    // Basic security test to ensure compilation works
    println!("Security test running");
    Ok(())
}

#[tokio::test]
fn test_error_handling() {
    let error = BearDogError::validation("Test error");
    assert!(error.to_string().contains("Test error"));
}

#[test]
fn test_hashmap_creation() {
    let mut map: HashMap<String, String> = HashMap::with_capacity(16);
    map.insert("test".to_string(), "value".to_string());
    assert_eq!(map.len(), 1);
}
