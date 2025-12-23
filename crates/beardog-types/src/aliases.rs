// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::collections::HashMap;

// Keep only essential type aliases that provide clear semantic meaning
pub type JsonValue = serde_json::Value;
pub type JsonMap = serde_json::Map<String, serde_json::Value>;
pub type StringMap = HashMap<String, String>;
pub type MetricsMap = HashMap<String, JsonValue>;
pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedValue<T> {
    /// Value
    /// The value value
    pub value: T,
    /// Type Name
    /// Name of the type
    pub type_name: &'static str,
}

impl<T> TypedValue<T> {
    /// Creates a new instance
    pub fn new(value: T, type_name: &'static str) -> Self {
        Self { value, type_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_typed_value() {
        let typed_val = TypedValue::new(42, "i32");
        assert_eq!(typed_val.value, 42);
        assert_eq!(typed_val.type_name, "i32");
    }
}
