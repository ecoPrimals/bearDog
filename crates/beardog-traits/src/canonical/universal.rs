// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::{BaseProvider, ConnectionStatus};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Temporary type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Whether success is enabled
    pub success: bool,
    /// The result value
    pub result: serde_json::Value,
    /// The message value
    pub message: String,
}

pub trait UniversalProvider: BaseProvider {
    /// Executes operation
    fn execute_operation(
        operation: &str,
        params: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<OperationResult, BearDogError>> + Send;

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
