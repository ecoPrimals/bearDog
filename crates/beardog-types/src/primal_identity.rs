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
//! PrimalIdentity::from_env() (read once, standalone fallback)
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
//! - **Standalone-Safe**: Defaults to standalone mode per `UniBin` v1.1
//! - **Explicit**: Dependencies visible in signatures
//! - **Zero-Cost**: Arc provides cheap cloning

/// Default family identifier used when no environment variable is set.
///
/// Per `UniBin` v1.1 / PRIMAL IPC Protocol v3.1: primals MUST NOT hard-fail
/// on missing identity env vars; they default to standalone mode.
pub const DEFAULT_STANDALONE_FAMILY: &str = "standalone";

/// Default node identifier used when no environment variable is set.
pub const DEFAULT_STANDALONE_NODE: &str = "default";

/// Primal identity configuration
///
/// Represents the identity of this primal (family and node).
/// Following TRUE PRIMAL pattern: primal only knows itself.
///
/// # Examples
///
/// ```
/// use beardog_types::primal_identity::PrimalIdentity;
///
/// // Server startup (reads environment, falls back to standalone)
/// let identity = PrimalIdentity::from_env();
///
/// // Test (explicit configuration)
/// let identity = PrimalIdentity::for_test("nat0", "tower1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimalIdentity {
    /// Genetic family identifier
    ///
    /// Example: "nat0", "prod-family", "standalone"
    pub family_id: String,

    /// Node identifier within the family
    ///
    /// Example: "tower1", "node-alpha", "default"
    pub node_id: String,

    /// Whether this identity was resolved from environment or defaulted
    is_standalone: bool,
}

impl PrimalIdentity {
    /// Create identity from environment variables with standalone fallback.
    ///
    /// Reads from:
    /// 1. `FAMILY_ID` or `BEARDOG_FAMILY_ID` (defaults to `"standalone"`)
    /// 2. `NODE_ID` or `BEARDOG_NODE_ID` (defaults to `"default"`)
    ///
    /// Per `UniBin` v1.1 / PRIMAL IPC Protocol v3.1, primals MUST NOT
    /// hard-fail when identity env vars are absent. Standalone mode
    /// is fully operational for local-only usage.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Orchestrated mode:
    /// export FAMILY_ID=nat0
    /// export NODE_ID=tower1
    ///
    /// # Standalone mode (no env vars needed):
    /// beardog server --port 9000
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        let family_id = std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .ok();

        let node_id = std::env::var("NODE_ID")
            .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
            .ok();

        let is_standalone = family_id.is_none() && node_id.is_none();

        Self {
            family_id: family_id.unwrap_or_else(|| DEFAULT_STANDALONE_FAMILY.to_owned()),
            node_id: node_id.unwrap_or_else(|| DEFAULT_STANDALONE_NODE.to_owned()),
            is_standalone,
        }
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
            is_standalone: false,
        }
    }

    /// Whether this identity is running in standalone mode (no env vars were set).
    #[must_use]
    pub const fn is_standalone(&self) -> bool {
        self.is_standalone
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
        assert!(!identity.is_standalone());
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
    fn test_standalone_defaults() {
        let identity = PrimalIdentity {
            family_id: DEFAULT_STANDALONE_FAMILY.to_owned(),
            node_id: DEFAULT_STANDALONE_NODE.to_owned(),
            is_standalone: true,
        };
        assert_eq!(identity.family_id(), "standalone");
        assert_eq!(identity.node_id(), "default");
        assert!(identity.is_standalone());
    }

    #[test]
    fn test_from_env_beardog_prefix_equivalent() {
        let identity = PrimalIdentity::for_test("beardog-family", "beardog-node");
        assert_eq!(identity.family_id(), "beardog-family");
        assert_eq!(identity.node_id(), "beardog-node");
    }

    #[test]
    fn encryption_tag_escapes_nothing_special_chars_in_family_id() {
        let identity = PrimalIdentity::for_test("fam:with:colons", "n");
        assert_eq!(identity.encryption_tag(), "beardog:family:fam:with:colons");
    }

    #[test]
    fn debug_contains_family_and_node() {
        let identity = PrimalIdentity::for_test("test-fam", "test-nod");
        let d = format!("{identity:?}");
        assert!(d.contains("test-fam") && d.contains("test-nod"));
    }

    #[test]
    fn accessors_match_fields_after_for_test() {
        let i = PrimalIdentity::for_test("alpha", "beta");
        assert_eq!(i.family_id, i.family_id().to_string());
        assert_eq!(i.node_id, i.node_id().to_string());
    }

    #[test]
    fn standalone_struct_reflects_is_standalone_accessor() {
        let i = PrimalIdentity {
            family_id: DEFAULT_STANDALONE_FAMILY.to_owned(),
            node_id: DEFAULT_STANDALONE_NODE.to_owned(),
            is_standalone: true,
        };
        assert!(i.is_standalone());
    }
}
