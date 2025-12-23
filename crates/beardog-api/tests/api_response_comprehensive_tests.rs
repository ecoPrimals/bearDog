//! Comprehensive Tests for API Response Types
//!
//! Coverage expansion for beardog-api crate

use beardog_api::ApiResponse;

#[test]
fn test_api_response_success_creation() {
    let response = ApiResponse::success("test_data");

    assert!(response.success);
    assert_eq!(response.data, Some("test_data"));
    assert!(response.error.is_none());
    assert!(response.timestamp <= chrono::Utc::now());
}

#[test]
fn test_api_response_success_with_various_types() {
    // Test with String
    let response_string = ApiResponse::success(String::from("hello"));
    assert_eq!(response_string.data, Some(String::from("hello")));

    // Test with number
    let response_num = ApiResponse::success(42);
    assert_eq!(response_num.data, Some(42));

    // Test with Vec
    let response_vec = ApiResponse::success(vec![1, 2, 3]);
    assert_eq!(response_vec.data, Some(vec![1, 2, 3]));
}

#[test]
fn test_api_response_error_creation() {
    let response: ApiResponse<String> = ApiResponse::error("Something went wrong".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Something went wrong".to_string()));
    assert!(response.timestamp <= chrono::Utc::now());
}

#[test]
fn test_api_response_error_with_empty_message() {
    let response: ApiResponse<i32> = ApiResponse::error(String::new());

    assert!(!response.success);
    assert_eq!(response.error, Some(String::new()));
}

#[test]
fn test_api_response_error_with_long_message() {
    let long_message = "a".repeat(1000);
    let response: ApiResponse<()> = ApiResponse::error(long_message.clone());

    assert_eq!(response.error, Some(long_message));
}

#[test]
fn test_api_response_serialization_success() {
    let response = ApiResponse::success(vec!["item1", "item2"]);

    let json = serde_json::to_string(&response).expect("Serialization should succeed");
    assert!(json.contains("\"success\":true"));
    assert!(json.contains("item1"));
    assert!(json.contains("item2"));
}

#[test]
fn test_api_response_serialization_error() {
    let response: ApiResponse<String> = ApiResponse::error("Test error".to_string());

    let json = serde_json::to_string(&response).expect("Serialization should succeed");
    assert!(json.contains("\"success\":false"));
    assert!(json.contains("Test error"));
}

#[test]
fn test_api_response_deserialization_success() {
    let json = r#"{
        "success": true,
        "data": "test_value",
        "error": null,
        "timestamp": "2025-12-17T12:00:00Z"
    }"#;

    let response: ApiResponse<String> =
        serde_json::from_str(json).expect("Deserialization should succeed");

    assert!(response.success);
    assert_eq!(response.data, Some("test_value".to_string()));
    assert!(response.error.is_none());
}

#[test]
fn test_api_response_deserialization_error() {
    let json = r#"{
        "success": false,
        "data": null,
        "error": "Error message",
        "timestamp": "2025-12-17T12:00:00Z"
    }"#;

    let response: ApiResponse<String> =
        serde_json::from_str(json).expect("Deserialization should succeed");

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Error message".to_string()));
}

#[test]
fn test_api_response_clone() {
    let original = ApiResponse::success(42);
    let cloned = original.clone();

    assert_eq!(original.success, cloned.success);
    assert_eq!(original.data, cloned.data);
    assert_eq!(original.timestamp, cloned.timestamp);
}

#[test]
fn test_api_response_debug_format() {
    let response = ApiResponse::success("debug_test");
    let debug_str = format!("{:?}", response);

    assert!(debug_str.contains("success"));
    assert!(debug_str.contains("debug_test"));
}

#[test]
fn test_api_response_timestamp_ordering() {
    let response1 = ApiResponse::success(1);
    std::thread::sleep(std::time::Duration::from_millis(10));
    let response2 = ApiResponse::success(2);

    assert!(response2.timestamp >= response1.timestamp);
}

#[test]
fn test_api_response_with_complex_data() {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct ComplexData {
        id: u64,
        name: String,
        values: Vec<i32>,
    }

    let data = ComplexData {
        id: 123,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let response = ApiResponse::success(data.clone());

    assert_eq!(response.data, Some(data));
}

#[test]
fn test_api_response_roundtrip_serialization() {
    let original = ApiResponse::success(vec!["a", "b", "c"]);

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: ApiResponse<Vec<String>> = serde_json::from_str(&json).unwrap();

    assert_eq!(original.success, deserialized.success);
    assert_eq!(
        original
            .data
            .as_ref()
            .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        deserialized.data
    );
}

#[test]
fn test_api_response_with_option_data() {
    let response_some = ApiResponse::success(Some(42));
    assert_eq!(response_some.data, Some(Some(42)));

    let response_none: ApiResponse<Option<i32>> = ApiResponse::success(None);
    assert_eq!(response_none.data, Some(None));
}

#[test]
fn test_api_response_error_preserves_type_info() {
    let response: ApiResponse<Vec<String>> = ApiResponse::error("type test".to_string());

    // Verify it can deserialize with correct type
    let json = serde_json::to_string(&response).unwrap();
    let _: ApiResponse<Vec<String>> = serde_json::from_str(&json).unwrap();
}

#[test]
fn test_api_response_multiple_errors() {
    let errors = ["error1", "error2", "error3"];
    let error_msg = errors.join("; ");
    let response: ApiResponse<()> = ApiResponse::error(error_msg);

    assert!(response.error.unwrap().contains("error1"));
}
