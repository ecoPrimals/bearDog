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
    #[must_use] pub fn from_shared(s: Arc<str>) -> Self {
        Self { inner: s }
    }

    /// Create from owned string
    /// Creates instance from owned
    #[must_use] pub fn from_owned(s: &str) -> Self {
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
    #[must_use] pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Get length
    #[must_use] pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    /// Checks if empty
    /// Checks if empty
    #[must_use] pub fn is_empty(&self) -> bool {
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

#[derive(Debug, Clone, Copy)]
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
    #[must_use] pub fn parse_common_string(s: &str) -> Option<Self> {
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
    #[must_use] pub fn as_str(&self) -> &'static str {
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
#[must_use] pub fn is_likely_id(s: &str) -> bool {
    s.starts_with("id_")
        || s.ends_with("_id")
        || (s.len() > 10 && s.chars().all(|c| c.is_alphanumeric() || c == '-'))
}

/// Checks if common value
/// Checks if common value
#[must_use] pub fn is_common_value(s: &str) -> bool {
    CommonString::parse_common_string(s).is_some()
}
