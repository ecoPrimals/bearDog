//! Shared Handler Utilities
//!
//! Common utilities used across multiple JSON-RPC handlers.
//!
//! # Self-Knowledge Pattern
//!
//! The self-knowledge pattern allows primals to discover their own identity
//! at runtime from environment variables, eliminating hardcoded values.
//!
//! ## Environment Variables
//!
//! | Variable | Purpose | Example |
//! |----------|---------|---------|
//! | `PRIMAL_NAME` | Primary primal identifier | `songbird` |
//! | `BEARDOG_NAME` | BearDog-specific name (fallback) | `beardog` |
//! | `FAMILY_ID` | Genetic lineage family | `biomeOS` |
//! | `NODE_ID` | Unique node identifier | `tower-01` |

/// Get the primal name using self-knowledge pattern.
///
/// Discovers the primal's name from environment variables at runtime,
/// eliminating hardcoded names and enabling deployment flexibility.
///
/// # Priority Order
///
/// 1. `PRIMAL_NAME` - Universal primal identifier
/// 2. `BEARDOG_NAME` - BearDog-specific fallback
/// 3. `"beardog"` - Default when no env var is set
///
/// # Examples
///
/// ```rust
/// // When PRIMAL_NAME=songbird
/// // get_primal_name() returns "songbird"
///
/// // When no env vars are set
/// // get_primal_name() returns "beardog"
/// ```
///
/// # See Also
///
/// - `get_family_id()` - Genetic lineage family discovery
/// - `get_node_id()` - Unique node identifier discovery
pub fn get_primal_name() -> String {
    std::env::var("PRIMAL_NAME")
        .or_else(|_| std::env::var("BEARDOG_NAME"))
        .unwrap_or_else(|_| "beardog".to_string())
}

/// Get the family ID using self-knowledge pattern.
///
/// Discovers the genetic lineage family from environment variables.
///
/// # Priority Order
///
/// 1. `FAMILY_ID` - Explicit family identifier
/// 2. `BIOMEOS_FAMILY` - BiomeOS-specific fallback
/// 3. `"unknown"` - Default when no env var is set
pub fn get_family_id() -> String {
    std::env::var("FAMILY_ID")
        .or_else(|_| std::env::var("BIOMEOS_FAMILY"))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Get the node ID using self-knowledge pattern.
///
/// Discovers the unique node identifier from environment variables.
///
/// # Priority Order
///
/// 1. `NODE_ID` - Explicit node identifier
/// 2. `HOSTNAME` - System hostname fallback
/// 3. `"unknown"` - Default when no env var is set
pub fn get_node_id() -> String {
    std::env::var("NODE_ID")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_primal_name_default() {
        // Remove env vars to test default
        std::env::remove_var("PRIMAL_NAME");
        std::env::remove_var("BEARDOG_NAME");

        assert_eq!(get_primal_name(), "beardog");
    }

    #[test]
    fn test_get_primal_name_from_env() {
        std::env::set_var("PRIMAL_NAME", "test_primal");
        assert_eq!(get_primal_name(), "test_primal");
        std::env::remove_var("PRIMAL_NAME");
    }

    #[test]
    fn test_get_primal_name_beardog_fallback() {
        std::env::remove_var("PRIMAL_NAME");
        std::env::set_var("BEARDOG_NAME", "custom_beardog");
        assert_eq!(get_primal_name(), "custom_beardog");
        std::env::remove_var("BEARDOG_NAME");
    }

    #[test]
    fn test_get_family_id_default() {
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BIOMEOS_FAMILY");
        assert_eq!(get_family_id(), "unknown");
    }

    #[test]
    fn test_get_node_id_default() {
        std::env::remove_var("NODE_ID");
        // HOSTNAME might be set by the system, so just verify it returns something
        let node_id = get_node_id();
        assert!(!node_id.is_empty());
    }
}
