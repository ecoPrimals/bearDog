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


/// Security Error Categories
///
/// **SECURITY-RELATED ERROR TYPES**
/// This module contains all error types related to security operations including
/// cryptography, authentication, authorization, HSM operations, and threat detection.
use thiserror::Error;

/// Security-related error types
#[derive(Error, Debug, Clone)]
pub enum SecurityError {
    /// Encryption/decryption operation errors
    #[error("Encryption error in {operation}: {message}")]
    Encryption {
        /// The encryption operation that failed
        operation: String,
        /// Error message describing the encryption issue
        message: String,
    },
    /// Key management errors
    #[error("Key management error: {message}")]
    KeyManagement {
        /// Error message describing the key management issue
        message: String,
        /// Additional context about the key management error
        context: Vec<String>,
    },
    
    /// Key derivation errors
    #[error("Key derivation error: {message}")]
    KeyDerivation {
        /// Error message describing the key derivation issue
        message: String,
        /// Additional context about the key derivation error
        context: Vec<String>,
    },
    
    /// Cryptographic operation errors
    #[error("Cryptographic error: {message}")]
    Crypto {
        /// Error message describing the cryptographic issue
        message: String,
        /// Additional context about the cryptographic error
        context: Vec<String>,
    },
    
    /// HSM operation errors
    #[error("HSM error: {message}")]
    Hsm {
        /// Error message describing the HSM issue
        message: String,
        /// Additional context about the HSM error
        context: Vec<String>,
    },
    
    /// Authentication errors
    #[error("Authentication error: {message}")]
    Authentication {
        /// Error message describing the authentication issue
        message: String,
        /// Additional context about the authentication error
        context: Vec<String>,
    },
    
    /// Authorization errors
    #[error("Authorization error: {message}")]
    Authorization {
        /// Error message describing the authorization issue
        message: String,
        /// Additional context about the authorization error
        context: Vec<String>,
    },
    
    /// Threat detection errors
    #[error("Threat detection error: {message}")]
    ThreatDetection {
        /// Error message describing the threat detection issue
        message: String,
        /// Additional context about the threat detection error
        context: Vec<String>,
    },
    
    /// Entropy source not available errors
    #[error("Entropy source not available: {message}")]
    EntropySourceNotAvailable {
        /// Error message describing the entropy issue
        message: String,
        /// Additional context about the entropy error
        context: Vec<String>,
    },
    
    /// Permission denied errors
    #[error("Permission denied: {message}")]
    Permission {
        /// Error message describing the permission issue
        message: String,
        /// Additional context about the permission error
        context: Vec<String>,
    },
    
    /// Security policy violation errors
    #[error("Security policy violation: {message}")]
    SecurityPolicyViolation {
        /// Error message describing the policy violation
        message: String,
        /// Additional context about the policy violation
        context: Vec<String>,
    },
    
    /// Certificate errors
    #[error("Certificate error: {message}")]
    Certificate {
        /// Error message describing the certificate issue
        message: String,
        /// Additional context about the certificate error
        context: Vec<String>,
    },
    
    /// Signature verification errors
    #[error("Signature verification failed: {message}")]
    SignatureVerification {
        /// Error message describing the signature verification issue
        message: String,
        /// Additional context about the signature verification error
        context: Vec<String>,
    },
    
    /// Entropy quality errors
    #[error("Entropy quality insufficient: {message}")]
    EntropyQuality {
        /// Error message describing the entropy quality issue
        message: String,
        /// Additional context about the entropy quality error
        context: Vec<String>,
    },
    
    /// Insufficient entropy errors
    #[error("Insufficient entropy: {message}")]
    InsufficientEntropy {
        /// Error message describing the insufficient entropy issue
        message: String,
        /// Additional context about the insufficient entropy error
        context: Vec<String>,
    },
    
    /// No entropy source available
    #[error("No entropy source available: {message}")]
    NoEntropySource {
        /// Error message describing the lack of entropy sources
        message: String,
        /// Additional context about the entropy source error
        context: Vec<String>,
    },
    /// Capability not supported
    #[error("Capability not supported: {message}")]
    Capability {
        /// Error message describing the unsupported capability
        message: String,
        /// Additional context about the capability
        context: Vec<String>,
    },
    
    /// Rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimited {
        /// Error message describing the rate limit issue
        message: String,
        /// Additional context about the rate limit
        context: Vec<String>,
    },
}
impl SecurityError {
    /// Create a new encryption error}


    pub fn encryption(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Encryption {
            operation: operation.into(),
            message: message.into(),
        }
    }
    /// Create a new key management error
    pub fn key_management(message: impl Into<String>) -> Self {
        Self::KeyManagement {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new cryptographic error
    pub fn crypto(message: impl Into<String>) -> Self {
        Self::Crypto {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new HSM error
    pub fn hsm(message: impl Into<String>) -> Self {
        Self::Hsm {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new threat detection error
    pub fn threat_detection(message: impl Into<String>) -> Self {
        Self::ThreatDetection {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new entropy quality error
    pub fn entropy_quality(message: impl Into<String>) -> Self {
        Self::EntropyQuality {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Check if this is a critical security error
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            SecurityError::SecurityPolicyViolation { .. }
                | SecurityError::ThreatDetection { .. }
                | SecurityError::SignatureVerification { .. }
                | SecurityError::Certificate { .. }
        )
    }
    
    /// Check if this is an authentication/authorization error
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            SecurityError::Authentication { .. }
                | SecurityError::Authorization { .. }
                | SecurityError::Permission { .. }
        )
    }
    
    /// Check if this is a cryptographic error
    pub fn is_crypto_error(&self) -> bool {
        matches!(
            self,
            SecurityError::Encryption { .. }
                | SecurityError::Crypto { .. }
                | SecurityError::KeyManagement { .. }
                | SecurityError::KeyDerivation { .. }
                | SecurityError::Hsm { .. }
        )
    }
    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            SecurityError::Encryption { .. } => "encryption",
            SecurityError::KeyManagement { .. } => "key_management",
            SecurityError::KeyDerivation { .. } => "key_derivation",
            SecurityError::Crypto { .. } => "cryptography",
            SecurityError::Hsm { .. } => "hsm",
            SecurityError::Authentication { .. } => "authentication",
            SecurityError::Authorization { .. } => "authorization",
            SecurityError::ThreatDetection { .. } => "threat_detection",
            SecurityError::EntropySourceNotAvailable { .. } => "entropy_source",
            SecurityError::Permission { .. } => "permission",
            SecurityError::SecurityPolicyViolation { .. } => "security_policy",
            SecurityError::Certificate { .. } => "certificate",
            SecurityError::SignatureVerification { .. } => "signature",
            SecurityError::EntropyQuality { .. } => "entropy_quality",
            SecurityError::InsufficientEntropy { .. } => "insufficient_entropy",
            SecurityError::NoEntropySource { .. } => "no_entropy_source",
            SecurityError::Capability { .. } => "capability",
            SecurityError::RateLimited { .. } => "rate_limit",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_security_error_creation() {
        let error = SecurityError::encryption("AES", "Invalid key size");
        assert!(matches!(error, SecurityError::Encryption { .. }));
        assert_eq!(error.category(), "encryption");
    }
    
    #[test]
    fn test_error_classification() {
        let auth_error = SecurityError::authentication("Invalid credentials");
        assert!(auth_error.is_auth_error());
        assert!(!auth_error.is_crypto_error());
        let crypto_error = SecurityError::crypto("Decryption failed");
        assert!(crypto_error.is_crypto_error());
        assert!(!crypto_error.is_auth_error());
        let threat_error = SecurityError::threat_detection("Suspicious activity");
        assert!(threat_error.is_critical());
    }
    
    #[test]
    fn test_error_display() {
        let error = SecurityError::hsm("Hardware failure");
        let error_string = format!("{}", error);
        assert!(error_string.contains("HSM error"));
        assert!(error_string.contains("Hardware failure"));
    }
}
