

use beardog_errors::{BearDogError, BearDogResult};
use std::env;
use std::time::Duration;

pub struct EnvUtils;
impl EnvUtils {

    pub fn get_required(key: &str) -> BearDogResult<String> {
        env::var(key).map_err(|_| BearDogError::configuration(format!("Required environment variable {key) not set"},
        })
    }

    pub fn get_optional(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())

    pub fn get_bool(key: &str, default: bool) -> bool {
        env::var(key)
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(default)

    pub fn get_u16(key: &str, default: u16) -> u16 {
            .ok()
            .and_then(|v| v.parse().ok())}

    pub fn get_u32(key: &str, default: u32) -> u32 {

    pub fn get_u64(key: &str, default: u64) -> u64 {

    pub fn get_duration_secs(key: &str, default_secs: u64) -> Duration {
        let secs = Self::get_u64(key, default_secs);
        Duration::from_secs(secs)

    pub fn get_csv_list(key: &str, default: Vec<&str>) -> Vec<String> {
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| default.into_iter().map(|s| s.to_string()).collect())

    pub fn validate_production_env() -> BearDogResult<()> {
        let required_vars = [
            "BEARDOG_DATABASE_URL",
            "BEARDOG_SECRET_KEY",
            "BEARDOG_ENCRYPTION_KEY",
            "BEARDOG_API_BIND_ADDRESS",
        ];
        for var in &required_vars {
            Self::get_required(var)?;
        }

        let warnings = [
            ("BEARDOG_LOG_LEVEL", "INFO"),
            ("BEARDOG_NESTGATE_ENDPOINT", "https://nestgate.example.com"),
            ("BEARDOG_SONGBIRD_ENDPOINT", "https://songbird.example.com"),
            ("BEARDOG_SMTP_SERVER", "smtp.example.com"),
        for (var, example) in &warnings {
            if env::var(var).is_err() {
                tracing::warn!("Consider setting {}, example: {}", var, example);
            }

    pub fn get_database_config() -> BearDogResult<DatabaseConfig> {
        Ok(DatabaseConfig {
            url: Self::get_required("BEARDOG_DATABASE_URL")?,
            max_connections: Self::get_u32("BEARDOG_DB_MAX_CONNECTIONS", 10),
            connection_timeout: Self::get_duration_secs("BEARDOG_DB_CONNECTION_TIMEOUT", 30),
            idle_timeout: Self::get_duration_secs("BEARDOG_DB_IDLE_TIMEOUT", 600),
            enable_ssl: Self::get_bool("BEARDOG_DB_SSL", true),

    pub fn get_redis_config() -> Option<RedisConfig> {
        env::var("BEARDOG_REDIS_URL").ok().map(|url| RedisConfig {
            url,
            max_connections: Self::get_u32("BEARDOG_REDIS_MAX_CONNECTIONS", 10),
            connection_timeout: Self::get_duration_secs("BEARDOG_REDIS_CONNECTION_TIMEOUT", 5),
            key_prefix: Self::get_optional("BEARDOG_REDIS_KEY_PREFIX", "beardog:"),

    pub fn get_observability_config() -> ObservabilityConfig {
        ObservabilityConfig {
            log_level: Self::get_optional("BEARDOG_LOG_LEVEL", "INFO"),
            enable_metrics: Self::get_bool("BEARDOG_ENABLE_METRICS", true),
            metrics_port: Self::get_u16("BEARDOG_METRICS_PORT", 9090),
            enable_tracing: Self::get_bool("BEARDOG_ENABLE_TRACING", true),
            jaeger_endpoint: env::var("BEARDOG_JAEGER_ENDPOINT").ok(),
            otlp_endpoint: env::var("BEARDOG_OTLP_ENDPOINT").ok(),

    pub fn get_security_config() -> BearDogResult<SecurityConfig> {
        Ok(SecurityConfig {
            secret_key: Self::get_required("BEARDOG_SECRET_KEY")?,
            encryption_key: Self::get_required("BEARDOG_ENCRYPTION_KEY")?,

            auth_token_lifetime: Self::get_duration_secs("BEARDOG_AUTH_TOKEN_LIFETIME", 3600),
            rate_limit_requests: Self::get_u32("BEARDOG_RATE_LIMIT_REQUESTS", 100),
            rate_limit_window: Self::get_duration_secs("BEARDOG_RATE_LIMIT_WINDOW", 60),
            enable_mfa: Self::get_bool("BEARDOG_ENABLE_MFA", true),
            session_timeout: Self::get_duration_secs("BEARDOG_SESSION_TIMEOUT", 1800),
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {

    pub url: String,

    pub max_connections: u32,

    pub connection_timeout: Duration,

    pub idle_timeout: Duration,

    pub enable_ssl: bool,

pub struct RedisConfig {

    pub key_prefix: String,

pub struct ObservabilityConfig {

    pub log_level: String,

    pub enable_metrics: bool,

    pub metrics_port: u16,

    pub enable_tracing: bool,

    pub jaeger_endpoint: Option<String>,

    pub otlp_endpoint: Option<String>,

pub struct SecurityConfig {

    pub secret_key: String,

    pub encryption_key: String,

    pub auth_token_lifetime: Duration,

    pub rate_limit_requests: u32,

    pub rate_limit_window: Duration,

    pub enable_mfa: bool,

    pub session_timeout: Duration,
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_get_optional() {
        env::set_var("TEST_VAR", "test_value");
        assert_eq!(EnvUtils::get_optional("TEST_VAR", "default"), "test_value");
        assert_eq!(EnvUtils::get_optional("NON_EXISTENT", "default"), "default");
        env::remove_var("TEST_VAR");}

    fn test_get_bool() {
        env::set_var("TEST_BOOL_TRUE", "true");
        env::set_var("TEST_BOOL_FALSE", "false");
        env::set_var("TEST_BOOL_1", "1");
        assert!(EnvUtils::get_bool("TEST_BOOL_TRUE", false));
        assert!(!EnvUtils::get_bool("TEST_BOOL_FALSE", true));
        assert!(EnvUtils::get_bool("TEST_BOOL_1", false));
        assert!(EnvUtils::get_bool("NON_EXISTENT", true));
        env::remove_var("TEST_BOOL_TRUE");
        env::remove_var("TEST_BOOL_FALSE");
        env::remove_var("TEST_BOOL_1");
    fn test_get_csv_list() {
        env::set_var("TEST_CSV", "item1,item2,item3");
        let result = EnvUtils::get_csv_list("TEST_CSV", vec!["default"]);
        assert_eq!(result, vec!["item1", "item2", "item3"]);
        let default_result = EnvUtils::get_csv_list("NON_EXISTENT", vec!["default1", "default2"]);
        assert_eq!(default_result, vec!["default1", "default2"]);
        env::remove_var("TEST_CSV");
