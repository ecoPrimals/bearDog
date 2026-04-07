// SPDX-License-Identifier: AGPL-3.0-or-later

//! Graph Security Handler for JSON-RPC
//!
//! Implements JSON-RPC methods for graph security validation, authorization, and audit.
//!
//! # Methods
//! - `graph.validate_template` - Validate template structure and security
//! - `graph.audit_origin` - Audit template provenance and trust
//! - `graph.authorize_modification` - Authorize graph modifications in real-time
//!
//! # Integration
//! These handlers integrate with the complete graph security module at
//! `crates/beardog-tunnel/src/graph_security/` which implements:
//! - 5-layer security model (auth, authz, validation, threat detection, audit)
//! - Cryptographic signature verification
//! - Vulnerability scanning
//! - Threat pattern detection
//! - Provenance tracking

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, info};

// Import graph security types from the parent module
use crate::graph_security::{self, Graph, GraphModification, GraphTemplate};

/// Handler for graph security JSON-RPC methods
pub struct GraphSecurityHandler;

#[async_trait]
impl MethodHandler for GraphSecurityHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "graph.validate_template",
            "graph.audit_origin",
            "graph.authorize_modification",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&Value>,
        _btsp: &Arc<BeardogBtspProvider>,
    ) -> Result<Value, String> {
        debug!("🔒 Graph security handler: {}", method);

        match method {
            "graph.validate_template" => self.validate_template(params).await,
            "graph.audit_origin" => self.audit_origin(params).await,
            "graph.authorize_modification" => self.authorize_modification(params).await,
            _ => Err(format!("Method not found: {method}")),
        }
    }
}

impl GraphSecurityHandler {
    /// Validate a graph template for security issues
    ///
    /// Performs comprehensive validation:
    /// 1. Structure validation (nodes, edges, cycles)
    /// 2. Signature verification (if present)
    /// 3. Vulnerability scanning
    /// 4. Threat detection
    async fn validate_template(&self, params: Option<&Value>) -> Result<Value, String> {
        let params = params.ok_or("Missing params for graph.validate_template")?;

        // Parse template from params
        let template: GraphTemplate = serde_json::from_value(
            params
                .get("template")
                .cloned()
                .ok_or("Missing 'template' in params")?,
        )
        .map_err(|e| format!("Invalid template: {e}"))?;

        info!("🔍 Validating template: {}", template.id);

        // Call graph security validation
        let report = graph_security::validate_template(&template)
            .await
            .map_err(|e| e.to_string())?;

        info!(
            "✅ Template validation complete: {} (score: {})",
            if report.valid { "PASS" } else { "FAIL" },
            report.security_score
        );

        // Convert to JSON-RPC response
        serde_json::to_value(report)
            .map_err(|e| format!("Failed to serialize validation report: {e}"))
    }

    /// Audit the origin and provenance of a template
    ///
    /// Performs comprehensive origin verification:
    /// 1. Creator identity verification
    /// 2. Lineage tracking and chain of custody
    /// 3. Community usage metrics
    /// 4. Security assessment
    async fn audit_origin(&self, params: Option<&Value>) -> Result<Value, String> {
        let params = params.ok_or("Missing params for graph.audit_origin")?;

        // Parse template_id from params
        let template_id = params
            .get("template_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'template_id' in params")?
            .to_string();

        info!("📜 Auditing origin: {}", template_id);

        // Call graph security audit
        let audit = graph_security::audit_origin(&template_id)
            .await
            .map_err(|e| e.to_string())?;

        info!(
            "✅ Origin audit complete: trust_score={}, risk={:?}",
            audit.trust_score, audit.risk_level
        );

        // Convert to JSON-RPC response
        serde_json::to_value(audit).map_err(|e| format!("Failed to serialize audit: {e}"))
    }

    /// Authorize a graph modification in real-time
    ///
    /// Performs 5-layer security check:
    /// 1. Authentication (user identity via HSM)
    /// 2. Authorization (RBAC and ownership)
    /// 3. Validation (structure and safety)
    /// 4. Threat Detection (malicious patterns)
    /// 5. Audit (logging and provenance)
    async fn authorize_modification(&self, params: Option<&Value>) -> Result<Value, String> {
        let params = params.ok_or("Missing params for graph.authorize_modification")?;

        // Parse user_id
        let user_id = params
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'user_id' in params")?
            .to_string();

        // Parse graph
        let graph: Graph = serde_json::from_value(
            params
                .get("graph")
                .cloned()
                .ok_or("Missing 'graph' in params")?,
        )
        .map_err(|e| format!("Invalid graph: {e}"))?;

        // Parse modification
        let modification: GraphModification = serde_json::from_value(
            params
                .get("modification")
                .cloned()
                .ok_or("Missing 'modification' in params")?,
        )
        .map_err(|e| format!("Invalid modification: {e}"))?;

        info!(
            "🔐 Authorizing modification: user={}, graph={}, action={:?}",
            user_id, graph.id, modification.action
        );

        // Call graph security authorization
        let result = graph_security::authorize_modification(&user_id, &graph, &modification)
            .await
            .map_err(|e| e.to_string())?;

        info!(
            "✅ Authorization complete: {} (confidence: {})",
            if result.authorized {
                "GRANTED"
            } else {
                "DENIED"
            },
            result.confidence
        );

        // Convert to JSON-RPC response
        serde_json::to_value(result).map_err(|e| format!("Failed to serialize authorization: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::mocks;
    use serde_json::json;

    #[tokio::test]
    async fn test_graph_security_handler_methods() {
        let handler = GraphSecurityHandler;
        let methods = handler.methods();

        // Should support graph security methods
        assert_eq!(methods.len(), 3);
        assert!(methods.contains(&"graph.validate_template"));
        assert!(methods.contains(&"graph.audit_origin"));
        assert!(methods.contains(&"graph.authorize_modification"));
    }

    #[tokio::test]
    async fn test_validate_template_missing_params() {
        let handler = GraphSecurityHandler;
        let btsp = Arc::new(mocks::create_minimal_beardog_provider().await);

        let result = handler.handle("graph.validate_template", None, &btsp).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing params"));
    }

    #[tokio::test]
    async fn test_audit_origin_missing_template_id() {
        let handler = GraphSecurityHandler;
        let btsp = Arc::new(mocks::create_minimal_beardog_provider().await);

        let params = json!({});
        let result = handler
            .handle("graph.audit_origin", Some(&params), &btsp)
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("template_id"));
    }

    #[tokio::test]
    async fn test_authorize_modification_missing_user_id() {
        let handler = GraphSecurityHandler;
        let btsp = mocks::create_minimal_beardog_provider().await;

        let params = json!({
            "graph": {
                "id": "test-graph",
                "owner": "alice",
                "nodes": [],
                "edges": [],
                "metadata": {}
            },
            "modification": {
                "action": "add_node",
                "node": {
                    "id": "node-1",
                    "primal": "compute-peer",
                    "type": "compute",
                    "config": {}
                }
            }
        });

        let result = handler
            .handle("graph.authorize_modification", Some(&params), &btsp)
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("user_id"));
    }
}
