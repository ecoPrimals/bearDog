//! Trust Domain Types
//!
//! Test helper types for trust domains and certificate chains.

use beardog_errors::BearDogError;
use std::time::{Duration, Instant};

/// Trust domain
#[derive(Debug, Clone)]
pub struct TrustDomain {
    name: String,
    anchors: Vec<TrustAnchor>,
}

impl TrustDomain {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            anchors: Vec::new(),
        }
    }

    pub fn with_anchor(mut self, anchor: TrustAnchor) -> Self {
        self.anchors.push(anchor);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn anchor_count(&self) -> usize {
        self.anchors.len()
    }

    pub fn add_anchor(&mut self, anchor: TrustAnchor) -> Result<(), BearDogError> {
        self.anchors.push(anchor);
        Ok(())
    }

    pub fn trusts_anchor(&self, anchor: &TrustAnchor) -> bool {
        self.anchors.iter().any(|a| a.name() == anchor.name())
    }

    pub fn validate_chain(&self, chain: &CertificateChain) -> Result<(), BearDogError> {
        let root_trusted = self.anchors.iter().any(|a| a.name() == chain.root());

        if !root_trusted {
            return Err(BearDogError::security(
                "Root certificate not trusted".to_string(),
            ));
        }
        Ok(())
    }

    pub fn revoke_anchor(&mut self, anchor: &TrustAnchor) -> Result<(), BearDogError> {
        self.anchors.retain(|a| a.name() != anchor.name());
        Ok(())
    }

    pub fn is_anchor_valid(&self, anchor: &TrustAnchor) -> bool {
        !anchor.is_expired() && self.trusts_anchor(anchor)
    }

    pub fn merge(&self, other: &TrustDomain) -> TrustDomain {
        let mut merged = self.clone();
        for anchor in &other.anchors {
            if !merged.trusts_anchor(anchor) {
                merged.anchors.push(anchor.clone());
            }
        }
        merged
    }
}

#[derive(Debug, Clone)]
pub struct TrustAnchor {
    name: String,
    fingerprint: String,
    created_at: Instant,
    expires_after: Option<Duration>,
}

impl TrustAnchor {
    pub fn new(name: &str, fingerprint: &str) -> Self {
        Self {
            name: name.to_string(),
            fingerprint: fingerprint.to_string(),
            created_at: Instant::now(),
            expires_after: None,
        }
    }

    pub fn with_expiration(name: &str, fingerprint: &str, expires_after: Duration) -> Self {
        Self {
            name: name.to_string(),
            fingerprint: fingerprint.to_string(),
            created_at: Instant::now(),
            expires_after: Some(expires_after),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.expires_after {
            self.created_at.elapsed() > duration
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct CertificateChain {
    certificates: Vec<String>,
}

impl CertificateChain {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
        }
    }

    pub fn with_leaf(mut self, cert: &str) -> Self {
        self.certificates.insert(0, cert.to_string());
        self
    }

    pub fn with_intermediate(mut self, cert: &str) -> Self {
        self.certificates.push(cert.to_string());
        self
    }

    pub fn with_root(mut self, cert: &str) -> Self {
        self.certificates.push(cert.to_string());
        self
    }

    pub fn length(&self) -> usize {
        self.certificates.len()
    }

    pub fn leaf(&self) -> &str {
        &self.certificates[0]
    }

    pub fn root(&self) -> &str {
        self.certificates.last().unwrap()
    }
}
