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


/// # AI CLI HSM Operations
///
/// **EXTRACTED FROM LARGE FILE** - HSM operations and handlers (~140 lines)
/// This module contains Hardware Security Module (HSM) related CLI operations
/// including key management, attestation, and secure operations.
use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

/// HSM operations
#[derive(Debug, Subcommand)]
pub enum HsmOperation {
    /// Initialize HSM
    Initialize {
        /// HSM provider type
        #[arg(long, value_enum, default_value = "software")]
        provider: HsmProvider,
        /// Configuration file
        #[arg(long)]
        config: Option<PathBuf>,
        /// Force initialization (overwrite existing)
        force: bool,
        /// PIN for HSM access
        pin: Option<String>,
    },
    /// Generate key in HSM
    GenerateKey {
        /// Key type
        #[arg(long, value_enum, default_value = "rsa2048")]
        key_type: String,
        /// Key usage
        #[arg(long, value_enum, default_value = "general")]
        usage: String,
        /// Key label/ID
        label: String,
        /// Make key extractable
        extractable: bool,
        /// Key attributes
        attributes: Vec<String>,
    /// List HSM keys
    ListKeys {
        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
        /// Filter by key type
        #[arg(long, value_enum)]
        key_type: Option<String>,
        /// Filter by usage
        usage: Option<String>,
        /// Show detailed information
        detailed: bool,
    /// Delete HSM key
    DeleteKey {
        /// Key label/ID to delete
        /// Force deletion without confirmation
    /// Perform cryptographic operation
    CryptoOperation {
        /// Operation type
        #[arg(long, value_enum, default_value = "encrypt")]
        operation: CryptoOperationType,
        key_label: String,
        /// Input data or file
        input: String,
        /// Output file
        output: Option<PathBuf>,
        /// Algorithm parameters
        params: Vec<String>,
    /// Get HSM status
    Status {
        /// Include slot information
        include_slots: bool,
        /// Include mechanism information
        include_mechanisms: bool,
    /// Create attestation
    Attest {
        /// Key label/ID for attestation
        /// Data to attest
        data: String,
        /// Output attestation file
        output: PathBuf,
        /// Attestation format
        #[arg(long, default_value = "json")]
        format: String,
    /// Verify attestation
    VerifyAttestation {
        /// Attestation file
        attestation: PathBuf,
        /// Expected data
        expected_data: String,
        /// Trust anchor
        trust_anchor: Option<PathBuf>,
    /// Export public key
    ExportPublicKey {
        /// Key label/ID to export
        /// Export format
        #[arg(long, default_value = "PEM")]
    /// Import certificate
    ImportCertificate {
        /// Certificate file
        certificate: PathBuf,
        /// Certificate label/ID
        /// Certificate type
        #[arg(long, default_value = "X509")]
        cert_type: String,
    /// HSM diagnostics
    Diagnostics {
        /// Run comprehensive tests
        comprehensive: bool,
        /// Test specific operations
        test_operations: Vec<String>,
}
/// HSM provider types
#[derive(Debug, Clone, ValueEnum)]
pub enum HsmProvider {
    /// Software HSM (testing/development)
    Software,
    /// PKCS#11 HSM
    Pkcs11,
    /// AWS CloudHSM
    AwsCloudHsm,
    /// Azure Dedicated HSM
    AzureDedicatedHsm,
    /// Hardware Security Module
    Hardware,
/// Cryptographic operation types}


pub enum CryptoOperationType {
    /// Encrypt operation
    Encrypt,
    /// Decrypt operation
    Decrypt,
    /// Sign operation
    Sign,
    /// Verify operation
    Verify,
    /// Key derivation
    Derive,
    /// Key unwrapping
    Unwrap,
    /// Key wrapping
    Wrap,
