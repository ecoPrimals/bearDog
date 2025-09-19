use beardog_errors::BearDogError;

use crate::{{BearDogError}};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::BootstrapConfig;
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};
use serde_json::Value;

#[derive(Debug, Clone)]
    client: reqwest::Client,

    verification_cache: std::sync::RwLock<HashMap<String, (bool, std::time::Instant)>>,
}
impl NodeVerification {

/// New operation.
    /// Creates a new instance
    pub fn new(config: BootstrapConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            config,
            client,
            verification_cache: std::sync::RwLock::new(HashMap::with_capacity(16)),
        }
    }

/// Verify Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn verify_node(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {
        debug!("🔍 Verifying bootstrap node: {}", node_info.node_id);

        if let Some((verified, timestamp)) = self.get_cached_verification(&node_info.node_id) {
            if timestamp.elapsed() < Duration::from_secs(300) { // 5-minute cache
                return Ok(verified);
            }

        let result = self.perform_node_verification(node_info);

        self.cache_verification_result(&node_info.node_id, result.is_ok());
        result


    fn perform_node_verification(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {

        if !self.test_connectivity({:?}", node_info.node_id, trust_level);
        Ok(trust_level >= TrustLevel::Low)


    fn test_connectivity(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {
        let health_url = format!("{}/health", node_info.address);
        match timeout(
            Duration::from_secs({}", node_info.node_id, e);
                Ok(false)
            Err(_) => {
                warn!("⏰ Connectivity test timed out for {}", node_info.node_id);

    /// Validates node_info
    fn validate_node_info(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {

        if node_info.node_id.is_empty({}", node_info.node_id);

        let info_url = format!("{}/api/v1/node/info", node_info.address);
            self.client.get(&info_url).send()
                if response.status().is_success() {
                    if let Ok(remote_info) = response.json::<Value>() {
                        return self.validate_remote_node_info(&NodeInfo, remote_info: &Value) -> Result<bool, BearDogError> {

        if let Some(local={}, remote={}", local_info.node_id, remote_id);
                return Ok(false);

        if !local_info.public_key.is_empty() {
            if let Some(remote_key) = remote_info.get("public_key").and_then(|v| v.as_str()) {
                if remote_key != local_info.public_key {
                    warn!("❌ Public key mismatch for node {}", local_info.node_id);
                    return Ok(false);
        debug!("✅ Remote node info validation passed for {}", local_info.node_id);
        Ok(true)


    fn verify_node_identity(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {
        if node_info.public_key.is_empty() {
            debug!("⚠️ No public key for {}, skipping identity verification", node_info.node_id);
            return Ok(true);

        let challenge = self.generate_challenge();
        let challenge_url = format!("{}/api/v1/auth/challenge", node_info.address);

        let challenge_request = serde_json::json!({
            "challenge": challenge,
            "requester_id": "bootstrap_verifier"
        });
            Duration::from_secs(15),
            self.client.post(&challenge_url).json(&challenge_request).send()
                    if let Ok(challenge_response) = response.json::<Value>() {
                        return self.verify_challenge_response({}", node_info.node_id, e);
                warn!("⏰ Challenge request timed out for {}", node_info.node_id);


    fn generate_challenge(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let challenge_bytes: [u8; 32] = rng.gen();
        hex::encode(&str,
        response: &Value,
        node_info: &NodeInfo,
    ) -> Result<bool, BearDogError> {
        let signature = response.get("signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing signature in challenge response"))?;

        if signature.len() < 64 {
            warn!("❌ Invalid signature length for {}", node_info.node_id);

        debug!("✅ Challenge response verified for {}", node_info.node_id);


    fn verify_node_capabilities(&self, node_info: &NodeInfo) -> Result<bool, BearDogError> {
        let capabilities_url = format!("{}/api/v1/node/capabilities", node_info.address);
            self.client.get(&capabilities_url).send()
                    if let Ok(remote_caps) = response.json::<Value>() {
                        return self.validate_advertised_capabilities(&NodeInfo, remote_caps: &Value) -> Result<bool, BearDogError> {
        if let Some(capabilities_array) = remote_caps.get("capabilities").and_then(|v| v.as_array()) {
            let remote_caps: Vec<String> = capabilities_array
                .iter()
                .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
                .collect();

            let local_caps: Vec<String> = node_info.capabilities.iter()
                .map(|c| format!("{:?}", c))
            if !local_caps.is_empty() && !remote_caps.is_empty() {
                let common_caps = local_caps.iter()
                    .filter(|cap| remote_caps.contains(cap))
                    .count();
                
                if common_caps == 0 {
                    warn!("❌ No capability overlap for {}", node_info.node_id);
        debug!("✅ Capability validation passed for {}", node_info.node_id);


    fn assess_trust_level(&self, node_info: &NodeInfo) -> Result<TrustLevel, BearDogError> {
        let mut trust_score = 0;

        trust_score += 1;

        if !node_info.public_key.is_empty() {
            trust_score += 2;

        if !node_info.capabilities.is_empty() {
            trust_score += 1;

        trust_score += 2; // Assuming identity verification passed

        let trust_level = match trust_score {
            0..=2 => TrustLevel::Untrusted,
            3..=4 => TrustLevel::Low,
            5..=6 => TrustLevel::Medium,
            7.. => TrustLevel::High,
        };
        Ok(trust_level)

    /// Gets cached_verification
    fn get_cached_verification(&self, node_id: &str) -> Option<(bool, std::time::Instant)> {
        let cache = self.verification_cache.read(&str, verified: bool) {
        if let Ok(mut cache) = self.verification_cache.write() {
            cache.insert(node_id.to_string(), (verified, std::time::Instant::now()));

            if cache.len() > 1000 {

                let cutoff = std::time::Instant::now() - Duration::from_secs(600);
                cache.retain(|_, (_, timestamp)| *timestamp > cutoff);

/// Get Verification Stats operation.
    /// Gets verification_stats
    /// Gets verification_stats
    pub fn get_verification_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::with_capacity(16);
        if let Ok(cache) = self.verification_cache.read() {
            stats.insert("cached_verifications".to_string(), cache.len() as u32);
            let verified_count = cache.values()
                .filter(|(verified, _)| *verified)
                .count() as u32;
            stats.insert("verified_nodes".to_string(), verified_count);
            let failed_count = cache.len() as u32 - verified_count;
            stats.insert("failed_verifications".to_string(), failed_count);
        stats

/// Clear Cache operation.
    pub fn clear_cache(&self) {
            cache.clear();
} 
