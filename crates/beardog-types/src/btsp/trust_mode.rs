//! Trust Mode Types for BTSP Unified
//!
//! This module defines trust verification modes for tunnel establishment:
//! - Genetic Lineage: For internal primal-to-primal communication
//! - Certificate: For external HTTPS API communication

use serde::{Deserialize, Serialize};

/// Trust verification mode for tunnel establishment
///
/// BTSP Unified supports two trust models:
/// - **Genetic Lineage**: Verify cryptographic family trees (internal primals)
/// - **Certificate**: Verify X.509 certificate chains (external servers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TrustMode {
    /// Internal mode: Genetic lineage verification
    ///
    /// Used for primal-to-primal communication where peers share
    /// cryptographic family trees (genetic lineage).
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "genetic_lineage",
    ///   "required_family": "nat0",
    ///   "verify_ancestry": true
    /// }
    /// ```
    #[serde(rename = "genetic_lineage")]
    GeneticLineage {
        /// Required genetic family (e.g., "nat0")
        ///
        /// If specified, the peer must belong to this family.
        /// If `None`, any family is accepted.
        #[serde(skip_serializing_if = "Option::is_none")]
        required_family: Option<String>,

        /// Required generation number
        ///
        /// If specified, the peer must be at this generation.
        /// If `None`, any generation is accepted.
        #[serde(skip_serializing_if = "Option::is_none")]
        required_generation: Option<u32>,

        /// Verify full ancestry chain
        ///
        /// If `true`, verifies the complete genetic lineage chain.
        /// If `false`, only verifies direct genetic signature.
        #[serde(default = "default_true")]
        verify_ancestry: bool,
    },

    /// External mode: X.509 certificate chain verification
    ///
    /// Used for external HTTPS communication where servers present
    /// standard X.509 certificates.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "certificate",
    ///   "server_name": "api.anthropic.com",
    ///   "verify_chain": true,
    ///   "root_ca_bundle": "mozilla"
    /// }
    /// ```
    #[serde(rename = "certificate")]
    Certificate {
        /// Server name for SNI (Server Name Indication)
        ///
        /// This MUST match the certificate's Subject Alternative Name (SAN)
        /// or Common Name (CN) field.
        server_name: String,

        /// Verify certificate chain against root CAs
        ///
        /// If `true`, verifies the full chain back to a trusted root CA.
        /// If `false`, accepts any certificate (testing only!).
        #[serde(default = "default_true")]
        verify_chain: bool,

        /// Root CA bundle to use for verification
        ///
        /// Defaults to Mozilla's root CA bundle (webpki-roots).
        #[serde(default)]
        root_ca_bundle: CaBundle,

        /// Allow self-signed certificates (TESTING ONLY!)
        ///
        /// ⚠️  DANGER: Only use this in development/testing environments!
        /// Self-signed certificates bypass trust verification.
        #[serde(default)]
        allow_self_signed: bool,
    },
}

/// Root CA bundle selection for certificate verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CaBundle {
    /// Mozilla's root CA bundle (webpki-roots)
    ///
    /// This is the same bundle used by Firefox and is updated regularly.
    /// Recommended for most use cases.
    #[default]
    Mozilla,

    /// System root CAs (platform-specific)
    ///
    /// Uses the operating system's trusted root CA store:
    /// - Linux: /etc/ssl/certs/
    /// - macOS: Keychain
    /// - Windows: Certificate Store
    System,

    /// Custom PEM bundle (base64-encoded)
    ///
    /// Allows specifying a custom set of trusted root CAs.
    /// The `pem_bundle` field contains concatenated PEM certificates.
    Custom {
        /// PEM-encoded root certificates (base64)
        pem_bundle: String,
    },
}

impl TrustMode {
    /// Check if this is internal mode (genetic lineage)
    pub fn is_internal(&self) -> bool {
        matches!(self, TrustMode::GeneticLineage { .. })
    }

    /// Check if this is external mode (certificate)
    pub fn is_external(&self) -> bool {
        matches!(self, TrustMode::Certificate { .. })
    }

    /// Get the server name (for external mode)
    pub fn server_name(&self) -> Option<&str> {
        match self {
            TrustMode::Certificate { server_name, .. } => Some(server_name),
            _ => None,
        }
    }

    /// Get the required family (for internal mode)
    pub fn required_family(&self) -> Option<&str> {
        match self {
            TrustMode::GeneticLineage {
                required_family, ..
            } => required_family.as_deref(),
            _ => None,
        }
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_lineage_serialization() {
        let trust = TrustMode::GeneticLineage {
            required_family: Some("nat0".into()),
            required_generation: None,
            verify_ancestry: true,
        };

        let json = serde_json::to_string(&trust).expect("Serialization failed");
        assert!(json.contains("genetic_lineage"));
        assert!(json.contains("nat0"));

        let parsed: TrustMode = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(trust, parsed);
    }

    #[test]
    fn test_genetic_lineage_defaults() {
        let json = r#"{"type":"genetic_lineage"}"#;
        let trust: TrustMode = serde_json::from_str(json).expect("Deserialization failed");

        match trust {
            TrustMode::GeneticLineage {
                required_family,
                required_generation,
                verify_ancestry,
            } => {
                assert_eq!(required_family, None);
                assert_eq!(required_generation, None);
                assert!(verify_ancestry); // Should default to true
            }
            _ => panic!("Expected GeneticLineage"),
        }
    }

    #[test]
    fn test_certificate_serialization() {
        let trust = TrustMode::Certificate {
            server_name: "api.anthropic.com".into(),
            verify_chain: true,
            root_ca_bundle: CaBundle::Mozilla,
            allow_self_signed: false,
        };

        let json = serde_json::to_string(&trust).expect("Serialization failed");
        assert!(json.contains("certificate"));
        assert!(json.contains("api.anthropic.com"));

        let parsed: TrustMode = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(trust, parsed);
    }

    #[test]
    fn test_certificate_defaults() {
        let json = r#"{"type":"certificate","server_name":"example.com"}"#;
        let trust: TrustMode = serde_json::from_str(json).expect("Deserialization failed");

        match trust {
            TrustMode::Certificate {
                server_name,
                verify_chain,
                root_ca_bundle,
                allow_self_signed,
            } => {
                assert_eq!(server_name, "example.com");
                assert!(verify_chain); // Should default to true
                assert_eq!(root_ca_bundle, CaBundle::Mozilla); // Should default to Mozilla
                assert!(!allow_self_signed); // Should default to false
            }
            _ => panic!("Expected Certificate"),
        }
    }

    #[test]
    fn test_ca_bundle_serialization() {
        // Mozilla bundle
        let bundle = CaBundle::Mozilla;
        let json = serde_json::to_string(&bundle).expect("Serialization failed");
        assert_eq!(json, r#""mozilla""#);

        // System bundle
        let bundle = CaBundle::System;
        let json = serde_json::to_string(&bundle).expect("Serialization failed");
        assert_eq!(json, r#""system""#);

        // Custom bundle
        let bundle = CaBundle::Custom {
            pem_bundle: "-----BEGIN CERTIFICATE-----".into(),
        };
        let json = serde_json::to_string(&bundle).expect("Serialization failed");
        assert!(json.contains("BEGIN CERTIFICATE"));
    }

    #[test]
    fn test_trust_mode_helpers() {
        let internal = TrustMode::GeneticLineage {
            required_family: Some("nat0".into()),
            required_generation: None,
            verify_ancestry: true,
        };

        assert!(internal.is_internal());
        assert!(!internal.is_external());
        assert_eq!(internal.required_family(), Some("nat0"));
        assert_eq!(internal.server_name(), None);

        let external = TrustMode::Certificate {
            server_name: "api.example.com".into(),
            verify_chain: true,
            root_ca_bundle: CaBundle::Mozilla,
            allow_self_signed: false,
        };

        assert!(!external.is_internal());
        assert!(external.is_external());
        assert_eq!(external.server_name(), Some("api.example.com"));
        assert_eq!(external.required_family(), None);
    }

    #[test]
    fn test_self_signed_warning() {
        // Ensure allow_self_signed defaults to false (secure by default)
        let json = r#"{"type":"certificate","server_name":"test.local"}"#;
        let trust: TrustMode = serde_json::from_str(json).expect("Deserialization failed");

        if let TrustMode::Certificate {
            allow_self_signed, ..
        } = trust
        {
            assert!(
                !allow_self_signed,
                "Self-signed should default to false for security"
            );
        }
    }
}

