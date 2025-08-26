

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {

    pub enabled: bool,

    pub certificate: CertificateConfig,

    pub tls_version: TlsVersionConfig,

    pub cipher_suites: Vec<String>,

    pub client_cert_verification: ClientCertVerification,

    pub session: Option<SslSessionConfig>,
}

pub struct CertificateConfig {

    pub cert_path: PathBuf,

    pub key_path: PathBuf,

    pub chain_path: Option<PathBuf>,

    pub format: CertificateFormat,

    pub password: Option<String>,

    pub validity_check: bool,

    pub auto_renewal: bool,

pub struct TlsVersionConfig {

    pub min_version: TlsVersion,

    pub max_version: TlsVersion,

    pub preferred_version: Option<TlsVersion>,

pub struct SslSessionConfig {

    pub cache_enabled: bool,

    pub cache_size: u32,

    pub timeout: Duration,

    pub ticket_enabled: bool,

    pub ticket_key: Option<String>,

pub enum CertificateFormat {
    Pem,
    Pkcs12,
    Der,

pub enum TlsVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,

pub enum ClientCertVerification {
    None,
    Optional,
    Required,
    RequiredAndVerified,

pub struct SecurityHeadersConfig {

    pub hsts: Option<HstsConfig>,

    pub csp: Option<String>,

    pub x_frame_options: Option<String>,

    pub x_content_type_options: Option<String>,

    pub x_xss_protection: Option<String>,

    pub referrer_policy: Option<String>,

    pub custom_headers: HashMap<String, String>,

pub struct HstsConfig {

    pub max_age: u64,

    pub include_subdomains: bool,

    pub preload: bool,

pub struct NetworkSecurityConfig {

    pub ssl: Option<SslConfig>,

    pub headers: Option<SecurityHeadersConfig>,

    pub rate_limiting: Option<SecurityRateLimitingConfig>,

    pub ip_filtering: Option<IpFilteringConfig>,

    pub ddos_protection: Option<DdosProtectionConfig>,

pub struct SecurityRateLimitingConfig {

    pub requests_per_minute: u32,

    pub burst_limit: u32,

    pub blocked_ip_timeout: Duration,

    pub whitelist_ips: Vec<String>,

pub struct IpFilteringConfig {

    pub allowed_ips: Vec<String>,

    pub blocked_ips: Vec<String>,

    pub geo_filtering: bool,

    pub allowed_countries: Vec<String>,

    pub blocked_countries: Vec<String>,

pub struct DdosProtectionConfig {

    pub connection_rate_threshold: u32,

    pub request_rate_threshold: u32,

    pub protection_timeout: Duration,

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
