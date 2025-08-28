

use super::core_types::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::{KeyType, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyCeremonyType {

    KeyGeneration {

        key_type: KeyType,

        threshold: u32,

        total_participants: u32,
    },

    KeyRotation {

        old_key_id: String,

        new_key_type: KeyType,

        reason: String,

    KeyRecovery {

        key_id: String,

        participants: Vec<String>,

    MasterKeyGeneration {

        security_level: SecurityLevel,

        geographic_requirements: Option<Vec<String>>,

    EmergencyRevocation {

        emergency_auth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkParticipant {

    pub participant_id: Uuid,

    pub name: String,

    pub endpoint: String,

    pub public_key: Vec<u8>,

    pub capabilities: Vec<String>,

    pub status: ParticipantStatus,

    pub location: Option<String>,

    pub security_clearance: SecurityLevel,

    pub last_seen: SystemTime,

pub enum ParticipantStatus {

    Active,

    Unavailable,

    Offline,

    Compromised,

    Maintenance,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]}

pub enum SecurityLevel {

    Standard,

    High,

    Critical,

    Maximum,

pub enum DistributedHsmOperation {

    DistributedKeyGeneration {

        operation_id: Uuid,

        ceremony: KeyCeremonyType,

        participants: Vec<NetworkParticipant>,

        timeout: Duration,

    DistributedSigning {

        data: Vec<u8>,

    DistributedVerification {

        signature: Vec<u8>,

    DistributedBackup {

        strategy: BackupStrategy,

    DistributedAttestation {

        entity_id: String,

        challenge: Vec<u8>,

    Custom {

        operation_name: String,

        parameters: HashMap<String, String>,

pub enum BackupStrategy {

    Threshold {

        total_shares: u32,

    Geographic {

        regions: Vec<String>,

        redundancy: u32,

    TimeRotated {

        rotation_interval: Duration,

        generations: u32,

pub struct DistributedOperationResult {

    pub operation_id: Uuid,

    pub status: OperationStatus,

    pub result_data: Option<Vec<u8>>,

    pub participant_results: HashMap<Uuid, ParticipantResult>,

    pub metadata: HashMap<String, String>,

    pub started_at: SystemTime,

    pub completed_at: Option<SystemTime>,

    pub error: Option<String>,

pub enum OperationStatus {

    Initializing,

    InProgress,

    Completed,

    Failed,

    Cancelled,

    TimedOut,

pub struct ParticipantResult {

    pub status: ResponseStatus,

    pub response_data: Option<Vec<u8>>,

    pub timestamp: SystemTime,

pub enum ResponseStatus {

    Success,

    Timeout,

    Invalid,

#[derive(Debug)]}

pub struct DistributedOperationCoordinator {

    active_operations: HashMap<Uuid, DistributedHsmOperation>,

    operation_results: HashMap<Uuid, DistributedOperationResult>,

    participants: HashMap<Uuid, NetworkParticipant>,

    default_timeout: Duration,}

impl DistributedOperationCoordinator {

    pub fn new() -> Self {
        info!("🎭 Initializing Distributed Operation Coordinator");
        
        Self {
            active_operations: HashMap::with_capacity(16),
            operation_results: HashMap::with_capacity(16),
            participants: HashMap::with_capacity(16),
            default_timeout: Duration::from_secs(300), // 5 minutes default
        }
    }

    pub fn register_participant(&mut self, participant: NetworkParticipant) -> Result<(), BearDogError> {
        info!("📝 Registering participant: {}", participant.name);
        self.participants.insert(participant.participant_id, participant);
        Ok(())

    pub async fn start_operation(&mut self, operation: DistributedHsmOperation) -> Result<Uuid, BearDogError> {
        let operation_id = match &operation {
            DistributedHsmOperation::DistributedKeyGeneration { operation_id, .. } => *operation_id,
            DistributedHsmOperation::DistributedSigning { operation_id, .. } => *operation_id,
            DistributedHsmOperation::DistributedVerification { operation_id, .. } => *operation_id,
            DistributedHsmOperation::DistributedBackup { operation_id, .. } => *operation_id,
            DistributedHsmOperation::DistributedAttestation { operation_id, .. } => *operation_id,
            DistributedHsmOperation::Custom { operation_id, .. } => *operation_id,
        };
        info!("🚀 Starting distributed operation: {}", operation_id);

        let result = DistributedOperationResult {
            operation_id,
            status: OperationStatus::Initializing,
            result_data: None,
            participant_results: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(16),
            started_at: SystemTime::now(),
            completed_at: None,
            error: None,
        self.active_operations.insert(operation_id, operation);
        self.operation_results.insert(operation_id, result);

        debug!("🚀 Starting distributed operation {}", operation_id);

        debug!("✅ Operation {} initialized and tracking started", operation_id);
        Ok(operation_id)

    pub fn get_operation_status(&self, operation_id: Uuid) -> Result<OperationStatus, BearDogError> {
        if let Some(result) = self.operation_results.get(&operation_id) {
            Ok(result.status.clone())
        } else {
            Err(BearDogError::not_found(format_args!("Operation not found: {}", operation_id).to_string()))

    pub fn get_operation_result(&self, operation_id: Uuid) -> Result<DistributedOperationResult, BearDogError> {
            Ok(result.clone())
            Err(BearDogError::not_found(format_args!("Operation result not found: {}", operation_id).to_string()))

    pub async fn cancel_operation(&mut self, operation_id: Uuid) -> Result<(), BearDogError> {
        info!("🛑 Cancelling operation: {}", operation_id);
        if let Some(result) = self.operation_results.get_mut(&operation_id) {
            result.status = OperationStatus::Cancelled;
            result.completed_at = Some(SystemTime::now());

            self.active_operations.remove(&operation_id);
            Ok(())

    pub fn list_active_operations(&self) -> Vec<Uuid> {
        self.active_operations.keys().copied().collect()

    pub fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        let active_count = self.active_operations.len();
        let participant_count = self.participants.len();
        debug!("🏥 Coordinator health check: {} active operations, {} participants", 
               active_count, participant_count);

        if active_count > 100 {
            Ok(HealthStatus::Degraded)
        } else if participant_count == 0 {
            Ok(HealthStatus::Unhealthy)
            Ok(HealthStatus::Healthy)
