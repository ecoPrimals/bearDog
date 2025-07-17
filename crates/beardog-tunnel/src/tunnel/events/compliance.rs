//! Compliance and geographic event definitions
//!
//! Contains compliance requirements and geographic routing restrictions.

use serde::{Deserialize, Serialize};

/// Compliance requirement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    /// General Data Protection Regulation (European Union)
    ///
    /// Protects personal data in research involving European subjects
    /// or institutions, ensuring privacy rights for international collaboration.
    GDPR,

    /// Health Insurance Portability and Accountability Act (United States)
    ///
    /// Protects healthcare data in medical research, ensuring patient
    /// privacy in biological and health-related academic studies.
    HIPAA,

    /// Sarbanes-Oxley Act (United States)
    ///
    /// Protects financial data in economic research, ensuring proper
    /// handling of financial information in business and economic studies.
    SOX,

    /// National data sovereignty requirements
    ///
    /// Ensures research data remains within specific national boundaries
    /// to comply with data residency laws and protect national interests.
    DataSovereignty,

    /// Export control restrictions on sensitive research
    ///
    /// Prevents unauthorized international sharing of research that could
    /// have national security implications or dual-use applications.
    ExportControl,
}

/// Geographic regions for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// European Union and associated regions
    ///
    /// Includes EU member states and regions following European
    /// data protection and academic collaboration frameworks.
    EU,

    /// United States and territories
    ///
    /// Includes regions following US regulatory frameworks and
    /// federal research compliance requirements.
    US,

    /// China and associated regions
    ///
    /// Requires careful handling due to data sovereignty laws and
    /// potential export control restrictions on research collaboration.
    China,

    /// Russia and associated regions
    ///
    /// Requires special consideration for geopolitical tensions and
    /// restrictions on academic and research collaboration.
    Russia,

    /// Other geographic regions not explicitly categorized
    ///
    /// Covers emerging regions with specific requirements for
    /// academic collaboration and research data handling.
    Other(String),
}

/// Routing restrictions for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingRestriction {
    /// Prevent data from being stored in external cloud services
    ///
    /// Ensures sensitive research data remains on controlled infrastructure
    /// rather than potentially unsecured third-party cloud storage.
    NoCloudStorage,

    /// Keep all traffic within national or regional boundaries
    ///
    /// Prevents research data from crossing international borders
    /// to comply with data sovereignty and export control requirements.
    NoInternationalRouting,

    /// Mandate encryption for all data transmission
    ///
    /// Ensures that sensitive research data is cryptographically protected
    /// during transmission to prevent interception or tampering.
    EncryptionRequired,

    /// Require detailed logging of all data access and transmission
    ///
    /// Creates audit trails for sensitive research data to meet compliance
    /// requirements and enable forensic analysis if needed.
    AuditTrailRequired,

    /// Restrict data to specific geographic regions
    ///
    /// Ensures research data remains within specified jurisdictions
    /// to comply with data residency laws and institutional policies.
    DataResidency(GeographicRegion),
}
