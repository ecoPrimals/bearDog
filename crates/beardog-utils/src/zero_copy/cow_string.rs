// SPDX-License-Identifier: AGPL-3.0-only

// Safe zero-copy string utilities
//
// This module provides string optimizations using safe Rust patterns.

// use std::borrow::Cow; // Currently unused but kept for future zero-copy string operations
use std::sync::Arc;

/// Zero-copy string implementation
#[derive(Debug, Clone)]
pub struct ZeroCopyString {
    inner: Arc<str>,
}

impl ZeroCopyString {
    /// Create from shared string
    /// Creates instance from shared
    #[must_use]
    pub const fn from_shared(s: Arc<str>) -> Self {
        Self { inner: s }
    }

    /// Create from owned string
    /// Creates instance from owned
    #[must_use]
    pub fn from_owned(s: &str) -> Self {
        Self {
            inner: Arc::from(s),
        }
    }

    /// Create from string reference
    /// Creates instance from string
    pub fn from_string<S: AsRef<str>>(s: S) -> Self {
        Self::from_owned(s.as_ref())
    }

    /// Get as string slice
    /// Returns as str
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Get length
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    /// Checks if empty
    /// Checks if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl AsRef<str> for ZeroCopyString {
    /// Returns as ref
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<String> for ZeroCopyString {
    fn from(s: String) -> Self {
        Self::from_owned(&s)
    }
}

impl From<&str> for ZeroCopyString {
    fn from(s: &str) -> Self {
        Self::from_owned(s)
    }
}

impl From<Arc<str>> for ZeroCopyString {
    fn from(s: Arc<str>) -> Self {
        Self::from_shared(s)
    }
}

/// Frequently used literals interned as enum variants for cheap comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonString {
    /// Represents get variant
    Get,
    /// Represents post variant
    Post,
    /// Represents put variant
    Put,
    /// Represents delete variant
    Delete,
    /// Represents patch variant
    Patch,
    /// Represents application json variant
    ApplicationJson,
    /// Represents text plain variant
    TextPlain,
    /// Represents bearer variant
    Bearer,
    /// Represents api variant
    Api,
    /// Represents metrics variant
    Metrics,
    /// Represents health variant
    Health,
    /// Represents admin variant
    Admin,
}

impl CommonString {
    /// Try to create from string
    /// Parses `common_string`
    /// Parses `common_string`
    #[must_use]
    pub fn parse_common_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "get" => Some(Self::Get),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "delete" => Some(Self::Delete),
            "patch" => Some(Self::Patch),
            "application/json" => Some(Self::ApplicationJson),
            "text/plain" => Some(Self::TextPlain),
            "bearer" => Some(Self::Bearer),
            "api" => Some(Self::Api),
            "metrics" => Some(Self::Metrics),
            "health" => Some(Self::Health),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }

    /// Get as static string
    /// Returns as str
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::ApplicationJson => "application/json",
            Self::TextPlain => "text/plain",
            Self::Bearer => "Bearer",
            Self::Api => "api",
            Self::Metrics => "metrics",
            Self::Health => "health",
            Self::Admin => "admin",
        }
    }
}

/// Helper functions
/// Checks if likely id
#[must_use]
pub fn is_likely_id(s: &str) -> bool {
    s.starts_with("id_")
        || s.ends_with("_id")
        || (s.len() > 10 && s.chars().all(|c| c.is_alphanumeric() || c == '-'))
}

/// Checks if common value
/// Checks if common value
#[must_use]
pub fn is_common_value(s: &str) -> bool {
    CommonString::parse_common_string(s).is_some()
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_string_from_owned() {
        let s = ZeroCopyString::from_owned("test string");
        assert_eq!(s.as_str(), "test string");
        assert_eq!(s.len(), 11);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_zero_copy_string_from_string() {
        let s = ZeroCopyString::from_string("hello world");
        assert_eq!(s.as_str(), "hello world");
        assert_eq!(s.len(), 11);
    }

    #[test]
    fn test_zero_copy_string_from_shared() {
        let arc_str: Arc<str> = Arc::from("shared");
        let s = ZeroCopyString::from_shared(arc_str);
        assert_eq!(s.as_str(), "shared");
        assert_eq!(s.len(), 6);
    }

    #[test]
    fn test_zero_copy_string_empty() {
        let s = ZeroCopyString::from_owned("");
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_zero_copy_string_from_string_impl() {
        let s: ZeroCopyString = String::from("test").into();
        assert_eq!(s.as_str(), "test");
    }

    #[test]
    fn test_zero_copy_string_from_str_impl() {
        let s: ZeroCopyString = "test".into();
        assert_eq!(s.as_str(), "test");
    }

    #[test]
    fn test_zero_copy_string_from_arc_impl() {
        let arc: Arc<str> = Arc::from("test");
        let s: ZeroCopyString = arc.into();
        assert_eq!(s.as_str(), "test");
    }

    #[test]
    fn test_zero_copy_string_as_ref() {
        let s = ZeroCopyString::from_owned("reference");
        let as_ref: &str = s.as_ref();
        assert_eq!(as_ref, "reference");
    }

    #[test]
    fn test_zero_copy_string_clone() {
        let s1 = ZeroCopyString::from_owned("original");
        let s2 = s1.clone();
        assert_eq!(s1.as_str(), s2.as_str());
    }

    #[test]
    fn test_common_string_http_methods() {
        assert_eq!(CommonString::Get.as_str(), "GET");
        assert_eq!(CommonString::Post.as_str(), "POST");
        assert_eq!(CommonString::Put.as_str(), "PUT");
        assert_eq!(CommonString::Delete.as_str(), "DELETE");
        assert_eq!(CommonString::Patch.as_str(), "PATCH");
    }

    #[test]
    fn test_common_string_content_types() {
        assert_eq!(CommonString::ApplicationJson.as_str(), "application/json");
        assert_eq!(CommonString::TextPlain.as_str(), "text/plain");
    }

    #[test]
    fn test_common_string_auth() {
        assert_eq!(CommonString::Bearer.as_str(), "Bearer");
    }

    #[test]
    fn test_common_string_paths() {
        assert_eq!(CommonString::Api.as_str(), "api");
        assert_eq!(CommonString::Metrics.as_str(), "metrics");
        assert_eq!(CommonString::Health.as_str(), "health");
        assert_eq!(CommonString::Admin.as_str(), "admin");
    }

    #[test]
    fn test_parse_common_string_http_methods() {
        assert!(matches!(
            CommonString::parse_common_string("get"),
            Some(CommonString::Get)
        ));
        assert!(matches!(
            CommonString::parse_common_string("POST"),
            Some(CommonString::Post)
        ));
        assert!(matches!(
            CommonString::parse_common_string("Put"),
            Some(CommonString::Put)
        ));
    }

    #[test]
    fn test_parse_common_string_content_types() {
        assert!(matches!(
            CommonString::parse_common_string("application/json"),
            Some(CommonString::ApplicationJson)
        ));
        assert!(matches!(
            CommonString::parse_common_string("text/plain"),
            Some(CommonString::TextPlain)
        ));
    }

    #[test]
    fn test_parse_common_string_invalid() {
        assert_eq!(CommonString::parse_common_string("invalid"), None);
        assert_eq!(CommonString::parse_common_string(""), None);
        assert_eq!(CommonString::parse_common_string("xyz"), None);
    }

    #[test]
    fn test_is_likely_id_with_prefix() {
        assert!(is_likely_id("id_12345"));
        assert!(is_likely_id("id_user"));
        assert!(is_likely_id("id_"));
    }

    #[test]
    fn test_is_likely_id_with_suffix() {
        assert!(is_likely_id("user_id"));
        assert!(is_likely_id("account_id"));
        assert!(is_likely_id("_id"));
    }

    #[test]
    fn test_is_likely_id_with_long_alphanumeric() {
        assert!(is_likely_id("abc123def456ghi"));
        assert!(is_likely_id("12345678901"));
        assert!(is_likely_id("uuid-1234-5678"));
    }

    #[test]
    fn test_is_likely_id_negative() {
        assert!(!is_likely_id("short"));
        assert!(!is_likely_id("name"));
        assert!(!is_likely_id("test@example.com"));
    }

    #[test]
    fn test_is_common_value() {
        assert!(is_common_value("get"));
        assert!(is_common_value("POST"));
        assert!(is_common_value("application/json"));
        assert!(is_common_value("bearer"));
        assert!(!is_common_value("invalid"));
        assert!(!is_common_value("xyz"));
    }
}
