// SPDX-License-Identifier: AGPL-3.0-only

// ! Lineage ID wrapper for biomeOS integration
//!
//! Provides a simplified LineageID type that wraps chain_id + node_id
//! for easier API integration with biomeOS / HTTP gateway peers.

use serde::{Deserialize, Serialize};
use std::fmt;

use beardog_errors::BearDogError;

/// Unique identifier for a genetic lineage
///
/// Format: `lineage:service_type:timestamp:hash`
///
/// Example: `lineage:tower1:1735000000:abc123def456`
///
/// This wraps a chain_id and node_id internally for simplified API usage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineageID(String);

impl LineageID {
    /// Create a new LineageID from a formatted string
    ///
    /// # Arguments
    ///
    /// * `id` - Formatted lineage ID string
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_genetics::birdsong::LineageID;
    ///
    /// let id = LineageID::new("lineage:tower1:1735000000:abc123");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Format a lineage ID from components
    ///
    /// # Arguments
    ///
    /// * `service_type` - Type of service (e.g., "tower", "mesh-relay")
    /// * `chain_id` - Internal chain ID
    /// * `node_id` - Internal node ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_genetics::birdsong::LineageID;
    ///
    /// let id = LineageID::format("tower", "chain-abc", "node-123");
    /// assert!(id.as_str().starts_with("lineage:tower:"));
    /// ```
    pub fn format(service_type: &str, chain_id: &str, node_id: &str) -> Self {
        use std::time::SystemTime;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0); // Fallback to 0 if system time is before UNIX_EPOCH (extremely rare)

        // Create a short hash from chain_id + node_id for readability
        let hash = format!("{:x}", {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            chain_id.hash(&mut hasher);
            node_id.hash(&mut hasher);
            hasher.finish()
        });

        Self(format!(
            "lineage:{}:{}:{}:{}",
            service_type,
            timestamp,
            &hash[..12], // First 12 chars of hash
            node_id
        ))
    }

    /// Parse a lineage ID into its components
    ///
    /// Returns (service_type, timestamp, hash, node_id)
    ///
    /// # Errors
    ///
    /// Returns error if the lineage ID format is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_genetics::birdsong::LineageID;
    ///
    /// let id = LineageID::new("lineage:tower:1735000000:abc123:node-1");
    /// let (service_type, timestamp, hash, node_id) = id.parse().expect("valid lineage ID");
    /// assert_eq!(service_type, "tower");
    /// assert_eq!(node_id, "node-1");
    /// ```
    pub fn parse(&self) -> Result<(String, u64, String, String), BearDogError> {
        let parts: Vec<&str> = self.0.split(':').collect();

        if parts.len() != 5 {
            return Err(BearDogError::invalid_input(
                "Invalid lineage ID format (expected lineage:type:timestamp:hash:node_id)",
            ));
        }

        if parts[0] != "lineage" {
            return Err(BearDogError::invalid_input(
                "LineageID must start with 'lineage:'",
            ));
        }

        let service_type = parts[1].to_string();
        let timestamp = parts[2]
            .parse::<u64>()
            .map_err(|_| BearDogError::invalid_input("Invalid timestamp in lineage ID"))?;
        let hash = parts[3].to_string();
        let node_id = parts[4].to_string();

        Ok((service_type, timestamp, hash, node_id))
    }

    /// Extract the node_id from this lineage ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_genetics::birdsong::LineageID;
    ///
    /// let id = LineageID::new("lineage:tower:1735000000:abc123:node-1");
    /// assert_eq!(id.node_id().expect("valid lineage ID"), "node-1");
    /// ```
    pub fn node_id(&self) -> Result<String, BearDogError> {
        let (_, _, _, node_id) = self.parse()?;
        Ok(node_id)
    }

    /// Extract the service_type from this lineage ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_genetics::birdsong::LineageID;
    ///
    /// let id = LineageID::new("lineage:tower:1735000000:abc123:node-1");
    /// assert_eq!(id.service_type().expect("valid lineage ID"), "tower");
    /// ```
    pub fn service_type(&self) -> Result<String, BearDogError> {
        let (service_type, _, _, _) = self.parse()?;
        Ok(service_type)
    }

    /// Get the raw string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for LineageID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for LineageID {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for LineageID {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for LineageID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lineage_id_format() {
        let id = LineageID::format("tower", "chain-abc", "node-123");
        assert!(id.as_str().starts_with("lineage:tower:"));
        assert!(id.as_str().contains("node-123"));
    }

    #[test]
    fn test_lineage_id_parse() {
        let id = LineageID::new("lineage:tower:1735000000:abc123def456:node-1");
        let (service_type, timestamp, hash, node_id) =
            id.parse().expect("parse canonical lineage ID in test");

        assert_eq!(service_type, "tower");
        assert_eq!(timestamp, 1_735_000_000);
        assert_eq!(hash, "abc123def456");
        assert_eq!(node_id, "node-1");
    }

    #[test]
    fn test_lineage_id_node_id() {
        let id = LineageID::new("lineage:mesh-relay:1735000000:xyz789:relay-node-1");
        assert_eq!(
            id.node_id().expect("node_id from mesh-relay lineage ID"),
            "relay-node-1"
        );
    }

    #[test]
    fn test_lineage_id_service_type() {
        let id = LineageID::new("lineage:mesh-relay:1735000000:xyz789:node-1");
        assert_eq!(
            id.service_type().expect("service_type from lineage ID"),
            "mesh-relay"
        );
    }

    #[test]
    fn test_lineage_id_invalid_format() {
        let id = LineageID::new("invalid:format");
        assert!(id.parse().is_err());
    }

    #[test]
    fn test_lineage_id_roundtrip() {
        let id1 = LineageID::format("tower", "chain-1", "node-1");
        let node_id = id1.node_id().expect("node_id roundtrip");
        let service_type = id1.service_type().expect("service_type roundtrip");

        assert_eq!(node_id, "node-1");
        assert_eq!(service_type, "tower");
    }

    #[test]
    fn test_lineage_id_display() {
        let id = LineageID::new("lineage:test:123:abc:node");
        assert_eq!(format!("{id}"), "lineage:test:123:abc:node");
    }

    #[test]
    fn test_lineage_id_from_str() {
        let id: LineageID = "lineage:tower:123:abc:node".into();
        assert_eq!(id.as_str(), "lineage:tower:123:abc:node");
    }

    #[test]
    fn test_lineage_id_equality() {
        let id1 = LineageID::new("lineage:tower:123:abc:node-1");
        let id2 = LineageID::new("lineage:tower:123:abc:node-1");
        let id3 = LineageID::new("lineage:tower:123:abc:node-2");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
