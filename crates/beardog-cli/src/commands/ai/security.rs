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


/// # AI CLI Security Operations
///
/// **EXTRACTED FROM LARGE FILE** - Security operations and handlers (~150 lines)
/// This module contains security-related CLI operations including encryption,
/// decryption, signing, and verification.
use super::types::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

/// Security operations
#[derive(Debug, Subcommand)]
pub enum SecurityOperation {
    /// Encrypt data
    Encrypt {
        /// Input data or file
        #[arg(long)]
        input: String,
        /// Key ID
        key_id: String,
        /// Output file
        output: Option<PathBuf>,
        /// Encryption algorithm
        #[arg(long, default_value = "AES256")]
        algorithm: String,
    },
    /// Decrypt data
    Decrypt {
        /// Decryption algorithm
        #[arg(long, value_enum)]
        algorithm: Option<String>,
    /// Sign data
    Sign {
        /// Signing key ID
        /// Output signature file
        /// Signature algorithm
        #[arg(long, default_value = "RSA_PSS")]
        /// Hash algorithm
        #[arg(long, default_value = "SHA256")]
        hash: String,
    /// Verify signature
    Verify {
        /// Signature file
        signature: String,
        /// Verification key ID
    /// Generate key pair
    GenerateKey {
        /// Key type
        #[arg(long, value_enum, default_value = "rsa2048")]
        key_type: String,
        /// Key usage
        #[arg(long, default_value = "general")]
        usage: String,
        /// Key ID (optional, auto-generated if not provided)
        key_id: Option<String>,
        /// Export public key to file
        export_public: Option<PathBuf>,
        /// Key attributes
        attributes: Vec<String>,
    /// List available keys
    ListKeys {
        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
        /// Filter by key type
        key_type: Option<String>,
        /// Filter by usage
        usage: Option<String>,
        /// Show key details
        detailed: bool,
    /// Delete key
    DeleteKey {
        /// Key ID to delete
        /// Force deletion without confirmation
        force: bool,
    /// Export key
    ExportKey {
        /// Key ID to export
        output: PathBuf,
        /// Export format
        #[arg(long, default_value = "PEM")]
        format: String,
        /// Include private key (if available)
        include_private: bool,
    /// Import key
    ImportKey {
        /// Input file
        input: PathBuf,
        /// Key ID for imported key
    /// Audit security operations
    Audit {
        /// Start date filter (ISO 8601)
        from: Option<String>,
        /// End date filter (ISO 8601)
        to: Option<String>,
        /// Filter by operation type
        operation: Option<String>,
        /// Filter by key ID
        /// Show detailed audit information
}
