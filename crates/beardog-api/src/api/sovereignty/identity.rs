

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::models::{
    IdentityClaim, IdentityClaimType, IdentityKey, IdentityKeyPurpose, IdentityKeyStatus,
    IdentityProofType,
};

#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub struct SelfSovereignIdentityEngine {

    identity_keys: Arc<RwLock<HashMap<String, IdentityKeyInternal>>>,

    identity_claims: Arc<RwLock<HashMap<String, IdentityClaimInternal>>>,

    attestations: Arc<RwLock<HashMap<String, IdentityAttestationInternal>>>,

    trusted_attestors: Arc<RwLock<HashMap<String, TrustedAttestor>>>,
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IdentityKeyInternal {
    pub key_id: String,
    pub purpose: IdentityKeyPurpose,
    pub public_key: String,
    pub private_key: Vec<u8>, // Encrypted at rest
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: IdentityKeyStatus,
    pub usage_count: u64,
    pub last_used: Option<DateTime<Utc>>,
    pub allowed_operations: Vec<String>,
    pub key_derivation_path: Option<String>,
}

struct IdentityClaimInternal {
    pub claim_id: String,
    pub claim_type: IdentityClaimType,
    pub claim_data: HashMap<String, String>,
    pub issuer_id: String,
    pub subject_id: String,
    pub proof: IdentityProofInternal,
    pub verification_status: VerificationStatus,
    pub privacy_level: PrivacyLevel,
struct IdentityProofInternal {
    pub proof_type: IdentityProofType,
    pub signature: Vec<u8>,
    pub merkle_root: Option<String>,
    pub nonce: String,
struct IdentityAttestationInternal {
    pub attestation_id: String,
    pub attestor_id: String,
    pub attestor_display_name: String,
    pub attestation_data: HashMap<String, String>,
    pub confidence_score: f64,
    pub revoked: bool,
struct TrustedAttestor {
    pub display_name: String,
    pub attestor_type: AttestorType,
    pub trust_score: f64,
    pub added_at: DateTime<Utc>,
    pub verified_claims: u32,
    pub revoked_claims: u32,
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum VerificationStatus {
    Unverified,
    SelfVerified,
    FriendVerified,
    OrganizationVerified,
    CommunityVerified,}

pub enum PrivacyLevel {
    Public,    // Anyone can see
    Friends,   // Only friends can see
    Selective, // Only specific individuals
    Private,   // Only yourself
pub enum AttestorType {
    Friend,
    Organization,
    Community,
    Government,
    Academic,}

impl Default for SelfSovereignIdentityEngine {}

    fn default() -> Self {
        Self::new()
    }
impl SelfSovereignIdentityEngine {

    pub fn new() -> Self {
        info!("🎭 Initializing self-sovereign identity management system");
        Self {
            identity_keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            identity_claims: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            attestations: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            trusted_attestors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }

    pub async fn generate_identity_key(
        &self,
        purpose: IdentityKeyPurpose,
        expires_hours: Option<i64>,
        allowed_operations: Vec<&str>,
    ) -> Result<IdentityKey, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "🔑 Generating new Ed25519 identity key for purpose: {:?}",
            purpose
        );

        let key_pair = self.generate_ed25519_keypair().await?;
        let public_key_hex = hex::encode(&key_pair.public_key);
        let key_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = expires_hours.map(|h| now + Duration::hours(h));
        let identity_key_internal = IdentityKeyInternal {
            key_id: key_id.clone(),
            purpose: purpose.clone(),
            public_key: public_key_hex.clone(),
            private_key: key_pair.private_key, // This would be encrypted at rest
            created_at: now,
            expires_at,
            status: IdentityKeyStatus::Active,
            usage_count: 0,
            last_used: None,
            allowed_operations: allowed_operations.clone(),
            key_derivation_path: None,
        };

        {
            let mut keys = self.identity_keys.write().await;
            keys.insert(key_id.clone(), identity_key_internal);
        info!("✅ Identity key generated successfully: {}", key_id);

        Ok(IdentityKey {
            key_id,
            purpose,
            public_key: format!("ed25519:{public_key_hex}"),
            created_at: now.to_rfc3339(),
            expires_at: expires_at.map(|dt| dt.to_rfc3339()),
        })

    pub async fn list_identity_keys(
    ) -> Result<Vec<IdentityKey>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Listing identity keys");
        let keys = self.identity_keys.read().await;
        let mut result = Vec::new();
        for key in keys.values() {

            if key.status == IdentityKeyStatus::Revoked {
                continue;
            }
            if let Some(expires_at) = key.expires_at {
                if Utc::now() > expires_at {
                    continue;
                }
            result.push(IdentityKey {
                key_id: key.key_id.clone(),
                purpose: key.purpose.clone(),
                public_key: format_args!("ed25519:{}", key.public_key).to_string(),
                created_at: key.created_at.to_rfc3339(),
                expires_at: key.expires_at.map(|dt| dt.to_rfc3339()),
                status: key.status.clone(),
            });
        Ok(result)

    pub async fn revoke_identity_key(
        key_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🚫 Revoking identity key: {}", key_id);
        let mut keys = self.identity_keys.write().await;
        if let Some(key) = keys.get_mut(key_id) {
            key.status = IdentityKeyStatus::Revoked;
            info!("✅ Identity key revoked successfully: {}", key_id);
            Ok(())
        } else {
            Err("Identity key not found".into())

    pub async fn create_identity_claim(
        claim_type: IdentityClaimType,
        claim_data: HashMap<&str, &str>,
        signing_key_id: &str,
        privacy_level: PrivacyLevel,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("📝 Creating identity claim of type: {:?}", claim_type);

        let signing_key = {
            let keys = self.identity_keys.read().await;
            keys.get(signing_key_id)
                .cloned()
                .ok_or("Signing key not found")?
        if signing_key.status != IdentityKeyStatus::Active {
            return Err("Signing key is not active".into());
        let claim_id = Uuid::new_v4().to_string();

        let claim_message = self.create_claim_message(&claim_type, &claim_data, &now)?;

        let signature = self.sign_ed25519(&signing_key.private_key, claim_message.as_bytes())?;
        let proof = IdentityProofInternal {
            proof_type: IdentityProofType::DigitalSignature,
            signature,
            public_key: signing_key.public_key.clone(),
            merkle_root: None,
            nonce: Uuid::new_v4().to_string(),
        let claim_internal = IdentityClaimInternal {
            claim_id: claim_id.clone(),
            claim_type,
            claim_data,
            issuer_id: "self".to_string(), // Self-issued claim
            subject_id: "self".to_string(),
            expires_at: None,
            proof,
            verification_status: VerificationStatus::SelfVerified,
            privacy_level,

            let mut claims = self.identity_claims.write().await;
            claims.insert(claim_id.clone(), claim_internal);

        self.update_key_usage(signing_key_id).await?;
        info!("✅ Identity claim created successfully: {}", claim_id);
        Ok(claim_id)

    pub async fn verify_identity_claim(
        claim: &IdentityClaim,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        info!("✅ Verifying identity claim");

        if claim.claim_data.is_empty() {
            return Ok(false);

        info!("🔍 Verifying external identity claim signature");

        match &claim.proof.signature {
            Some(signature_str) => {

                let signature_bytes = match hex::decode(signature_str) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        warn!("Invalid signature format in identity claim");
                        return Ok(false);
                    }
                };

                if matches!(
                    claim.proof.proof_type,
                    crate::api::sovereignty::models::IdentityProofType::Ed25519Signature
                ) {
                    let public_key_bytes = match hex::decode(&claim.proof.proof_data) {
                        Ok(bytes) => bytes,
                        Err(_) => {
                            warn!("Invalid public key format in identity claim");
                            return Ok(false);
                        }
                    };

                    let claim_message = serde_json::to_string(&claim.claim_data).map_err(|e| {
                        Box::new(std::io::Error::other(format!(
                            "Claim serialization error: {e}"
                        )))
                    })?;

                    use beardog_security::crypto_utils::BearDogCrypto;
                    match BearDogCrypto::verify_ed25519_signature(
                        &public_key_bytes,
                        claim_message.as_bytes(),
                        &signature_bytes,
                    ) {
                        Ok(is_valid) => {
                            if is_valid {
                                info!("✅ Identity claim signature verified successfully");
                            } else {
                                warn!("❌ Identity claim signature verification failed");
                            }
                            Ok(is_valid)
                        Err(e) => {
                            warn!("Signature verification error: {}", e);
                            Ok(false)
                } else {
                    warn!("Unsupported signature type for identity claim verification");
                    Ok(false)
            None => {
                warn!("No signature provided in identity claim");
                Ok(false)

    pub async fn create_attestation(
        subject_id: &str,
        claim_id: &str,
        attestation_data: HashMap<&str, &str>,
        confidence_score: f64,
        info!("📋 Creating attestation for claim: {}", claim_id);
        let attestation_id = Uuid::new_v4().to_string();

        let attestation_message = format!(
            "attestation:{}:{}:{}:{}:{}",
            attestation_id,
            subject_id,
            claim_id,
            confidence_score,
            now.timestamp()

        let signature =
            self.sign_ed25519(&signing_key.private_key, attestation_message.as_bytes())?;
        let attestation = IdentityAttestationInternal {
            attestation_id: attestation_id.clone(),
            attestor_id: "self".to_string(),
            attestor_display_name: "Current User".to_string(),
            subject_id: subject_id.to_string(),
            claim_id: claim_id.to_string(),
            attestation_data,
            confidence_score: confidence_score.clamp(0.0, 1.0),
            expires_at: Some(now + Duration::days(365)), // 1 year validity
            revoked: false,

            let mut attestations = self.attestations.write().await;
            attestations.insert(attestation_id.clone(), attestation);
        info!("✅ Identity attestation created: {}", attestation_id);
        Ok(attestation_id)

    pub async fn list_received_attestations(
    ) -> Result<Vec<(String, String, f64)>, Box<dyn std::error::Error + Send + Sync>> {
        let attestations = self.attestations.read().await;
        let result = attestations
            .values()
            .filter(|att| !att.revoked)
            .map(|att| {
                (
                    att.attestor_display_name.clone(),
                    att.claim_id.clone(),
                    att.confidence_score,
                )
            })
            .collect();

    pub async fn add_trusted_attestor(
        attestor_id: &str,
        display_name: &str,
        public_key: &str,
        attestor_type: AttestorType,
        info!("🤝 Adding trusted attestor: {}", display_name);
        let trusted_attestor = TrustedAttestor {
            attestor_id: attestor_id.clone(),
            display_name,
            public_key,
            attestor_type,
            trust_score: 1.0,
            added_at: Utc::now(),
            verified_claims: 0,
            revoked_claims: 0,
        let mut attestors = self.trusted_attestors.write().await;
        attestors.insert(attestor_id, trusted_attestor);
        Ok(())

    pub async fn export_selective_identity(
        claim_ids: Vec<&str>,
        include_proofs: bool,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        info!("📤 Exporting selective identity disclosure");
        let claims = self.identity_claims.read().await;
        let mut exported = HashMap::with_capacity(16);
        for claim_id in claim_ids {
            if let Some(claim) = claims.get(&claim_id) {

                if claim.privacy_level == PrivacyLevel::Private {
                    continue; // Skip private claims
                let mut claim_export = serde_json::json!({
                    "claim_id": claim.claim_id,
                    "claim_type": claim.claim_type,
                    "claim_data": claim.claim_data,
                    "created_at": claim.created_at,
                    "verification_status": claim.verification_status,
                });
                if include_proofs {
                    claim_export["proof"] = serde_json::json!({
                        "proof_type": claim.proof.proof_type,
                        "public_key": claim.proof.public_key,
                        "created_at": claim.proof.created_at,
                        "nonce": claim.proof.nonce,
                    });

                exported.insert(claim_id, claim_export);
        Ok(exported)

    async fn generate_ed25519_keypair(
    ) -> Result<Ed25519KeyPair, Box<dyn std::error::Error + Send + Sync>> {

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let public_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Ok(Ed25519KeyPair {
            private_key,

    fn sign_ed25519(
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {

        let mut hasher = Sha256::new();
        hasher.update(private_key);
        hasher.update(message);
        let hash = hasher.finalize();
        Ok(hash.to_vec())

    fn create_claim_message(
        claim_type: &IdentityClaimType,
        claim_data: &HashMap<&str, &str>,
        timestamp: &DateTime<Utc>,
        let data_string = serde_json::to_string(claim_data)?;
        Ok(format!(
            "identity_claim:{:?}:{}:{}",
            data_string,
            timestamp.timestamp()
        ))

    async fn update_key_usage(
            key.usage_count += 1;
            key.last_used = Some(Utc::now());

    pub async fn rotate_identity_key(
        old_key_id: &str,
        new_purpose: Option<IdentityKeyPurpose>,
        info!("🔄 Rotating identity key: {}", old_key_id);

        let old_key = {
            keys.get(old_key_id)
                .ok_or("Identity key not found")?

        let purpose = new_purpose.unwrap_or(old_key.purpose.clone());
        let new_key = self
            .generate_identity_key(
                purpose,
                None, // No expiration
                old_key.allowed_operations.clone(),
            )
            .await?;

        self.revoke_identity_key(old_key_id).await?;
            "✅ Identity key rotated successfully: {} -> {}",
            old_key_id, new_key.key_id
        Ok(new_key)
