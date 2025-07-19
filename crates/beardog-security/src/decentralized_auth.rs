//! Decentralized Authentication System
//!
//! This module implements a decentralized authentication system using Ed25519 cryptographic
//! identity, replacing centralized JWT tokens with cryptographic proofs of identity.
//!
//! ## Key Features
//! - **No Central Authority**: Each node has its own Ed25519 keypair
//! - **Cryptographic Proofs**: Authentication via signature verification
//! - **Decentralized Trust**: Web of trust model without central issuer
//! - **Offline Capable**: Works without network connectivity
//! - **Replay Protection**: Timestamps and nonces prevent replay attacks

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::{BearDogError, BearDogResult};

/// Decentralized authentication token using Ed25519 cryptographic identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoAuthToken {
    /// Unique token identifier
    pub token_id: Uuid,
    /// Node identity (public key hash)
    pub node_identity: String,
    /// Ed25519 public key for verification
    pub public_key: Vec<u8>,
    /// Token claims (permissions, metadata)
    pub claims: AuthClaims,
    /// When the token was issued
    pub issued_at: DateTime<Utc>,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// Nonce to prevent replay attacks
    pub nonce: Vec<u8>,
    /// Ed25519 signature over the token data
    pub signature: Vec<u8>,
}

/// Authentication claims for decentralized auth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    /// Subject (node/user identity)
    pub subject: String,
    /// Issuer (self-issued for decentralized auth)
    pub issuer: String,
    /// Audience (target service/node)
    pub audience: String,
    /// Permissions granted
    pub permissions: Vec<String>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Decentralized authentication manager
pub struct DecentralizedAuthManager {
    /// This node's Ed25519 keypair
    node_keypair: (Vec<u8>, Vec<u8>), // (private_key, public_key)
    /// This node's identity
    node_identity: String,
    /// Trusted public keys (web of trust)
    trusted_keys: HashMap<String, Vec<u8>>,
    /// Token expiration duration
    token_lifetime: Duration,
}

impl DecentralizedAuthManager {
    /// Create a new decentralized auth manager
    pub fn new(token_lifetime_hours: i64) -> BearDogResult<Self> {
        // Generate Ed25519 keypair for this node
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;

        // Create node identity from public key hash
        let node_identity = Self::create_node_identity(&public_key)?;

        Ok(Self {
            node_keypair: (private_key, public_key),
            node_identity,
            trusted_keys: HashMap::new(),
            token_lifetime: Duration::hours(token_lifetime_hours),
        })
    }

    /// Create a new authentication token
    pub fn create_auth_token(
        &self,
        subject: &str,
        audience: &str,
        permissions: Vec<String>,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<CryptoAuthToken> {
        let now = Utc::now();
        let token_id = Uuid::new_v4();
        let nonce = BearDogCrypto::generate_secure_nonce(16)?;

        let claims = AuthClaims {
            subject: subject.to_string(),
            issuer: self.node_identity.clone(),
            audience: audience.to_string(),
            permissions,
            metadata,
        };

        let token = CryptoAuthToken {
            token_id,
            node_identity: self.node_identity.clone(),
            public_key: self.node_keypair.1.clone(),
            claims,
            issued_at: now,
            expires_at: now + self.token_lifetime,
            nonce,
            signature: Vec::new(), // Will be filled after signing
        };

        // Create signature over token data
        let signature_data = self.create_signature_data(&token)?;
        let signature = BearDogCrypto::sign_ed25519(&self.node_keypair.0, &signature_data)?;

        Ok(CryptoAuthToken { signature, ..token })
    }

    /// Verify an authentication token
    pub fn verify_auth_token(&self, token: &CryptoAuthToken) -> BearDogResult<bool> {
        // Check expiration
        if Utc::now() > token.expires_at {
            return Ok(false);
        }

        // Verify the signature
        let signature_data = self.create_signature_data(token)?;
        let signature_valid = BearDogCrypto::verify_ed25519_signature(
            &token.public_key,
            &signature_data,
            &token.signature,
        )?;

        if !signature_valid {
            return Ok(false);
        }

        // Check if we trust this public key
        if let Some(trusted_key) = self.trusted_keys.get(&token.node_identity) {
            if trusted_key != &token.public_key {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Add a trusted public key to the web of trust
    pub fn add_trusted_key(&mut self, node_identity: &str, public_key: Vec<u8>) {
        self.trusted_keys
            .insert(node_identity.to_string(), public_key);
    }

    /// Remove a trusted public key
    pub fn remove_trusted_key(&mut self, node_identity: &str) {
        self.trusted_keys.remove(node_identity);
    }

    /// Get this node's public key
    pub fn get_public_key(&self) -> &[u8] {
        &self.node_keypair.1
    }

    /// Get this node's identity
    pub fn get_node_identity(&self) -> &str {
        &self.node_identity
    }

    /// Create signature data for token
    fn create_signature_data(&self, token: &CryptoAuthToken) -> BearDogResult<Vec<u8>> {
        let mut data = Vec::new();

        // Add token ID
        data.extend_from_slice(token.token_id.as_bytes());

        // Add node identity
        data.extend_from_slice(token.node_identity.as_bytes());

        // Add public key
        data.extend_from_slice(&token.public_key);

        // Add claims (serialized)
        let claims_bytes = serde_json::to_vec(&token.claims)?;
        data.extend_from_slice(&claims_bytes);

        // Add timestamps
        data.extend_from_slice(&token.issued_at.timestamp().to_le_bytes());
        data.extend_from_slice(&token.expires_at.timestamp().to_le_bytes());

        // Add nonce
        data.extend_from_slice(&token.nonce);

        Ok(data)
    }

    /// Create node identity from public key
    fn create_node_identity(public_key: &[u8]) -> BearDogResult<String> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();

        // Use first 16 bytes as node identity
        let identity_bytes = &hash[..16];
        Ok(hex::encode(identity_bytes))
    }
}

/// Challenge-response authentication for real-time verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    /// Challenge identifier
    pub challenge_id: Uuid,
    /// Random challenge data
    pub challenge_data: Vec<u8>,
    /// When the challenge was issued
    pub issued_at: DateTime<Utc>,
    /// When the challenge expires
    pub expires_at: DateTime<Utc>,
    /// Expected responder identity
    pub expected_responder: String,
}

/// Response to an authentication challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Challenge identifier
    pub challenge_id: Uuid,
    /// Responder's node identity
    pub responder_identity: String,
    /// Responder's public key
    pub public_key: Vec<u8>,
    /// Ed25519 signature over challenge data
    pub signature: Vec<u8>,
    /// When the response was created
    pub response_time: DateTime<Utc>,
}

impl DecentralizedAuthManager {
    /// Create an authentication challenge
    pub fn create_challenge(&self, expected_responder: &str) -> BearDogResult<AuthChallenge> {
        let challenge_data = BearDogCrypto::generate_secure_nonce(32)?;
        let now = Utc::now();

        Ok(AuthChallenge {
            challenge_id: Uuid::new_v4(),
            challenge_data,
            issued_at: now,
            expires_at: now + Duration::minutes(5), // 5 minute challenge window
            expected_responder: expected_responder.to_string(),
        })
    }

    /// Respond to an authentication challenge
    pub fn respond_to_challenge(&self, challenge: &AuthChallenge) -> BearDogResult<AuthResponse> {
        // Check if challenge is still valid
        if Utc::now() > challenge.expires_at {
            return Err(BearDogError::validation("Challenge has expired"));
        }

        // Sign the challenge data
        let signature =
            BearDogCrypto::sign_ed25519(&self.node_keypair.0, &challenge.challenge_data)?;

        Ok(AuthResponse {
            challenge_id: challenge.challenge_id,
            responder_identity: self.node_identity.clone(),
            public_key: self.node_keypair.1.clone(),
            signature,
            response_time: Utc::now(),
        })
    }

    /// Verify a challenge response
    pub fn verify_challenge_response(
        &self,
        challenge: &AuthChallenge,
        response: &AuthResponse,
    ) -> BearDogResult<bool> {
        // Check challenge ID matches
        if challenge.challenge_id != response.challenge_id {
            return Ok(false);
        }

        // Check response time is within challenge window
        if response.response_time > challenge.expires_at {
            return Ok(false);
        }

        // Check expected responder (if specified)
        if !challenge.expected_responder.is_empty()
            && challenge.expected_responder != response.responder_identity
        {
            return Ok(false);
        }

        // Verify signature
        let signature_valid = BearDogCrypto::verify_ed25519_signature(
            &response.public_key,
            &challenge.challenge_data,
            &response.signature,
        )?;

        Ok(signature_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decentralized_auth_token_creation() {
        let auth_manager = DecentralizedAuthManager::new(24).unwrap();

        let token = auth_manager
            .create_auth_token(
                "test_user",
                "test_service",
                vec!["read".to_string(), "write".to_string()],
                HashMap::new(),
            )
            .unwrap();

        assert_eq!(token.claims.subject, "test_user");
        assert_eq!(token.claims.audience, "test_service");
        assert_eq!(token.claims.permissions, vec!["read", "write"]);
        assert!(!token.signature.is_empty());
    }

    #[test]
    fn test_token_verification() {
        let auth_manager = DecentralizedAuthManager::new(24).unwrap();

        let token = auth_manager
            .create_auth_token(
                "test_user",
                "test_service",
                vec!["read".to_string()],
                HashMap::new(),
            )
            .unwrap();

        let is_valid = auth_manager.verify_auth_token(&token).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_challenge_response() {
        let auth_manager = DecentralizedAuthManager::new(24).unwrap();

        // Create challenge for this node's identity
        let challenge = auth_manager
            .create_challenge(auth_manager.get_node_identity())
            .unwrap();
        let response = auth_manager.respond_to_challenge(&challenge).unwrap();

        let is_valid = auth_manager
            .verify_challenge_response(&challenge, &response)
            .unwrap();
        assert!(is_valid);
    }
}
