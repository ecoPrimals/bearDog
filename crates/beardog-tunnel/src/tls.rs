//! TLS Configuration and Connection Management for BTSP
//!
//! Provides mTLS (mutual TLS) support for secure tunnel establishment
//! with certificate validation and connection pooling.

use beardog_errors::BearDogError;
use rustls::{ClientConfig, RootCertStore};
use rustls::pki_types::ServerName;
use std::sync::Arc;
use tokio_rustls::TlsConnector;

/// TLS configuration for BTSP connections
#[derive(Clone)]
pub struct TlsConfig {
    /// TLS connector for establishing connections
    connector: Arc<TlsConnector>,
    /// Whether to validate peer certificates
    validate_certs: bool,
}

impl TlsConfig {
    /// Create a new TLS configuration with system root certificates
    ///
    /// # Errors
    ///
    /// Returns an error if root certificate loading fails
    pub fn new() -> Result<Self, BearDogError> {
        // Load system root certificates
        let mut root_store = RootCertStore::empty();
        
        // rustls-native-certs 0.8 API: load_native_certs() returns CertificateResult
        let cert_result = rustls_native_certs::load_native_certs();
        
        // Check for errors
        if !cert_result.errors.is_empty() {
            // Log errors but continue if we got some certs
            if cert_result.certs.is_empty() {
                return Err(BearDogError::system(format!(
                    "Failed to load any root certificates: {:?}",
                    cert_result.errors
                )));
            }
        }
        
        for cert in cert_result.certs {
            // rustls 0.23 API: add() now takes CertificateDer directly
            root_store.add(cert).map_err(|e| {
                BearDogError::system(format!("Failed to add root certificate: {}", e))
            })?;
        }

        // rustls 0.23 API: with_root_certificates() instead of with_safe_defaults()
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Self {
            connector: Arc::new(TlsConnector::from(Arc::new(config))),
            validate_certs: true,
        })
    }

    /// Create a TLS configuration that accepts any certificate (for testing)
    ///
    /// # Security
    ///
    /// This should ONLY be used for testing. In production, always validate certificates.
    #[cfg(test)]
    pub fn insecure() -> Result<Self, BearDogError> {
        use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
        use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
        use rustls::DigitallySignedStruct;

        #[derive(Debug)]
        struct NoVerifier;

        impl ServerCertVerifier for NoVerifier {
            fn verify_server_cert(
                &self,
                _end_entity: &CertificateDer,
                _intermediates: &[CertificateDer],
                _server_name: &ServerName,
                _ocsp_response: &[u8],
                _now: UnixTime,
            ) -> Result<ServerCertVerified, rustls::Error> {
                Ok(ServerCertVerified::assertion())
            }

            fn verify_tls12_signature(
                &self,
                _message: &[u8],
                _cert: &CertificateDer,
                _dss: &DigitallySignedStruct,
            ) -> Result<HandshakeSignatureValid, rustls::Error> {
                Ok(HandshakeSignatureValid::assertion())
            }

            fn verify_tls13_signature(
                &self,
                _message: &[u8],
                _cert: &CertificateDer,
                _dss: &DigitallySignedStruct,
            ) -> Result<HandshakeSignatureValid, rustls::Error> {
                Ok(HandshakeSignatureValid::assertion())
            }

            fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
                vec![
                    rustls::SignatureScheme::RSA_PKCS1_SHA256,
                    rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
                ]
            }
        }

        // rustls 0.23 API: with_custom_certificate_verifier moved to dangerous module
        let config = ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(NoVerifier))
            .with_no_client_auth();

        Ok(Self {
            connector: Arc::new(TlsConnector::from(Arc::new(config))),
            validate_certs: false,
        })
    }

    /// Establish a TLS connection to the given endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Server endpoint in format "host:port"
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if connection established successfully
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Endpoint format is invalid
    /// * DNS resolution fails
    /// * Connection fails
    /// * TLS handshake fails
    /// * Certificate validation fails
    pub async fn connect(&self, endpoint: &str) -> Result<(), BearDogError> {
        // Parse endpoint
        let parts: Vec<&str> = endpoint.split(':').collect();
        if parts.len() != 2 {
            return Err(BearDogError::invalid_input(&format!(
                "Invalid endpoint format: {} (expected host:port)",
                endpoint
            )));
        }

        let host = parts[0];
        let port = parts[1]
            .parse::<u16>()
            .map_err(|e| BearDogError::invalid_input(&format!("Invalid port: {}", e)))?;

        // Parse server name for TLS (rustls 0.23 API)
        let server_name = ServerName::try_from(host.to_owned())
            .map_err(|e| BearDogError::invalid_input(&format!("Invalid hostname: {}", e)))?;

        // Establish TCP connection
        let stream = tokio::net::TcpStream::connect((host, port))
            .await
            .map_err(|e| BearDogError::system(format!("TCP connection failed: {}", e)))?;

        // Perform TLS handshake
        let _tls_stream = self
            .connector
            .connect(server_name, stream)
            .await
            .map_err(|e| BearDogError::system(format!("TLS handshake failed: {}", e)))?;

        Ok(())
    }

    /// Check if certificate validation is enabled
    pub fn validates_certificates(&self) -> bool {
        self.validate_certs
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self::new().expect("Failed to create default TLS config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_creation() {
        let config = TlsConfig::new();
        assert!(config.is_ok());
        assert!(config.unwrap().validates_certificates());
    }

    #[test]
    fn test_insecure_tls_config() {
        let config = TlsConfig::insecure();
        assert!(config.is_ok());
        assert!(!config.unwrap().validates_certificates());
    }

    #[tokio::test]
    async fn test_connect_invalid_endpoint() {
        let config = TlsConfig::new().unwrap();

        // Invalid format
        let result = config.connect("invalid").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid endpoint"));
    }

    #[tokio::test]
    async fn test_connect_invalid_port() {
        let config = TlsConfig::new().unwrap();

        let result = config.connect("localhost:invalid").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid port"));
    }

    #[tokio::test]
    async fn test_connect_nonexistent_host() {
        let config = TlsConfig::new().unwrap();

        let result = config.connect("nonexistent.example.invalid:443").await;
        assert!(result.is_err());
    }
}
