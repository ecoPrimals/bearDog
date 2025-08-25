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


/// # Consent Management Types
///
/// **EXTRACTED FROM LARGE FILE** - Type definitions and enums (~80 lines)
/// This module contains all the type definitions, enums, and constants used
/// throughout the consent management system.

use serde::{Deserialize, Serialize};
/// Consent status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentStatus {
    Granted,
    Denied,
    Revoked,
    Expired,
    Pending,
    Suspended,
}
/// Urgency levels for consent requests
pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
/// Types of resources that consent can cover}


pub enum ResourceType {
    PersonalData,
    ContactInformation,
    LocationData,
    BehavioralData,
    BiometricData,
    FinancialData,
    HealthData,
    CommunicationRecords,
    DeviceAccess,
    StorageAccess,
    NetworkAccess,
    ComputeResources,
    Custom(String),
/// Actions that can be permitted through consent
pub enum ActionType {
    Read,
    Write,
    Delete,
    Share,
    Process,
    Store,
    Transmit,
    Analyze,
    Transform,
    Backup,
    Archive,
/// Privacy levels for consent decisions}


pub enum PrivacyLevel {
    Public,
    Limited,
    Private,
    Confidential,
    Restricted,
/// Types of consent conditions
pub enum ConditionType {
    TimeLimit,
    UsageLimit,
    LocationRestriction,
    PurposeRestriction,
    ThirdPartyRestriction,
    DataRetention,
/// Consent audit event types}


pub enum ConsentAuditEventType {
    Used,
    Modified,
    Reinstated,
    Delegated,
    TransferRequested,
/// Delegation types for consent
pub enum DelegationType {
    Full,
    Temporary,
    Conditional,
/// Consent template categories}


pub enum TemplateCategory {
    DataSharing,
    ResourceAccess,
    Communication,
    Medical,
    Financial,
    Research,
    Marketing,
