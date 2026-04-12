// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability-Based Port Discovery
//!
//! Evolves hardcoded ports to runtime discovery with primal awareness.
//!
//! # Philosophy
//!
//! - **No Hardcoding**: Discover available ports at runtime
//! - **Primal Awareness**: Query other primals to avoid conflicts
//! - **Human Sovereignty**: Explicit configuration always wins
//! - **Graceful Fallback**: Multiple discovery strategies
//!
//! # Discovery Hierarchy
//!
//! ```text
//! 1. CLI Arguments      (Explicit human intent - highest priority)
//! 2. Environment Vars   (Human configuration)
//! 3. Config File        (Persistent settings)
//! 4. Primal Discovery   (Query running primals)
//! 5. System Query       (Find available port)
//! 6. Default Constant   (Last resort fallback)
//! ```

mod config;
mod discoverer;
mod env;
mod hierarchical;

pub use config::{
    DiscoveryStrategy, FALLBACK_EXCLUDED_DEV_PORTS, FALLBACK_PORT_DISCOVERY_TIMEOUT_MS,
    FALLBACK_PORT_SCAN_MAX, FALLBACK_PORT_SCAN_MIN, PortDiscoveryConfig,
};
pub use discoverer::PortDiscoverer;
pub use hierarchical::discover_port_hierarchical;

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "test assertions")]
mod tests {
    use super::*;
    use beardog_errors::process_env;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static PORT_DISCOVERY_ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn port_discovery_env_lock() -> MutexGuard<'static, ()> {
        PORT_DISCOVERY_ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("port discovery env test mutex poisoned")
    }

    #[tokio::test]
    async fn test_system_query_strategy() {
        // Modern idiomatic: Initialize struct with all values at once
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::SystemQuery,
            min_port: 58000, // Use high ports for testing
            max_port: 58100,
            ..Default::default()
        };

        let discoverer = PortDiscoverer::new(config);
        let result = discoverer.discover().await;

        assert!(result.is_ok(), "Should find available port in range");
        let port = result.expect("system query finds port in range");
        // Modern idiomatic: Use range contains
        assert!((58000..=58100).contains(&port));
    }

    #[tokio::test]
    async fn test_explicit_only_strategy() {
        // Modern idiomatic: Initialize struct with all values at once
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::ExplicitOnly,
            ..Default::default()
        };

        let discoverer = PortDiscoverer::new(config);
        let result = discoverer.discover().await;

        assert!(
            result.is_err(),
            "Explicit only should require configuration"
        );
    }

    #[tokio::test]
    async fn test_hierarchical_discovery_cli_override() {
        let result = discover_port_hierarchical(
            "NONEXISTENT_VAR",
            Some(12345), // CLI override
            Some(8080),  // Config value
            9090,        // Default
            PortDiscoveryConfig::default(),
        )
        .await;

        assert_eq!(
            result.expect("hierarchical discovery with CLI override"),
            12345,
            "CLI override should win"
        );
    }

    #[tokio::test]
    async fn test_hierarchical_discovery_default() {
        let result = discover_port_hierarchical(
            "NONEXISTENT_VAR",
            None, // No CLI
            None, // No config
            9090, // Default
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly, // Disable discovery
                ..Default::default()
            },
        )
        .await;

        assert_eq!(
            result.expect("hierarchical discovery falls back to default"),
            9090,
            "Should fall back to default"
        );
    }

    #[test]
    fn default_config_is_pure_and_from_env_matches_explicit_overrides() {
        let d = PortDiscoveryConfig::default();
        assert_eq!(d.min_port, FALLBACK_PORT_SCAN_MIN);
        assert_eq!(d.max_port, FALLBACK_PORT_SCAN_MAX);

        let c = PortDiscoveryConfig {
            min_port: 9100,
            max_port: 9101,
            discovery_timeout_ms: 1500,
            excluded_ports: vec![9100, 9101],
            ..Default::default()
        };
        assert_eq!(c.min_port, 9100);
        assert_eq!(c.max_port, 9101);
        assert_eq!(c.discovery_timeout_ms, 1500);
        assert_eq!(c.excluded_ports, vec![9100, 9101]);
    }

    #[tokio::test]
    async fn hierarchical_uses_config_when_env_unset() {
        let port = discover_port_hierarchical(
            "HIER_PORT_TEST_XYZ",
            None,
            Some(1111),
            2222,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical uses config when env unset");
        assert_eq!(port, 1111);
    }

    #[tokio::test]
    async fn hierarchical_invalid_env_falls_through_to_config() {
        let port = discover_port_hierarchical(
            "HIER_PORT_BAD",
            None,
            Some(3333),
            4444,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical invalid env falls through to config");
        assert_eq!(port, 3333);
    }

    #[tokio::test]
    async fn hierarchical_runtime_discovery_then_default() {
        let port = discover_port_hierarchical(
            "NONEXISTENT_HIER_PORT_999",
            None,
            None,
            4242,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical runtime discovery then default");
        assert_eq!(port, 4242);
    }

    #[tokio::test]
    async fn full_strategy_falls_back_to_system_when_primal_fails() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::Full,
            min_port: 58200,
            max_port: 58250,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let p = discoverer
            .discover()
            .await
            .expect("full strategy discovers port when primal fails");
        assert!((58200..=58250).contains(&p));
    }

    #[tokio::test]
    async fn primal_query_strategy_smoke() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58300,
            max_port: 58320,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let res = discoverer.discover().await;
        assert!(res.is_ok());
        let p = res.expect("primal query strategy discovers port");
        assert!((58300..=58320).contains(&p));
    }

    #[tokio::test]
    async fn discover_errors_when_no_free_port_in_range() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::SystemQuery,
            min_port: 60000,
            max_port: 60000,
            excluded_ports: vec![60000],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let err = discoverer.discover().await.unwrap_err();
        assert!(err.to_string().contains("No available ports"));
    }

    #[test]
    fn from_env_reads_process_env_overlay() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MIN", "9100");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MAX", "9200");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS", "1500");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_EXCLUDE", "9101, 9102");

        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.min_port, 9100);
        assert_eq!(c.max_port, 9200);
        assert_eq!(c.discovery_timeout_ms, 1500);
        assert_eq!(c.excluded_ports, vec![9101u16, 9102]);

        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MIN");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MAX");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_EXCLUDE");
    }

    #[test]
    fn parse_u16_env_invalid_falls_back_to_fallback() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MIN", "not-a-number");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.min_port, FALLBACK_PORT_SCAN_MIN);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MIN");
    }

    #[tokio::test]
    async fn hierarchical_falls_back_to_default_when_discovery_range_invalid() {
        let port = discover_port_hierarchical(
            "HIER_INVALID_RANGE_PORT",
            None,
            None,
            7777,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::SystemQuery,
                min_port: 65500,
                max_port: 65400,
                excluded_ports: vec![],
                discovery_timeout_ms: 50,
            },
        )
        .await
        .expect("default when system discovery cannot find a port");
        assert_eq!(port, 7777);
    }

    #[tokio::test]
    async fn discovered_primal_ports_env_merges_valid_and_ignores_invalid() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_DISCOVERED_PRIMAL_PORTS", "8443,not-a-port,9001");

        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58400,
            max_port: 58500,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let res = discoverer.discover().await;
        assert!(res.is_ok());

        process_env::remove_var("BEARDOG_DISCOVERED_PRIMAL_PORTS");
    }

    #[tokio::test]
    async fn hierarchical_prefers_env_overlay_over_config() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("HIER_PORT_ENV_OVERLAY", "4411");

        let port = discover_port_hierarchical(
            "HIER_PORT_ENV_OVERLAY",
            None,
            Some(9911),
            8822,
            PortDiscoveryConfig::default(),
        )
        .await
        .expect("env should win over config");

        assert_eq!(port, 4411);

        process_env::remove_var("HIER_PORT_ENV_OVERLAY");
    }

    #[tokio::test]
    async fn primal_query_errors_when_no_candidate_in_tiny_excluded_range() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 60100,
            max_port: 60100,
            excluded_ports: vec![60100],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let err = discoverer.discover().await.expect_err("no port available");
        assert!(err.to_string().contains("No available ports"));
        assert!(err.to_string().contains("primal"));
    }

    #[test]
    fn from_env_invalid_timeout_ms_falls_back_to_fallback() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS", "not-a-number");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.discovery_timeout_ms, FALLBACK_PORT_DISCOVERY_TIMEOUT_MS);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS");
    }

    #[test]
    fn from_env_invalid_max_port_falls_back() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MAX", "bogus");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.max_port, FALLBACK_PORT_SCAN_MAX);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MAX");
    }

    #[tokio::test]
    async fn discovered_primal_ports_skips_empty_csv_segments() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_DISCOVERED_PRIMAL_PORTS", "8443,,,9001");

        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58600,
            max_port: 58700,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        assert!(discoverer.discover().await.is_ok());

        process_env::remove_var("BEARDOG_DISCOVERED_PRIMAL_PORTS");
    }
}
