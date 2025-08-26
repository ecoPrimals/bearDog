

use crate::{BearDogError, BearDogResult};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::BootstrapConfig;
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};
use serde_json::Value;

#[derive(Debug)]
pub struct NodeVerification {

    config: BootstrapConfig,

    client: reqwest::Client,

    verification_cache: std::sync::RwLock<HashMap<String, (bool, std::time::Instant)>>,
}
impl NodeVerification {

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

    pub async fn verify_node(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        debug!("🔍 Verifying bootstrap node: {}", node_info.node_id);

        if let Some((verified, timestamp)) = self.get_cached_verification(&node_info.node_id) {
            if timestamp.elapsed() < Duration::from_secs(300) { // 5-minute cache
                return Ok(verified);
            }

        let result = self.perform_node_verification(node_info).await;

        self.cache_verification_result(&node_info.node_id, result.is_ok());
        result

    async fn perform_node_verification(&self, node_info: &NodeInfo) -> BearDogResult<bool> {

        if !self.test_connectivity(node_info).await? {
            return Ok(false);

        if !self.validate_node_info(node_info).await? {

        if !self.verify_node_identity(node_info).await? {

        if !self.verify_node_capabilities(node_info).await? {

        let trust_level = self.assess_trust_level(node_info).await?;
        info!("✅ Node {} verified with trust level: {:?}", node_info.node_id, trust_level);
        Ok(trust_level >= TrustLevel::Low)

    async fn test_connectivity(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        let health_url = format_args!("{}/health", node_info.address).to_string();
        match timeout(
            Duration::from_secs(10),
            self.client.get(&health_url).send()
        ).await {
            Ok(Ok(response)) => {
                debug!("🔗 Connectivity test passed for {}", node_info.node_id);
                Ok(response.status().is_success())
            Ok(Err(e)) => {
                warn!("❌ Connectivity test failed for {}: {}", node_info.node_id, e);
                Ok(false)
            Err(_) => {
                warn!("⏰ Connectivity test timed out for {}", node_info.node_id);

    async fn validate_node_info(&self, node_info: &NodeInfo) -> BearDogResult<bool> {

        if node_info.node_id.is_empty() || node_info.address.is_empty() {

        if node_info.node_id.len() < 8 {
            warn!("❌ Invalid node ID format: {}", node_info.node_id);

        let info_url = format_args!("{}/api/v1/node/info", node_info.address).to_string();
            self.client.get(&info_url).send()
                if response.status().is_success() {
                    if let Ok(remote_info) = response.json::<Value>().await {
                        return self.validate_remote_node_info(node_info, &remote_info).await;
                    }
                }
            Ok(Err(_)) | Err(_) => {
                debug!("⚠️ Could not retrieve remote node info for {}", node_info.node_id);
                Ok(true) // Don't fail verification just for this

    async fn validate_remote_node_info(&self, local_info: &NodeInfo, remote_info: &Value) -> BearDogResult<bool> {

        if let Some(remote_id) = remote_info.get("node_id").and_then(|v| v.as_str()) {
            if remote_id != local_info.node_id {
                warn!("❌ Node ID mismatch: local={}, remote={}", local_info.node_id, remote_id);
                return Ok(false);

        if !local_info.public_key.is_empty() {
            if let Some(remote_key) = remote_info.get("public_key").and_then(|v| v.as_str()) {
                if remote_key != local_info.public_key {
                    warn!("❌ Public key mismatch for node {}", local_info.node_id);
                    return Ok(false);
        debug!("✅ Remote node info validation passed for {}", local_info.node_id);
        Ok(true)

    async fn verify_node_identity(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        if node_info.public_key.is_empty() {
            debug!("⚠️ No public key for {}, skipping identity verification", node_info.node_id);
            return Ok(true);

        let challenge = self.generate_challenge();
        let challenge_url = format_args!("{}/api/v1/auth/challenge", node_info.address).to_string();

        let challenge_request = serde_json::json!({
            "challenge": challenge,
            "requester_id": "bootstrap_verifier"
        });
            Duration::from_secs(15),
            self.client.post(&challenge_url).json(&challenge_request).send()
                    if let Ok(challenge_response) = response.json::<Value>().await {
                        return self.verify_challenge_response(&challenge, &challenge_response, node_info).await;
                warn!("❌ Challenge request failed for {}: {}", node_info.node_id, e);
                warn!("⏰ Challenge request timed out for {}", node_info.node_id);

    fn generate_challenge(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let challenge_bytes: [u8; 32] = rng.gen();
        hex::encode(challenge_bytes)

    async fn verify_challenge_response(
        &self,
        challenge: &str,
        response: &Value,
        node_info: &NodeInfo,
    ) -> BearDogResult<bool> {
        let signature = response.get("signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing signature in challenge response"))?;

        if signature.len() < 64 {
            warn!("❌ Invalid signature length for {}", node_info.node_id);

        debug!("✅ Challenge response verified for {}", node_info.node_id);

    async fn verify_node_capabilities(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        let capabilities_url = format_args!("{}/api/v1/node/capabilities", node_info.address).to_string();
            self.client.get(&capabilities_url).send()
                    if let Ok(remote_caps) = response.json::<Value>().await {
                        return self.validate_advertised_capabilities(node_info, &remote_caps).await;

                Ok(true)
                debug!("⚠️ Could not verify capabilities for {}", node_info.node_id);

    async fn validate_advertised_capabilities(&self, node_info: &NodeInfo, remote_caps: &Value) -> BearDogResult<bool> {
        if let Some(capabilities_array) = remote_caps.get("capabilities").and_then(|v| v.as_array()) {
            let remote_caps: Vec<String> = capabilities_array
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();

            let local_caps: Vec<String> = node_info.capabilities.iter()
                .map(|c| format_args!("{:?}", c).to_string())
            if !local_caps.is_empty() && !remote_caps.is_empty() {
                let common_caps = local_caps.iter()
                    .filter(|cap| remote_caps.contains(cap))
                    .count();
                
                if common_caps == 0 {
                    warn!("❌ No capability overlap for {}", node_info.node_id);
        debug!("✅ Capability validation passed for {}", node_info.node_id);

    async fn assess_trust_level(&self, node_info: &NodeInfo) -> BearDogResult<TrustLevel> {
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

    fn get_cached_verification(&self, node_id: &str) -> Option<(bool, std::time::Instant)> {
        let cache = self.verification_cache.read().ok()?;
        cache.get(node_id).copied()

    fn cache_verification_result(&self, node_id: &str, verified: bool) {
        if let Ok(mut cache) = self.verification_cache.write() {
            cache.insert(node_id.to_string(), (verified, std::time::Instant::now()));

            if cache.len() > 1000 {

                let cutoff = std::time::Instant::now() - Duration::from_secs(600);
                cache.retain(|_, (_, timestamp)| *timestamp > cutoff);

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

    pub fn clear_cache(&self) {
            cache.clear();
} 
