

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {

    GDPR,

    HIPAA,

    SOX,

    DataSovereignty,

    ExportControl,
}

pub enum GeographicRegion {

    EU,

    US,

    China,

    Russia,

    Other(String),

pub enum RoutingRestriction {

    NoCloudStorage,

    NoInternationalRouting,

    EncryptionRequired,

    AuditTrailRequired,

    DataResidency(GeographicRegion),
