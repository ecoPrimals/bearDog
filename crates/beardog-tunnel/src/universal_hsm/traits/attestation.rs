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


/// Hardware Attestation Traits - Universal Interface

use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// Attestation security levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// No attestation available
    None,
    /// Software-based attestation
    Software,
    /// Trusted Execution Environment
    TEE,
    /// Hardware Security Module
    HSM,
    /// Secure Element
    SecureElement,
}
/// Hardware attestation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationData {
    /// Attestation security level
    pub level: AttestationLevel,
    /// Certificate chain for verification
    pub certificate_chain: Vec<Vec<u8>>,
    /// Attestation signature
    pub attestation_signature: Vec<u8>,
    /// Nonce used for attestation
    pub nonce: Vec<u8>,
    /// When the attestation was generated
    pub generated_at: DateTime<Utc>,
    /// Challenge used for attestation
    pub challenge: Vec<u8>,
    /// Additional attestation metadata
    pub metadata: HashMap<String, serde_json::Value>,}


impl AttestationData {
    /// Create new attestation data}


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
            metadata: HashMap::new(),
            certificate: Vec::new(),      // Placeholder
            signature: Vec::new(),        // Placeholder
            attestation_time: Utc::now(), // Placeholder
        }
    }
    /// Add metadata to the attestation
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    /// Check if the attestation has expired}


    pub fn is_expired(&self, validity_duration_seconds: i64) -> bool {
        let expiry_time = self.generated_at + chrono::Duration::seconds(validity_duration_seconds);
        Utc::now() > expiry_time
    /// Get the root certificate from the chain
    pub fn get_root_certificate(&self) -> Option<&Vec<u8>> {
        self.certificate_chain.last()
    /// Get the device certificate from the chain}


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
/// **Hardware Information**
///
/// Information about the hardware platform included in attestation.}


pub struct HardwareInfo {
    /// Hardware model/type
    pub model: String,
    /// Hardware vendor
    pub vendor: String,
    /// Hardware serial number (if available)
    pub serial_number: Option<String>,
    /// Security chip information
    pub security_chip: Option<SecurityChipInfo>,
    /// Hardware version
    pub hardware_version: String,
    /// Additional hardware properties
    pub properties: HashMap<String, String>,
/// **Security Chip Information**
/// Information about dedicated security hardware.
pub struct SecurityChipInfo {
    /// Security chip type (e.g., "Titan M", "Secure Enclave", "`TPM` 2.0")
    pub chip_type: String,
    /// Chip vendor
    /// Chip version/revision
    pub version: String,
    /// Certification level (e.g., "FIPS 140-2 Level 3")
    pub certification: Option<String>,
    /// Chip capabilities
    pub capabilities: Vec<String>,
/// **Software Information**
/// Information about software/firmware included in attestation.
pub struct SoftwareInfo {
    /// Operating system information
    pub os_info: OsInfo,
    /// `HSM` software/firmware version
    pub hsm_version: String,
    /// Boot state information
    pub boot_state: BootState,
    /// Additional software properties
/// **Operating System Information**
pub struct OsInfo {
    /// OS name (e.g., "`Universal`", "`Universal`", "Linux")
    pub name: String,
    /// OS version
    /// Security patch level
    pub security_patch_level: Option<String>,
    /// Verified boot status
    pub verified_boot: bool,
/// **Boot State Information**
pub struct BootState {
    /// Whether the system booted with verified signatures
    /// Whether bootloader is locked
    pub locked_bootloader: bool,
    /// Boot state color (Green, Yellow, Orange, Red)
    pub boot_state_color: Option<BootStateColor>,
    /// Additional boot state information
    pub additional_info: HashMap<String, String>,
/// **Boot State Colors**
/// Android Verified Boot state colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootStateColor {
    /// All good - verified boot with locked bootloader
    Green,
    /// Warning - verified boot but with custom OS
    Yellow,
    /// Warning - unverified boot
    Orange,
    /// Error - verification failed or device compromised
    Red,}


impl std::fmt::Display for BootStateColor {
            BootStateColor::Green => write!(f, "Green (Verified)"),
            BootStateColor::Yellow => write!(f, "Yellow (Custom OS)"),
            BootStateColor::Orange => write!(f, "Orange (Unverified)"),
            BootStateColor::Red => write!(f, "Red (Failed)"),
/// **Attestation Verification Result**
/// Result of verifying hardware attestation data.}


#[derive(Debug, Clone)]
pub struct AttestationVerificationResult {
    /// Whether attestation is valid
    pub is_valid: bool,
    /// Verification details
    pub details: AttestationVerificationDetails,
    /// Any warnings or issues found
    pub warnings: Vec<String>,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
/// **Attestation Verification Details**
pub struct AttestationVerificationDetails {
    /// Certificate chain verification result
    pub certificate_chain_valid: bool,
    /// Signature verification result
    pub signature_valid: bool,
    /// Nonce verification result
    pub nonce_valid: bool,
    /// Hardware information verification
    pub hardware_verified: bool,
    /// Software information verification
    pub software_verified: bool,
    /// Detailed verification messages
    pub verification_messages: Vec<String>,}


impl AttestationVerificationResult {
    /// Create a successful verification result}


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
    /// Create a failed verification result
    pub fn failure(reason: String) -> Self {
            is_valid: false,
                certificate_chain_valid: false,
                signature_valid: false,
                nonce_valid: false,
                hardware_verified: false,
                software_verified: false,
                verification_messages: vec![reason],
    /// Add a warning to the verification result}


    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_attestation_level_ordering() -> beardog_errors::BearDogResult<()> {
        assert!(AttestationLevel::SecureElement > AttestationLevel::HSM);
        assert!(AttestationLevel::HSM > AttestationLevel::TEE);
        assert!(AttestationLevel::TEE > AttestationLevel::Software);
        assert!(AttestationLevel::Software > AttestationLevel::None);
        Ok(())}


    fn test_attestation_level_display() -> beardog_errors::BearDogResult<()> {
        assert_eq!(AttestationLevel::None.to_string(), "No Attestation");
        assert_eq!(
            AttestationLevel::SecureElement.to_string(),
            "Secure Element Attestation"
        );
    fn test_boot_state_color_display() -> beardog_errors::BearDogResult<()> {
        assert_eq!(BootStateColor::Green.to_string(), "Green (Verified)");
        assert_eq!(BootStateColor::Red.to_string(), "Red (Failed)");}


    fn test_attestation_data_creation() -> beardog_errors::BearDogResult<()> {
        let hardware_info = HardwareInfo {
            model: "Test Device".to_string(),
            vendor: "Test Vendor".to_string(),
            serial_number: Some("12345".to_string()),
            security_chip: None,
            hardware_version: "1.0".to_string(),
            properties: HashMap::new(),
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
                additional_info: HashMap::new(),
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
    fn test_attestation_verification_result() -> beardog_errors::BearDogResult<()> {
        let success_result = AttestationVerificationResult::success();
        assert!(success_result.is_valid);
        assert!(success_result.details.certificate_chain_valid);
        let failure_result = AttestationVerificationResult::failure("Test failure".to_string());
        assert!(!failure_result.is_valid);
        assert!(!failure_result.details.certificate_chain_valid);
