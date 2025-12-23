//! # Type-Safe ID Newtypes
//!
//! This module provides strongly-typed ID wrappers to prevent mixing different
//! types of identifiers at compile time.
//!
//! ## Rationale
//!
//! Previously, we used type aliases like `pub type KeyId = String`, which allowed
//! accidentally passing a `ServiceInstanceId` where a `KeyId` was expected.
//!
//! With newtypes, the compiler enforces type safety at zero runtime cost:
//! ```rust
//! use beardog_types::canonical::types::ids::{KeyId, ServiceInstanceId};
//!
//! let key_id = KeyId::new("key-123");
//! let instance_id = ServiceInstanceId::new("instance-456");
//!
//! // This won't compile (type mismatch):
//! // let id: KeyId = instance_id;  // ❌ Error!
//! ```
//!
//! ## Zero-Cost Abstraction
//!
//! These newtypes have **zero runtime overhead**. The compiler optimizes them
//! to be identical to raw `String` values in memory and performance.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;

/// Cryptographic key identifier
///
/// Used throughout the HSM and key management systems to uniquely identify
/// cryptographic keys. Cannot be mixed with other ID types at compile time.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::types::ids::KeyId;
///
/// let key_id = KeyId::new("hsm-key-ed25519-2024");
/// assert_eq!(key_id.as_str(), "hsm-key-ed25519-2024");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyId(String);

impl KeyId {
    /// Creates a new KeyId from any string-like value
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the KeyId and returns the inner String
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for KeyId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for KeyId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for KeyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for KeyId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl KeyId {
    /// Returns the ID as bytes for cryptographic operations
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Checks if the ID contains a substring
    pub fn contains(&self, pat: &str) -> bool {
        self.0.contains(pat)
    }
}

/// Service instance identifier
///
/// Used in service discovery to uniquely identify running service instances.
/// Type-safe at compile time to prevent confusion with other ID types.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::types::ids::ServiceInstanceId;
///
/// let instance = ServiceInstanceId::new("beardog-api-node-03");
/// println!("Instance: {}", instance);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceInstanceId(String);

impl ServiceInstanceId {
    /// Creates a new ServiceInstanceId from any string-like value
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the ServiceInstanceId and returns the inner String
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ServiceInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ServiceInstanceId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for ServiceInstanceId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for ServiceInstanceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ServiceInstanceId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Registration identifier
///
/// Used in service registration systems to track registered service entries.
/// Type-safe to prevent mixing with service instance IDs or key IDs.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::types::ids::RegistrationId;
///
/// let reg_id = RegistrationId::new("reg-2024-11-09-abc123");
/// assert_eq!(reg_id.as_str(), "reg-2024-11-09-abc123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegistrationId(String);

impl RegistrationId {
    /// Creates a new RegistrationId from any string-like value
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the RegistrationId and returns the inner String
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for RegistrationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RegistrationId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for RegistrationId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for RegistrationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for RegistrationId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_id_creation() {
        let key_id = KeyId::new("test-key-123");
        assert_eq!(key_id.as_str(), "test-key-123");
        assert_eq!(key_id.to_string(), "test-key-123");
    }

    #[test]
    fn test_key_id_from_string() {
        let key_id: KeyId = "another-key".into();
        assert_eq!(key_id.as_str(), "another-key");
    }

    #[test]
    fn test_service_instance_id() {
        let instance = ServiceInstanceId::new("instance-001");
        assert_eq!(instance.as_str(), "instance-001");
    }

    #[test]
    fn test_registration_id() {
        let reg = RegistrationId::new("reg-456");
        assert_eq!(reg.as_str(), "reg-456");
    }

    #[test]
    fn test_different_types_are_not_interchangeable() {
        let key_id = KeyId::new("id-123");
        let _instance_id = ServiceInstanceId::new("id-123");

        // This test verifies that the types are distinct
        // If you try to assign instance_id to a KeyId variable, it won't compile
        assert_eq!(key_id.as_str(), "id-123");
    }

    #[test]
    fn test_serialization() {
        let key_id = KeyId::new("serialize-test");
        let json = serde_json::to_string(&key_id).unwrap();
        assert_eq!(json, "\"serialize-test\"");

        let deserialized: KeyId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, key_id);
    }

    #[test]
    fn test_hash_and_eq() {
        use std::collections::HashSet;

        let id1 = KeyId::new("same-id");
        let id2 = KeyId::new("same-id");
        let id3 = KeyId::new("different-id");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        let mut set = HashSet::new();
        set.insert(id1.clone());
        assert!(set.contains(&id2));
        assert!(!set.contains(&id3));
    }

    #[test]
    fn test_key_id_into_inner() {
        let key_id = KeyId::new("extract-me");
        let inner = key_id.into_inner();
        assert_eq!(inner, "extract-me");
    }

    #[test]
    fn test_key_id_as_bytes() {
        let key_id = KeyId::new("bytes-test");
        let bytes = key_id.as_bytes();
        assert_eq!(bytes, b"bytes-test");
    }

    #[test]
    fn test_key_id_contains() {
        let key_id = KeyId::new("hsm-key-ed25519-2024");
        assert!(key_id.contains("ed25519"));
        assert!(key_id.contains("hsm"));
        assert!(!key_id.contains("rsa"));
    }

    #[test]
    fn test_key_id_as_ref() {
        let key_id = KeyId::new("ref-test");
        let as_ref: &str = key_id.as_ref();
        assert_eq!(as_ref, "ref-test");
    }

    #[test]
    fn test_key_id_borrow() {
        use std::collections::HashMap;

        let key_id = KeyId::new("borrow-test");
        let mut map = HashMap::new();
        map.insert(key_id.clone(), "value");

        // Borrow trait allows using &str to query HashMap<KeyId, _>
        assert_eq!(map.get("borrow-test" as &str), Some(&"value"));
    }

    #[test]
    fn test_key_id_clone() {
        let key_id1 = KeyId::new("clone-test");
        let key_id2 = key_id1.clone();
        assert_eq!(key_id1, key_id2);
    }

    #[test]
    fn test_key_id_debug_format() {
        let key_id = KeyId::new("debug-test");
        let debug_str = format!("{:?}", key_id);
        assert!(debug_str.contains("debug-test"));
    }

    #[test]
    fn test_service_instance_id_into_inner() {
        let instance = ServiceInstanceId::new("instance-extract");
        let inner = instance.into_inner();
        assert_eq!(inner, "instance-extract");
    }

    #[test]
    fn test_service_instance_id_conversions() {
        let instance1: ServiceInstanceId = "instance-from-str".into();
        let instance2: ServiceInstanceId = String::from("instance-from-string").into();

        assert_eq!(instance1.as_str(), "instance-from-str");
        assert_eq!(instance2.as_str(), "instance-from-string");
    }

    #[test]
    fn test_service_instance_id_as_ref() {
        let instance = ServiceInstanceId::new("instance-ref");
        let as_ref: &str = instance.as_ref();
        assert_eq!(as_ref, "instance-ref");
    }

    #[test]
    fn test_service_instance_id_display() {
        let instance = ServiceInstanceId::new("display-test");
        assert_eq!(format!("{}", instance), "display-test");
    }

    #[test]
    fn test_registration_id_into_inner() {
        let reg = RegistrationId::new("reg-extract");
        let inner = reg.into_inner();
        assert_eq!(inner, "reg-extract");
    }

    #[test]
    fn test_registration_id_conversions() {
        let reg1: RegistrationId = "reg-from-str".into();
        let reg2: RegistrationId = String::from("reg-from-string").into();

        assert_eq!(reg1.as_str(), "reg-from-str");
        assert_eq!(reg2.as_str(), "reg-from-string");
    }

    #[test]
    fn test_registration_id_as_ref() {
        let reg = RegistrationId::new("reg-ref");
        let as_ref: &str = reg.as_ref();
        assert_eq!(as_ref, "reg-ref");
    }

    #[test]
    fn test_registration_id_display() {
        let reg = RegistrationId::new("reg-display");
        assert_eq!(format!("{}", reg), "reg-display");
    }

    #[test]
    fn test_empty_ids() {
        let key_id = KeyId::new("");
        let instance_id = ServiceInstanceId::new("");
        let reg_id = RegistrationId::new("");

        assert_eq!(key_id.as_str(), "");
        assert_eq!(instance_id.as_str(), "");
        assert_eq!(reg_id.as_str(), "");
    }

    #[test]
    fn test_special_characters_in_ids() {
        let key_id = KeyId::new("key-with-dashes-and_underscores.123");
        assert!(key_id.contains("dashes"));
        assert!(key_id.contains("underscores"));
        assert!(key_id.contains(".123"));
    }

    #[test]
    fn test_unicode_in_ids() {
        let key_id = KeyId::new("key-with-emoji-🔑");
        assert!(key_id.contains("emoji"));
        assert_eq!(key_id.as_str(), "key-with-emoji-🔑");
    }

    #[test]
    fn test_all_id_types_equality() {
        let key1 = KeyId::new("test-id");
        let key2 = KeyId::new("test-id");
        let key3 = KeyId::new("different-id");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);

        let inst1 = ServiceInstanceId::new("test-id");
        let inst2 = ServiceInstanceId::new("test-id");

        assert_eq!(inst1, inst2);

        let reg1 = RegistrationId::new("test-id");
        let reg2 = RegistrationId::new("test-id");

        assert_eq!(reg1, reg2);
    }
}
