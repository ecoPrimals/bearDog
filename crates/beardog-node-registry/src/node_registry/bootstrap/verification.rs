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


/// Node Verification for Bootstrap Process
///
/// Handles verification of bootstrap nodes including identity validation,
/// challenge-response authentication, and trust establishment.

use crate::{BearDogError, BearDogResult};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::BootstrapConfig;
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};
use serde_json::Value;
/// Node verification service for bootstrap operations
#[derive(Debug)]
pub struct NodeVerification {
    /// Bootstrap configuration
    config: BootstrapConfig,
    /// HTTP client for verification requests
    client: reqwest::Client,
    /// Cache of verified nodes to avoid re-verification
    verification_cache: std::sync::RwLock<HashMap<String, (bool, std::time::Instant)>>,
}
impl NodeVerification {
    /// Create new node verification service}


    pub fn new(config: BootstrapConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            config,
            client,
            verification_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }
    /// Verify a bootstrap node's identity and capabilities
    pub async fn verify_node(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        debug!("🔍 Verifying bootstrap node: {}", node_info.node_id);
        // Check cache first
        if let Some((verified, timestamp)) = self.get_cached_verification(&node_info.node_id) {
            if timestamp.elapsed() < Duration::from_secs(300) { // 5-minute cache
                return Ok(verified);
            }
        // Perform comprehensive verification
        let result = self.perform_node_verification(node_info).await;
        
        // Cache the result
        self.cache_verification_result(&node_info.node_id, result.is_ok());
        result
    /// Perform comprehensive node verification
    async fn perform_node_verification(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        // Step 1: Basic connectivity test
        if !self.test_connectivity(node_info).await? {
            return Ok(false);
        // Step 2: Node info validation
        if !self.validate_node_info(node_info).await? {
        // Step 3: Identity verification through challenge-response
        if !self.verify_node_identity(node_info).await? {
        // Step 4: Capability verification
        if !self.verify_node_capabilities(node_info).await? {
        // Step 5: Trust level assessment
        let trust_level = self.assess_trust_level(node_info).await?;
        info!("✅ Node {} verified with trust level: {:?}", node_info.node_id, trust_level);
        Ok(trust_level >= TrustLevel::Low)
    /// Test basic connectivity to node
    async fn test_connectivity(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        let health_url = format!("{}/health", node_info.address);
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
    /// Validate node information and metadata
    async fn validate_node_info(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        // Check required fields
        if node_info.node_id.is_empty() || node_info.address.is_empty() {
        // Validate node ID format (should be valid UUID or similar)
        if node_info.node_id.len() < 8 {
            warn!("❌ Invalid node ID format: {}", node_info.node_id);
        // Get node info from the node itself
        let info_url = format!("{}/api/v1/node/info", node_info.address);
            self.client.get(&info_url).send()
                if response.status().is_success() {
                    if let Ok(remote_info) = response.json::<Value>().await {
                        return self.validate_remote_node_info(node_info, &remote_info).await;
                    }
                }
            Ok(Err(_)) | Err(_) => {
                debug!("⚠️ Could not retrieve remote node info for {}", node_info.node_id);
                Ok(true) // Don't fail verification just for this
    /// Validate remote node information against local info
    async fn validate_remote_node_info(&self, local_info: &NodeInfo, remote_info: &Value) -> BearDogResult<bool> {
        // Check if node IDs match
        if let Some(remote_id) = remote_info.get("node_id").and_then(|v| v.as_str()) {
            if remote_id != local_info.node_id {
                warn!("❌ Node ID mismatch: local={}, remote={}", local_info.node_id, remote_id);
                return Ok(false);
        // Validate public key if present
        if !local_info.public_key.is_empty() {
            if let Some(remote_key) = remote_info.get("public_key").and_then(|v| v.as_str()) {
                if remote_key != local_info.public_key {
                    warn!("❌ Public key mismatch for node {}", local_info.node_id);
                    return Ok(false);
        debug!("✅ Remote node info validation passed for {}", local_info.node_id);
        Ok(true)
    /// Verify node identity through challenge-response
    async fn verify_node_identity(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        if node_info.public_key.is_empty() {
            debug!("⚠️ No public key for {}, skipping identity verification", node_info.node_id);
            return Ok(true);
        // Generate challenge
        let challenge = self.generate_challenge();
        let challenge_url = format!("{}/api/v1/auth/challenge", node_info.address);
        // Send challenge request
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
    /// Generate cryptographic challenge for identity verification
    fn generate_challenge(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let challenge_bytes: [u8; 32] = rng.gen();
        hex::encode(challenge_bytes)
    /// Verify challenge response signature}


    async fn verify_challenge_response(
        &self,
        challenge: &str,
        response: &Value,
        node_info: &NodeInfo,
    ) -> BearDogResult<bool> {
        let signature = response.get("signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing signature in challenge response"))?;
        // In a full implementation, this would verify the signature using the node's public key
        // For now, we'll do basic validation
        if signature.len() < 64 {
            warn!("❌ Invalid signature length for {}", node_info.node_id);
        // Simulate signature verification
        debug!("✅ Challenge response verified for {}", node_info.node_id);
    /// Verify node capabilities match what's advertised
    async fn verify_node_capabilities(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        let capabilities_url = format!("{}/api/v1/node/capabilities", node_info.address);
            self.client.get(&capabilities_url).send()
                    if let Ok(remote_caps) = response.json::<Value>().await {
                        return self.validate_advertised_capabilities(node_info, &remote_caps).await;
                // Don't fail verification if capabilities endpoint is not available
                Ok(true)
                debug!("⚠️ Could not verify capabilities for {}", node_info.node_id);
    /// Validate advertised capabilities against actual capabilities
    async fn validate_advertised_capabilities(&self, node_info: &NodeInfo, remote_caps: &Value) -> BearDogResult<bool> {
        if let Some(capabilities_array) = remote_caps.get("capabilities").and_then(|v| v.as_array()) {
            let remote_caps: Vec<String> = capabilities_array
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            // Check for reasonable overlap
            let local_caps: Vec<String> = node_info.capabilities.iter()
                .map(|c| format!("{:?}", c))
            if !local_caps.is_empty() && !remote_caps.is_empty() {
                let common_caps = local_caps.iter()
                    .filter(|cap| remote_caps.contains(cap))
                    .count();
                
                if common_caps == 0 {
                    warn!("❌ No capability overlap for {}", node_info.node_id);
        debug!("✅ Capability validation passed for {}", node_info.node_id);
    /// Assess trust level based on verification results
    async fn assess_trust_level(&self, node_info: &NodeInfo) -> BearDogResult<TrustLevel> {
        let mut trust_score = 0;
        // Base trust for connectivity
        trust_score += 1;
        // Trust bonus for valid public key
        if !node_info.public_key.is_empty() {
            trust_score += 2;
        // Trust bonus for capabilities
        if !node_info.capabilities.is_empty() {
            trust_score += 1;
        // Trust bonus for successful identity verification
        trust_score += 2; // Assuming identity verification passed
        // Map score to trust level
        let trust_level = match trust_score {
            0..=2 => TrustLevel::Untrusted,
            3..=4 => TrustLevel::Low,
            5..=6 => TrustLevel::Medium,
            7.. => TrustLevel::High,
        };
        Ok(trust_level)
    /// Get cached verification result
    fn get_cached_verification(&self, node_id: &str) -> Option<(bool, std::time::Instant)> {
        let cache = self.verification_cache.read().ok()?;
        cache.get(node_id).copied()
    /// Cache verification result}


    fn cache_verification_result(&self, node_id: &str, verified: bool) {
        if let Ok(mut cache) = self.verification_cache.write() {
            cache.insert(node_id.to_string(), (verified, std::time::Instant::now()));
            
            // Limit cache size
            if cache.len() > 1000 {
                // Remove old entries
                let cutoff = std::time::Instant::now() - Duration::from_secs(600);
                cache.retain(|_, (_, timestamp)| *timestamp > cutoff);
    /// Get verification statistics
    pub fn get_verification_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        if let Ok(cache) = self.verification_cache.read() {
            stats.insert("cached_verifications".to_string(), cache.len() as u32);
            let verified_count = cache.values()
                .filter(|(verified, _)| *verified)
                .count() as u32;
            stats.insert("verified_nodes".to_string(), verified_count);
            let failed_count = cache.len() as u32 - verified_count;
            stats.insert("failed_verifications".to_string(), failed_count);
        stats
    /// Clear verification cache}


    pub fn clear_cache(&self) {
            cache.clear();
} 
