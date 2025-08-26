

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentStatus {
    Granted,
    Denied,
    Revoked,
    Expired,
    Pending,
    Suspended,
}

pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Critical,
    Emergency,

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

pub enum PrivacyLevel {
    Public,
    Limited,
    Private,
    Confidential,
    Restricted,

pub enum ConditionType {
    TimeLimit,
    UsageLimit,
    LocationRestriction,
    PurposeRestriction,
    ThirdPartyRestriction,
    DataRetention,

pub enum ConsentAuditEventType {
    Used,
    Modified,
    Reinstated,
    Delegated,
    TransferRequested,

pub enum DelegationType {
    Full,
    Temporary,
    Conditional,

pub enum TemplateCategory {
    DataSharing,
    ResourceAccess,
    Communication,
    Medical,
    Financial,
    Research,
    Marketing,
