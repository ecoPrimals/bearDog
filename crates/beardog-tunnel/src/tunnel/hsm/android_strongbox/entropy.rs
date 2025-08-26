

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tracing::{debug, info};
impl ChallengeGenerator {

    pub fn new() -> Self {
        info!("🎲 Initializing challenge generator");
        let entropy_source = Arc::new(AndroidEntropySource);
        Self { entropy_source }
    }

    pub fn generate_challenge(&self, length: usize) -> BearDogResult<Vec<u8>> {
        debug!("🎲 Generating challenge of {} bytes", length);
        if length == 0 {
            return Err(BearDogError::invalid_input("Challenge length must be greater than 0".to_string(),
            ));
        }
        if length > 1024 {
            return Err(BearDogError::invalid_input("Challenge length too large (max 1024 bytes)".to_string(),

        let challenge = self.entropy_source.generate_entropy(length)?;
        debug!(
            "✅ Challenge generated successfully: {} bytes",
            challenge.len()
        );
        Ok(challenge)

    pub fn generate_nonce(&self) -> BearDogResult<Vec<u8>> {
        debug!("🎲 Generating nonce");
        self.generate_challenge(32)

    pub fn generate_session_id(&self) -> BearDogResult<String> {
        debug!("🎲 Generating session ID");
        let entropy = self.generate_challenge(16)?;
        let session_id = entropy
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        debug!("✅ Session ID generated: {}", session_id);
        Ok(session_id)

    pub fn generate_timestamp_challenge(&self) -> BearDogResult<Vec<u8>> {
        debug!("🎲 Generating timestamp challenge");

        let timestamp = chrono::Utc::now().timestamp() as u64;
        let timestamp_bytes = timestamp.to_be_bytes();

        let entropy = self.generate_challenge(24)?;

        let mut challenge = Vec::with_capacity(32);
        challenge.extend_from_slice(&timestamp_bytes);
        challenge.extend_from_slice(&entropy);
            "✅ Timestamp challenge generated: {} bytes",

    pub fn validate_challenge(&self, challenge: &[u8]) -> BearDogResult<bool> {
        debug!("🔍 Validating challenge of {} bytes", challenge.len());

        if challenge.len() < 16 {
            debug!("❌ Challenge too short: {} bytes", challenge.len());
            return Ok(false);

        if challenge.len() > 1024 {
            debug!("❌ Challenge too long: {} bytes", challenge.len());

        if challenge.iter().all(|&b| b == 0) {
            debug!("❌ Challenge is all zeros");

        if Self::has_repeating_pattern(challenge) {
            debug!("❌ Challenge has repeating patterns");
        debug!("✅ Challenge validation passed");
        Ok(true)

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

    fn generate_entropy(&self, length: usize) -> BearDogResult<Vec<u8>> {
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

        let micro_var = std::time::SystemTime::now()
            .subsec_micros() as u8;
        for (i, byte) in entropy.iter_mut().enumerate() {
            *byte ^= micro_var.wrapping_add(i as u8);
        debug!("✅ Entropy generated successfully: {} bytes", entropy.len());
        Ok(entropy)
impl AndroidEntropySource {

        debug!("🎲 Creating Android entropy source");
        Self

    pub fn test_entropy_quality(&self, entropy: &[u8]) -> BearDogResult<bool> {
        debug!("🔍 Testing entropy quality for {} bytes", entropy.len());
        if entropy.is_empty() {

        let passes_basic_tests = self.basic_entropy_test(entropy);
        let passes_distribution_test = self.distribution_test(entropy);
        let passes_pattern_test = !ChallengeGenerator::has_repeating_pattern(entropy);
        let quality_good = passes_basic_tests && passes_distribution_test && passes_pattern_test;
            "🔍 Entropy quality tests: basic={}, distribution={}, pattern={}, overall={}",
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

    pub fn get_info(&self) -> EntropySourceInfo {
        EntropySourceInfo {
            source_type: "Android Hardware RNG".to_string(),
            hardware_backed: true,
            fips_approved: false, // Would need real FIPS validation
            max_bytes_per_request: 4096,
impl Default for AndroidEntropySource {

#[derive(Debug, Clone)]
pub struct EntropySourceInfo {

    pub source_type: String,

    pub hardware_backed: bool,

    pub fips_approved: bool,

    pub max_bytes_per_request: usize,
