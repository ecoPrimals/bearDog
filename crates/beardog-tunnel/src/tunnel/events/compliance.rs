

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {

    GDPR,

    HIPAA,

    SOX,

    DataSovereignty,

    ExportControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeographicRegion {

    EU,

    US,

    China,

    Russia,

    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingRestriction {

    NoCloudStorage,

    NoInternationalRouting,

    EncryptionRequired,

    AuditTrailRequired,

    DataResidency(GeographicRegion),
}
