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
/// - `client_random`: 32-byte client random from ClientHello
/// - `handshake_secrets`: Optional tuple of (client_hs_secret, server_hs_secret)
/// - `application_secrets`: Optional tuple of (client_app_secret, server_app_secret)
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
/// ```rust,no_run
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
) -> Result<(), String> {
    // ALWAYS log that we're attempting export (for debugging)
    info!("🔐 export_to_sslkeylogfile() called");
    info!("   client_random: {} bytes", client_random.len());
    info!("   handshake_secrets: {}", if handshake_secrets.is_some() { "provided" } else { "none" });
    info!("   application_secrets: {}", if application_secrets.is_some() { "provided" } else { "none" });
    
    // Check if SSLKEYLOGFILE env var is set
    let keylog_path = match std::env::var("SSLKEYLOGFILE") {
        Ok(path) if !path.is_empty() => {
            info!("   ✅ SSLKEYLOGFILE is set: {}", path);
            path
        },
        Ok(_path) => {
            info!("   ⚠️  SSLKEYLOGFILE is set but empty");
            return Ok(());
        },
        Err(_) => {
            info!("   ℹ️  SSLKEYLOGFILE not set (this is normal in production)");
            return Ok(());
        }
    };
    
    if client_random.len() != 32 {
        return Err(format!("client_random must be 32 bytes, got {}", client_random.len()));
    }
    
    info!("🔐 Exporting TLS session keys to SSLKEYLOGFILE: {}", keylog_path);
    
    // Open file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&keylog_path)
        .map_err(|e| format!("Failed to open SSLKEYLOGFILE: {}", e))?;
    
    let client_random_hex = hex::encode(client_random);
    
    // Export handshake secrets (for encrypted handshake messages)
    if let Some((client_hs_secret, server_hs_secret)) = handshake_secrets {
        writeln!(file, "CLIENT_HANDSHAKE_TRAFFIC_SECRET {} {}", 
                 client_random_hex, hex::encode(client_hs_secret))
            .map_err(|e| format!("Failed to write to SSLKEYLOGFILE: {}", e))?;
        
        writeln!(file, "SERVER_HANDSHAKE_TRAFFIC_SECRET {} {}", 
                 client_random_hex, hex::encode(server_hs_secret))
            .map_err(|e| format!("Failed to write to SSLKEYLOGFILE: {}", e))?;
        
        info!("  ✅ Exported handshake traffic secrets");
    }
    
    // Export application secrets (for HTTP data)
    if let Some((client_app_secret, server_app_secret)) = application_secrets {
        writeln!(file, "CLIENT_TRAFFIC_SECRET_0 {} {}", 
                 client_random_hex, hex::encode(client_app_secret))
            .map_err(|e| format!("Failed to write to SSLKEYLOGFILE: {}", e))?;
        
        writeln!(file, "SERVER_TRAFFIC_SECRET_0 {} {}", 
                 client_random_hex, hex::encode(server_app_secret))
            .map_err(|e| format!("Failed to write to SSLKEYLOGFILE: {}", e))?;
        
        info!("  ✅ Exported application traffic secrets");
    }
    
    info!("🔐 Session keys successfully exported to SSLKEYLOGFILE!");
    info!("   Wireshark can now decrypt this TLS 1.3 session!");
    info!("   Open {} in Wireshark: Preferences → Protocols → TLS", keylog_path);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_without_env_var() {
        // Should succeed gracefully when SSLKEYLOGFILE is not set
        std::env::remove_var("SSLKEYLOGFILE");
        
        let client_random = vec![0u8; 32];
        let result = export_to_sslkeylogfile(&client_random, None, None);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_client_random_length() {
        // Set a temporary keylog file
        std::env::set_var("SSLKEYLOGFILE", "/tmp/test-keylog.log");
        
        let client_random = vec![0u8; 16]; // Wrong length!
        let result = export_to_sslkeylogfile(&client_random, None, None);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 32 bytes"));
        
        std::env::remove_var("SSLKEYLOGFILE");
    }

    #[test]
    fn test_export_with_handshake_secrets() {
        use std::fs;
        
        let temp_file = "/tmp/beardog-test-keylog.log";
        std::env::set_var("SSLKEYLOGFILE", temp_file);
        
        // Clean up any existing file
        let _ = fs::remove_file(temp_file);
        
        let client_random = vec![0xAA; 32];
        let client_hs_secret = vec![0xBB; 32];
        let server_hs_secret = vec![0xCC; 32];
        
        let result = export_to_sslkeylogfile(
            &client_random,
            Some((&client_hs_secret, &server_hs_secret)),
            None,
        );
        
        assert!(result.is_ok());
        
        // Verify file was created and contains expected entries
        let content = fs::read_to_string(temp_file).unwrap();
        assert!(content.contains("CLIENT_HANDSHAKE_TRAFFIC_SECRET"));
        assert!(content.contains("SERVER_HANDSHAKE_TRAFFIC_SECRET"));
        assert!(content.contains(&hex::encode(&client_random)));
        
        // Clean up
        let _ = fs::remove_file(temp_file);
        std::env::remove_var("SSLKEYLOGFILE");
    }
}

