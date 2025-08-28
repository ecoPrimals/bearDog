use beardog_errors::BearDogError;


use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttestationLevel {

    None,

    Software,

    TEE,

    HSM,

    SecureElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationData {

    pub level: AttestationLevel,

    pub certificate_chain: Vec<Vec<u8>>,

    pub attestation_signature: Vec<u8>,

    pub nonce: Vec<u8>,

    pub generated_at: DateTime<Utc>,

    pub challenge: Vec<u8>,

    pub metadata: HashMap<String, serde_json::Value>,}

impl AttestationData {

    pub fn new(
        _hardware_info: HardwareInfo,
        _software_info: SoftwareInfo,
        challenge: Vec<u8>,
    ) -> Self {
        Self {
            level: AttestationLevel::None, // Placeholder, will be updated by software
            certificate_chain: Vec::new(), // Placeholder
            attestation_signature: Vec::new(), // Placeholder
            nonce: Vec::new(),             // Placeholder
            generated_at: Utc::now(),      // Placeholder
            challenge,
            metadata: HashMap::with_capacity(16),
            certificate: Vec::new(),      // Placeholder
            signature: Vec::new(),        // Placeholder
            attestation_time: Utc::now(), // Placeholder
        }
    }

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self

    pub fn is_expired(&self, validity_duration_seconds: i64) -> bool {
        let expiry_time = self.generated_at + chrono::Duration::seconds(validity_duration_seconds);
        Utc::now() > expiry_time

    pub fn get_root_certificate(&self) -> Option<&Vec<u8>> {
        self.certificate_chain.last()

    pub fn get_device_certificate(&self) -> Option<&Vec<u8>> {
        self.certificate_chain.first()
impl std::fmt::Display for AttestationLevel {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttestationLevel::None => write!(f, "No Attestation"),
            AttestationLevel::Software => write!(f, "Software Attestation"),
            AttestationLevel::TEE => write!(f, "TEE Attestation"),
            AttestationLevel::HSM => write!(f, "HSM Attestation"),
            AttestationLevel::SecureElement => write!(f, "Secure Element Attestation"),

pub struct HardwareInfo {

    pub model: String,

    pub vendor: String,

    pub serial_number: Option<String>,

    pub security_chip: Option<SecurityChipInfo>,

    pub hardware_version: String,

    pub properties: HashMap<String, String>,

pub struct SecurityChipInfo {

    pub chip_type: String,

    pub version: String,

    pub certification: Option<String>,

    pub capabilities: Vec<String>,

pub struct SoftwareInfo {

    pub os_info: OsInfo,

    pub hsm_version: String,

    pub boot_state: BootState,

pub struct OsInfo {

    pub name: String,

    pub security_patch_level: Option<String>,

    pub verified_boot: bool,

pub struct BootState {

    pub locked_bootloader: bool,

    pub boot_state_color: Option<BootStateColor>,

    pub additional_info: HashMap<String, String>,

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootStateColor {

    Green,

    Yellow,

    Orange,

    Red,}

impl std::fmt::Display for BootStateColor {
            BootStateColor::Green => write!(f, "Green (Verified)"),
            BootStateColor::Yellow => write!(f, "Yellow (Custom OS)"),
            BootStateColor::Orange => write!(f, "Orange (Unverified)"),
            BootStateColor::Red => write!(f, "Red (Failed)"),

#[derive(Debug, Clone)]
pub struct AttestationVerificationResult {

    pub is_valid: bool,

    pub details: AttestationVerificationDetails,

    pub warnings: Vec<String>,

    pub verified_at: DateTime<Utc>,

pub struct AttestationVerificationDetails {

    pub certificate_chain_valid: bool,

    pub signature_valid: bool,

    pub nonce_valid: bool,

    pub hardware_verified: bool,

    pub software_verified: bool,

    pub verification_messages: Vec<String>,}

impl AttestationVerificationResult {

    pub fn success() -> Self {
            is_valid: true,
            details: AttestationVerificationDetails {
                certificate_chain_valid: true,
                signature_valid: true,
                nonce_valid: true,
                hardware_verified: true,
                software_verified: true,
                verification_messages: vec!["Attestation verification successful".to_string()],
            },
            warnings: Vec::new(),
            verified_at: Utc::now(),

    pub fn failure(reason: &str) -> Self {
            is_valid: false,
                certificate_chain_valid: false,
                signature_valid: false,
                nonce_valid: false,
                hardware_verified: false,
                software_verified: false,
                verification_messages: vec![reason],

    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warnings.push(warning);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_attestation_level_ordering() -> Result<(), BearDogError> {
        assert!(AttestationLevel::SecureElement > AttestationLevel::HSM);
        assert!(AttestationLevel::HSM > AttestationLevel::TEE);
        assert!(AttestationLevel::TEE > AttestationLevel::Software);
        assert!(AttestationLevel::Software > AttestationLevel::None);
        Ok(())}

    fn test_attestation_level_display() -> Result<(), BearDogError> {
        assert_eq!(AttestationLevel::None.to_string(), "No Attestation");
        assert_eq!(
            AttestationLevel::SecureElement.to_string(),
            "Secure Element Attestation"
        );
    fn test_boot_state_color_display() -> Result<(), BearDogError> {
        assert_eq!(BootStateColor::Green.to_string(), "Green (Verified)");
        assert_eq!(BootStateColor::Red.to_string(), "Red (Failed)");}

    fn test_attestation_data_creation() -> Result<(), BearDogError> {
        let hardware_info = HardwareInfo {
            model: "Test Device".to_string(),
            vendor: "Test Vendor".to_string(),
            serial_number: Some("12345".to_string()),
            security_chip: None,
            hardware_version: "1.0".to_string(),
            properties: HashMap::with_capacity(16),
        };
        let software_info = SoftwareInfo {
            os_info: OsInfo {
                name: "TestOS".to_string(),
                version: "1.0".to_string(),
                security_patch_level: None,
                verified_boot: true,
            hsm_version: "1.0.0".to_string(),
            boot_state: BootState {
                locked_bootloader: true,
                boot_state_color: Some(BootStateColor::Green),
                additional_info: HashMap::with_capacity(16),
        let attestation = AttestationData::new(
            AttestationLevel::HSM,
            vec![vec![1, 2, 3, 4]],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            hardware_info,
            software_info,
        assert_eq!(attestation.level, AttestationLevel::HSM);
        assert!(!attestation.is_expired(3600)); // 1 hour validity
            attestation.get_device_certificate(),
            Some(&vec![1, 2, 3, 4])
    fn test_attestation_verification_result() -> Result<(), BearDogError> {
        let success_result = AttestationVerificationResult::success();
        assert!(success_result.is_valid);
        assert!(success_result.details.certificate_chain_valid);
        let failure_result = AttestationVerificationResult::failure("Test failure".to_string());
        assert!(!failure_result.is_valid);
        assert!(!failure_result.details.certificate_chain_valid);
