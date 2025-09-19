

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::core_types::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::{KeyType, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait HsmAdapter: Send + Sync + std::fmt::Debug {


    fn connect(UniversalOperation,
    ) -> Result<OperationResult, BearDogError>;


    fn health_check(Uuid,

    /// Name of the item
    pub name: String,

    /// The version value
    pub version: String,

    /// The backend type value
    pub backend_type: HsmBackendType,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Current status of the connection
    pub connection_status: ConnectionStatus,

    /// Optional last health check
    pub last_health_check: Option<HealthStatus>,

#[derive(Debug, Clone)]
    /// The operation type value
    pub operation_type: OperationType,

    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Collection of data
    pub data: Vec<u8>,


    pub timeout_ms: u64,
/// Types of operation
pub enum OperationType {

    GenerateKeyPair(String },
    GenerateKeyPair(String },
    GenerateKeyPair(String },

    Verify { key_id: String },

    Encrypt { key_id: String },

    Decrypt { key_id: String },

    DeleteKey { key_id: String },


    ListKeys,

    GetKeyInfo { key_id: String },

pub struct OperationResult {

    /// Whether success is enabled
    pub success: bool,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,

    /// Optional error
    pub error: Option<String>,


    pub execution_time_ms: u64,

#[derive(Debug, Clone)]
    library_path: String,

    slot_id: u32,

    user_pin: String,

    connection_handle: Option<u64>,}

impl Pkcs11Adapter {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, slot_id: u32, user_pin: &str) -> Self {
        Self {
            info: HsmAdapterInfo {
                adapter_id: Uuid::new_v4(),
                name: "PKCS#11 HSM Adapter".to_string(),
                version: "1.0.0".to_string();

        self.connection_handle = Some(12345);
        self.info.connection_status = ConnectionStatus::Connected;
        debug!("✅ PKCS#11 HSM connected successfully");
        Ok(())
    fn disconnect(&mut self) -> Result<(), BearDogError> {
        info!("🔌 Disconnecting from PKCS#11 HSM");
        self.connection_handle = None;
        self.info.connection_status = ConnectionStatus::Disconnected;
        debug!("✅ PKCS#11 HSM disconnected successfully");}

    /// Executes operation
    fn execute_operation(&self, operation: UniversalOperation) -> Result<OperationResult, BearDogError> {
        debug!("⚡ Executing PKCS#11 operation: {:?}", operation.operation_type);
        if self.connection_handle.is_none() {
            return Err(BearDogError::internal(operation.operation_id,
            success: true,
            data: vec![0u8; 32], // Placeholder result
            metadata: HashMap::with_capacity(None,
            execution_time_ms: 100,
        };
        Ok(result)
    fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        if self.connection_handle.is_some() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
    /// Gets adapter_info
    fn get_adapter_info(String,
    connected: bool,}

impl AndroidStrongBoxAdapter {

/// New operation.
    /// Creates a new instance
    pub fn new(keystore_alias: &str) -> Self {
                name: "Android StrongBox Adapter".to_string();
        if !self.connected {
            return Err(BearDogError::internal(50,
        if self.connected {

pub struct BearDogNativeAdapter {

    config: HashMap<String, String>,}

impl BearDogNativeAdapter {

/// New operation.
    /// Creates a new instance
    pub fn new(HashMap<&str, &str>) -> Self {
                name: "BearDog Native HSM Adapter".to_string();
            return Err(BearDogError::internal(25, // Optimized performance
            "GeneticKeyGeneration".to_string()))

/// List Adapters operation.
    pub fn list_adapters(&self) -> Vec<HsmAdapterInfo> {
        self.adapters.values()
            .map(|adapter| adapter.get_adapter_info())
            .collect()
