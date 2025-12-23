//! Modern Trait Implementations for Type Conversions
//!
//! This module provides idiomatic Rust trait implementations (From/Into, Display, etc.)
//! for common BearDog types, enabling seamless type conversions and better ergonomics.
//!
//! # Examples
//!
//! ```rust
//! use beardog_types::modern_traits::*;
//!
//! // String to NodeId conversion
//! let node_id: NodeId = "node-123".into();
//!
//! // Display for debugging
//! println!("Node: {}", node_id);
//! ```

use std::fmt;

/// Node identifier with zero-copy support
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    /// Create a new NodeId
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into inner string
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// From/Into implementations for seamless conversions
impl From<String> for NodeId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<NodeId> for String {
    fn from(id: NodeId) -> Self {
        id.0
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Display implementation for better formatting
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Resource identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    /// Create a new ResourceId
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into inner string
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// From/Into implementations
impl From<String> for ResourceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ResourceId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<ResourceId> for String {
    fn from(id: ResourceId) -> Self {
        id.0
    }
}

impl AsRef<str> for ResourceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Service identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId(String);

impl ServiceId {
    /// Create a new ServiceId
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into inner string
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// From/Into implementations
impl From<String> for ServiceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ServiceId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<ServiceId> for String {
    fn from(id: ServiceId) -> Self {
        id.0
    }
}

impl AsRef<str> for ServiceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Port number with validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(u16);

impl Port {
    /// Create a new Port
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use beardog_types::modern_traits::Port;
    /// let port = Port::new(8080);
    /// assert_eq!(port.value(), 8080);
    /// ```
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    /// Get the port value
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Check if this is a privileged port (<1024)
    #[must_use]
    pub const fn is_privileged(self) -> bool {
        self.0 < 1024
    }

    /// Check if this is a registered port (1024-49151)
    #[must_use]
    pub const fn is_registered(self) -> bool {
        self.0 >= 1024 && self.0 <= 49151
    }

    /// Check if this is an ephemeral port (>49151)
    #[must_use]
    pub const fn is_ephemeral(self) -> bool {
        self.0 > 49151
    }
}

// From/Into implementations
impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Self(port)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trust level (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TrustLevel(f64);

impl TrustLevel {
    /// Create a new TrustLevel, clamping to valid range
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use beardog_types::modern_traits::TrustLevel;
    /// let trust = TrustLevel::new(0.75);
    /// assert_eq!(trust.value(), 0.75);
    ///
    /// let clamped = TrustLevel::new(1.5);
    /// assert_eq!(clamped.value(), 1.0);
    /// ```
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the trust level value
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Check if this is a high trust level (>= 0.8)
    #[must_use]
    pub const fn is_high(self) -> bool {
        self.0 >= 0.8
    }

    /// Check if this is a medium trust level (0.5-0.8)
    #[must_use]
    pub const fn is_medium(self) -> bool {
        self.0 >= 0.5 && self.0 < 0.8
    }

    /// Check if this is a low trust level (< 0.5)
    #[must_use]
    pub const fn is_low(self) -> bool {
        self.0 < 0.5
    }
}

// From/Into implementations
impl From<f64> for TrustLevel {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<TrustLevel> for f64 {
    fn from(trust: TrustLevel) -> Self {
        trust.0
    }
}

impl fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NodeId tests
    #[test]
    fn test_node_id_from_string() {
        let id: NodeId = "node-123".to_string().into();
        assert_eq!(id.as_str(), "node-123");
    }

    #[test]
    fn test_node_id_from_str() {
        let id: NodeId = "node-456".into();
        assert_eq!(id.as_str(), "node-456");
    }

    #[test]
    fn test_node_id_into_string() {
        let id = NodeId::new("node-789");
        let s: String = id.into();
        assert_eq!(s, "node-789");
    }

    #[test]
    fn test_node_id_display() {
        let id = NodeId::new("node-display");
        assert_eq!(format!("{id}"), "node-display");
    }

    #[test]
    fn test_node_id_as_ref() {
        let id = NodeId::new("node-ref");
        let s: &str = id.as_ref();
        assert_eq!(s, "node-ref");
    }

    // ResourceId tests
    #[test]
    fn test_resource_id_conversions() {
        let id: ResourceId = "resource-1".into();
        assert_eq!(id.as_str(), "resource-1");
        assert_eq!(format!("{id}"), "resource-1");
    }

    // ServiceId tests
    #[test]
    fn test_service_id_conversions() {
        let id: ServiceId = "beardog-auth".into();
        assert_eq!(id.as_str(), "beardog-auth");
        assert_eq!(format!("{id}"), "beardog-auth");
    }

    // Port tests
    #[test]
    fn test_port_new() {
        let port = Port::new(8080);
        assert_eq!(port.value(), 8080);
    }

    #[test]
    fn test_port_from_u16() {
        let port: Port = 9000.into();
        assert_eq!(port.value(), 9000);
    }

    #[test]
    fn test_port_into_u16() {
        let port = Port::new(3000);
        let value: u16 = port.into();
        assert_eq!(value, 3000);
    }

    #[test]
    fn test_port_is_privileged() {
        assert!(Port::new(80).is_privileged());
        assert!(Port::new(443).is_privileged());
        assert!(!Port::new(8080).is_privileged());
    }

    #[test]
    fn test_port_is_registered() {
        assert!(Port::new(8080).is_registered());
        assert!(Port::new(3000).is_registered());
        assert!(!Port::new(80).is_registered());
        assert!(!Port::new(50000).is_registered());
    }

    #[test]
    fn test_port_is_ephemeral() {
        assert!(Port::new(50000).is_ephemeral());
        assert!(Port::new(60000).is_ephemeral());
        assert!(!Port::new(8080).is_ephemeral());
    }

    #[test]
    fn test_port_display() {
        let port = Port::new(8080);
        assert_eq!(format!("{port}"), "8080");
    }

    // TrustLevel tests
    #[test]
    fn test_trust_level_new() {
        let trust = TrustLevel::new(0.75);
        assert_eq!(trust.value(), 0.75);
    }

    #[test]
    fn test_trust_level_clamping() {
        let too_high = TrustLevel::new(1.5);
        assert_eq!(too_high.value(), 1.0);

        let too_low = TrustLevel::new(-0.5);
        assert_eq!(too_low.value(), 0.0);
    }

    #[test]
    fn test_trust_level_from_f64() {
        let trust: TrustLevel = 0.85.into();
        assert_eq!(trust.value(), 0.85);
    }

    #[test]
    fn test_trust_level_into_f64() {
        let trust = TrustLevel::new(0.6);
        let value: f64 = trust.into();
        assert_eq!(value, 0.6);
    }

    #[test]
    fn test_trust_level_is_high() {
        assert!(TrustLevel::new(0.9).is_high());
        assert!(TrustLevel::new(0.8).is_high());
        assert!(!TrustLevel::new(0.7).is_high());
    }

    #[test]
    fn test_trust_level_is_medium() {
        assert!(TrustLevel::new(0.6).is_medium());
        assert!(TrustLevel::new(0.7).is_medium());
        assert!(!TrustLevel::new(0.4).is_medium());
        assert!(!TrustLevel::new(0.9).is_medium());
    }

    #[test]
    fn test_trust_level_is_low() {
        assert!(TrustLevel::new(0.3).is_low());
        assert!(TrustLevel::new(0.1).is_low());
        assert!(!TrustLevel::new(0.6).is_low());
    }

    #[test]
    fn test_trust_level_display() {
        let trust = TrustLevel::new(0.754);
        assert_eq!(format!("{trust}"), "0.75");
    }

    #[test]
    fn test_trust_level_default() {
        let trust = TrustLevel::default();
        assert_eq!(trust.value(), 0.0);
    }

    #[test]
    fn test_trust_level_ordering() {
        let low = TrustLevel::new(0.3);
        let high = TrustLevel::new(0.8);
        assert!(low < high);
        assert!(high > low);
    }
}
