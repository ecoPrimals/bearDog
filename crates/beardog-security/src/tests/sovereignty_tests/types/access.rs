// SPDX-License-Identifier: AGPL-3.0-only

//! Access Control Types
//!
//! Test helper types for identity, access policies, and ABAC.

use beardog_errors::BearDogError;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Identity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum IdentityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Identity
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Identity {
    user_id: String,
    level: IdentityLevel,
    pub(crate) issuer: String,
    region: String,
}

impl Identity {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            level: IdentityLevel::Low,
            issuer: String::new(),
            region: String::new(),
        }
    }

    pub fn with_level(mut self, level: IdentityLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_issuer(mut self, issuer: &str) -> Self {
        self.issuer = issuer.to_string();
        self
    }

    pub fn with_region(mut self, region: &str) -> Self {
        self.region = region.to_string();
        self
    }

    pub fn level(&self) -> IdentityLevel {
        self.level
    }

    pub fn region(&self) -> &str {
        &self.region
    }
}

/// Sovereign access policy
#[derive(Debug, Clone)]
pub struct SovereignAccessPolicy {
    allowed_regions: HashSet<String>,
    identity_level: IdentityLevel,
}

impl SovereignAccessPolicy {
    pub fn new() -> Self {
        Self {
            allowed_regions: HashSet::new(),
            identity_level: IdentityLevel::Low,
        }
    }

    pub fn with_allowed_region(mut self, region: &str) -> Self {
        self.allowed_regions.insert(region.to_string());
        self
    }

    pub fn with_identity_verification(mut self, level: IdentityLevel) -> Self {
        self.identity_level = level;
        self
    }

    pub fn allowed_regions(&self) -> &HashSet<String> {
        &self.allowed_regions
    }

    pub fn allows_region(&self, region: &str) -> bool {
        self.allowed_regions.contains(region)
    }

    pub fn validate_access(&self, request: &AccessRequest) -> Result<(), BearDogError> {
        if !self.allows_region(request.region()) {
            return Err(BearDogError::security(format!(
                "Access from region {} not allowed",
                request.region()
            )));
        }
        Ok(())
    }

    pub fn validate_identity(&self, identity: &Identity) -> Result<(), BearDogError> {
        if identity.level() < self.identity_level {
            return Err(BearDogError::security(
                "Insufficient identity level".to_string(),
            ));
        }
        if identity.issuer.is_empty() || identity.issuer == "Unknown" {
            return Err(BearDogError::security(
                "Unknown identity issuer".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccessRequest {
    user_id: String,
    region: String,
    resource: String,
}

impl AccessRequest {
    pub fn new(user_id: &str, region: &str, resource: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            region: region.to_string(),
            resource: resource.to_string(),
        }
    }

    pub fn region(&self) -> &str {
        &self.region
    }
}

/// Attribute set for ABAC
#[derive(Debug, Clone)]
pub struct AttributeSet {
    attributes: HashMap<String, String>,
}

impl AttributeSet {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
}

/// ABAC policy
#[derive(Debug, Clone)]
pub struct ABACPolicy {
    required_attributes: HashMap<String, String>,
}

impl ABACPolicy {
    pub fn new() -> Self {
        Self {
            required_attributes: HashMap::new(),
        }
    }

    pub fn require_attribute(mut self, key: &str, value: &str) -> Self {
        self.required_attributes
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn evaluate(&self, attributes: &AttributeSet) -> Result<(), BearDogError> {
        for (key, required_value) in &self.required_attributes {
            match attributes.get(key) {
                Some(actual_value) if actual_value == required_value => continue,
                Some(_) => {
                    return Err(BearDogError::security(format!(
                        "Attribute {} has wrong value",
                        key
                    )));
                }
                None => {
                    return Err(BearDogError::security(format!(
                        "Missing required attribute: {}",
                        key
                    )));
                }
            }
        }
        Ok(())
    }
}

/// Delegation
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Delegation {
    delegator_region: String,
    delegatee_region: String,
    permissions: HashSet<String>,
    created_at: Instant,
    expires_after: Duration,
}

impl Delegation {
    pub fn new(delegator: &Identity, delegatee: &Identity) -> Self {
        Self {
            delegator_region: delegator.region().to_string(),
            delegatee_region: delegatee.region().to_string(),
            permissions: HashSet::new(),
            created_at: Instant::now(),
            expires_after: Duration::from_secs(86400), // 24 hours default
        }
    }

    pub fn with_permission(mut self, permission: &str) -> Self {
        self.permissions.insert(permission.to_string());
        self
    }

    pub fn with_expiration(mut self, duration: Duration) -> Self {
        self.expires_after = duration;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.expires_after
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}
