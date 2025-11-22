//! # Idiomatic Rust Patterns for BearDog
//!
//! This file demonstrates the transformation from non-idiomatic to idiomatic Rust,
//! showing best practices for error handling, zero-copy optimization, and type safety.

#![allow(dead_code)]

use beardog_errors::BearDogError;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// PATTERN 1: Proper Error Handling (No Unwraps in Production)
// ============================================================================

/// ❌ NON-IDIOMATIC: Using unwrap() in production code
mod bad_error_handling {
    use super::*;

    pub fn process_key_bad(key_id: &str) -> String {
        let key = get_key(key_id).unwrap(); // PANIC POINT!
        key.to_uppercase()
    }

    pub fn get_key(key_id: &str) -> Option<String> {
        if key_id.is_empty() {
            None
        } else {
            Some(format!("key-{}", key_id))
        }
    }
}

/// ✅ IDIOMATIC: Proper error propagation with context
mod good_error_handling {
    use super::*;

    pub fn process_key_good(key_id: &str) -> Result<String, BearDogError> {
        let key = get_key(key_id).ok_or_else(|| BearDogError::key_not_found(key_id))?;

        Ok(key.to_uppercase())
    }

    pub fn get_key(key_id: &str) -> Option<String> {
        if key_id.is_empty() {
            None
        } else {
            Some(format!("key-{}", key_id))
        }
    }
}

// ============================================================================
// PATTERN 2: Zero-Copy with Cow (Clone-on-Write)
// ============================================================================

/// ❌ NON-IDIOMATIC: Always allocating/cloning
mod bad_string_handling {
    pub fn format_message(msg: &str, add_prefix: bool) -> String {
        if add_prefix {
            format!("[INFO] {}", msg) // Allocation
        } else {
            msg.to_string() // Unnecessary clone!
        }
    }

    pub fn process_batch(messages: &[String]) -> Vec<String> {
        messages.iter()
            .map(|m| m.clone()) // Unnecessary clones!
            .collect()
    }
}

/// ✅ IDIOMATIC: Zero-copy when possible
mod good_string_handling {
    use super::*;

    pub fn format_message(msg: &str, add_prefix: bool) -> Cow<'_, str> {
        if add_prefix {
            Cow::Owned(format!("[INFO] {}", msg))
        } else {
            Cow::Borrowed(msg) // Zero-copy!
        }
    }

    pub fn process_batch(messages: &[impl AsRef<str>]) -> Vec<&str> {
        messages.iter()
            .map(|m| m.as_ref()) // Zero-copy!
            .collect()
    }
}

// ============================================================================
// PATTERN 3: Shared Ownership with Arc (Not Clone)
// ============================================================================

/// ❌ NON-IDIOMATIC: Cloning large data structures
mod bad_shared_data {
    #[derive(Clone)]
    pub struct LargeConfig {
        data: Vec<u8>, // Imagine this is large
    }

    pub fn share_config(config: &LargeConfig) -> LargeConfig {
        config.clone() // Expensive deep copy!
    }
}

/// ✅ IDIOMATIC: Reference counting for shared ownership
mod good_shared_data {
    use super::*;

    pub struct LargeConfig {
        data: Vec<u8>,
    }

    pub fn share_config(config: &Arc<LargeConfig>) -> Arc<LargeConfig> {
        Arc::clone(config) // Cheap reference count increment!
    }
}

// ============================================================================
// PATTERN 4: Builder Pattern for Complex Configuration
// ============================================================================

/// ❌ NON-IDIOMATIC: Constructor with many parameters
mod bad_configuration {
    use super::*;

    pub struct Config {
        pub port: u16,
        pub timeout: Duration,
        pub retry_count: u32,
        pub max_connections: u32,
    }

    impl Config {
        // Unclear which parameter is which
        pub fn new(port: u16, timeout: Duration, retry: u32, max: u32) -> Self {
            Self {
                port,
                timeout,
                retry_count: retry,
                max_connections: max,
            }
        }
    }
}

/// ✅ IDIOMATIC: Builder pattern with clear intent
mod good_configuration {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Config {
        port: u16,
        timeout: Duration,
        retry_count: u32,
        max_connections: u32,
    }

    pub struct ConfigBuilder {
        port: Option<u16>,
        timeout: Option<Duration>,
        retry_count: Option<u32>,
        max_connections: Option<u32>,
    }

    impl Config {
        pub fn builder() -> ConfigBuilder {
            ConfigBuilder::new()
        }

        pub fn port(&self) -> u16 {
            self.port
        }
        pub fn timeout(&self) -> Duration {
            self.timeout
        }
        pub fn retry_count(&self) -> u32 {
            self.retry_count
        }
        pub fn max_connections(&self) -> u32 {
            self.max_connections
        }
    }

    impl ConfigBuilder {
        pub fn new() -> Self {
            Self {
                port: None,
                timeout: None,
                retry_count: None,
                max_connections: None,
            }
        }

        pub fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }

        pub fn timeout(mut self, timeout: Duration) -> Self {
            self.timeout = Some(timeout);
            self
        }

        pub fn retry_count(mut self, count: u32) -> Self {
            self.retry_count = Some(count);
            self
        }

        pub fn max_connections(mut self, max: u32) -> Self {
            self.max_connections = Some(max);
            self
        }

        pub fn build(self) -> Config {
            Config {
                port: self.port.unwrap_or(8080),
                timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
                retry_count: self.retry_count.unwrap_or(3),
                max_connections: self.max_connections.unwrap_or(100),
            }
        }
    }

    impl Default for ConfigBuilder {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ============================================================================
// PATTERN 5: Newtype Pattern for Type Safety
// ============================================================================

/// ❌ NON-IDIOMATIC: Stringly-typed APIs
mod bad_type_safety {
    pub fn register_user(user_id: String, key_id: String) {
        // Easy to swap arguments!
        println!("User: {}, Key: {}", user_id, key_id);
    }

    pub fn example_usage() {
        let user = "user-123".to_string();
        let key = "key-456".to_string();

        // Oops! Arguments swapped, compiles fine
        register_user(key, user);
    }
}

/// ✅ IDIOMATIC: Newtype pattern prevents mistakes
mod good_type_safety {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserId(String);

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct KeyId(String);

    impl UserId {
        pub fn new(id: impl Into<String>) -> Result<Self, BearDogError> {
            let id = id.into();
            if id.is_empty() {
                return Err(BearDogError::Validation {
                    message: "User ID cannot be empty".to_string(),
                });
            }
            Ok(Self(id))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl KeyId {
        pub fn new(id: impl Into<String>) -> Result<Self, BearDogError> {
            let id = id.into();
            if id.is_empty() {
                return Err(BearDogError::Validation {
                    message: "Key ID cannot be empty".to_string(),
                });
            }
            Ok(Self(id))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    pub fn register_user(user_id: UserId, key_id: KeyId) {
        println!("User: {}, Key: {}", user_id.as_str(), key_id.as_str());
    }

    pub fn example_usage() -> Result<(), BearDogError> {
        let user = UserId::new("user-123")?;
        let key = KeyId::new("key-456")?;

        // Cannot swap! Type system prevents mistakes
        register_user(user, key);
        // register_user(key, user); // Compile error!

        Ok(())
    }
}

// ============================================================================
// PATTERN 6: Option Combinators (Avoid if-let Chains)
// ============================================================================

/// ❌ NON-IDIOMATIC: Nested if-let
mod bad_option_handling {
    pub fn get_value(map: &std::collections::HashMap<String, Option<i32>>, key: &str) -> i32 {
        if let Some(opt_value) = map.get(key) {
            if let Some(value) = opt_value {
                *value
            } else {
                0
            }
        } else {
            0
        }
    }
}

/// ✅ IDIOMATIC: Option combinators
mod good_option_handling {
    pub fn get_value(map: &std::collections::HashMap<String, Option<i32>>, key: &str) -> i32 {
        map.get(key).and_then(|opt| *opt).unwrap_or(0)
    }
}

// ============================================================================
// PATTERN 7: Must-Use Annotations
// ============================================================================

/// ✅ IDIOMATIC: Marking important returns
mod good_must_use {
    use super::*;

    #[must_use = "Ignoring this result means the operation has no effect"]
    pub fn configure_security(level: u8) -> Result<(), BearDogError> {
        if level < 1 || level > 10 {
            return Err(BearDogError::Validation {
                message: "Security level must be 1-10".to_string(),
            });
        }
        // Apply configuration...
        Ok(())
    }
}

// ============================================================================
// PATTERN 8: Non-Exhaustive Enums for Future-Proofing
// ============================================================================

/// ✅ IDIOMATIC: Future-proof API
mod good_enum_design {
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SecurityLevel {
        Low,
        Medium,
        High,
        // Can add more variants without breaking API!
    }

    impl SecurityLevel {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Low => "low",
                Self::Medium => "medium",
                Self::High => "high",
            }
        }
    }
}

// ============================================================================
// PATTERN 9: Iterator Chains (Avoid Collecting Intermediate)
// ============================================================================

/// ❌ NON-IDIOMATIC: Unnecessary intermediate collections
mod bad_iteration {
    pub fn process_ids(ids: Vec<String>) -> Vec<String> {
        let filtered: Vec<_> = ids.into_iter().filter(|id| !id.is_empty()).collect(); // Unnecessary allocation!

        let uppercased: Vec<_> = filtered.into_iter().map(|id| id.to_uppercase()).collect();

        uppercased
    }
}

/// ✅ IDIOMATIC: Single iterator chain
mod good_iteration {
    pub fn process_ids(ids: Vec<String>) -> Vec<String> {
        ids.into_iter()
            .filter(|id| !id.is_empty())
            .map(|id| id.to_uppercase())
            .collect() // Single allocation!
    }
}

// ============================================================================
// PATTERN 10: Error Context with map_err
// ============================================================================

/// ❌ NON-IDIOMATIC: Losing error context
mod bad_error_context {
    use super::*;

    pub fn load_config() -> Result<String, BearDogError> {
        std::fs::read_to_string("config.toml")
            .map_err(|_| BearDogError::internal("Failed to load config"))
        // Lost: Which file? What was the actual error?
    }
}

/// ✅ IDIOMATIC: Preserving error context
mod good_error_context {
    use super::*;

    pub fn load_config() -> Result<String, BearDogError> {
        std::fs::read_to_string("config.toml")
            .map_err(|e| BearDogError::system(format!("Failed to load config.toml: {}", e)))
        // Clear: Which file failed and why
    }
}

// ============================================================================
// COMPLETE EXAMPLE: Before and After
// ============================================================================

/// Example showing multiple anti-patterns
mod complete_bad_example {
    use std::collections::HashMap;

    pub struct UserService {
        users: HashMap<String, String>,
    }

    impl UserService {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }

        // ❌ Unwrap in production
        // ❌ No error handling
        // ❌ Cloning unnecessarily
        pub fn get_user(&self, id: String) -> String {
            self.users.get(&id).unwrap().clone()
        }

        // ❌ Takes ownership when only needs reference
        pub fn add_user(&mut self, id: String, name: String) {
            self.users.insert(id, name);
        }
    }
}

/// Same example with idiomatic patterns
mod complete_good_example {
    use super::*;
    use std::collections::HashMap;

    pub struct UserService {
        users: HashMap<UserId, String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserId(String);

    impl UserId {
        pub fn new(id: impl Into<String>) -> Result<Self, BearDogError> {
            let id = id.into();
            if id.is_empty() {
                return Err(BearDogError::validation("User ID cannot be empty"));
            }
            Ok(Self(id))
        }
    }

    impl UserService {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }

        // ✅ Proper error handling
        // ✅ Returns reference (zero-copy)
        // ✅ Type-safe with UserId
        pub fn get_user(&self, id: &UserId) -> Result<&str, BearDogError> {
            self.users
                .get(id)
                .map(|s| s.as_str())
                .ok_or_else(|| BearDogError::user_not_found(id))
        }

        // ✅ Takes references where possible
        pub fn add_user(&mut self, id: UserId, name: impl Into<String>) {
            self.users.insert(id, name.into());
        }
    }

    impl Default for UserService {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ============================================================================
// Usage Examples
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_builder_pattern() {
        let config = good_configuration::Config::builder()
            .port(9090)
            .timeout(Duration::from_secs(60))
            .retry_count(5)
            .build();

        assert_eq!(config.port(), 9090);
        assert_eq!(config.retry_count(), 5);
    }

    #[test]
    fn demo_cow_usage() {
        let msg = "Hello, World!";

        // No allocation
        let result1 = good_string_handling::format_message(msg, false);
        assert!(matches!(result1, Cow::Borrowed(_)));

        // Allocates
        let result2 = good_string_handling::format_message(msg, true);
        assert!(matches!(result2, Cow::Owned(_)));
    }

    #[test]
    fn demo_newtype_safety() {
        use good_type_safety::*;

        let user_id = UserId::new("user-123").unwrap();
        let key_id = KeyId::new("key-456").unwrap();

        // Type system prevents swapping
        register_user(user_id, key_id);
    }
}
