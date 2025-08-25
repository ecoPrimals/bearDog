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


/// Security operations for AI interface

use serde::{Deserialize, Serialize};
/// AI Encryption Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEncryptRequest {
    pub data: String,
}
/// AI Encryption Response
pub struct AIEncryptResponse {
    pub encrypted_data: String,
    pub success: bool,
/// AI Decryption Request
pub struct AIDecryptRequest {
/// AI Decryption Response
pub struct AIDecryptResponse {
    pub decrypted_data: String,
/// AI Signature Request
pub struct AISignRequest {
/// AI Signature Response
pub struct AISignResponse {
    pub signature: String,
/// AI Verification Request
pub struct AIVerifyRequest {
/// AI Verification Response
pub struct AIVerifyResponse {
    pub valid: bool,
/// AI Key Generation Request
pub struct AIGenerateKeyRequest {
    pub key_type: String,
/// AI Key Generation Response
pub struct AIGenerateKeyResponse {
    pub public_key: String,
/// Security operation types for AI
pub enum AISecurityOperation {
    /// Encrypt data
    Encrypt { data: String },
    /// Decrypt data
    Decrypt { encrypted_data: String },
    /// Sign data
    Sign { data: String },
    /// Verify signature
    Verify { data: String, signature: String },
    /// Generate key pair
    GenerateKey { key_type: String },
    /// Derive key from password
    DeriveKey { password: String, salt: String },
    /// Hash data
    Hash { data: String, algorithm: String },
    /// Generate random bytes
    Random { length: usize },
    /// Validate certificate
    ValidateCertificate { certificate: String },
    /// Create certificate
    CreateCertificate { subject: String },
