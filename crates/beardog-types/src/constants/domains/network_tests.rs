//! Tests for Network Constants
//!
//! Comprehensive tests for all network constants, ports, timeouts, and limits.

use super::*;

// Config module tests
#[test]
fn test_localhost_constants() {
    assert_eq!(config::LOCALHOST_IPV4, "127.0.0.1");
    assert_eq!(config::LOCALHOST_IPV6, "::1");
    assert_eq!(config::LOCALHOST_NAME, "localhost");
}

#[test]
fn test_default_service_host() {
    let host = config::default_service_host();
    assert!(!host.is_empty());
}

#[test]
fn test_default_service_port() {
    let port = config::default_service_port();
    // Port is u16, so always valid - just verify it's non-zero
    assert!(port > 0);
}

#[test]
fn test_default_database_url() {
    let url = config::default_database_url();
    assert!(!url.is_empty());
    // Should contain database connection components
    assert!(
        url.contains("beardog")
            || url.contains("postgresql")
            || std::env::var("DATABASE_URL").is_ok()
    );
}

#[test]
fn test_default_discovery_endpoint() {
    let endpoint = config::default_discovery_endpoint();
    assert!(!endpoint.is_empty());
    assert!(endpoint.contains("discovery") || endpoint.contains("http"));
}

// Defaults module tests
#[test]
fn test_default_ports() {
    let api_port = defaults::default_api_port();
    let metrics_port = defaults::default_metrics_port();
    let health_port = defaults::default_health_port();
    let admin_port = defaults::default_admin_port();
    let debug_port = defaults::default_debug_port();

    // All ports should be valid u16 values (> 0 is always true for u16)
    assert!(api_port > 0);
    assert!(metrics_port > 0);
    assert!(health_port > 0);
    assert!(admin_port > 0);
    assert!(debug_port > 0);
}

#[test]
fn test_timeout_constants() {
    assert_eq!(defaults::DEFAULT_CONNECTION_TIMEOUT.as_secs(), 30);
    assert_eq!(defaults::DEFAULT_READ_TIMEOUT.as_secs(), 60);
    assert_eq!(defaults::DEFAULT_WRITE_TIMEOUT.as_secs(), 30);
    assert_eq!(defaults::DEFAULT_IDLE_TIMEOUT.as_secs(), 300);
    assert_eq!(defaults::DEFAULT_KEEP_ALIVE_TIMEOUT.as_secs(), 60);
}

#[test]
fn test_buffer_size_constants() {
    assert_eq!(defaults::DEFAULT_SOCKET_BUFFER_SIZE, 65536);
    assert_eq!(defaults::DEFAULT_SEND_BUFFER_SIZE, 32768);
    assert_eq!(defaults::DEFAULT_RECEIVE_BUFFER_SIZE, 32768);
    assert_eq!(defaults::DEFAULT_BACKLOG_SIZE, 128);
}

#[test]
fn test_connection_pool_defaults() {
    assert_eq!(defaults::DEFAULT_MAX_CONNECTIONS, 1000);
    assert_eq!(defaults::DEFAULT_MIN_CONNECTIONS, 1);
    assert_eq!(defaults::DEFAULT_CONNECTION_POOL_SIZE, 10);
    assert_eq!(defaults::DEFAULT_MAX_IDLE_CONNECTIONS, 5);
}

// Addresses module tests
#[test]
fn test_address_constants() {
    assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
    assert_eq!(addresses::LOCALHOST_IPV6, "::1");
    assert_eq!(addresses::WILDCARD_IPV4, "0.0.0.0");
    assert_eq!(addresses::WILDCARD_IPV6, "::");
    assert_eq!(addresses::BROADCAST_ADDRESS, "255.255.255.255");
}

#[test]
fn test_default_bind_address() {
    let bind_addr = addresses::default_bind_address();
    assert!(!bind_addr.is_empty());
}

#[test]
fn test_default_api_bind() {
    let api_bind = addresses::default_api_bind();
    assert!(!api_bind.is_empty());
    assert!(api_bind.contains(':'));
}

#[test]
fn test_multicast_address() {
    let multicast = addresses::multicast_address();
    assert!(!multicast.is_empty());
}

#[test]
fn test_dns_settings() {
    assert_eq!(addresses::DEFAULT_DNS_PORT, 53);
    // Verify DNS servers array is populated
    #[allow(clippy::len_zero)] // Const arrays don't have is_empty() at compile time
    {
        assert!(addresses::DEFAULT_DNS_SERVERS.len() > 0);
    }
    assert!(addresses::DEFAULT_DNS_SERVERS.contains(&"8.8.8.8"));
}

// Ports module tests
#[test]
fn test_port_ranges() {
    assert_eq!(ports::WELL_KNOWN_PORT_MIN, 1);
    assert_eq!(ports::WELL_KNOWN_PORT_MAX, 1023);
    assert_eq!(ports::REGISTERED_PORT_MIN, 1024);
    assert_eq!(ports::REGISTERED_PORT_MAX, 49151);
    assert_eq!(ports::DYNAMIC_PORT_MIN, 49152);
    assert_eq!(ports::DYNAMIC_PORT_MAX, 65535);
}

#[test]
fn test_standard_ports() {
    assert_eq!(ports::HTTP_PORT, 80);
    assert_eq!(ports::HTTPS_PORT, 443);
    assert_eq!(ports::SSH_PORT, 22);
    assert_eq!(ports::DNS_PORT, 53);
}

#[test]
fn test_beardog_port_range() {
    assert_eq!(ports::BEARDOG_PORT_RANGE_START, 8080);
    assert_eq!(ports::BEARDOG_PORT_RANGE_END, 8099);
    // Port range validation (compile-time constant check)
    let _ = ports::BEARDOG_PORT_RANGE_END - ports::BEARDOG_PORT_RANGE_START;
}

// Timeouts module tests
#[test]
fn test_connection_timeouts() {
    assert_eq!(timeouts::CONNECTION_TIMEOUT.as_secs(), 30);
    assert_eq!(timeouts::HANDSHAKE_TIMEOUT.as_secs(), 10);
    assert_eq!(timeouts::TLS_HANDSHAKE_TIMEOUT.as_secs(), 30);
    assert_eq!(timeouts::KEEP_ALIVE_TIMEOUT.as_secs(), 60);
}

#[test]
fn test_io_timeouts() {
    assert_eq!(timeouts::READ_TIMEOUT.as_secs(), 60);
    assert_eq!(timeouts::WRITE_TIMEOUT.as_secs(), 30);
    assert_eq!(timeouts::SEND_TIMEOUT.as_secs(), 30);
    assert_eq!(timeouts::RECEIVE_TIMEOUT.as_secs(), 60);
}

#[test]
fn test_health_check_timeouts() {
    assert_eq!(timeouts::HEALTH_CHECK_TIMEOUT.as_secs(), 5);
    assert_eq!(timeouts::PING_TIMEOUT.as_secs(), 1);
    assert_eq!(timeouts::HEARTBEAT_TIMEOUT.as_secs(), 30);
}

// Limits module tests
#[test]
fn test_connection_limits() {
    assert_eq!(limits::MAX_CONNECTIONS, 10000);
    assert_eq!(limits::MIN_CONNECTIONS, 1);
    // Connection limit validation (compile-time constant check)
    let _ = limits::MAX_CONNECTIONS - limits::MIN_CONNECTIONS;
}

#[test]
fn test_message_size_limits() {
    assert_eq!(limits::MAX_MESSAGE_SIZE, 16 * 1024 * 1024);
    assert_eq!(limits::MIN_MESSAGE_SIZE, 1);
    assert_eq!(limits::MAX_HEADER_SIZE, 8192);
    assert_eq!(limits::MAX_URL_LENGTH, 2048);
}

#[test]
fn test_rate_limits() {
    assert_eq!(limits::MAX_REQUESTS_PER_SECOND, 1000);
    assert_eq!(limits::DEFAULT_RATE_LIMIT, 100);
    // Rate limit validation (compile-time constant check)
    let _ = limits::MAX_REQUESTS_PER_SECOND - limits::DEFAULT_RATE_LIMIT;
}

// Protocol tests
#[test]
fn test_http_versions() {
    assert_eq!(protocols::http::HTTP_1_0, "HTTP/1.0");
    assert_eq!(protocols::http::HTTP_1_1, "HTTP/1.1");
    assert_eq!(protocols::http::HTTP_2_0, "HTTP/2.0");
    assert_eq!(protocols::http::HTTP_3_0, "HTTP/3.0");
}

#[test]
fn test_http_methods() {
    assert_eq!(protocols::http::GET, "GET");
    assert_eq!(protocols::http::POST, "POST");
    assert_eq!(protocols::http::PUT, "PUT");
    assert_eq!(protocols::http::DELETE, "DELETE");
}

#[test]
fn test_http_status_codes() {
    assert_eq!(protocols::http::OK, 200);
    assert_eq!(protocols::http::CREATED, 201);
    assert_eq!(protocols::http::BAD_REQUEST, 400);
    assert_eq!(protocols::http::NOT_FOUND, 404);
    assert_eq!(protocols::http::INTERNAL_SERVER_ERROR, 500);
}

#[test]
fn test_http_status_ranges() {
    // Compile-time validation that NOT_FOUND is in correct range
    const _: () = assert!(protocols::http::NOT_FOUND >= protocols::http::CLIENT_ERROR_MIN);
    const _: () = assert!(protocols::http::NOT_FOUND <= protocols::http::CLIENT_ERROR_MAX);

    // Verify HTTP status code values
    assert_eq!(protocols::http::OK, 200);
}

#[test]
fn test_tls_versions() {
    assert_eq!(protocols::tls::TLS_1_3, "TLSv1.3");
    assert_eq!(protocols::tls::DEFAULT_TLS_VERSION, "TLSv1.3");
    assert_eq!(protocols::tls::MIN_TLS_VERSION, "TLSv1.2");
}

// Headers tests
#[test]
fn test_request_headers() {
    assert_eq!(headers::ACCEPT, "Accept");
    assert_eq!(headers::CONTENT_TYPE, "Content-Type");
    assert_eq!(headers::AUTHORIZATION, "Authorization");
    assert_eq!(headers::USER_AGENT, "User-Agent");
}

#[test]
fn test_beardog_headers() {
    assert_eq!(headers::X_BEARDOG_VERSION, "X-BearDog-Version");
    assert_eq!(headers::X_BEARDOG_REQUEST_ID, "X-BearDog-Request-ID");
    assert_eq!(headers::X_BEARDOG_TRACE_ID, "X-BearDog-Trace-ID");
}

#[test]
fn test_content_types() {
    assert_eq!(headers::APPLICATION_JSON, "application/json");
    assert_eq!(headers::TEXT_PLAIN, "text/plain");
    assert_eq!(headers::TEXT_HTML, "text/html");
}

#[test]
fn test_default_headers() {
    assert!(headers::DEFAULT_USER_AGENT.contains("BearDog"));
    assert!(headers::DEFAULT_SERVER.contains("BearDog"));
    assert_eq!(headers::DEFAULT_CONTENT_TYPE, headers::APPLICATION_JSON);
}

// Load balancing tests
#[test]
fn test_load_balancing_algorithms() {
    assert_eq!(load_balancing::ROUND_ROBIN, "round_robin");
    assert_eq!(load_balancing::LEAST_CONNECTIONS, "least_connections");
    assert_eq!(load_balancing::IP_HASH, "ip_hash");
}

#[test]
fn test_health_check_defaults() {
    assert_eq!(load_balancing::DEFAULT_HEALTH_CHECK_PATH, "/health");
    assert_eq!(load_balancing::DEFAULT_HEALTH_CHECK_METHOD, "GET");
    assert_eq!(load_balancing::DEFAULT_HEALTH_CHECK_INTERVAL.as_secs(), 30);
}

// Circuit breaker tests
#[test]
fn test_circuit_breaker_states() {
    assert_eq!(circuit_breaker::STATE_CLOSED, "closed");
    assert_eq!(circuit_breaker::STATE_OPEN, "open");
    assert_eq!(circuit_breaker::STATE_HALF_OPEN, "half_open");
}

#[test]
fn test_circuit_breaker_thresholds() {
    // Compile-time validation of threshold relationship
    const _: () = assert!(
        circuit_breaker::DEFAULT_FAILURE_THRESHOLD > circuit_breaker::DEFAULT_SUCCESS_THRESHOLD
    );

    assert_eq!(circuit_breaker::DEFAULT_FAILURE_THRESHOLD, 5);
    assert_eq!(circuit_breaker::DEFAULT_SUCCESS_THRESHOLD, 3);
}

// Rate limiting tests
#[test]
fn test_rate_limiting_algorithms() {
    assert_eq!(rate_limiting::TOKEN_BUCKET, "token_bucket");
    assert_eq!(rate_limiting::LEAKY_BUCKET, "leaky_bucket");
    assert_eq!(rate_limiting::FIXED_WINDOW, "fixed_window");
}

#[test]
fn test_rate_limiting_headers() {
    assert_eq!(rate_limiting::X_RATELIMIT_LIMIT, "X-RateLimit-Limit");
    assert_eq!(
        rate_limiting::X_RATELIMIT_REMAINING,
        "X-RateLimit-Remaining"
    );
    assert_eq!(rate_limiting::X_RATELIMIT_RESET, "X-RateLimit-Reset");
}

// Integration tests
#[test]
fn test_port_consistency() {
    // Ensure deprecated constants match their functional equivalents
    #[allow(deprecated)]
    {
        assert_eq!(defaults::default_api_port(), 8080);
        // metrics_port is now 9100 (from config), not 9090
        assert_eq!(defaults::default_metrics_port(), 9100);
    }
}

#[test]
fn test_timeout_consistency() {
    // Timeouts should be reasonable
    assert!(timeouts::CONNECTION_TIMEOUT < timeouts::IDLE_CONNECTION_TIMEOUT);
    assert!(timeouts::PING_TIMEOUT < timeouts::HEALTH_CHECK_TIMEOUT);
    assert!(timeouts::RETRY_TIMEOUT < timeouts::MAX_RETRY_TIMEOUT);
}

#[test]
fn test_limit_consistency() {
    // Compile-time validation that limits are logically consistent
    const _: () = assert!(limits::MIN_CONNECTIONS < limits::MAX_CONNECTIONS);
    const _: () = assert!(limits::MIN_MESSAGE_SIZE < limits::MAX_MESSAGE_SIZE);
    const _: () = assert!(limits::MIN_CONNECTION_POOL_SIZE < limits::MAX_CONNECTION_POOL_SIZE);

    // Verify the constants exist and are accessible
    let _ = limits::MIN_CONNECTIONS;
    let _ = limits::MAX_CONNECTIONS;
}
