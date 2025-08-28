

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralRecoveryKey {

    pub id: String,

    pub user_id: String,

    pub key_value: String,

    pub generated_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub used: bool,

    pub permissions: EphemeralPermissions,

    pub time_restrictions: TimeRestrictions,
}
impl EphemeralRecoveryKey {

    pub fn new(id: &str, user_id: &str, permissions: EphemeralPermissions) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            key_value: Self::generate_secure_key(), // Securely generated key
            generated_at: now,
            expires_at: now + chrono::Duration::hours(24), // 24 hour expiry
            used: false,
            permissions,
            time_restrictions: TimeRestrictions::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.used && Utc::now() < self.expires_at

    pub fn mark_used(&mut self) {
        self.used = true;

    fn generate_secure_key() -> String {
        use rand::Rng;

        let random_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();

        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(random_bytes)

    pub fn derive_time_bound_key(&self, salt: &[u8], info: &[u8]) -> Result<String, String> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let master_key = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(&self.key_value)
            .map_err(|_| "Invalid key format")?;

        let hk = Hkdf::<Sha256>::new(Some(salt), &master_key);
        let mut derived_key = [0u8; 32];
        hk.expand(info, &mut derived_key)
            .map_err(|_| "Key derivation failed")?;

        Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(derived_key))

    pub fn generate_possession_proof(&self, challenge: &[u8]) -> Result<String, String> {
        use hmac::{Hmac, Mac};

use beardog_errors::BearDogError;
        let key_bytes = base64::engine::general_purpose::STANDARD_NO_PAD

        let mut mac =
            Hmac::<Sha256>::new_from_slice(&key_bytes).map_err(|_| "HMAC creation failed")?;
        mac.update(challenge);
        let proof = mac.finalize().into_bytes();

        Ok(hex::encode(proof))

pub struct EphemeralPermissions {

    pub can_read_data: bool,

    pub can_modify_settings: bool,

    pub can_initiate_recovery: bool,

    pub can_access_encrypted: bool,

    pub can_admin: bool,

    pub max_operations: u32,

    pub operations_used: u32,

pub struct TimeRestrictions {

    pub valid_from: Option<DateTime<Utc>>,

    pub valid_until: Option<DateTime<Utc>>,

    pub max_usage_duration_hours: u32,

    pub cooldown_minutes: u32,}

impl Default for EphemeralPermissions {}

    fn default() -> Self {
            can_read_data: false,
            can_modify_settings: false,
            can_initiate_recovery: true,
            can_access_encrypted: false,
            can_admin: false,
            max_operations: 10,
            operations_used: 0,
impl Default for TimeRestrictions {
            valid_from: None,
            valid_until: None,
            max_usage_duration_hours: 24,
            cooldown_minutes: 5,
