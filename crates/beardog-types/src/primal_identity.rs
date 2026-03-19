// SPDX-License-Identifier: AGPL-3.0-only

//! Primal Identity - Self-knowledge configuration
//!
//! Following TRUE PRIMAL pattern: each primal only knows itself.
//! Identity is established at startup and injected throughout the system.
//!
//! ## Architecture
//!
//! ```text
//! Server Startup
//!   ↓
//! PrimalIdentity::from_env() (read once, fail-fast)
//!   ↓
//! Arc<PrimalIdentity> (immutable, shared)
//!   ↓
//! Injected into all handlers
//!   ↓
//! Zero environment variable reads in business logic
//! ```
//!
//! ## Benefits
//!
//! - **Concurrent-Safe**: No global mutable state
//! - **Testable**: Explicit configuration in tests
//! - **Fail-Fast**: Errors at startup, not runtime
//! - **Explicit**: Dependencies visible in signatures
//! - **Zero-Cost**: Arc provides cheap cloning

use beardog_errors::BearDogError;

/// Primal identity configuration
///
/// Represents the identity of this primal (family and node).
/// Following TRUE PRIMAL pattern: primal only knows itself.
///
/// # Examples
///
/// ```no_run
/// use beardog_types::primal_identity::PrimalIdentity;
///
/// // Server startup (reads environment once)
/// let identity = PrimalIdentity::from_env()?;
///
/// // Test (explicit configuration)
/// let identity = PrimalIdentity::for_test("nat0", "tower1");
/// # Ok::<(), beardog_errors::BearDogError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimalIdentity {
    /// Genetic family identifier
    ///
    /// Example: "nat0", "prod-family"
    pub family_id: String,

    /// Node identifier within the family
    ///
    /// Example: "tower1", "node-alpha"
    pub node_id: String,
}

impl PrimalIdentity {
    /// Create identity from environment variables
    ///
    /// Reads from:
    /// 1. `FAMILY_ID` or `BEARDOG_FAMILY_ID`
    /// 2. `NODE_ID` or `BEARDOG_NODE_ID`
    ///
    /// # Errors
    ///
    /// Returns error if either environment variable is not set.
    /// This is intentional - we want to fail fast at startup if
    /// identity is not configured.
    ///
    /// # Examples
    ///
    /// ```bash
    /// export FAMILY_ID=nat0
    /// export NODE_ID=tower1
    /// ```
    pub fn from_env() -> Result<Self, BearDogError> {
        let family_id = std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .map_err(|_| {
                BearDogError::configuration(
                    "FAMILY_ID or BEARDOG_FAMILY_ID must be set. \
                     This identifies the genetic family this primal belongs to. \
                     Example: export FAMILY_ID=nat0",
                )
            })?;

        let node_id = std::env::var("NODE_ID")
            .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
            .map_err(|_| {
                BearDogError::configuration(
                    "NODE_ID or BEARDOG_NODE_ID must be set. \
                     This identifies this specific node within the family. \
                     Example: export NODE_ID=tower1",
                )
            })?;

        Ok(Self { family_id, node_id })
    }

    /// Create identity for testing
    ///
    /// Provides explicit configuration without environment variables.
    /// This enables concurrent tests with different identities.
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_types::primal_identity::PrimalIdentity;
    ///
    /// let identity = PrimalIdentity::for_test("nat0", "tower1");
    /// assert_eq!(identity.family_id, "nat0");
    /// assert_eq!(identity.node_id, "tower1");
    /// ```
    pub fn for_test(family_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            family_id: family_id.into(),
            node_id: node_id.into(),
        }
    }

    /// Get family ID
    #[must_use]
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    /// Get node ID
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get encryption tag for discovery/federation
    ///
    /// Format: `beardog:family:{family_id}`
    #[must_use]
    pub fn encryption_tag(&self) -> String {
        format!("beardog:family:{}", self.family_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_test_constructor() {
        let identity = PrimalIdentity::for_test("test-family", "test-node");

        assert_eq!(identity.family_id(), "test-family");
        assert_eq!(identity.node_id(), "test-node");
    }

    #[test]
    fn test_encryption_tag() {
        let identity = PrimalIdentity::for_test("nat0", "tower1");

        assert_eq!(identity.encryption_tag(), "beardog:family:nat0");
    }

    #[test]
    fn test_clone_and_equality() {
        let identity1 = PrimalIdentity::for_test("family1", "node1");
        let identity2 = identity1.clone();

        assert_eq!(identity1, identity2);
    }

    #[test]
    fn test_different_identities_not_equal() {
        let identity1 = PrimalIdentity::for_test("family1", "node1");
        let identity2 = PrimalIdentity::for_test("family2", "node2");

        assert_ne!(identity1, identity2);
    }

    #[test]
    #[serial_test::serial] // Only this test needs serial (env vars)
    fn test_from_env_success() {
        // Set environment variables
        std::env::set_var("FAMILY_ID", "env-family");
        std::env::set_var("NODE_ID", "env-node");

        let identity = PrimalIdentity::from_env().expect("Should read from env");

        assert_eq!(identity.family_id(), "env-family");
        assert_eq!(identity.node_id(), "env-node");

        // Cleanup
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("NODE_ID");
    }

    #[test]
    #[serial_test::serial] // Only this test needs serial (env vars)
    fn test_from_env_beardog_prefix() {
        // Clear standard vars
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("NODE_ID");

        // Set BEARDOG_ prefixed vars
        std::env::set_var("BEARDOG_FAMILY_ID", "beardog-family");
        std::env::set_var("BEARDOG_NODE_ID", "beardog-node");

        let identity = PrimalIdentity::from_env().expect("Should read BEARDOG_ vars");

        assert_eq!(identity.family_id(), "beardog-family");
        assert_eq!(identity.node_id(), "beardog-node");

        // Cleanup
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    #[serial_test::serial] // Only this test needs serial (env vars)
    fn test_from_env_missing_family_id() {
        // Clear all relevant env vars
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::set_var("NODE_ID", "test-node");

        let result = PrimalIdentity::from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("FAMILY_ID"));

        // Cleanup
        std::env::remove_var("NODE_ID");
    }

    #[test]
    #[serial_test::serial] // Only this test needs serial (env vars)
    fn test_from_env_missing_node_id() {
        // Clear all relevant env vars
        std::env::set_var("FAMILY_ID", "test-family");
        std::env::remove_var("NODE_ID");
        std::env::remove_var("BEARDOG_NODE_ID");

        let result = PrimalIdentity::from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("NODE_ID"));

        // Cleanup
        std::env::remove_var("FAMILY_ID");
    }
}
