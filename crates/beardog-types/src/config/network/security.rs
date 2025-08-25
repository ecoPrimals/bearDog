// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Network Security Configuration
///
/// **SSL/TLS AND NETWORK SECURITY SETTINGS**
/// Contains network security configuration structs including SSL/TLS,
/// certificates, and network-level security configurations.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// **CANONICAL SSL CONFIGURATION** - Consolidates SSL config duplicates
/// Unifies SSL configurations from multiple locations to eliminate fragmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    /// SSL/TLS enabled
    pub enabled: bool,
    /// Certificate configuration
    pub certificate: CertificateConfig,
    /// TLS version configuration
    pub tls_version: TlsVersionConfig,
    /// Cipher suite configuration
    pub cipher_suites: Vec<String>,
    /// Client certificate verification
    pub client_cert_verification: ClientCertVerification,
    /// SSL session configuration
    pub session: Option<SslSessionConfig>,
}
/// TLS configuration (alias for SslConfig for backward compatibility)
// MIGRATION COMPLETE: Use SslConfig directly instead of TlsConfig alias
/// Certificate configuration
pub struct CertificateConfig {
    /// Certificate file path
    pub cert_path: PathBuf,
    /// Private key file path  
    pub key_path: PathBuf,
    /// Certificate chain file path
    pub chain_path: Option<PathBuf>,
    /// Certificate format (PEM, PKCS12)
    pub format: CertificateFormat,
    /// Certificate password (for encrypted certificates)
    pub password: Option<String>,
    /// Certificate validity check enabled
    pub validity_check: bool,
    /// Certificate auto-renewal enabled
    pub auto_renewal: bool,
/// SSL/TLS version configuration
pub struct TlsVersionConfig {
    /// Minimum TLS version
    pub min_version: TlsVersion,
    /// Maximum TLS version
    pub max_version: TlsVersion,
    /// Preferred TLS version
    pub preferred_version: Option<TlsVersion>,
/// SSL session configuration
pub struct SslSessionConfig {
    /// Session cache enabled
    pub cache_enabled: bool,
    /// Session cache size
    pub cache_size: u32,
    /// Session timeout
    pub timeout: Duration,
    /// Session ticket enabled
    pub ticket_enabled: bool,
    /// Session ticket key
    pub ticket_key: Option<String>,
/// Certificate formats
pub enum CertificateFormat {
    Pem,
    Pkcs12,
    Der,
/// TLS versions}


pub enum TlsVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
/// Client certificate verification modes
pub enum ClientCertVerification {
    None,
    Optional,
    Required,
    RequiredAndVerified,
/// Network security headers configuration}


pub struct SecurityHeadersConfig {
    /// Security headers enabled
    /// HSTS (HTTP Strict Transport Security) configuration
    pub hsts: Option<HstsConfig>,
    /// Content Security Policy
    pub csp: Option<String>,
    /// X-Frame-Options
    pub x_frame_options: Option<String>,
    /// X-Content-Type-Options
    pub x_content_type_options: Option<String>,
    /// X-XSS-Protection  
    pub x_xss_protection: Option<String>,
    /// Referrer-Policy
    pub referrer_policy: Option<String>,
    /// Custom security headers
    pub custom_headers: HashMap<String, String>,
/// HSTS (HTTP Strict Transport Security) configuration
pub struct HstsConfig {
    /// HSTS enabled
    /// Max age in seconds
    pub max_age: u64,
    /// Include subdomains
    pub include_subdomains: bool,
    /// Preload enabled
    pub preload: bool,
/// Comprehensive network security configuration
pub struct NetworkSecurityConfig {
    /// SSL/TLS configuration
    pub ssl: Option<SslConfig>,
    /// Security headers configuration
    pub headers: Option<SecurityHeadersConfig>,
    /// Rate limiting for security
    pub rate_limiting: Option<SecurityRateLimitingConfig>,
    /// IP filtering configuration
    pub ip_filtering: Option<IpFilteringConfig>,
    /// DDoS protection settings
    pub ddos_protection: Option<DdosProtectionConfig>,
/// Security-focused rate limiting
pub struct SecurityRateLimitingConfig {
    /// Rate limiting enabled
    /// Requests per minute limit
    pub requests_per_minute: u32,
    /// Burst limit
    pub burst_limit: u32,
    /// Blocked IP timeout
    pub blocked_ip_timeout: Duration,
    /// Whitelist IPs (exempt from rate limiting)  
    pub whitelist_ips: Vec<String>,
/// IP filtering configuration
pub struct IpFilteringConfig {
    /// IP filtering enabled
    /// Allowed IP ranges
    pub allowed_ips: Vec<String>,
    /// Blocked IP ranges
    pub blocked_ips: Vec<String>,
    /// Geolocation filtering enabled
    pub geo_filtering: bool,
    /// Allowed countries
    pub allowed_countries: Vec<String>,
    /// Blocked countries
    pub blocked_countries: Vec<String>,
/// DDoS protection configuration
pub struct DdosProtectionConfig {
    /// DDoS protection enabled
    /// Connection rate threshold
    pub connection_rate_threshold: u32,
    /// Request rate threshold
    pub request_rate_threshold: u32,
    /// Protection timeout
    pub protection_timeout: Duration,
    /// Automatic blocking enabled
    pub auto_blocking: bool,}


impl Default for SslConfig {}


    fn default() -> Self {
        Self {
            enabled: false,
            certificate: CertificateConfig::default(),
            tls_version: TlsVersionConfig::default(),
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
            ],
            client_cert_verification: ClientCertVerification::None,
            session: Some(SslSessionConfig::default()),
        }
    }
impl Default for CertificateConfig {
            cert_path: PathBuf::from("cert.pem"),
            key_path: PathBuf::from("key.pem"),
            chain_path: None,
            format: CertificateFormat::Pem,
            password: None,
            validity_check: true,
            auto_renewal: false,}


impl Default for TlsVersionConfig {
            min_version: TlsVersion::V1_2,
            max_version: TlsVersion::V1_3,
            preferred_version: Some(TlsVersion::V1_3),
impl Default for SslSessionConfig {
            cache_enabled: true,
            cache_size: 20480,
            timeout: Duration::from_secs(300),
            ticket_enabled: true,
            ticket_key: None,
