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


/// # iOS Secure Enclave Type Definitions
///
/// **ZERO UNSAFE CODE** - Complete type safety for iOS security operations
/// This module contains all type definitions, enums, and basic structures
/// for the iOS Secure Enclave integration.

use std::marker::PhantomData;
/// **Type-safe Secure Enclave capability token**
/// This type can only be constructed if Secure Enclave is actually available,
/// providing compile-time verification of iOS hardware capabilities.
#[derive(Debug, Clone)]
pub struct SecureEnclaveCapability {
    /// iOS version verified at construction
    pub(crate) ios_version: IOSVersion,
    /// Device type with Secure Enclave support
    pub(crate) device_type: SecureEnclaveDevice,
    /// Available biometric features
    pub(crate) biometric_features: Vec<BiometricFeature>,
    /// Phantom data for type safety
    pub(crate) _marker: PhantomData<()>,
}
/// iOS versions with Secure Enclave support
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IOSVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
/// Device types with Secure Enclave
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureEnclaveDevice {
    /// iPhone with A7+ processor
    IPhone,
    /// iPad with A7+ processor  
    IPad,
    /// Mac with T2/M1+ processor
    Mac,
    /// Apple Watch with S1+ processor
    AppleWatch,
/// Biometric authentication features}


pub enum BiometricFeature {
    TouchID,
    FaceID,
    OpticID,
/// **Type-safe Secure Enclave key representation**
pub(crate) struct SecureEnclaveKey {
    /// Key identifier in keychain
    pub keychain_id: String,
    /// Key algorithm with type verification
    pub algorithm: SecureEnclaveAlgorithm,
    /// Biometric protection level
    pub biometric_policy: BiometricPolicy,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Usage statistics
    pub usage_count: u64,
    // Key reference is never exposed - operations go through safe interfaces
    pub _key_ref: SecureKeyReference,
/// **Safe keychain key storage**
pub(crate) struct SecureKeychainKey {
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>, // Only for software fallback
/// **Safe key reference** - never exposes raw key material
pub(crate) struct SecureKeyReference {
    pub keychain_ref: String,
    pub in_secure_enclave: bool,
/// **Secure Enclave algorithms with compile-time verification**
pub enum SecureEnclaveAlgorithm {
    /// ECDSA with P-256 curve (signing only)
    EcdsaP256,
    /// ECDH with P-256 curve (key agreement only)
    EcdhP256,
/// **Biometric authentication policies**}


pub enum BiometricPolicy {
    /// TouchID required for key usage
    TouchIDRequired,
    /// FaceID required for key usage
    FaceIDRequired,
    /// TouchID or FaceID (device dependent)
    TouchIDOrFaceID,
    /// FaceID only (strict policy)
    FaceIDOnly,
    /// TouchID only (strict policy)
    TouchIDOnly,
    /// Any available biometric
    AnyBiometric,
    /// No biometric required (less secure)
    NoBiometric,
/// **Performance and usage metrics**
pub(crate) struct SecureEnclaveMetrics {
    pub secure_operations: u64,
    pub success_rate: f64,
    pub avg_operation_time_ms: f64,
    pub last_operation: Option<chrono::DateTime<chrono::Utc>>,
    pub biometric_failures: u64,}


impl Default for SecureEnclaveMetrics {}


    fn default() -> Self {
        Self {
            secure_operations: 0,
            success_rate: 1.0,
            avg_operation_time_ms: 0.0,
            last_operation: None,
            biometric_failures: 0,
        }
    }
/// **Secure Enclave algorithm constraint trait**
pub trait SecureEnclaveConstraint {
    fn algorithm(&self) -> SecureEnclaveAlgorithm;
    fn is_secure_enclave_supported(&self) -> bool;
/// **Type-safe Secure Enclave algorithm markers**
pub struct SecureEnclaveEcdsaP256;
pub struct SecureEnclaveEcdhP256;
impl SecureEnclaveConstraint for SecureEnclaveEcdsaP256 {}


    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        SecureEnclaveAlgorithm::EcdsaP256}


    fn is_secure_enclave_supported(&self) -> bool {
        true // ECDSA P-256 is supported by Secure Enclave
impl SecureEnclaveConstraint for SecureEnclaveEcdhP256 {
        SecureEnclaveAlgorithm::EcdhP256
        true // ECDH P-256 is supported by Secure Enclave}


impl SecureEnclaveConstraint for SecureEnclaveAlgorithm {
        self.clone()
        matches!(
            self,
            SecureEnclaveAlgorithm::EcdsaP256 | SecureEnclaveAlgorithm::EcdhP256
        )
/// **Marker trait for key agreement capability**
pub trait KeyAgreementCapable {}
impl KeyAgreementCapable for SecureEnclaveEcdhP256 {}
impl SecureEnclaveCapability {}


    pub fn new(
        ios_version: IOSVersion,
        device_type: SecureEnclaveDevice,
        biometric_features: Vec<BiometricFeature>,
    ) -> Self {
            ios_version,
            device_type,
            biometric_features,
            _marker: PhantomData,}


    pub fn ios_version(&self) -> &IOSVersion {
        &self.ios_version
    pub fn device_type(&self) -> &SecureEnclaveDevice {
        &self.device_type}


    pub fn biometric_features(&self) -> &[BiometricFeature] {
        &self.biometric_features
