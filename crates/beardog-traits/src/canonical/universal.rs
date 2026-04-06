// SPDX-License-Identifier: AGPL-3.0-or-later

//! Dynamic “invoke anything” provider for legacy integrations.

use super::base::{BaseProvider, ConnectionStatus};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Success flag plus JSON payload from a dynamic operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Whether success is enabled
    pub success: bool,
    /// The result value
    pub result: serde_json::Value,
    /// The message value
    pub message: String,
}

/// String-dispatched RPC-style surface over heterogeneous backends.
pub trait UniversalProvider: BaseProvider {
    /// Executes operation
    fn execute_operation(
        operation: &str,
        params: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<OperationResult, BearDogError>> + Send;

    /// Link state for a named endpoint or shard.
    fn connection_status(
        endpoint: &str,
    ) -> impl std::future::Future<Output = Result<ConnectionStatus, BearDogError>> + Send;

    /// Gets capabilities
    fn get_capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;

    /// Validates operation
    fn validate_operation(
        &self,
        operation: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;
}
