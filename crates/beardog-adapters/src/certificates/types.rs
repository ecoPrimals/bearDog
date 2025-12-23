// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Types for adapter certificates

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Cryptographically-signed certificate that unlocks an external adapter
///
/// This certificate proves that:
/// 1. The request has been classified (Human vs Commercial)
/// 2. Commercial requests have valid licenses
/// 3. The certificate is time-bound and must be renewed
/// 4. The BearDog daemon has authorized this specific adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterUnlockCertificate {
    /// Unique certificate ID
    pub id: String,

    /// Which adapter this unlocks (e.g., "prometheus", "grafana", "consul")
    pub adapter_id: String,

    /// Commercial classification of the requester
    pub classification: CommercialClassification,

    /// When this certificate expires
    pub expires_at: DateTime<Utc>,

    /// What this certificate allows
    pub scope: CertificateScope,

    /// Ed25519 signature from BearDog root key
    pub signature: Vec<u8>,

    /// Key ID that signed this certificate
    pub signed_by: String,

    /// When this certificate was issued
    pub issued_at: DateTime<Utc>,

    /// License information (required for commercial usage)
    pub license: Option<LicenseInfo>,

    /// Optional constraints (integrates with key constraint system)
    pub constraints: Option<AdapterConstraints>,
}

/// Commercial classification from the detector
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommercialClassification {
    /// Individual human user
    Human {
        /// Confidence score (0.0-1.0)
        confidence: u8, // 0-100 stored as u8

        /// Classification reasons
        reasons: Vec<String>,
    },

    /// Small team (< 10 people)
    SmallTeam {
        /// Confidence score (0.0-1.0)
        confidence: u8,

        /// Estimated team size
        team_size: u32,
    },

    /// Commercial/Corporate usage
    Commercial {
        /// Risk level
        risk: RiskLevel,

        /// Indicators of commercial use
        indicators: Vec<CommercialIndicator>,

        /// Confidence score (0.0-1.0)
        confidence: u8,
    },

    /// Unable to determine
    Uncertain {
        /// Reasons for uncertainty
        reasons: Vec<String>,
    },
}

/// Risk level for commercial classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Low risk - small commercial use
    Low,

    /// Medium risk - moderate commercial use
    Medium,

    /// High risk - large-scale commercial extraction
    High,

    /// Critical - attempted bypass or abuse
    Critical,
}

/// Indicators of commercial usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommercialIndicator {
    /// High request volume
    HighVolume { requests_per_hour: u64 },

    /// Automated/bot-like patterns
    AutomatedPatterns,

    /// No human interaction detected
    NoHumanInteraction,

    /// Corporate IP range
    CorporateIpRange { range: String },

    /// Known commercial entity
    KnownCommercialEntity { name: String },

    /// CI/CD system detected
    CiCdSystem { system: String },
}

/// License information for commercial usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    /// License key
    pub key: String,

    /// License type
    pub license_type: LicenseType,

    /// When license expires
    pub expires_at: DateTime<Utc>,

    /// Licensed entity
    pub entity: String,

    /// Maximum allowed requests per day
    pub rate_limit: Option<u64>,
}

/// Types of licenses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LicenseType {
    /// Free tier (limited)
    Free,

    /// Startup tier
    Startup,

    /// Professional tier
    Professional,

    /// Enterprise tier
    Enterprise,

    /// Custom/negotiated tier
    Custom,
}

/// What operations this certificate allows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateScope {
    /// Allowed adapter operations
    pub operations: Vec<AdapterOperation>,

    /// Read-only access
    pub read_only: bool,

    /// Maximum requests per hour
    pub rate_limit: Option<u64>,
}

/// Operations an adapter can perform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterOperation {
    /// Read metrics/data
    Read,

    /// Write metrics/data
    Write,

    /// Query/search
    Query,

    /// Configure adapter
    Configure,

    /// Administrative operations
    Admin,
}

/// Constraints specific to adapter usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConstraints {
    /// Allowed network endpoints
    pub allowed_endpoints: Vec<String>,

    /// Forbidden operations
    pub forbidden_operations: Vec<String>,

    /// Data access patterns
    pub data_patterns: Vec<String>,
}

/// How quickly certificates expire
#[derive(Debug, Clone, Copy)]
pub enum CertificateExpiry {
    /// For humans (24 hours)
    Human,

    /// For small teams (12 hours)
    SmallTeam,

    /// For commercial (15 minutes - must renew frequently)
    Commercial,

    /// Custom duration
    Custom { hours: u64 },
}

impl CertificateExpiry {
    /// Get duration in hours
    pub fn hours(&self) -> u64 {
        match self {
            Self::Human => 24,
            Self::SmallTeam => 12,
            Self::Commercial => 0, // 15 minutes = 0.25 hours, rounded down
            Self::Custom { hours } => *hours,
        }
    }

    /// Get duration in minutes
    pub fn minutes(&self) -> u64 {
        match self {
            Self::Human => 24 * 60,
            Self::SmallTeam => 12 * 60,
            Self::Commercial => 15,
            Self::Custom { hours } => hours * 60,
        }
    }
}

impl Default for CertificateScope {
    fn default() -> Self {
        Self {
            operations: vec![AdapterOperation::Read, AdapterOperation::Query],
            read_only: true,
            rate_limit: Some(1000), // 1000 req/hour default
        }
    }
}

impl AdapterUnlockCertificate {
    /// Check if certificate is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if certificate allows an operation
    pub fn allows_operation(&self, operation: &AdapterOperation) -> bool {
        self.scope.operations.contains(operation)
    }

    /// Get time until expiry
    pub fn time_until_expiry(&self) -> chrono::Duration {
        self.expires_at - Utc::now()
    }

    /// Check if renewal is recommended
    pub fn needs_renewal(&self) -> bool {
        // Recommend renewal when < 10% of lifetime remains
        let time_left = self.time_until_expiry();
        let lifetime = self.expires_at - self.issued_at;

        time_left < lifetime / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_expiry_durations() {
        assert_eq!(CertificateExpiry::Human.hours(), 24);
        assert_eq!(CertificateExpiry::SmallTeam.hours(), 12);
        assert_eq!(CertificateExpiry::Commercial.hours(), 0);

        assert_eq!(CertificateExpiry::Commercial.minutes(), 15);
    }

    #[test]
    fn test_certificate_expiry_check() {
        use chrono::Duration;

        let expired = AdapterUnlockCertificate {
            id: "test".to_string(),
            adapter_id: "prometheus".to_string(),
            classification: CommercialClassification::Human {
                confidence: 95,
                reasons: vec![],
            },
            expires_at: Utc::now() - Duration::hours(1),
            scope: CertificateScope::default(),
            signature: vec![],
            signed_by: "test_key".to_string(),
            issued_at: Utc::now() - Duration::hours(25),
            license: None,
            constraints: None,
        };

        assert!(expired.is_expired());
    }

    #[test]
    fn test_operation_check() {
        let cert = AdapterUnlockCertificate {
            id: "test".to_string(),
            adapter_id: "prometheus".to_string(),
            classification: CommercialClassification::Human {
                confidence: 95,
                reasons: vec![],
            },
            expires_at: Utc::now() + chrono::Duration::hours(24),
            scope: CertificateScope {
                operations: vec![AdapterOperation::Read, AdapterOperation::Query],
                read_only: true,
                rate_limit: Some(1000),
            },
            signature: vec![],
            signed_by: "test_key".to_string(),
            issued_at: Utc::now(),
            license: None,
            constraints: None,
        };

        assert!(cert.allows_operation(&AdapterOperation::Read));
        assert!(cert.allows_operation(&AdapterOperation::Query));
        assert!(!cert.allows_operation(&AdapterOperation::Write));
        assert!(!cert.allows_operation(&AdapterOperation::Admin));
    }
}
