// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME order lifecycle — represents a certificate request in progress.

use serde::{Deserialize, Serialize};

/// The state of an ACME order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// Order created, authorizations pending.
    Pending,
    /// All authorizations satisfied, ready to finalize.
    Ready,
    /// CSR submitted, CA is issuing the certificate.
    Processing,
    /// Certificate issued and available for download.
    Valid,
    /// Order failed or expired.
    Invalid,
}

/// Represents an in-progress ACME certificate order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateOrder {
    /// The order URL (for polling status).
    pub order_url: String,

    /// Current order status.
    pub status: OrderStatus,

    /// Domain identifiers this order covers.
    pub identifiers: Vec<String>,

    /// Authorization URLs that must be satisfied.
    pub authorization_urls: Vec<String>,

    /// Finalize URL (submit CSR here when ready).
    pub finalize_url: String,

    /// Certificate URL (available after status = valid).
    pub certificate_url: Option<String>,
}

impl CertificateOrder {
    /// Whether this order is ready for CSR submission.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.status == OrderStatus::Ready
    }

    /// Whether the certificate has been issued.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status == OrderStatus::Valid
    }

    /// Whether the order has failed.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        self.status == OrderStatus::Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_order(status: OrderStatus) -> CertificateOrder {
        CertificateOrder {
            order_url: "https://acme.example/order/1".to_string(),
            status,
            identifiers: vec!["example.com".to_string()],
            authorization_urls: vec!["https://acme.example/authz/1".to_string()],
            finalize_url: "https://acme.example/finalize/1".to_string(),
            certificate_url: None,
        }
    }

    #[test]
    fn is_ready_when_ready() {
        assert!(sample_order(OrderStatus::Ready).is_ready());
        assert!(!sample_order(OrderStatus::Pending).is_ready());
    }

    #[test]
    fn is_valid_when_valid() {
        let mut order = sample_order(OrderStatus::Valid);
        order.certificate_url = Some("https://acme.example/cert/1".to_string());
        assert!(order.is_valid());
        assert!(!order.is_invalid());
    }

    #[test]
    fn is_invalid_when_invalid() {
        assert!(sample_order(OrderStatus::Invalid).is_invalid());
        assert!(!sample_order(OrderStatus::Valid).is_invalid());
    }

    #[test]
    fn order_status_serialization() {
        let json = serde_json::to_string(&OrderStatus::Processing).expect("serialize");
        assert_eq!(json, r#""processing""#);
        let parsed: OrderStatus = serde_json::from_str(r#""valid""#).expect("parse");
        assert_eq!(parsed, OrderStatus::Valid);
    }
}
