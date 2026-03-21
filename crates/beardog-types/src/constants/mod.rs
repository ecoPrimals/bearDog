// SPDX-License-Identifier: AGPL-3.0-only

// Domain-Specific Constants
//
// This module provides domain-organized constants that replace the large monolithic
// unified.rs file with maintainable, modular constant definitions.
//
// ## Architecture
//
// Constants are organized by functional domain:
// - **System**: Core system constants, versions, limits, timeouts
// - **Network**: Network addresses, ports, headers, timeouts
// - **Security**: Cryptographic, authentication, session constants
// - **Database**: Database connection, query, schema constants
// - **Monitoring**: Metrics, logging, health check constants
//
// ## Documentation
//
// See `/CONSTANTS_DOCUMENTATION.md` for detailed rationale for all "hardcoded" values.
// All constants are industry standards (IANA, RFC, NIST) and overridable via config.

/// Constants grouped by domain (system, network, security, validation, etc.).
pub mod domains;

/// Network port constants from IANA registry and common conventions.
///
/// Names use industry-standard values (IANA, RFC). These are **default fallbacks** for
/// documentation and static helpers; production code should prefer `beardog_config` /
/// environment (`BEARDOG_*_PORT`) or discovery.
pub mod network {

    /// HTTPS standard port (IANA assigned)
    ///
    /// **Why 443**: Officially assigned by IANA for HTTPS traffic
    /// **Standard**: RFC 2818
    /// **Override**: Set `BEARDOG_API_PORT=<port>` or use config file
    pub const HTTPS_PORT: u16 = 443;

    /// HTTP standard port (IANA assigned)
    ///
    /// **Why 80**: Officially assigned by IANA for HTTP traffic
    /// **Standard**: RFC 2616
    /// **Override**: Set `BEARDOG_HTTP_PORT=<port>` or use config file
    pub const HTTP_PORT: u16 = 80;

    /// Alternative HTTPS port (common convention)
    ///
    /// **Why 8443**: Common alternative for HTTPS when 443 is unavailable
    /// **Convention**: Widely used in development and corporate environments
    /// **Override**: Set `BEARDOG_API_PORT=<port>` or use config file
    pub const HTTPS_ALT_PORT: u16 = 8443;

    /// HTTP development port (common convention)
    ///
    /// **Why 8080**: De facto standard for HTTP development servers
    /// **Convention**: Used by Tomcat, Jetty, and most frameworks
    /// **Override**: Set `BEARDOG_DEV_PORT=<port>` or use config file
    pub const HTTP_DEV_PORT: u16 = 8080;

    /// Prometheus/Metrics port (convention)
    ///
    /// **Why 9090**: Standard port for Prometheus and metrics endpoints
    /// **Convention**: Prometheus project default
    /// **Override**: Set `BEARDOG_METRICS_PORT=<port>` or use config file
    pub const METRICS_PORT: u16 = 9090;

    /// PostgreSQL standard port (IANA assigned)
    ///
    /// **Why 5432**: Officially assigned by IANA for PostgreSQL
    /// **Standard**: IANA registry
    /// **Override**: Set `DATABASE_PORT=<port>` or use config file
    pub const POSTGRESQL_PORT: u16 = 5432;

    /// Grafana standard port (convention)
    ///
    /// **Why 3000**: Grafana project default
    /// **Convention**: Grafana documentation
    /// **Override**: Set `GRAFANA_PORT=<port>` or use config file
    pub const GRAFANA_PORT: u16 = 3000;
}

/// Standard localhost addresses defined by RFC and operating systems.
///
/// These are **universal standards** from RFCs 3330, 4291, and POSIX.
pub mod localhost {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    /// IPv4 localhost address
    ///
    /// **Why 127.0.0.1**: Defined by RFC 3330 as loopback address
    /// **Standard**: RFC 3330, RFC 5735
    /// **Use**: Testing, local development, secure defaults
    pub const LOCALHOST_V4: Ipv4Addr = Ipv4Addr::LOCALHOST;

    /// IPv6 localhost address
    ///
    /// **Why ::1**: Defined by RFC 4291 as IPv6 loopback
    /// **Standard**: RFC 4291
    /// **Use**: IPv6 testing, local development
    pub const LOCALHOST_V6: Ipv6Addr = Ipv6Addr::LOCALHOST;

    /// Bind to all interfaces (IPv4)
    ///
    /// **Why 0.0.0.0**: Standard "any address" binding
    /// **Standard**: POSIX, BSD sockets
    /// **Security**: Use with caution - exposes to network
    pub const ANY_V4: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

    /// Default safe bind address (localhost only)
    ///
    /// **Why localhost**: Security best practice - don't expose by default
    /// **Rationale**: Principle of least privilege
    /// **Override**: Set `BEARDOG_BIND_ADDRESS=0.0.0.0` to expose
    pub const DEFAULT_BIND: IpAddr = IpAddr::V4(LOCALHOST_V4);
}

/// Cryptographic parameters from NIST, FIPS, and industry recommendations.
///
/// All values follow security standards and can be overridden via configuration.
pub mod crypto {

    /// RSA key size (bits) - NIST recommendation
    ///
    /// **Why 2048**: NIST SP 800-57 minimum for 2030+
    /// **Standard**: NIST SP 800-57, FIPS 186-4
    /// **Override**: Set `BEARDOG_RSA_KEY_SIZE=<bits>` for different security level
    pub const RSA_KEY_SIZE_BITS: u32 = 2048;

    /// AES key size (bits) - FIPS requirement
    ///
    /// **Why 256**: AES-256 is FIPS 140-2 approved and provides 128-bit security
    /// **Standard**: FIPS 197, NIST SP 800-38D
    /// **Override**: Set `BEARDOG_AES_KEY_SIZE=<bits>` (128, 192, or 256)
    pub const AES_KEY_SIZE_BITS: u32 = 256;

    /// Default hash algorithm - FIPS approved
    ///
    /// **Why SHA3-256**: Modern, secure, FIPS 202 approved
    /// **Standard**: FIPS 202
    /// **Alternatives**: SHA-256 (FIPS 180-4), BLAKE3 (performance)
    pub const DEFAULT_HASH: &str = "SHA3-256";

    /// Minimum entropy bytes for secure random
    ///
    /// **Why 32**: 256 bits = 128-bit security level (NIST recommendation)
    /// **Standard**: NIST SP 800-90A
    /// **Use**: Key generation, nonce creation
    pub const MIN_ENTROPY_BYTES: usize = 32;
}

/// Standard timeout values based on industry research.
///
/// These are **research-backed defaults** for good UX and can be overridden.
pub mod timeouts {

    use std::time::Duration;

    /// HTTP request timeout - Industry standard
    ///
    /// **Why 30s**: HTTP/1.1 RFC 2616 recommendation
    /// **Standard**: RFC 2616, common practice
    /// **Override**: Set `BEARDOG_REQUEST_TIMEOUT_SECS=<seconds>`
    pub const HTTP_REQUEST: Duration = Duration::from_secs(30);

    /// Database query timeout - Best practice
    ///
    /// **Why 5s**: Prevents long-running queries from blocking
    /// **Rationale**: 95th percentile should be < 1s, 5s catches outliers
    /// **Override**: Set `BEARDOG_DB_TIMEOUT_SECS=<seconds>`
    pub const DATABASE_QUERY: Duration = Duration::from_secs(5);

    /// Connection timeout - TCP best practice
    ///
    /// **Why 10s**: Balance between fast failure and network latency
    /// **Standard**: Common practice (Kubernetes, AWS)
    /// **Override**: Set `BEARDOG_CONNECT_TIMEOUT_SECS=<seconds>`
    pub const CONNECT: Duration = Duration::from_secs(10);

    /// Health check interval - Monitoring standard
    ///
    /// **Why 30s**: Balance between responsiveness and load
    /// **Rationale**: Kubernetes default, industry practice
    /// **Override**: Set `BEARDOG_HEALTH_CHECK_INTERVAL_SECS=<seconds>`
    pub const HEALTH_CHECK: Duration = Duration::from_secs(30);
}

/// Standard buffer sizes based on OS page sizes and network MTU.
///
/// Optimized for performance with common hardware configurations.
pub mod buffers {

    /// Default buffer size - Matches OS page size
    ///
    /// **Why 8KB**: Common OS page size (8192 bytes)
    /// **Rationale**: Aligns with memory pages for efficiency
    /// **Use**: General I/O operations
    pub const DEFAULT_SIZE: usize = 8192;

    /// Small buffer size - For small messages
    ///
    /// **Why 4KB**: Half page size, good for small operations
    /// **Use**: Small messages, headers
    pub const SMALL_SIZE: usize = 4096;

    /// Large buffer size - For bulk operations
    ///
    /// **Why 64KB**: Multiple pages, good for streaming
    /// **Use**: File I/O, bulk transfers
    pub const LARGE_SIZE: usize = 65536;

    /// Network MTU - Standard Ethernet
    ///
    /// **Why 1500**: Standard Ethernet MTU
    /// **Standard**: IEEE 802.3
    /// **Use**: Network packet sizing
    pub const NETWORK_MTU: usize = 1500;
}

/// Standard retry parameters based on reliability engineering.
///
/// These follow **exponential backoff best practices** and are configurable.
pub mod retry {

    use std::time::Duration;

    /// Maximum retry attempts - Best practice
    ///
    /// **Why 3**: Balance between resilience and fast failure
    /// **Rationale**: Covers transient failures without excessive delay
    /// **Override**: Set `BEARDOG_MAX_RETRIES=<count>`
    pub const MAX_ATTEMPTS: usize = 3;

    /// Initial retry delay - Exponential backoff
    ///
    /// **Why 100ms**: Fast enough for transient failures
    /// **Pattern**: 100ms, 200ms, 400ms (exponential)
    /// **Override**: Set `BEARDOG_INITIAL_RETRY_MS=<millis>`
    pub const INITIAL_DELAY: Duration = Duration::from_millis(100);

    /// Maximum retry delay - Prevent excessive waiting
    ///
    /// **Why 10s**: Upper bound to prevent indefinite delays
    /// **Rationale**: User patience threshold
    /// **Override**: Set `BEARDOG_MAX_RETRY_DELAY_SECS=<seconds>`
    pub const MAX_DELAY: Duration = Duration::from_secs(10);
}

/// Standard connection pool sizes based on resource optimization.
///
/// These are **research-backed** for typical workloads and can be configured.
pub mod pools {

    /// Minimum connections in pool
    ///
    /// **Why 5**: Keep connections warm, handle burst traffic
    /// **Rationale**: Balance between resource usage and availability
    pub const MIN_CONNECTIONS: usize = 5;

    /// Maximum connections in pool
    ///
    /// **Why 100**: Typical database connection limit
    /// **Rationale**: PostgreSQL default, prevents resource exhaustion
    /// **Override**: Set `BEARDOG_MAX_CONNECTIONS=<count>`
    pub const MAX_CONNECTIONS: usize = 100;

    /// Connection idle timeout
    ///
    /// **Why 10 minutes**: Balance between keeping alive and cleanup
    /// **Rationale**: Most databases close idle connections at 10-30 min
    pub const IDLE_TIMEOUT_SECS: u64 = 600; // 10 minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_ports_are_standard() {
        assert_eq!(network::HTTPS_PORT, 443);
        assert_eq!(network::HTTP_PORT, 80);
        assert_eq!(network::POSTGRESQL_PORT, 5432);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)] // Documents compliance with standards
    fn test_crypto_constants_meet_nist_requirements() {
        assert!(crypto::RSA_KEY_SIZE_BITS >= 2048, "NIST SP 800-57");
        assert!(crypto::AES_KEY_SIZE_BITS >= 256, "FIPS 197");
        assert!(crypto::MIN_ENTROPY_BYTES >= 32, "NIST SP 800-90A");
    }

    #[test]
    fn test_buffer_sizes_align_with_pages() {
        // Common page sizes are 4KB, 8KB, 64KB
        assert_eq!(buffers::SMALL_SIZE, 4096);
        assert_eq!(buffers::DEFAULT_SIZE, 8192);
        assert_eq!(buffers::LARGE_SIZE, 65536);
    }

    #[test]
    fn test_localhost_addresses() {
        use std::net::IpAddr;

        assert_eq!(IpAddr::V4(localhost::LOCALHOST_V4).to_string(), "127.0.0.1");
        assert_eq!(IpAddr::V6(localhost::LOCALHOST_V6).to_string(), "::1");
    }
}
