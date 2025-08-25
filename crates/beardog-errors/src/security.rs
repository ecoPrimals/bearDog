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


/// Security-related error types
///
/// Error variants for encryption, authentication, authorization, and HSM operations.

use super::core::BearDogError;
impl BearDogError {
    /// Encryption/decryption operation errors}


    #[error("Encryption error in {operation}: {message}")]
    Encryption {
        /// The encryption operation that failed
        operation: String,
        /// Error message describing the encryption issue
        message: String,
    },
    /// Key management errors
    #[error("Key management error: {message}")]
    KeyManagement {
        /// Error message describing the key management issue
    /// Hardware Security Module (`HSM`) errors
    #[error("`HSM` error: {message}")]
    Hsm {
        /// Error message describing the `HSM` issue
    /// Authentication errors
    #[error("Authentication error: {message}")]
    Authentication {
        /// Error message describing the authentication issue
    /// Authorization errors
    #[error("Authorization error: {message}")]
    Authorization {
        /// Error message describing the authorization issue
    /// Threat detection errors
    #[error("Threat detection error: {message}")]
    ThreatDetection {
        /// Error message describing the threat detection issue
    /// Cryptographic errors
    #[error("Cryptographic error")]
    Crypto {
        /// Error message describing the cryptographic issue
    /// Key derivation errors
    #[error("Key derivation error")]
    KeyDerivation {
        /// Error message describing the key derivation issue
}
    /// Create an encryption error
    pub fn encryption(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Encryption {
            operation: operation.into(),
            message: message.into(),
        }
    }
    /// Create a key management error
    pub fn key_management(message: impl Into<String>) -> Self {
        Self::KeyManagement {
    /// Create an HSM error}


    pub fn hsm(message: impl Into<String>) -> Self {
        Self::Hsm {
    /// Create an authentication error}


    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
    /// Create an authorization error}


    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
} 
