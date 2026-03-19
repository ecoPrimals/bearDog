// SPDX-License-Identifier: AGPL-3.0-only

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

// Comprehensive tests for network configuration module

use crate::universal_discovery::network::*;
use std::net::{IpAddr, Ipv4Addr};

// Import AuthenticationMethod explicitly
use super::super::network::AuthenticationMethod;

#[test]
fn test_network_config_default() {
    let config = NetworkConfig::default();

    assert!(config.enable_ipv6);
    assert!(config.max_packet_size > 0);
    assert!(config.connection_timeout_ms > 0);
}

#[test]
fn test_network_config_creation() {
    let config = NetworkConfig {
        bind_address: "0.0.0.0:8080".parse().unwrap(),
        multicast_address: "224.0.0.251".parse().unwrap(),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 5000,
        read_timeout_ms: 3000,
        write_timeout_ms: 3000,
        enable_ipv6: true,
        interface: Some("eth0".to_string()),
        tls_config: None,
    };

    assert_eq!(config.multicast_port, 5353);
    assert_eq!(config.discovery_port_range, (8081, 8090));
    assert_eq!(config.max_packet_size, 65536);
}

#[test]
fn test_tls_version_variants() {
    let versions = [TlsVersion::TlsV1_2, TlsVersion::TlsV1_3];

    assert_eq!(versions.len(), 2);
}

#[test]
fn test_tls_config_creation() {
    let tls_config = TlsConfig {
        cert_file: "/path/to/cert.pem".to_string(),
        key_file: "/path/to/key.pem".to_string(),
        ca_file: Some("/path/to/ca.pem".to_string()),
        verify_client: true,
        min_version: TlsVersion::TlsV1_3,
        cipher_suites: vec![
            "TLS_AES_256_GCM_SHA384".to_string(),
            "TLS_AES_128_GCM_SHA256".to_string(),
        ],
    };

    assert!(tls_config.verify_client);
    assert_eq!(tls_config.cipher_suites.len(), 2);
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();

    assert!(config.enable_cache);
    assert!(config.cache_ttl_secs > 0);
    assert!(config.max_cache_entries > 0);
}

#[test]
fn test_cache_config_creation() {
    let config = CacheConfig {
        enable_cache: true,
        cache_ttl_secs: 600,
        max_cache_entries: 5000,
        cleanup_interval_secs: 120,
        enable_compression: false,
    };

    assert_eq!(config.cache_ttl_secs, 600);
    assert_eq!(config.max_cache_entries, 5000);
}

#[test]
fn test_security_config_default() {
    let config = SecurityConfig::default();

    assert!(!config.enable_auth());
    assert!(config.enable_rate_limiting());
}

#[test]
fn test_security_config_creation() {
    let config = SecurityConfig {
        auth: Some(AuthConfig {
            method: AuthenticationMethod::JWT,
            api_keys: vec!["key1".to_string(), "key2".to_string()],
            jwt_secret: Some("secret".to_string()),
        }),
        rate_limit: Some(RateLimitConfig { rpm: 5000 }),
        ip_allowlist: Some(vec!["127.0.0.1".parse().unwrap()]),
        encryption: Some(EncryptionConfig {
            algorithm: "AES-256-GCM".to_string(),
        }),
    };

    assert!(config.enable_auth());
    assert_eq!(config.auth.as_ref().unwrap().api_keys.len(), 2);
    assert!(config.enable_encryption());
}

#[test]
fn test_network_config_with_ipv6() {
    let config = NetworkConfig {
        bind_address: "[::]:8080".parse().unwrap(),
        multicast_address: "ff02::fb".parse().unwrap(),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 5000,
        read_timeout_ms: 3000,
        write_timeout_ms: 3000,
        enable_ipv6: true,
        interface: None,
        tls_config: None,
    };

    assert!(config.enable_ipv6);
}

#[test]
fn test_network_config_serialization() {
    let config = NetworkConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        multicast_address: IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 5000,
        read_timeout_ms: 3000,
        write_timeout_ms: 3000,
        enable_ipv6: false,
        interface: Some("lo".to_string()),
        tls_config: None,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: NetworkConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.multicast_port, deserialized.multicast_port);
    assert_eq!(config.max_packet_size, deserialized.max_packet_size);
}

#[test]
fn test_tls_config_serialization() {
    let tls_config = TlsConfig {
        cert_file: "/cert.pem".to_string(),
        key_file: "/key.pem".to_string(),
        ca_file: None,
        verify_client: false,
        min_version: TlsVersion::TlsV1_2,
        cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
    };

    let serialized = serde_json::to_string(&tls_config).unwrap();
    let deserialized: TlsConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(tls_config.cert_file, deserialized.cert_file);
    assert_eq!(tls_config.verify_client, deserialized.verify_client);
}

#[test]
fn test_cache_config_serialization() {
    let config = CacheConfig {
        enable_cache: false,
        cache_ttl_secs: 300,
        max_cache_entries: 1000,
        cleanup_interval_secs: 60,
        enable_compression: true,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CacheConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.enable_cache, deserialized.enable_cache);
    assert_eq!(config.cache_ttl_secs, deserialized.cache_ttl_secs);
}

#[test]
fn test_security_config_serialization() {
    let config = SecurityConfig {
        auth: None,
        rate_limit: None,
        ip_allowlist: None,
        encryption: None,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: SecurityConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.enable_auth(), deserialized.enable_auth());
    assert_eq!(config.enable_encryption(), deserialized.enable_encryption());
}

#[test]
fn test_network_config_with_tls() {
    let tls_config = TlsConfig {
        cert_file: "/etc/ssl/cert.pem".to_string(),
        key_file: "/etc/ssl/key.pem".to_string(),
        ca_file: Some("/etc/ssl/ca.pem".to_string()),
        verify_client: true,
        min_version: TlsVersion::TlsV1_3,
        cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
    };

    let config = NetworkConfig {
        bind_address: "0.0.0.0:443".parse().unwrap(),
        multicast_address: "224.0.0.251".parse().unwrap(),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 5000,
        read_timeout_ms: 3000,
        write_timeout_ms: 3000,
        enable_ipv6: true,
        interface: None,
        tls_config: Some(tls_config),
    };

    assert!(config.tls_config.is_some());
}

#[test]
fn test_port_range_validation() {
    let config = NetworkConfig {
        bind_address: "0.0.0.0:8080".parse().unwrap(),
        multicast_address: "224.0.0.251".parse().unwrap(),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 5000,
        read_timeout_ms: 3000,
        write_timeout_ms: 3000,
        enable_ipv6: true,
        interface: None,
        tls_config: None,
    };

    let (start, end) = config.discovery_port_range;
    assert!(start < end);
    assert!(end - start >= 9);
}

#[test]
fn test_timeout_configurations() {
    let config = NetworkConfig {
        bind_address: "0.0.0.0:8080".parse().unwrap(),
        multicast_address: "224.0.0.251".parse().unwrap(),
        multicast_port: 5353,
        discovery_port_range: (8081, 8090),
        max_packet_size: 65536,
        connection_timeout_ms: 10000,
        read_timeout_ms: 5000,
        write_timeout_ms: 5000,
        enable_ipv6: true,
        interface: None,
        tls_config: None,
    };

    assert_eq!(config.connection_timeout_ms, 10000);
    assert_eq!(config.read_timeout_ms, 5000);
    assert_eq!(config.write_timeout_ms, 5000);
}
