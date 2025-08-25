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


/// # Compliance Constants
///
/// **CANONICAL COMPLIANCE-RELATED CONSTANTS**
/// Compliance standards, violation types, and regulatory framework constants.

use std::time::Duration;
// ============================================================================
// COMPLIANCE ERROR CODES AND MESSAGES
/// Audit retention period exceeded error code
pub const AUDIT_RETENTION_EXCEEDED: &str = "AuditRetentionExceeded";
/// Audit retention period exceeded error message
pub const AUDIT_RETENTION_MSG: &str = "Event timestamp exceeds audit retention period";
/// Missing consent error code
pub const MISSING_CONSENT: &str = "MissingConsent";
/// Missing consent error message
pub const MISSING_CONSENT_MSG: &str = "Data access without explicit consent";
/// Missing purpose error code
pub const MISSING_PURPOSE: &str = "MissingPurpose";
/// Missing purpose error message
pub const MISSING_PURPOSE_MSG: &str = "Data access purpose not specified";
/// Illegal transfer error code
pub const ILLEGAL_TRANSFER: &str = "IllegalTransfer";
/// Illegal transfer error message
pub const ILLEGAL_TRANSFER_MSG: &str = "Data transfer without adequate protection";
/// Unauthorized financial access error code
pub const UNAUTHORIZED_FINANCIAL: &str = "UnauthorizedFinancialAccess";
/// Unauthorized financial access error message
pub const UNAUTHORIZED_FINANCIAL_MSG: &str = "Financial data access without proper authorization";
/// Unencrypted payment data error code
pub const UNENCRYPTED_PAYMENT: &str = "UnencryptedPaymentData";
/// Unencrypted payment data error message
pub const UNENCRYPTED_PAYMENT_MSG: &str = "Payment data processed without encryption";
/// Minimum necessary violation error code
pub const MINIMUM_NECESSARY: &str = "MinimumNecessaryViolation";
/// Minimum necessary violation error message
pub const MINIMUM_NECESSARY_MSG: &str = "PHI access not limited to minimum necessary";
/// Missing audit log error code
pub const MISSING_AUDIT_LOG: &str = "MissingAuditLog";
/// Missing audit log error message
pub const MISSING_AUDIT_LOG_MSG: &str = "PHI access not properly logged";
// COMPLIANCE MONITORING INTERVALS
/// Default compliance check interval
pub const COMPLIANCE_CHECK_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
/// Audit log retention period
pub const AUDIT_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 365); // 1 year
/// Data access monitoring window
pub const DATA_ACCESS_MONITORING_WINDOW: Duration = Duration::from_secs(86400); // 24 hours
/// Consent validation period
pub const CONSENT_VALIDATION_PERIOD: Duration = Duration::from_secs(86400 * 30); // 30 days
// REGULATORY FRAMEWORK CONSTANTS
/// GDPR compliance requirements
pub mod gdpr {
    use super::Duration;
    /// Data subject request response time (72 hours)
    pub const REQUEST_RESPONSE_TIME: Duration = Duration::from_secs(72 * 3600);
    /// Data breach notification time (72 hours)
    pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(72 * 3600);
    /// Right to be forgotten processing time (30 days)
    pub const ERASURE_PROCESSING_TIME: Duration = Duration::from_secs(30 * 86400);
}
/// HIPAA compliance requirements
pub mod hipaa {
    /// PHI access audit retention (6 years)
    pub const PHI_AUDIT_RETENTION: Duration = Duration::from_secs(6 * 365 * 86400);
    /// Breach notification time (60 days)
    pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(60 * 86400);
    /// Risk assessment interval (annually)
    pub const RISK_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);
/// PCI DSS compliance requirements
pub mod pci_dss {
    /// Vulnerability scan frequency (quarterly)
    pub const VULNERABILITY_SCAN_INTERVAL: Duration = Duration::from_secs(90 * 86400);
    /// Security assessment frequency (annually)
    pub const SECURITY_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);
    /// Log retention period (1 year)
    pub const LOG_RETENTION_PERIOD: Duration = Duration::from_secs(365 * 86400);
/// **CANONICAL COMPLIANCE VIOLATION CONSTANTS** - Standard violation type identifiers
/// **MIGRATED FROM**: `beardog-compliance/src/compliance/types.rs`
pub mod violations {
    /// Illegal data transfer violation
    pub const ILLEGAL_TRANSFER: &str = "ILLEGAL_DATA_TRANSFER";
    /// Unauthorized financial access violation
    pub const UNAUTHORIZED_FINANCIAL: &str = "UNAUTHORIZED_FINANCIAL_ACCESS";
    /// Unencrypted payment data violation
    pub const UNENCRYPTED_PAYMENT: &str = "UNENCRYPTED_PAYMENT_DATA";
    /// Minimum necessary principle violation
    pub const MINIMUM_NECESSARY: &str = "MINIMUM_NECESSARY_VIOLATION";
    /// Missing audit log violation
    pub const MISSING_AUDIT_LOG: &str = "MISSING_AUDIT_LOG";
