

use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use beardog_core::{HsmHealthStatus, HsmKey};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrongBoxImplementation {

    TitanM {

        version: String,

        security_level: String,
    },

    QualcommSpu {

        attestation_support: bool,

    SamsungKnox {

    MediaTekHsm {

        features: Vec<String>,

    Generic {

        vendor: String,

        capabilities: Vec<String>,
}
impl Default for StrongBoxImplementation {}

    fn default() -> Self {
        StrongBoxImplementation::TitanM {
            version: "1.0".to_string(),
            security_level: "EAL4+".to_string(),
        }
    }

pub struct AndroidStrongBoxHsm {

    pub config: AndroidHsmConfig,

    pub keystore: Arc<AndroidKeystore>,

    pub attestation_service: Arc<AndroidAttestationService>,

    pub device_info: Arc<AndroidDeviceInfo>,

    pub key_cache: Arc<RwLock<HashMap<String, CachedKeyInfo>>>,

    pub health_monitor: Arc<AndroidHealthMonitor>,}

impl AndroidStrongBoxHsm {

    pub async fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {

        let keystore = Arc::new(AndroidKeystore::new(config.clone())?);

        let attestation_service = Arc::new(AndroidAttestationService::new(
            config.attestation_level.clone(),
        ));

        let device_info = Arc::new(AndroidDeviceInfo::new().await?);

        let key_cache = Arc::new(RwLock::new(HashMap::with_capacity(16)));

        let health_monitor = Arc::new(AndroidHealthMonitor::new());
        Ok(Self {
            config,
            keystore,
            attestation_service,
            device_info,
            key_cache,
            health_monitor,
        })

    pub async fn initialize(&mut self) -> BearDogResult<()> {

        self.keystore.test_keystore_access().await?;
        self.attestation_service.initialize().await?;

        self.health_monitor.start_monitoring().await?;
        Ok(())
impl HsmProvider for AndroidStrongBoxHsm {}

    async fn generate_key(&mut self, key_type: &str, key_id: &str) -> BearDogResult<HsmKey> {

        let key_params = AndroidKeyParams::new().set_algorithm(match key_type {
            "EC" => "EcP256",
            "RSA" => "Rsa2048",
            _ => {
                return Err(BearDogError::NotImplemented {
                    message: format_args!("Key type not supported: {}", key_type).to_string(),
                })
            }
        });
        self.keystore.generate_key(key_id, &key_params).await?;

        Ok(HsmKey {
            id: key_id.to_string(),
            key_type: key_type.to_string(),
            created_at: chrono::Utc::now(),
            metadata: HashMap::with_capacity(16),
    async fn sign(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.sign(key_id, data).await}

    async fn verify(&mut self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        self.keystore.verify(key_id, data, signature).await
    async fn encrypt(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.encrypt(key_id, data).await}

    async fn decrypt(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.decrypt(key_id, data).await
    async fn delete_key(&mut self, key_id: &str) -> BearDogResult<()> {
        self.keystore.delete_key(key_id).await}

    async fn list_keys(&mut self) -> BearDogResult<Vec<String>> {

        let cache = self.key_cache.read().await;
        Ok(cache.keys().cloned().collect())
    async fn get_health_status(&mut self) -> BearDogResult<HsmHealthStatus> {
        self.health_monitor.get_health_status().await

pub struct AndroidKeystore {

    pub config: KeystoreConfig,

    pub strongbox_available: bool,

    pub strongbox_implementation: StrongBoxImplementation,

    #[cfg(target_os = "android")]
    pub native_handle: Option<AndroidNativeHandle>,

#[cfg(target_os = "android")]
pub struct AndroidNativeHandle {

    pub device_context: String,

pub struct AndroidAttestationService {

    pub config: AttestationConfig,

    pub trusted_certificates: Vec<Vec<u8>>,

    pub challenge_generator: Arc<ChallengeGenerator>,

pub struct AndroidDeviceInfo {

    pub manufacturer: String,

    pub model: String,

    pub android_version: String,

    pub strongbox_version: Option<String>,

    pub titan_m_version: Option<String>,

    pub security_patch_level: String,

    pub verified_boot_state: VerifiedBootState,}

impl AndroidDeviceInfo {

    pub async fn new() -> BearDogResult<Self> {
            manufacturer: "Google".to_string(),}

            model: "Pixel 8".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: Some("1.0".to_string()),
            security_patch_level: "2024-01-01".to_string(),
            verified_boot_state: VerifiedBootState::Green,

    pub async fn detect() -> BearDogResult<Self> {

        Self::new().await

#[derive(Debug, Clone)]
pub struct CachedKeyInfo {

    pub key_id: String,

    pub key_type: KeyType,

    pub hsm_type: HsmTier,

    pub strongbox_backed: bool,

    pub attestation_verified: bool,

    pub last_used: chrono::DateTime<Utc>,

    pub usage_count: u64,

    pub created_at: chrono::DateTime<Utc>,

    pub usage_policy: KeyUsagePolicy,

    pub health_status: KeyHealthStatus,

pub struct AndroidHealthMonitor {

    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,

    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,

    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,

pub struct ChallengeGenerator {

    pub entropy_source: Arc<dyn EntropySource>,

pub trait EntropySource: Send + Sync {

    fn generate_entropy(&self, length: usize) -> BearDogResult<Vec<u8>>;

pub struct AndroidEntropySource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifiedBootState {

    Green, // Verified boot with locked bootloader

    Yellow, // Verified boot with unlocked bootloader

    Orange, // Custom OS

    Red, // Boot failure

    Unknown,

pub struct AndroidKeyParams {

    pub algorithm: AndroidKeyAlgorithm,

    pub key_size: u32,

    pub purposes: Vec<AndroidKeyPurpose>,

    pub strongbox_required: bool,

    pub user_authentication_required: bool,

    pub user_authentication_validity_duration: Option<u32>,

    pub attestation_challenge: Option<Vec<u8>>,

    pub key_validity_end: Option<chrono::DateTime<Utc>>,

    pub curve: Option<AndroidEcCurve>,

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyAlgorithm {

    Ec,

    Rsa,

    Aes,

pub enum AndroidKeyPurpose {

    Encrypt,

    Decrypt,

    Sign,

    Verify,

    Wrap,

    Unwrap,

pub enum AndroidEcCurve {

    P256,

    P384,

    P521,}

impl AndroidKeyParams {

    pub fn new() -> Self {
        Self {
            algorithm: AndroidKeyAlgorithm::Ec,
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: true,
            user_authentication_required: false,
            user_authentication_validity_duration: None,
            attestation_challenge: None,
            key_validity_end: None,
            curve: Some(AndroidEcCurve::P256),

    pub fn set_algorithm(&mut self, algorithm: AndroidKeyAlgorithm) {
        self.algorithm = algorithm;

    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;

    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;

    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;

    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;

    pub fn set_user_authentication_validity_duration(&mut self, duration: u32) {
        self.user_authentication_validity_duration = Some(duration);

    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);

    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<Utc>) {
        self.key_validity_end = Some(end);

    pub fn set_curve(&mut self, curve: AndroidEcCurve) {
        self.curve = Some(curve);
impl Default for AndroidKeyParams {
        Self::new()}

impl From<StrongBoxError> for BearDogError {
    fn from(err: StrongBoxError) -> Self {
        BearDogError::Hsm {
            message: format!("StrongBox error: {err:?}"),}

impl StrongBoxError {

    pub fn from_strongbox_error(code: i32, message: &str) -> BearDogError {
            message: format!("StrongBox error (code {code}): {message}"),
