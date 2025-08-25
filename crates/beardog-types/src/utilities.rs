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


/// # Canonical Utilities System  
///
/// **ELIMINATES HELPER FUNCTION FRAGMENTATION**
/// This module consolidates duplicate helper functions from across the entire `BearDog` codebase
/// into a single source of truth. All modules should use utilities from here
/// rather than defining their own duplicate helper functions.
/// ## Consolidated Utilities From:
/// - Multiple `test_*_helper` functions across test modules
/// - String processing utilities from various modules  
/// - Validation utilities from different components
/// - Common operations scattered across the codebase
/// - Formatting utilities from test files and examples

use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::{Duration, SystemTime};
// ================================================================================
// STRING UTILITIES - CONSOLIDATED
/// **CANONICAL STRING UTILITIES** - Consolidates string helper functions
pub mod string {
    use super::*;
    /// Check if a string is likely an ID (UUID, hash, etc.)
    /// Consolidates: `is_likely_id` from multiple modules
    pub fn is_likely_id(s: &str) -> bool {
        // UUID pattern (8-4-4-4-12 hex digits)
        if s.len() == 36 && s.matches('-').count() == 4 {
            return true;
        }
        // Hash patterns (32, 40, 64 hex chars)
        if matches!(s.len(), 32 | 40 | 64) && s.chars().all(|c| c.is_ascii_hexdigit()) {
        // Base64-like patterns
        if s.len() > 20
            && s.chars()
                .all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=')
        {
        false
    }
    /// Check if a string is a common/cached value
    /// Consolidates: `is_common_value`, `is_common_string` from multiple modules
    pub fn is_common_value(s: &str) -> bool {
        matches!(
            s,
            "true"
                | "false"
                | "null"
                | "undefined"
                | "ok"
                | "error"
                | "success"
                | "failure"
                | "enabled"
                | "disabled"
                | "active"
                | "inactive"
                | "pending"
                | "complete"
                | "admin"
                | "user"
                | "guest"
                | "system"
                | "api"
                | "web"
                | "mobile"
                | "desktop"
                | "production"
                | "development"
                | "staging"
                | "test"
                | "local"
        )
    /// Safe string validation with length constraints  
    /// Consolidates: validation utilities from multiple modules}


    pub fn validate_string(
        s: &str,
        min_len: usize,
        max_len: usize,
        field_name: &str,
    ) -> BearDogResult<()> {
        if s.len() < min_len {
            return Err(BearDogError::validation(format!("{field_name) must be at least {min_len} characters long"),
            });
        if s.len() > max_len {
                message: format!("{field_name} must be at most {max_len} characters long"),
        Ok(())
    /// Sanitize string for safe usage (remove control characters)
    pub fn sanitize_string(s: &str) -> String {
        s.chars()
            .filter(|c| !c.is_control() || c.is_whitespace())
            .collect()
    /// Sanitize user input for HTML safety
    /// Consolidates: `sanitize_user_input` from test files}


    pub fn sanitize_user_input(input: &str) -> String {
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
    /// Truncate string to maximum length with ellipsis
    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else if max_len < 3 {
            s.chars().take(max_len).collect()
        } else {
            format!("{}...", s.chars().take(max_len - 3).collect::<String>())
    /// Check if a string is a valid identifier (alphanumeric + underscore)
    /// Consolidates: `is_valid_identifier` from test files
    pub fn is_valid_identifier(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}
// VALIDATION UTILITIES - CONSOLIDATED
/// **CANONICAL VALIDATION UTILITIES** - Consolidates validation helper functions
pub mod validation {
    // No imports needed - all functions use standard library types
    /// Validate email address format
    pub fn is_valid_email(email: &str) -> bool {
        // Basic email validation - can be enhanced with regex
        email.contains('@')
            && email.len() > 3
            && email.len() < 255
            && !email.starts_with('@')
            && !email.ends_with('@')
            && email.matches('@').count() == 1
    /// Validate URL format}


    pub fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    /// Validate IP address (basic check)
    /// Consolidates: `is_valid_ip_address` from test files
    pub fn is_valid_ip(ip: &str) -> bool {
        ip.parse::<std::net::IpAddr>().is_ok()
    /// Validate port number
    /// Consolidates: `is_valid_port` from test files with consistent signature}


    pub fn is_valid_port(port: u32) -> bool {
        port > 0 && port <= 65535
    /// Validate UUID format
    pub fn is_valid_uuid(uuid: &str) -> bool {
        uuid.len() == 36
            && uuid.matches('-').count() == 4
            && uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
    /// Validate hex string
    /// Consolidates: `is_valid_hex_string` from test files}


    pub fn is_valid_hex(hex: &str) -> bool {
        !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit())
    /// Validate base64 string
    pub fn is_valid_base64(base64: &str) -> bool {
        use std::collections::HashSet;
        let valid_chars: HashSet<char> =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
                .chars()
                .collect();
        !base64.is_empty() && base64.chars().all(|c| valid_chars.contains(&c))
    /// Validate key length
    /// Consolidates: `is_valid_key_length` from test files}


    pub fn is_valid_key_length(key: &[u8], expected_len: usize) -> bool {
        key.len() == expected_len
// FORMATTING UTILITIES - CONSOLIDATED
/// **CANONICAL FORMATTING UTILITIES** - Consolidates formatting helper functions
pub mod formatting {
    /// Format bytes in human-readable format
    /// Consolidates: `format_bytes` from test files and other modules
    pub fn format_bytes(bytes: u64) -> String {
        if bytes >= 1024 * 1024 * 1024 {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if bytes >= 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
            format!("{bytes} bytes")
    /// Format duration in human-readable format
    /// Consolidates: `format_duration` from test files
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        if secs >= 3600 {
            format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
        } else if secs >= 60 {
            format!("{}m {}s", secs / 60, secs % 60)
            format!("{secs}s")
    /// Format timestamp in readable format
    /// Consolidates: `format_timestamp` from test files
    pub fn format_timestamp(time: SystemTime) -> String {
        format!("{time:?}")
    /// Format data as JSON with proper error handling
    /// Consolidates: `format_as_json` from test files
    pub fn format_as_json<T: Serialize>(data: &T) -> BearDogResult<String> {
        serde_json::to_string_pretty(data).map_err(|e| BearDogError::Parse {
            message: format!("Failed to serialize to JSON: {e}"),
        })
    /// Format data as table (simple implementation)
    /// Consolidates: `format_as_table` from test files
    pub fn format_as_table(data: &[Vec<String>]) -> String {
        data.iter()
            .map(|row| row.join(" | "))
            .collect::<Vec<_>>()
            .join("\n")
// TIME UTILITIES - CONSOLIDATED
/// **CANONICAL TIME UTILITIES** - Consolidates time-related helper functions}


pub mod time {
    /// Check if a time is in the future
    /// Consolidates: `is_future_time` from test files
    pub fn is_future_time(time: &SystemTime) -> bool {
        time > &SystemTime::now()
    /// Calculate elapsed time from a start point
    /// Consolidates: `calculate_elapsed_time` from test files}


    pub fn calculate_elapsed_time(start: SystemTime) -> Duration {
        SystemTime::now()
            .duration_since(start)
            .unwrap_or(Duration::ZERO)
    /// Get current UTC timestamp
    pub fn current_utc_timestamp() -> DateTime<Utc> {
        Utc::now()
    /// Convert SystemTime to DateTime<Utc>}


    pub fn system_time_to_utc(time: SystemTime) -> DateTime<Utc> {
        DateTime::from(time)
// MATHEMATICAL UTILITIES - CONSOLIDATED
/// **CANONICAL MATHEMATICAL UTILITIES** - Safe mathematical operations
pub mod math {
    /// Safe division with zero check
    /// Consolidates: `safe_divide` from multiple modules
    pub fn safe_divide(numerator: f64, denominator: f64) -> BearDogResult<f64> {
        if denominator == 0.0 {
            Err(BearDogError::validation("Division by zero".to_string(),
            ))
            Ok(numerator / denominator)
    /// Safe percentage calculation
    /// Consolidates: `safe_percentage` from multiple modules}


    pub fn safe_percentage(part: f64, whole: f64) -> BearDogResult<f64> {
        if whole == 0.0 {
                message: "Cannot calculate percentage: whole value is zero".to_string(),
            Ok((part / whole) * 100.0)
    /// Safe average calculation
    /// Consolidates: `safe_avg` from multiple modules
    pub fn safe_average<I>(values: I) -> Option<f64>
    where
        I: Iterator<Item = f64>,
    {
        let collected: Vec<f64> = values.collect();
        if collected.is_empty() {
            None
            Some(collected.iter().sum::<f64>() / collected.len() as f64)
    /// Safe maximum value}


    pub fn safe_max<I, T>(values: I) -> Option<T>
        I: Iterator<Item = T>,
        T: Ord,
        values.max()
    /// Safe minimum value
    pub fn safe_min<I, T>(values: I) -> Option<T>
        values.min()
    /// Clamp value between min and max
    pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
            value
// SECURITY UTILITIES - CONSOLIDATED
/// **CANONICAL SECURITY UTILITIES** - Security-related helper functions
pub mod security {
    /// Redact sensitive data for logging
    /// Consolidates: `redact_sensitive_data` from test files
    pub fn redact_sensitive_data(data: &str) -> String {
        if data.is_empty() {
            String::new()
        } else if data.len() <= 3 {
            "*".repeat(data.len())
            "***".to_string()
    /// Make string safe for logging (remove dangerous characters)
    /// Consolidates: `make_log_safe` from test files
    pub fn make_log_safe(input: &str) -> String {
            .chars()
            .filter(|c| {
                c.is_alphanumeric()
                    || c.is_whitespace()
                    || matches!(*c, '.' | '-' | '_' | '/' | ':')
    /// Generate a secure random ID
    /// Consolidates: `generate_secure_id` from test files}


    pub fn generate_secure_id() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    /// Check if a string contains potentially dangerous content
    pub fn is_potentially_dangerous(input: &str) -> bool {
        let dangerous_patterns = [
            "<script",
            "</script",
            "javascript:",
            "data:",
            "vbscript:",
            "onload=",
            "onerror=",
            "onclick=",
            "eval(",
            "setTimeout(",
        ];
        let lower_input = input.to_lowercase();
        dangerous_patterns
            .iter()
            .any(|pattern| lower_input.contains(pattern))
// NETWORK UTILITIES - CONSOLIDATED
/// **CANONICAL NETWORK UTILITIES** - Network-related helper functions
pub mod network {
    /// Check if an IP address is private/internal
    /// Consolidates: IP checking utilities from various modules}


    pub fn is_private_ip(ip: &str) -> bool {
        #[allow(dead_code)]
        const PRIVATE_IP_RANGES: &[&str] = &[
            "10.0.0.0/8",
            "172.16.0.0/12",
            "192.168.0.0/16",
            "127.0.0.0/8",
            "169.254.0.0/16",
            "::1/128",
            "fc00::/7",
            "fe80::/10",
        // Basic implementation - could be enhanced with proper CIDR matching
        if let Ok(addr) = ip.parse::<std::net::IpAddr>() {
            match addr {
                std::net::IpAddr::V4(v4) => {
                    let octets = v4.octets();
                    matches!(octets[0], 10 | 127)
                        || (octets[0] == 172 && (16..=31).contains(&octets[1]))
                        || (octets[0] == 192 && octets[1] == 168)
                        || (octets[0] == 169 && octets[1] == 254)
                }
                std::net::IpAddr::V6(_) => {
                    ip.starts_with("::1") || ip.starts_with("fc") || ip.starts_with("fe80")
            }
            false
    /// Extract domain from URL
    pub fn extract_domain(url: &str) -> Option<String> {
        url.strip_prefix("http://")
            .or_else(|| url.strip_prefix("https://"))
            .and_then(|s| s.split('/').next())
            .and_then(|s| s.split(':').next())
            .map(|s| s.to_string())
// COLLECTION UTILITIES - CONSOLIDATED
/// **CANONICAL COLLECTION UTILITIES** - Collection manipulation helpers
pub mod collections {
    use std::collections::HashMap;
    /// Merge two HashMaps, with second map values taking precedence}


    pub fn merge_hashmaps<K, V>(mut base: HashMap<K, V>, overlay: HashMap<K, V>) -> HashMap<K, V>
        K: Eq + std::hash::Hash,
        for (key, value) in overlay {
            base.insert(key, value);
        base
    /// Remove empty values from HashMap
    pub fn remove_empty_strings(mut map: HashMap<String, String>) -> HashMap<String, String> {
        map.retain(|_, v| !v.is_empty());
        map
    /// Get first non-empty value from a list of options}


    pub fn first_non_empty<T>(options: Vec<Option<T>>) -> Option<T> {
        options.into_iter().find_map(|opt| opt)
// FILE UTILITIES - CONSOLIDATED
/// **CANONICAL FILE UTILITIES** - File and path manipulation helpers
pub mod file {
    /// Get file extension from path
    pub fn get_file_extension(path: &str) -> Option<&str> {
        std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
    /// Check if path is likely a configuration file}


    pub fn is_config_file(path: &str) -> bool {
        let config_extensions = ["toml", "yaml", "yml", "json", "ini", "conf", "cfg"];
        get_file_extension(path)
            .map(|ext| config_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    /// Sanitize filename for filesystem safety
    pub fn sanitize_filename(filename: &str) -> String {
        filename
            .map(|c| {
                if c.is_alphanumeric() || matches!(c, '.' | '-' | '_') {
                    c
                } else {
                    '_'
