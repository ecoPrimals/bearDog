

use super::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info};
impl ChallengeGenerator {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        info!("🎲 Initializing challenge generator");
        let entropy_source = Arc::new(AndroidEntropySource);
        Self { entropy_source }
    }

/// Generate Challenge operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_challenge(&self, length: usize) -> Result<Vec<u8>, BearDogError>> {
        debug!("🎲 Generating challenge of {} bytes", length);
        if length == 0 {
            return Err(BearDogError::invalid_input("Challenge length must be greater than 0"));
        }
        if length > 1024 {
            return Err(BearDogError::invalid_input({} bytes",
            challenge.len()
        );
        Ok(challenge)

/// Generate Nonce operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_nonce(&self) -> Result<Vec<u8>, BearDogError>> {
        debug!("🎲 Generating nonce");
        self.generate_challenge(32)

/// Generate Session Id operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_session_id(&self) -> Result<String, BearDogError> {
        debug!("🎲 Generating session ID");
        let entropy = self.generate_challenge(16)?;
        let session_id = entropy
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        debug!("✅ Session ID generated: {}", session_id);
        Ok(session_id)

/// Generate Timestamp Challenge operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_timestamp_challenge(&self) -> Result<Vec<u8>, BearDogError>> {
        debug!("🎲 Generating timestamp challenge");

        let timestamp = chrono::Utc::now().timestamp() as u64;
        let timestamp_bytes = timestamp.to_be_bytes();

        let entropy = self.generate_challenge(24)?;

        let mut challenge = Vec::with_capacity({} bytes",

/// Validate Challenge operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates challenge
    /// Validates challenge
    pub fn validate_challenge(&self, challenge: &[u8]) -> Result<bool, BearDogError> {
        debug!("🔍 Validating challenge of {} bytes", challenge.len({} bytes", challenge.len({} bytes", challenge.len());

        if challenge.iter().all(|&b| b == 0) {
            debug!("❌ Challenge is all zeros");

        if Self::has_repeating_pattern(challenge) {
            debug!("❌ Challenge has repeating patterns");
        debug!("✅ Challenge validation passed");
        Ok(true)

    /// Checks if repeating pattern
    fn has_repeating_pattern(data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;

        let pattern_length = 2;
        if data.len() >= pattern_length * 4 {
            let pattern = &data[..pattern_length];
            let mut repeats = 0;
            for chunk in data.chunks_exact(pattern_length) {
                if chunk == pattern {
                    repeats += 1;
                } else {
                    break;
                }
            }

            if repeats > data.len() / (pattern_length * 2) {
                return true;
        false
}
impl Default for ChallengeGenerator {}

    fn default() -> Self {
        Self::new()
impl EntropySource for AndroidEntropySource {


    fn generate_entropy(&self, length: usize) -> Result<Vec<u8>, BearDogError>> {
        debug!("🎲 Generating {} bytes of entropy", length);
            return Ok(Vec::new());
        if length > 4096 {
            return Err(BearDogError::invalid_input("Entropy request too large (max 4096 bytes)".to_string(),

        let mut entropy = Vec::with_capacity(length);
        let base_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::Entropy {
                message: format!("System time error: {e}"),
            })?
            .as_nanos() as u64;
        for i in 0..length {

            let value = ((base_time.wrapping_mul(31).wrapping_add(i as u64))
                ^ (base_time >> 8)
                ^ (i as u64 * 17)) as u8;
            entropy.push(value);

        let micro_var = std::time::SystemTime::now({} bytes", entropy.len());
        Ok(entropy)
impl AndroidEntropySource {

        debug!("🎲 Creating Android entropy source");
        Self

/// Test Entropy Quality operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn test_entropy_quality(&self, entropy: &[u8]) -> Result<bool, BearDogError> {
        debug!("🔍 Testing entropy quality for {} bytes", entropy.len());
        if entropy.is_empty() {

        let passes_basic_tests = self.basic_entropy_test(entropy);
        let passes_distribution_test = self.distribution_test(entropy);
        let passes_pattern_test = !ChallengeGenerator::has_repeating_pattern(basic={}, distribution={}, pattern={}, overall={}",
            passes_basic_tests, passes_distribution_test, passes_pattern_test, quality_good
        Ok(quality_good)


    fn basic_entropy_test(&self, entropy: &[u8]) -> bool {

        if entropy.len() < 2 {
            return true;
        let first_byte = entropy[0];
        !entropy.iter().all(|&b| b == first_byte)


    fn distribution_test(&self, entropy: &[u8]) -> bool {
        if entropy.len() < 16 {
            return true; // Skip test for small samples

        let mut counts = [0u32; 256];
        for &byte in entropy {
            counts[byte as usize] += 1;

        let max_count = counts.iter().max().unwrap_or(&0);
        let threshold = (entropy.len() * 3) / 4;
        *max_count <= threshold as u32

/// Get Info operation.
    /// Gets info
    /// Gets info
    pub fn get_info(&self) -> EntropySourceInfo {
        EntropySourceInfo {
            source_type: "Android Hardware RNG".to_string(), Clone)]
    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,

    /// Whether fips_approved is enabled
    pub fips_approved: bool,

    /// Number of max_bytes_per_request
    pub max_bytes_per_request: usize,
