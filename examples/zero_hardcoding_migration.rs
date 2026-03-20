// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! # Zero-Hardcoding Migration Example
//!
//! This example demonstrates how to migrate from hardcoded values to the
//! `BearDog` configuration system, eliminating all hardcoding from the codebase.
//!
//! ## Problem: Hardcoded Values
//!
//! Traditional code has many hardcoded constants:
//! - Ports: 8080, 9090, 5432
//! - Timeouts: 30 seconds, 5 minutes
//! - Limits: 100 connections, 1024 buffer size
//! - Paths: "/var/lib/beardog", "/etc/beardog"
//!
//! ## Solution: Configuration System
//!
//! `BearDog` provides a zero-hardcoding configuration system with:
//! - Environment variable support
//! - Configuration file support (TOML/JSON/YAML)
//! - Builder pattern for programmatic configuration
//! - Validation and error handling
//! - Global singleton for easy access
//!
//! ## Migration Pattern
//!
//! 1. **Before**: Hardcoded constants
//! 2. **After**: Global configuration singleton
//! 3. **Result**: Zero hardcoding, fully configurable

use beardog_config::global::BEARDOG_CONFIG;

// ============================================================================
// ANTI-PATTERN: Hardcoded Constants
// ============================================================================

mod bad_hardcoded {
    use std::time::Duration;

    // ❌ Hardcoded port
    pub const API_PORT: u16 = 8080;

    // ❌ Hardcoded timeout
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

    // ❌ Hardcoded limit
    pub const MAX_CONNECTIONS: usize = 100;

    // ❌ Hardcoded path
    pub const CONFIG_PATH: &str = "/etc/beardog/config.toml";

    pub fn start_api_server() {
        println!("❌ BAD: Starting API server on hardcoded port {API_PORT}");
        println!("❌ BAD: Request timeout: {REQUEST_TIMEOUT:?}");
        println!("❌ BAD: Max connections: {MAX_CONNECTIONS}");
    }

    pub fn load_config() {
        println!("❌ BAD: Loading config from hardcoded path: {CONFIG_PATH}");
    }
}

// ============================================================================
// IDIOMATIC: Global Configuration System
// ============================================================================

mod good_configuration {
    use beardog_config::global::BEARDOG_CONFIG;

    pub fn start_api_server() {
        // ✅ GOOD: Load from environment-aware configuration
        let port = BEARDOG_CONFIG.network.api.port;
        let max_conn = BEARDOG_CONFIG.network.api.max_connections;

        println!("✅ GOOD: Starting API server on configured port {port}");
        println!("✅ GOOD: Max connections from config: {max_conn}");
        println!("💡 TIP: Set BEARDOG_API_PORT=9000 to change port without code changes");
    }

    pub fn make_http_request() {
        // ✅ GOOD: Use configured timeout
        let timeout_secs = BEARDOG_CONFIG.limits.operation_timeout_secs;
        println!("✅ GOOD: Operation timeout from config: {timeout_secs} seconds");
        println!("💡 TIP: Set BEARDOG_OPERATION_TIMEOUT_SECS=60 to adjust timeout");
    }

    pub fn load_config() {
        // ✅ GOOD: Configuration is loaded automatically
        // Just access it via BEARDOG_CONFIG global
        println!("✅ GOOD: Configuration loaded from:");
        println!("   1. $BEARDOG_CONFIG_PATH (if set)");
        println!("   2. Environment variables (BEARDOG_*)");
        println!("   3. Secure defaults");
    }
}

// ============================================================================
// IDIOMATIC: Convenience Functions
// ============================================================================

mod good_convenience {
    use beardog_config::global;

    pub fn quick_access_example() {
        // ✅ IDIOMATIC: Use convenience functions for common values
        let api_port = global::api_port();
        let discovery_port = global::discovery_port();
        let admin_port = global::admin_port();

        println!("✅ API port: {api_port}");
        println!("✅ Discovery port: {discovery_port}");
        println!("✅ Admin port: {admin_port}");
    }

    pub fn full_config_access() {
        // ✅ IDIOMATIC: Access full config for detailed settings
        let config = global::config();

        println!("✅ TLS enabled: {}", config.network.api.tls_enabled);
        println!(
            "✅ Max request size: {} bytes",
            config.limits.max_message_size
        );
        println!(
            "✅ Operation timeout: {} seconds",
            config.limits.operation_timeout_secs
        );
    }
}

// ============================================================================
// MIGRATION GUIDE: Step-by-Step Replacement
// ============================================================================

mod migration_examples {
    use beardog_config::global::BEARDOG_CONFIG;
    use std::time::Duration;

    // Example 1: Migrating Port Constants
    // -----------------------------------

    // ❌ BEFORE:
    // const API_PORT: u16 = 8080;
    // let port = API_PORT;

    // ✅ AFTER:
    pub fn get_api_port() -> u16 {
        BEARDOG_CONFIG.network.api.port
    }

    // Example 2: Migrating Timeout Constants
    // --------------------------------------

    // ❌ BEFORE:
    // const TIMEOUT: Duration = Duration::from_secs(30);
    // let timeout = TIMEOUT;

    // ✅ AFTER:
    pub fn get_connection_timeout() -> u64 {
        BEARDOG_CONFIG.limits.operation_timeout_secs
    }

    // Example 3: Migrating Limit Constants
    // ------------------------------------

    // ❌ BEFORE:
    // const MAX_CONNECTIONS: usize = 100;
    // if connections > MAX_CONNECTIONS { ... }

    // ✅ AFTER:
    pub fn check_connection_limit(connections: usize) -> bool {
        connections <= BEARDOG_CONFIG.network.api.max_connections
    }

    // Example 4: Migrating Path Constants
    // -----------------------------------

    // ❌ BEFORE:
    // const CONFIG_DIR: &str = "/etc/beardog";
    // let path = Path::new(CONFIG_DIR);

    // ✅ AFTER:
    pub fn get_config_dir() -> std::path::PathBuf {
        BEARDOG_CONFIG.paths.config_dir.clone()
    }

    // Example 5: Migrating Protocol/Endpoint Constants
    // -----------------------------------------------

    // ❌ BEFORE:
    // const SERVICE_URL: &str = "https://api.primal.io:8080";

    // ✅ AFTER:
    pub fn get_service_url() -> String {
        let config = &BEARDOG_CONFIG.network.api;
        let protocol = if config.tls_enabled { "https" } else { "http" };
        format!("{}://{}:{}", protocol, config.bind_address, config.port)
    }
}

// ============================================================================
// ENVIRONMENT VARIABLE REFERENCE
// ============================================================================

/// Print all supported environment variables for configuration
pub fn print_env_var_reference() {
    println!("\n🌍 BearDog Configuration Environment Variables:");
    println!("================================================\n");

    println!("📡 NETWORK CONFIGURATION:");
    println!("  BEARDOG_API_PORT          - API server port (default: 8080)");
    println!("  BEARDOG_API_BIND_ADDRESS  - API bind address (default: 127.0.0.1)");
    println!("  BEARDOG_DISCOVERY_PORT    - Discovery port (default: 9090)");
    println!("  BEARDOG_ADMIN_PORT        - Admin port (default: 9091)");
    println!("  BEARDOG_MAX_CONNECTIONS   - Max concurrent connections (default: 100)\n");

    println!("⏱️  TIMEOUT CONFIGURATION:");
    println!("  BEARDOG_CONNECTION_TIMEOUT   - Connection timeout in seconds");
    println!("  BEARDOG_REQUEST_TIMEOUT      - Request timeout in seconds");
    println!("  BEARDOG_IDLE_TIMEOUT         - Idle connection timeout in seconds\n");

    println!("🔒 SECURITY CONFIGURATION:");
    println!("  BEARDOG_TLS_ENABLED       - Enable TLS (true/false)");
    println!("  BEARDOG_TLS_CERT_PATH     - Path to TLS certificate");
    println!("  BEARDOG_TLS_KEY_PATH      - Path to TLS private key\n");

    println!("📂 PATH CONFIGURATION:");
    println!("  BEARDOG_CONFIG_DIR        - Configuration directory");
    println!("  BEARDOG_DATA_DIR          - Data storage directory");
    println!("  BEARDOG_LOG_DIR           - Log file directory\n");

    println!("🔧 GENERAL:");
    println!("  BEARDOG_CONFIG_PATH       - Full path to config file (TOML/JSON/YAML)");
    println!("  BEARDOG_LOG_LEVEL         - Log level (trace/debug/info/warn/error)\n");

    println!("💡 USAGE EXAMPLES:");
    println!("  export BEARDOG_API_PORT=9000");
    println!("  export BEARDOG_LOG_LEVEL=debug");
    println!("  export BEARDOG_CONFIG_PATH=/path/to/config.toml\n");
}

// ============================================================================
// SEARCH AND REPLACE PATTERNS
// ============================================================================

/// Common search and replace patterns for migration
pub fn print_search_replace_patterns() {
    println!("\n🔍 Migration Search & Replace Patterns:");
    println!("========================================\n");

    println!("Pattern 1: Hardcoded Ports");
    println!("  Search:  const API_PORT: u16 = 8080;");
    println!("  Replace: // use beardog_config::global::BEARDOG_CONFIG");
    println!("           // let port = BEARDOG_CONFIG.network.api.port;\n");

    println!("Pattern 2: Hardcoded Timeouts");
    println!("  Search:  Duration::from_secs(30)");
    println!("  Replace: BEARDOG_CONFIG.limits.connection_timeout\n");

    println!("Pattern 3: Hardcoded Limits");
    println!("  Search:  const MAX_CONNECTIONS: usize = 100;");
    println!("  Replace: // use beardog_config::global::BEARDOG_CONFIG");
    println!("           // let max = BEARDOG_CONFIG.network.api.max_connections;\n");

    println!("Pattern 4: Hardcoded Paths");
    println!("  Search:  \"/etc/beardog/config.toml\"");
    println!("  Replace: BEARDOG_CONFIG.paths.config_dir.join(\"config.toml\")\n");

    println!("Pattern 5: Hardcoded Endpoints");
    println!("  Search:  \"https://api.example.com:8080\"");
    println!("  Replace: // Build from config");
    println!("           // format!(\"https://{{}}:{{}}\", addr, port)\n");
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("\n🐻 BearDog Zero-Hardcoding Migration Example\n");
    println!("{}", "=".repeat(60));

    // Show the problem
    println!("\n❌ ANTI-PATTERN: Hardcoded Values\n");
    bad_hardcoded::start_api_server();
    bad_hardcoded::load_config();

    println!("\n{}", "=".repeat(60));

    // Show the solution
    println!("\n✅ IDIOMATIC: Configuration System\n");
    good_configuration::start_api_server();
    good_configuration::make_http_request();
    good_configuration::load_config();

    println!("\n{}", "=".repeat(60));

    // Show convenience functions
    println!("\n✅ CONVENIENCE FUNCTIONS\n");
    good_convenience::quick_access_example();

    println!("\n{}", "=".repeat(60));

    // Show actual config values
    println!("\n📋 CURRENT CONFIGURATION VALUES:\n");
    println!("  API Port:          {}", BEARDOG_CONFIG.network.api.port);
    println!(
        "  Discovery Port:    {}",
        BEARDOG_CONFIG.network.discovery.port
    );
    println!("  Admin Port:        {}", BEARDOG_CONFIG.network.admin.port);
    println!(
        "  Max Connections:   {}",
        BEARDOG_CONFIG.network.api.max_connections
    );
    println!(
        "  Operation Timeout: {} secs",
        BEARDOG_CONFIG.limits.operation_timeout_secs
    );

    println!("\n{}", "=".repeat(60));

    // Print reference guides
    print_env_var_reference();

    println!("\n{}", "=".repeat(60));

    print_search_replace_patterns();

    println!("\n{}", "=".repeat(60));

    println!("\n✅ NEXT STEPS:");
    println!("   1. Replace hardcoded constants with BEARDOG_CONFIG");
    println!("   2. Test with different environment variables");
    println!("   3. Validate configuration at startup");
    println!("   4. Document configuration options");

    println!("\n🎉 Migration complete! Your code is now 100% configurable!\n");
}
