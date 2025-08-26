

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEncryptRequest {
    pub data: String,
}

pub struct AIEncryptResponse {
    pub encrypted_data: String,
    pub success: bool,

pub struct AIDecryptRequest {

pub struct AIDecryptResponse {
    pub decrypted_data: String,

pub struct AISignRequest {

pub struct AISignResponse {
    pub signature: String,

pub struct AIVerifyRequest {

pub struct AIVerifyResponse {
    pub valid: bool,

pub struct AIGenerateKeyRequest {
    pub key_type: String,

pub struct AIGenerateKeyResponse {
    pub public_key: String,

pub enum AISecurityOperation {

    Encrypt { data: String },

    Decrypt { encrypted_data: String },

    Sign { data: String },

    Verify { data: String, signature: String },

    GenerateKey { key_type: String },

    DeriveKey { password: String, salt: String },

    Hash { data: String, algorithm: String },

    Random { length: usize },

    ValidateCertificate { certificate: String },

    CreateCertificate { subject: String },
