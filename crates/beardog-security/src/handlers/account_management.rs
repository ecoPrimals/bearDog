//! Account Management Module
//!
//! Handles account locking, unlocking, and security policies.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Check if account is locked
    pub async fn is_account_locked(&self, user_id: &str) -> bool {
        let locked_accounts = self.locked_accounts.read().await;
        if let Some(lockout_time) = locked_accounts.get(user_id) {
            let unlock_time =
                *lockout_time + Duration::minutes(self.config.lockout_duration_minutes as i64);
            Utc::now() < unlock_time
        } else {
            false
        }
    }

    /// Lock an account due to security concerns
    pub async fn lock_account(
        &mut self,
        user_id: &str,
        reason: AccountLockReason,
    ) -> BearDogResult<()> {
        let unlock_time = {
            let mut locked_accounts = self.locked_accounts.write().await;
            let now = Utc::now();
            locked_accounts.insert(user_id.to_string(), now);
            Some(now + Duration::minutes(self.config.lockout_duration_minutes as i64))
        };

        // Create audit event for account locking
        self.create_audit_event(AuditEvent::AccountLocked {
            user_id: user_id.to_string(),
            reason: format!("{reason:?}"),
            locked_at: Utc::now(),
            unlock_time,
        })
        .await?;

        Ok(())
    }

    /// Unlock an account (admin function)
    pub async fn unlock_account(&mut self, user_id: &str, admin_id: &str) -> BearDogResult<()> {
        let was_locked = {
            let mut locked_accounts = self.locked_accounts.write().await;
            locked_accounts.remove(user_id).is_some()
        };

        if was_locked {
            // Create audit event for account unlocking
            self.create_audit_event(AuditEvent::AccountUnlocked {
                user_id: user_id.to_string(),
                admin_id: admin_id.to_string(),
                unlocked_at: Utc::now(),
            })
            .await?;
        }

        Ok(())
    }

    /// Get locked accounts status
    pub async fn get_locked_accounts(&self) -> BearDogResult<Vec<LockedAccountInfo>> {
        let locked_accounts = self.locked_accounts.read().await;
        let mut result = Vec::new();

        for (user_id, locked_at) in locked_accounts.iter() {
            let unlock_time =
                *locked_at + Duration::minutes(self.config.lockout_duration_minutes as i64);
            let is_still_locked = Utc::now() < unlock_time;

            result.push(LockedAccountInfo {
                user_id: user_id.clone(),
                locked_at: *locked_at,
                unlock_time,
                is_still_locked,
                remaining_time: if is_still_locked {
                    Some((unlock_time - Utc::now()).num_seconds() as u64)
                } else {
                    None
                },
            });
        }

        Ok(result)
    }

    /// Clean up expired account locks
    pub async fn cleanup_expired_locks(&self) -> BearDogResult<u32> {
        let mut locked_accounts = self.locked_accounts.write().await;
        let now = Utc::now();
        let unlock_duration = Duration::minutes(self.config.lockout_duration_minutes as i64);

        let initial_count = locked_accounts.len();
        locked_accounts.retain(|_, locked_at| now < *locked_at + unlock_duration);

        let removed_count = initial_count - locked_accounts.len();
        Ok(removed_count as u32)
    }
}

/// Reason for account locking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountLockReason {
    TooManyFailedAttempts,
    SuspiciousActivity,
    SecurityBreach,
    AdminAction,
    ComplianceViolation,
}

/// Information about a locked account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedAccountInfo {
    pub user_id: String,
    pub locked_at: chrono::DateTime<Utc>,
    pub unlock_time: chrono::DateTime<Utc>,
    pub is_still_locked: bool,
    pub remaining_time: Option<u64>, // seconds
}
