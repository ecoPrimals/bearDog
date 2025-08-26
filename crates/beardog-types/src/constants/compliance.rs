

use std::time::Duration;

pub const AUDIT_RETENTION_EXCEEDED: &str = "AuditRetentionExceeded";

pub const AUDIT_RETENTION_MSG: &str = "Event timestamp exceeds audit retention period";

pub const MISSING_CONSENT: &str = "MissingConsent";

pub const MISSING_CONSENT_MSG: &str = "Data access without explicit consent";

pub const MISSING_PURPOSE: &str = "MissingPurpose";

pub const MISSING_PURPOSE_MSG: &str = "Data access purpose not specified";

pub const ILLEGAL_TRANSFER: &str = "IllegalTransfer";

pub const ILLEGAL_TRANSFER_MSG: &str = "Data transfer without adequate protection";

pub const UNAUTHORIZED_FINANCIAL: &str = "UnauthorizedFinancialAccess";

pub const UNAUTHORIZED_FINANCIAL_MSG: &str = "Financial data access without proper authorization";

pub const UNENCRYPTED_PAYMENT: &str = "UnencryptedPaymentData";

pub const UNENCRYPTED_PAYMENT_MSG: &str = "Payment data processed without encryption";

pub const MINIMUM_NECESSARY: &str = "MinimumNecessaryViolation";

pub const MINIMUM_NECESSARY_MSG: &str = "PHI access not limited to minimum necessary";

pub const MISSING_AUDIT_LOG: &str = "MissingAuditLog";

pub const MISSING_AUDIT_LOG_MSG: &str = "PHI access not properly logged";

pub const COMPLIANCE_CHECK_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

pub const AUDIT_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 365); // 1 year

pub const DATA_ACCESS_MONITORING_WINDOW: Duration = Duration::from_secs(86400); // 24 hours

pub const CONSENT_VALIDATION_PERIOD: Duration = Duration::from_secs(86400 * 30); // 30 days

pub mod gdpr {
    use super::Duration;

    pub const REQUEST_RESPONSE_TIME: Duration = Duration::from_secs(72 * 3600);

    pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(72 * 3600);

    pub const ERASURE_PROCESSING_TIME: Duration = Duration::from_secs(30 * 86400);
}

pub mod hipaa {

    pub const PHI_AUDIT_RETENTION: Duration = Duration::from_secs(6 * 365 * 86400);

    pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(60 * 86400);

    pub const RISK_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);

pub mod pci_dss {

    pub const VULNERABILITY_SCAN_INTERVAL: Duration = Duration::from_secs(90 * 86400);

    pub const SECURITY_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);

    pub const LOG_RETENTION_PERIOD: Duration = Duration::from_secs(365 * 86400);

pub mod violations {

    pub const ILLEGAL_TRANSFER: &str = "ILLEGAL_DATA_TRANSFER";

    pub const UNAUTHORIZED_FINANCIAL: &str = "UNAUTHORIZED_FINANCIAL_ACCESS";

    pub const UNENCRYPTED_PAYMENT: &str = "UNENCRYPTED_PAYMENT_DATA";

    pub const MINIMUM_NECESSARY: &str = "MINIMUM_NECESSARY_VIOLATION";

    pub const MISSING_AUDIT_LOG: &str = "MISSING_AUDIT_LOG";
