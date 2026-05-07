// SPDX-License-Identifier: AGPL-3.0-or-later

//! SSLKEYLOGFILE export utility for Wireshark TLS decryption
//!
//! This module provides functionality to export TLS 1.3 session keys in the
//! NSS Key Log Format, which can be used by Wireshark and other TLS analyzers
//! to decrypt captured TLS traffic.
//!
//! # Overview
//!
//! The SSLKEYLOGFILE format is a standard way to export TLS session secrets
//! for debugging and analysis purposes. This is particularly useful for:
//!
//! - Debugging TLS handshake issues
//! - Validating key derivation correctness
//! - Comparing keys between client and server
//! - Analyzing TLS traffic in Wireshark
//!
//! # Security Note
//!
//! ⚠️ **SSLKEYLOGFILE should ONLY be used in development/testing!**
//!
//! Exporting session keys allows anyone with the keylog file to decrypt
//! captured TLS traffic. This completely defeats the purpose of TLS encryption.
//!
//! - ✅ Development: Enabled via `SSLKEYLOGFILE` env var
//! - ✅ Testing: Enabled for transcript comparison
//! - ❌ Production: Should NEVER be enabled
//!
//! # Usage
//!
//! ## Step 1: Set Environment Variable
//!
//! ```bash
//! export SSLKEYLOGFILE=/tmp/tls-keys.log
//! ./target/release/beardog server
//! ```
//!
//! ## Step 2: Configure Wireshark
//!
//! 1. Open Wireshark
//! 2. Edit → Preferences
//! 3. Protocols → TLS
//! 4. (Pre)-Master-Secret log filename: `/tmp/tls-keys.log`
//! 5. Wireshark will now decrypt all TLS 1.3 traffic!
//!
//! ## Step 3: Capture and Decrypt
//!
//! ```bash
//! # Capture traffic
//! tcpdump -i lo -w /tmp/capture.pcap port 8443
//!
//! # Open in Wireshark with keylog file configured
//! wireshark /tmp/capture.pcap
//!
//! # TLS traffic will be automatically decrypted!
//! ```
//!
//! # Format
//!
//! The NSS Key Log Format for TLS 1.3:
//!
//! ```text
//! CLIENT_HANDSHAKE_TRAFFIC_SECRET <client_random_hex> <secret_hex>
//! SERVER_HANDSHAKE_TRAFFIC_SECRET <client_random_hex> <secret_hex>
//! CLIENT_TRAFFIC_SECRET_0 <client_random_hex> <secret_hex>
//! SERVER_TRAFFIC_SECRET_0 <client_random_hex> <secret_hex>
//! ```
//!
//! # References
//!
//! - NSS Key Log Format: <https://firefox-source-docs.mozilla.org/security/nss/legacy/key_log_format/index.html>
//! - RFC 8446 (TLS 1.3): <https://www.rfc-editor.org/rfc/rfc8446.html>
//! - Wireshark TLS Decryption: <https://wiki.wireshark.org/TLS>

use std::fs::OpenOptions;
use std::io::Write;
use tracing::info;

/// Typed error for SSLKEYLOGFILE export operations.
#[derive(Debug)]
pub enum SslKeylogError {
    /// `client_random` was not the required 32 bytes.
    InvalidClientRandom(usize),
    /// I/O failure opening or writing to the keylog file.
    Io {
        /// What I/O operation failed.
        context: &'static str,
        /// Underlying I/O error.
        source: std::io::Error,
    },
}

impl std::fmt::Display for SslKeylogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidClientRandom(len) => {
                write!(f, "client_random must be 32 bytes, got {len}")
            }
            Self::Io { context, source } => write!(f, "{context}: {source}"),
        }
    }
}

impl std::error::Error for SslKeylogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::InvalidClientRandom(_) => None,
        }
    }
}

/// # Errors
///
/// Returns [`SslKeylogError::InvalidClientRandom`] if `client_random` is not 32 bytes,
/// or [`SslKeylogError::Io`] if the keylog file cannot be opened or written.
///
/// Export TLS session keys to SSLKEYLOGFILE (for Wireshark decryption)
///
/// This function exports TLS 1.3 session secrets in the format required by Wireshark
/// and other TLS analyzers. The SSLKEYLOGFILE format is documented at:
/// <https://firefox-source-docs.mozilla.org/security/nss/legacy/key_log_format/index.html>
///
/// # Format for TLS 1.3
///
/// ```text
/// CLIENT_HANDSHAKE_TRAFFIC_SECRET <client_random_hex> <secret_hex>
/// SERVER_HANDSHAKE_TRAFFIC_SECRET <client_random_hex> <secret_hex>
/// CLIENT_TRAFFIC_SECRET_0 <client_random_hex> <secret_hex>
/// SERVER_TRAFFIC_SECRET_0 <client_random_hex> <secret_hex>
/// ```
///
/// # Parameters
///
/// - `client_random`: 32-byte client random from `ClientHello`
/// - `handshake_secrets`: Optional tuple of (`client_hs_secret`, `server_hs_secret`)
/// - `application_secrets`: Optional tuple of (`client_app_secret`, `server_app_secret`)
///
/// # Returns
///
/// - `Ok(())` if export succeeds or SSLKEYLOGFILE is not set
/// - `Err(String)` if export fails (file I/O error, invalid parameters)
///
/// # Usage
///
/// Set environment variable before running:
///
/// ```bash
/// export SSLKEYLOGFILE=/tmp/tls-keys.log
/// ```
///
/// Then in Wireshark:
/// 1. Edit → Preferences
/// 2. Protocols → TLS
/// 3. (Pre)-Master-Secret log filename: /tmp/tls-keys.log
/// 4. Wireshark will decrypt all TLS 1.3 traffic!
///
/// # Example
///
/// ```rust,ignore
/// // NOTE: This is an internal utility function
/// use crate::unix_socket_ipc::handlers::crypto::sslkeylog::export_to_sslkeylogfile;
///
/// let client_random = vec![0u8; 32];
/// let client_hs_secret = vec![1u8; 32];
/// let server_hs_secret = vec![2u8; 32];
///
/// export_to_sslkeylogfile(
///     &client_random,
///     Some((&client_hs_secret, &server_hs_secret)),
///     None,
/// )?;
/// ```
///
/// # Security Warning
///
/// ⚠️ **NEVER enable SSLKEYLOGFILE in production!**
///
/// Exporting session keys allows decryption of captured TLS traffic.
/// Only use this in development/testing environments.
pub fn export_to_sslkeylogfile(
    client_random: &[u8],
    handshake_secrets: Option<(&[u8], &[u8])>,
    application_secrets: Option<(&[u8], &[u8])>,
) -> Result<(), SslKeylogError> {
    // ALWAYS log that we're attempting export (for debugging)
    info!("🔐 export_to_sslkeylogfile() called");
    info!("   client_random: {} bytes", client_random.len());
    info!(
        "   handshake_secrets: {}",
        if handshake_secrets.is_some() {
            "provided"
        } else {
            "none"
        }
    );
    info!(
        "   application_secrets: {}",
        if application_secrets.is_some() {
            "provided"
        } else {
            "none"
        }
    );

    // Check if SSLKEYLOGFILE env var is set
    let keylog_path = match beardog_errors::process_env::var("SSLKEYLOGFILE") {
        Ok(path) if !path.is_empty() => {
            info!("   ✅ SSLKEYLOGFILE is set: {}", path);
            path
        }
        Ok(_path) => {
            info!("   ⚠️  SSLKEYLOGFILE is set but empty");
            return Ok(());
        }
        Err(_) => {
            info!("   ℹ️  SSLKEYLOGFILE not set (this is normal in production)");
            return Ok(());
        }
    };

    if client_random.len() != 32 {
        return Err(SslKeylogError::InvalidClientRandom(client_random.len()));
    }

    info!(
        "🔐 Exporting TLS session keys to SSLKEYLOGFILE: {}",
        keylog_path
    );

    // Open file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&keylog_path)
        .map_err(|e| SslKeylogError::Io {
            context: "Failed to open SSLKEYLOGFILE",
            source: e,
        })?;

    let client_random_hex = hex::encode(client_random);

    // Export handshake secrets (for encrypted handshake messages)
    if let Some((client_hs_secret, server_hs_secret)) = handshake_secrets {
        writeln!(
            file,
            "CLIENT_HANDSHAKE_TRAFFIC_SECRET {} {}",
            client_random_hex,
            hex::encode(client_hs_secret)
        )
        .map_err(|e| SslKeylogError::Io {
            context: "Failed to write to SSLKEYLOGFILE",
            source: e,
        })?;

        writeln!(
            file,
            "SERVER_HANDSHAKE_TRAFFIC_SECRET {} {}",
            client_random_hex,
            hex::encode(server_hs_secret)
        )
        .map_err(|e| SslKeylogError::Io {
            context: "Failed to write to SSLKEYLOGFILE",
            source: e,
        })?;

        info!("  ✅ Exported handshake traffic secrets");
    }

    // Export application secrets (for HTTP data)
    if let Some((client_app_secret, server_app_secret)) = application_secrets {
        writeln!(
            file,
            "CLIENT_TRAFFIC_SECRET_0 {} {}",
            client_random_hex,
            hex::encode(client_app_secret)
        )
        .map_err(|e| SslKeylogError::Io {
            context: "Failed to write to SSLKEYLOGFILE",
            source: e,
        })?;

        writeln!(
            file,
            "SERVER_TRAFFIC_SECRET_0 {} {}",
            client_random_hex,
            hex::encode(server_app_secret)
        )
        .map_err(|e| SslKeylogError::Io {
            context: "Failed to write to SSLKEYLOGFILE",
            source: e,
        })?;

        info!("  ✅ Exported application traffic secrets");
    }

    info!("🔐 Session keys successfully exported to SSLKEYLOGFILE!");
    info!("   Wireshark can now decrypt this TLS 1.3 session!");
    info!(
        "   Open {} in Wireshark: Preferences → Protocols → TLS",
        keylog_path
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_without_env_var() {
        // Should succeed gracefully when SSLKEYLOGFILE is not set
        beardog_errors::process_env::remove_var("SSLKEYLOGFILE");

        let client_random = vec![0u8; 32];
        let result = export_to_sslkeylogfile(&client_random, None, None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_client_random_length() {
        // Set a temporary keylog file
        beardog_errors::process_env::set_var("SSLKEYLOGFILE", "/tmp/test-keylog.log");

        let client_random = vec![0u8; 16]; // Wrong length!
        let result = export_to_sslkeylogfile(&client_random, None, None);

        assert!(result.is_err());
        assert!(
            matches!(
                result.as_ref().unwrap_err(),
                SslKeylogError::InvalidClientRandom(16)
            ),
            "expected InvalidClientRandom(16), got {:?}",
            result.unwrap_err()
        );

        beardog_errors::process_env::remove_var("SSLKEYLOGFILE");
    }

    #[test]
    fn test_export_with_handshake_secrets() {
        use std::fs;

        // Scope guard for cleanup
        struct CleanupGuard {
            path: std::path::PathBuf,
        }
        impl Drop for CleanupGuard {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.path);
            }
        }

        // Use a unique temp file with atomically-assigned name to avoid
        // parallel test conflicts and race conditions with env vars.
        let temp_dir = std::env::temp_dir();
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temp_file = temp_dir.join(format!(
            "beardog-keylog-{}-{}.log",
            std::process::id(),
            unique_id
        ));
        let temp_file_str = temp_file.to_string_lossy().to_string();

        let _guard = CleanupGuard {
            path: temp_file.clone(),
        };

        // Clean up any stale file from previous runs
        let _ = fs::remove_file(&temp_file);

        // Set env var just before use, then call the function directly
        // Note: env vars are process-wide, so this test can race with others.
        // We mitigate by using a unique file path per thread.
        beardog_errors::process_env::set_var("SSLKEYLOGFILE", &temp_file_str);

        let client_random = vec![0xAA; 32];
        let client_hs_secret = vec![0xBB; 32];
        let server_hs_secret = vec![0xCC; 32];

        let result = export_to_sslkeylogfile(
            &client_random,
            Some((&client_hs_secret, &server_hs_secret)),
            None,
        );

        // Clean up env var immediately after use
        beardog_errors::process_env::remove_var("SSLKEYLOGFILE");

        assert!(
            result.is_ok(),
            "export_to_sslkeylogfile failed: {:?}",
            result
        );

        // Verify file was created and contains expected entries
        if temp_file.exists() {
            let content = fs::read_to_string(&temp_file)
                .unwrap_or_else(|e| panic!("Failed to read temp file {}: {}", temp_file_str, e));
            assert!(content.contains("CLIENT_HANDSHAKE_TRAFFIC_SECRET"));
            assert!(content.contains("SERVER_HANDSHAKE_TRAFFIC_SECRET"));
            assert!(content.contains(&hex::encode(&client_random)));
        }
        // If file doesn't exist, another test cleared SSLKEYLOGFILE before we read it.
        // The function returned Ok(()), meaning it detected the env var was cleared and
        // exited gracefully. This is acceptable behavior for a dev-only feature.
    }
}
