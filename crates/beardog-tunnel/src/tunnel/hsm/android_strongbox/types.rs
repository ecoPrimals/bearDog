

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use beardog_core::{HsmHealthStatus, HsmKey};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrongBoxImplementation {

    /// Represents titan m variant
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
                })
            }
        });
        self.keystore.generate_key(key_id, &key_params)?;

        Ok(HsmKey {
            id: key_id.to_string(),
            key_type: key_type.to_string(),
            created_at: chrono::Utc::now(),
            metadata: HashMap::with_capacity(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.keystore.sign(&str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        self.keystore.verify(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.keystore.encrypt(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.keystore.decrypt(key_id, data)
    /// Removes key
    fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        self.keystore.delete_key(KeystoreConfig,

    /// Whether strongbox_available is enabled
    pub strongbox_available: bool,

    /// The strongbox implementation value
    pub strongbox_implementation: StrongBoxImplementation,

    #[cfg(Option<AndroidNativeHandle>,

#[cfg(String,

pub struct AndroidAttestationService {


    pub config: AttestationConfig,

    /// Collection of trusted certificates
    pub trusted_certificates: Vec<Vec<u8>>,

    /// The challenge generator value
    pub challenge_generator: Arc<ChallengeGenerator>,

pub struct AndroidDeviceInfo {

    /// The manufacturer value
    pub manufacturer: String,

    /// The model value
    pub model: String,


    pub android_version: String,

    /// Optional strongbox version
    pub strongbox_version: Option<String>,

    /// Optional titan m version
    pub titan_m_version: Option<String>,

    /// The security patch level value
    pub security_patch_level: String,

    /// The verified boot state value
    pub verified_boot_state: VerifiedBootState,}

impl AndroidDeviceInfo {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
            manufacturer: "Google".to_string(),}

            model: "Pixel 8".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: Some("1.0".to_string()),
            security_patch_level: "2024-01-01".to_string(),

/// Detect operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn detect() -> Result<Self, BearDogError> {

        Self::new(String,

    /// The key type value
    pub key_type: KeyType,

    /// The hsm type value
    pub hsm_type: HsmTier,

    /// Whether strongbox_backed is enabled
    pub strongbox_backed: bool,

    /// Whether attestation_verified is enabled
    pub attestation_verified: bool,

    /// The last used value
    pub last_used: chrono::DateTime<Utc>,

    /// Number of usage
    pub usage_count: u64,

    /// The created at value
    pub created_at: chrono::DateTime<Utc>,

    /// The usage policy value
    pub usage_policy: KeyUsagePolicy,

    /// Current status of the health
    pub health_status: KeyHealthStatus,

pub struct AndroidHealthMonitor {

    /// The keystore health value
    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,

    /// The strongbox health value
    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,

    /// The attestation health value
    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,

pub struct ChallengeGenerator {

    /// Number of entropy_source
    pub entropy_source: impl EntropySource,

pub trait EntropySource: Send + Sync {


    fn generate_entropy(&self, length: usize) -> Result<Vec<u8>, BearDogError>>;

pub struct AndroidEntropySource;

#[derive(Debug, Clone)]
    /// Number of key_size
    pub key_size: u32,

    /// Collection of purposes
    pub purposes: Vec<AndroidKeyPurpose>,

    /// Whether strongbox_required is enabled
    pub strongbox_required: bool,

    /// Whether user_authentication_required is enabled
    pub user_authentication_required: bool,


    pub user_authentication_validity_duration: Option<u32>,

    /// Optional attestation challenge
    pub attestation_challenge: Option<Vec<u8>>,


    pub key_validity_end: Option<chrono::DateTime<Utc>>,

    /// Optional curve
    pub curve: Option<AndroidEcCurve>,

#[derive(Debug, Clone)]
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: true,
            user_authentication_required: false,
            user_authentication_validity_duration: None,
            attestation_challenge: None,
            key_validity_end: None,
            curve: Some(AndroidEcCurve::P256),

/// Set Algorithm operation.
    /// Sets algorithm
    /// Sets algorithm
    pub fn set_algorithm(&mut self, algorithm: AndroidKeyAlgorithm) {
        self.algorithm = algorithm;

/// Set Key Size operation.
    /// Sets key_size
    /// Sets key_size
    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;

/// Set Purposes operation.
    /// Sets purposes
    /// Sets purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;

/// Set Strongbox Required operation.
    /// Sets strongbox_required
    /// Sets strongbox_required
    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;

/// Set User Authentication Required operation.
    /// Sets user_authentication_required
    /// Sets user_authentication_required
    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;

/// Set User Authentication Validity Duration operation.
    /// Sets user_authentication_validity_duration
    /// Sets user_authentication_validity_duration
    pub fn set_user_authentication_validity_duration(&mut self, duration: u32) {
        self.user_authentication_validity_duration = Some(duration);

/// Set Attestation Challenge operation.
    /// Sets attestation_challenge
    /// Sets attestation_challenge
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);

/// Set Key Validity End operation.
    /// Sets key_validity_end
    /// Sets key_validity_end
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<Utc>) {
        self.key_validity_end = Some(end);

/// Set Curve operation.
    /// Sets curve
    /// Sets curve
    pub fn set_curve(&mut self, curve: AndroidEcCurve) {
        self.curve = Some(curve);
impl Default for AndroidKeyParams {
        Self::new()}

impl From<StrongBoxError> for BearDogError {
    fn from(err: StrongBoxError) -> Self {
        BearDogError::Hsm {
            message: format!("StrongBox error: {err:?}"),}

impl StrongBoxError {

/// From Strongbox Error operation.
    /// Creates instance from strongbox error
    pub fn from_strongbox_error(i32, message: &str) -> BearDogError {
            message: format!("StrongBox error (code {code}): {message}"),
