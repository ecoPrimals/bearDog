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


/// # Privacy Protection Types
///
/// **EXTRACTED FROM LARGE FILE** - Type definitions and enums (~80 lines)
/// This module contains all the type definitions, enums, and constants used
/// throughout the privacy protection system.

use serde::{Deserialize, Serialize};
/// Privacy protection mechanism types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyProtectionType {
    TrafficObfuscation,
    DataAnonymization,
    MetadataScrubbing,
    OnionRouting,
    NoiseInjection,
    EncryptionAtRest,
    EncryptionInTransit,
    AccessControlMatrix,
    ZeroKnowledgeProof,
    HomomorphicEncryption,
}
/// Types of privacy vulnerabilities that can be detected
pub enum VulnerabilityType {
    MetadataLeakage,
    TrafficAnalysis,
    TimingAttack,
    SideChannelAttack,
    DataResidue,
    UnencryptedTransport,
    WeakAuthentication,
    ExcessivePermissions,
/// Severity levels for privacy vulnerabilities}


pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
/// Types of surveillance indicators that can be detected
pub enum IndicatorType {
    NetworkScanning,
    DataExfiltration,
    UnusualTraffic,
    MetadataCollection,
    FingerprintingAttempt,
    CorrelationAttack,
/// Types of privacy audit events
pub use beardog_types::canonical::AuditEventType;
/// Privacy impact levels for audit events}


pub enum PrivacyImpactLevel {
    None,
/// Compliance status for privacy audit events};


pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    Remediated,
