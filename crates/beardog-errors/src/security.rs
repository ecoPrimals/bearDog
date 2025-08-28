use beardog_errors::BearDogError;

use super::core::BearDogError;
impl BearDogError {

    #[error("Encryption error in {operation}: {message}")]
    Encryption {

        operation: String,

        message: String,
    },

    #[error("Key management error: {message}")]
    KeyManagement {

    #[error("`HSM` error: {message}")]
    Hsm {

    #[error("Authentication error: {message}")]
    Authentication {

    #[error("Authorization error: {message}")]
    Authorization {

    #[error("Threat detection error: {message}")]
    ThreatDetection {

    #[error("Cryptographic error")]
    Crypto {

    #[error("Key derivation error")]
    KeyDerivation {

}

    pub fn encryption(operation: impl Into<&str>, message: impl Into<&str>) -> Self {
        Self::Encryption {
            operation: operation.into(),
            message: message.into(),
        }
    }

    pub fn key_management(message: impl Into<&str>) -> Self {
        Self::KeyManagement {

    pub fn hsm(message: impl Into<&str>) -> Self {
        Self::Hsm {

    pub fn authentication(message: impl Into<&str>) -> Self {
        Self::Authentication {

    pub fn authorization(message: impl Into<&str>) -> Self {
        Self::Authorization {
} 
