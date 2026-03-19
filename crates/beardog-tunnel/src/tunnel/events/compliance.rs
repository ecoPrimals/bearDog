// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of compliance
pub enum ComplianceType {
    /// Represents g d p r variant
    GDPR,

    /// Represents h i p a a variant
    HIPAA,

    /// Represents s o x variant
    SOX,

    /// Represents data sovereignty variant
    DataSovereignty,

    /// Represents export control variant
    ExportControl,
}

/// Geographic regions for data sovereignty compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// Represents e u variant
    EU,

    /// Represents u s variant
    US,

    /// Represents china variant
    China,

    /// Represents russia variant
    Russia,

    /// Represents other variant
    Other(String),
}

/// Data routing restrictions for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingRestriction {
    /// Represents no cloud storage variant
    NoCloudStorage,

    /// Currently nointernationalrouting
    NoInternationalRouting,

    /// State indicating encryptionrequired
    EncryptionRequired,

    /// State indicating audittrailrequired
    AuditTrailRequired,

    /// Represents data residency variant
    DataResidency(GeographicRegion),
}
