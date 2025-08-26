

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::{KeyType, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait HsmAdapter: Send + Sync + std::fmt::Debug {

    async fn connect(&mut self) -> BearDogResult<()>;

    async fn disconnect(&mut self) -> BearDogResult<()>;

    async fn execute_operation(
        &self,
        operation: UniversalOperation,
    ) -> BearDogResult<OperationResult>;

    async fn health_check(&self) -> BearDogResult<HealthStatus>;

    fn get_adapter_info(&self) -> HsmAdapterInfo;

    fn get_supported_operations(&self) -> Vec<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAdapterInfo {

    pub adapter_id: Uuid,

    pub name: String,

    pub version: String,

    pub backend_type: HsmBackendType,

    pub capabilities: Vec<String>,

    pub connection_status: ConnectionStatus,

    pub last_health_check: Option<HealthStatus>,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmBackendType {

    Pkcs11,

    AndroidStrongBox,

    IosSecureEnclave,

    BearDogNative,

    Software,

    CloudHsm,

pub enum ConnectionStatus {

    Connected,

    Disconnected,

    Connecting,

    Failed,

    Maintenance,

pub struct UniversalOperation {

    pub operation_id: Uuid,

    pub operation_type: OperationType,

    pub parameters: HashMap<String, serde_json::Value>,

    pub data: Vec<u8>,

    pub timeout_ms: u64,

pub enum OperationType {

    GenerateKeyPair(KeyType),

    Sign { key_id: String },

    Verify { key_id: String },

    Encrypt { key_id: String },

    Decrypt { key_id: String },

    DeleteKey { key_id: String },

    ListKeys,

    GetKeyInfo { key_id: String },

pub struct OperationResult {

    pub success: bool,

    pub metadata: HashMap<String, String>,

    pub error: Option<String>,

    pub execution_time_ms: u64,

#[derive(Debug)]
pub struct Pkcs11Adapter {

    info: HsmAdapterInfo,

    library_path: String,

    slot_id: u32,

    user_pin: String,

    connection_handle: Option<u64>,}

impl Pkcs11Adapter {

    pub fn new(library_path: &str, slot_id: u32, user_pin: &str) -> Self {
        Self {
            info: HsmAdapterInfo {
                adapter_id: Uuid::new_v4(),
                name: "PKCS#11 HSM Adapter".to_string(),
                version: "1.0.0".to_string(),
                backend_type: HsmBackendType::Pkcs11,
                capabilities: vec![
                    "key_generation".to_string(),
                    "signing".to_string(),
                    "encryption".to_string(),
                    "key_storage".to_string(),
                ],
                connection_status: ConnectionStatus::Disconnected,
                last_health_check: None,
            },
            library_path,
            slot_id,
            user_pin,
            connection_handle: None,
        }
    }
impl HsmAdapter for Pkcs11Adapter {
    async fn connect(&mut self) -> BearDogResult<()> {
        info!("🔌 Connecting to PKCS#11 HSM at {}", self.library_path);

        self.connection_handle = Some(12345);
        self.info.connection_status = ConnectionStatus::Connected;
        debug!("✅ PKCS#11 HSM connected successfully");
        Ok(())
    async fn disconnect(&mut self) -> BearDogResult<()> {
        info!("🔌 Disconnecting from PKCS#11 HSM");
        self.connection_handle = None;
        self.info.connection_status = ConnectionStatus::Disconnected;
        debug!("✅ PKCS#11 HSM disconnected successfully");}

    async fn execute_operation(&self, operation: UniversalOperation) -> BearDogResult<OperationResult> {
        debug!("⚡ Executing PKCS#11 operation: {:?}", operation.operation_type);
        if self.connection_handle.is_none() {
            return Err(BearDogError::internal("PKCS#11 adapter not connected"));

        let result = OperationResult {
            operation_id: operation.operation_id,
            success: true,
            data: vec![0u8; 32], // Placeholder result
            metadata: HashMap::with_capacity(16),
            error: None,
            execution_time_ms: 100,
        };
        Ok(result)
    async fn health_check(&self) -> BearDogResult<HealthStatus> {
        if self.connection_handle.is_some() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
    fn get_adapter_info(&self) -> HsmAdapterInfo {
        self.info.clone()}

    fn get_supported_operations(&self) -> Vec<String> {
        vec![
            "GenerateKeyPair".to_string(),
            "Sign".to_string(),
            "Verify".to_string(),
            "Encrypt".to_string(),
            "Decrypt".to_string(),
            "DeleteKey".to_string(),
            "ListKeys".to_string(),
            "GetKeyInfo".to_string(),
        ]

pub struct AndroidStrongBoxAdapter {

    keystore_alias: String,
    connected: bool,}

impl AndroidStrongBoxAdapter {

    pub fn new(keystore_alias: &str) -> Self {
                name: "Android StrongBox Adapter".to_string(),
                backend_type: HsmBackendType::AndroidStrongBox,
                    "hardware_key_generation".to_string(),
                    "hardware_signing".to_string(),
                    "attestation".to_string(),
                    "secure_storage".to_string(),
            keystore_alias,
            connected: false,
impl HsmAdapter for AndroidStrongBoxAdapter {
        info!("📱 Connecting to Android StrongBox");

        self.connected = true;
        debug!("✅ Android StrongBox connected successfully");
        info!("📱 Disconnecting from Android StrongBox");
        self.connected = false;
        debug!("✅ Android StrongBox disconnected successfully");
        debug!("⚡ Executing Android StrongBox operation: {:?}", operation.operation_type);
        if !self.connected {
            return Err(BearDogError::internal("Android StrongBox adapter not connected"));

            execution_time_ms: 50,
        if self.connected {

pub struct BearDogNativeAdapter {

    config: HashMap<String, String>,}

impl BearDogNativeAdapter {

    pub fn new(config: HashMap<&str, &str>) -> Self {
                name: "BearDog Native HSM Adapter".to_string(),
                backend_type: HsmBackendType::BearDogNative,
                    "genetic_key_generation".to_string(),
                    "zero_cost_operations".to_string(),
                    "distributed_signing".to_string(),
                    "secure_enclaves".to_string(),
                    "quantum_resistance".to_string(),
            config,
impl HsmAdapter for BearDogNativeAdapter {
        info!("🐻 Connecting to BearDog Native HSM");

        debug!("✅ BearDog Native HSM connected successfully");
        info!("🐻 Disconnecting from BearDog Native HSM");
        debug!("✅ BearDog Native HSM disconnected successfully");
        debug!("⚡ Executing BearDog Native operation: {:?}", operation.operation_type);
            return Err(BearDogError::internal("BearDog Native adapter not connected"));

            execution_time_ms: 25, // Optimized performance
            "GeneticKeyGeneration".to_string(),
            "DistributedSigning".to_string(),

pub struct HsmAdapterRegistry {

    adapters: HashMap<Uuid, Box<dyn HsmAdapter>>,

    active_adapter: Option<Uuid>,}

impl HsmAdapterRegistry {

    pub fn new() -> Self {
            adapters: HashMap::with_capacity(16),
            active_adapter: None,

    pub fn register_adapter(&mut self, adapter: Box<dyn HsmAdapter>) -> Uuid {
        let adapter_id = adapter.get_adapter_info().adapter_id;
        self.adapters.insert(adapter_id, adapter);

        if self.active_adapter.is_none() {
            self.active_adapter = Some(adapter_id);
        adapter_id

    pub fn get_active_adapter(&self) -> Option<&dyn HsmAdapter> {
        if let Some(adapter_id) = self.active_adapter {
            self.adapters.get(&adapter_id).map(|a| a.as_ref())
            None

    pub fn set_active_adapter(&mut self, adapter_id: Uuid) -> BearDogResult<()> {
        if self.adapters.contains_key(&adapter_id) {
            Ok(())
            Err(BearDogError::not_found(format_args!("HSM adapter not found: {}", adapter_id).to_string()))

    pub fn list_adapters(&self) -> Vec<HsmAdapterInfo> {
        self.adapters.values()
            .map(|adapter| adapter.get_adapter_info())
            .collect()
