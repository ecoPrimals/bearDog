// SPDX-License-Identifier: AGPL-3.0-only

// # Unified Provider Traits - The Foundation of BearDog Architecture
//
// This module defines the core provider traits that form the foundation of the BearDog
// ecosystem. These traits establish a unified interface for all system components,
// enabling seamless integration, testing, and extensibility.
//
// ## 🏗️ **Architectural Principles**
//
// - **Unified Interface**: All providers implement `BearDogProvider` as the base trait
// - **Native Async**: All operations are async-first for maximum performance
// - **Type Safety**: Strong typing with associated types for compile-time guarantees
// - **Error Handling**: Consistent error propagation with rich error context
// - **Configuration**: Unified configuration system with validation
// - **Observability**: Built-in metrics, health checks, and monitoring
//
// ## 🎯 **Provider Hierarchy**
//
// ```text
// BearDogProvider (base trait)
// ├── SecurityProvider (authentication, authorization)
// ├── CryptoProvider (encryption, signing, verification)
// ├── HsmProvider (hardware security modules)
// ├── GeneticsProvider (AI/ML genetics and optimization)
// ├── MonitoringProvider (metrics, alerting, observability)
// ├── AdapterProvider (external system integration)
// └── WorkflowProvider (process orchestration)
// ```
//
// ## 🚀 **Usage Examples**
//
// ```rust,no_run
// use beardog_traits::unified::providers::*;
// use beardog_errors::BearDogError;
//
// // Implementing a custom security provider
// struct MySecurityProvider {
//     config: MyConfig,
// }
//
// impl BearDogProvider for MySecurityProvider {
//     type Error = BearDogError;
//     type Config = MyConfig;
//
//     fn provider_info(&self) -> ProviderInfo {
//         ProviderInfo {
//             provider_id: "my_security".to_string(),
//             name: "My Security Provider".to_string(),
//             version: "1.0.0".to_string(),
//             provider_type: ProviderType::Security,
//             description: Some("Custom security implementation".to_string()),
//             vendor: Some("My Company".to_string()),
//             tags: vec!["security".to_string(), "custom".to_string()],
//         }
//     }
//
//     fn health_check(&self) -> impl std::future::Future<Output = Result<ProviderHealth, Self::Error>> + Send {
//         // Implementation
//         Ok(ProviderHealth { /* ... */ })
//     }
//
//     // ... other required methods
// }
// ```

use beardog_types::canonical::{
    providers_unified::traits::{ProviderCapability, ProviderHealth, ProviderMetrics},
    // Removed unused import: config::unified::BearDogConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

/// Core provider trait that all `BearDog` providers must implement
///
/// This trait defines the fundamental operations that every provider in the `BearDog`
/// ecosystem must support, including health monitoring, metrics collection, and
/// lifecycle management.
pub trait BearDogProvider: Send + Sync + 'static {
    /// Concrete error type for this provider; usually [`BearDogError`](beardog_errors::BearDogError) or a thin wrapper.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Configuration blob deserialized from TOML/JSON before `initialize`.
    type Config: Send + Sync + Clone;

    /// Provider identification
    fn provider_id(&self) -> &str;

    /// Get provider version
    fn provider_version(&self) -> &str;

    /// Get provider capabilities
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Health check - returns impl Future instead of fn
    fn health_check(&self) -> impl Future<Output = Result<ProviderHealth, Self::Error>> + Send;

    /// Get current metrics - returns impl Future instead of fn  
    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, Self::Error>> + Send;
}

/// **Security Provider Trait**
///
///
/// ## Security Principles
///
/// - **Zero Trust**: Never trust, always verify
/// - **Least Privilege**: Grant minimum necessary permissions
/// - **Defense in Depth**: Multiple layers of security
/// - **Audit Everything**: Comprehensive security logging
pub trait SecurityProvider: BearDogProvider {
    /// Authentication result type
    type AuthResult: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Session management type
    type Session: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Credentials type
    type Credentials: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **User Authentication**
    ///
    /// Authenticates user credentials and returns an authentication result.
    /// This should verify the user's identity using the provided credentials.
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl std::future::Future<Output = Result<Self::AuthResult, Self::Error>> + Send;

    /// **Session Management: Create**
    ///
    /// Sessions should have appropriate timeouts and security properties.
    /// Creates session
    fn create_session(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<Self::Session, Self::Error>> + Send;

    /// **Session Management: Validate**
    ///
    /// Validates an existing session and returns its validity status.
    /// Invalid or expired sessions should return false.
    /// Validates session
    fn validate_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// **Session Management: Revoke**
    ///
    /// Revokes an active session, immediately invalidating it.
    fn revoke_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **Authorization Check**
    ///
    /// This implements the core access control logic.
    fn authorize(
        &self,
        session_id: &str,
        resource: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// **Security Requirements**
    ///
    /// Gets `security_requirements`
    fn get_security_requirements(
        &self,
        resource: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send;
}

/// **Cryptographic Provider Trait**
///
/// decryption, digital signatures, and key management.
///
/// ## Cryptographic Principles
///
/// - **Strong Algorithms**: Use only proven cryptographic algorithms
/// - **Key Security**: Protect cryptographic keys at all costs
/// - **Perfect Forward Secrecy**: Compromise of long-term keys doesn't compromise past sessions
/// - **Constant Time**: Avoid timing attacks in cryptographic operations
pub trait CryptoProvider: BearDogProvider {
    /// Cryptographic key type
    type Key: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Digital signature type
    type Signature: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **Key Generation**
    ///
    /// Generates a new cryptographic key of the specified type.
    /// Keys should be generated using cryptographically secure random sources.
    fn generate_key(
        &self,
        key_type: &str,
    ) -> impl std::future::Future<Output = Result<Self::Key, Self::Error>> + Send;

    /// **Digital Signature: Sign**
    ///
    /// Signatures should be deterministic and verifiable.
    fn sign(
        &self,
        key: &Self::Key,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Self::Signature, Self::Error>> + Send;

    /// **Digital Signature: Verify**
    ///
    /// Verifies a digital signature against the original data and public key.
    /// This should be constant-time to prevent timing attacks.
    fn verify(
        &self,
        key: &Self::Key,
        data: &[u8],
        signature: &Self::Signature,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// **Symmetric Encryption**
    ///
    /// Encrypts data using symmetric encryption with the provided key.
    /// Should use authenticated encryption modes (AEAD) when possible.
    fn encrypt(
        &self,
        key: &Self::Key,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// **Symmetric Decryption**
    ///
    /// Decrypts data that was encrypted with the symmetric encrypt method.
    fn decrypt(
        &self,
        key: &Self::Key,
        encrypted_data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// **Supported Algorithms**
    ///
    /// Returns a list of cryptographic algorithms supported by this provider.
    fn supported_algorithms(&self) -> Vec<String>;
}

/// **Hardware Security Module (HSM) Provider Trait**
///
/// HSM providers offer the highest level of key security and tamper resistance.
///
/// # Migration (v0.10.0)
///
/// This trait requires the full `BearDogProvider + SecurityProvider + CryptoProvider`
/// hierarchy, making it impractical for dynamic dispatch.  New code should use
/// [`crate::hsm::HsmKeyProvider`] which is object-safe and supports
/// `Arc<dyn HsmKeyProvider>` through `HsmProviderRegistry`.
/// This trait will be removed in a future release.
pub trait HsmProvider: BearDogProvider + SecurityProvider + CryptoProvider {
    /// Serialized health, tamper, and utilization snapshot for dashboards.
    type HsmStatus: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// HSM key slot identifier type
    type KeySlot: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **HSM Status Check**
    ///
    /// This includes tamper status, key slot usage, and hardware health.
    fn hsm_status(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::HsmStatus, Self::Error>> + Send;

    /// **Key Slot Management: List**
    ///
    /// Returns a list of available key slots in the HSM.
    fn list_key_slots(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::KeySlot>, Self::Error>> + Send;

    /// **HSM Key Generation**
    ///
    /// Generates a cryptographic key directly in HSM hardware.
    /// The key never exists in software and cannot be extracted.
    fn generate_hsm_key(
        &self,
        slot: Self::KeySlot,
        key_type: &str,
    ) -> impl std::future::Future<Output = Result<Self::Key, Self::Error>> + Send;

    /// **Key Import**
    ///
    /// Imports a key into HSM hardware (if supported).
    fn import_key(
        &self,
        slot: Self::KeySlot,
        key: Self::Key,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **Key Deletion**
    ///
    /// Securely deletes a key from HSM hardware.
    /// This operation should be irreversible and tamper-evident.
    /// Removes key
    fn delete_key(
        &self,
        slot: Self::KeySlot,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **Key Enumeration**
    ///
    /// Lists all keys stored in the HSM.
    /// This may return key identifiers rather than the actual keys.
    fn list_keys(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send;

    /// **Key Metadata**
    ///
    /// Returns metadata about a specific key without exposing the key itself.
    /// This includes key type, creation time, usage counters, etc.
    /// Gets `key_metadata`
    fn get_key_metadata(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<HashMap<String, serde_json::Value>, Self::Error>> + Send;

    /// **Generic HSM Operation**
    ///
    /// This provides access to specialized HSM functions not covered by standard operations.
    fn hsm_operation(
        &self,
        operation: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> impl std::future::Future<Output = Result<serde_json::Value, Self::Error>> + Send;

    /// **Hardware Attestation**
    ///
    /// Returns a hardware attestation proving the authenticity of the HSM.
    /// Gets `hardware_attestation`
    fn get_hardware_attestation(
        &self,
    ) -> impl std::future::Future<Output = Result<Option<Vec<u8>>, Self::Error>> + Send;

    /// **Hardware Backing Verification**
    ///
    /// Returns true if this provider is backed by actual hardware.
    /// Software-based implementations should return false.
    /// Checks if hardware backed
    fn is_hardware_backed(&self) -> bool;

    /// **Security Level**
    ///
    /// Returns the security certification level of the HSM hardware.
    /// Examples: "FIPS 140-2 Level 3", "Common Criteria EAL4+"
    /// Gets `security_level`
    fn get_security_level(&self) -> String;
}

/// **Genetics Provider Trait**
///
/// and optimization strategies. This enables adaptive and self-improving systems.
pub trait GeneticsProvider: BearDogProvider {
    /// Genetic algorithm result type
    type GeneticResult: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Optimization parameters type
    type OptimizationParams: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **Genetic Optimization**
    ///
    fn optimize(
        &self,
        params: Self::OptimizationParams,
    ) -> impl std::future::Future<Output = Result<Self::GeneticResult, Self::Error>> + Send;

    /// **Evolution Strategy**
    ///
    /// This enables continuous adaptation and learning.
    fn evolve(
        &self,
        current_state: &str,
        fitness_score: f64,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;

    /// **Genetic Analysis**
    ///
    /// Analyzes the genetic diversity and health of the optimization population.
    /// This helps prevent premature convergence and maintains solution quality.
    fn analyze_population(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, f64>, Self::Error>> + Send;
}

/// **Monitoring Provider Trait**
///
/// and observability operations.
pub trait MonitoringProvider: BearDogProvider {
    /// Metric data point type
    type MetricPoint: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Alert configuration type
    type AlertConfig: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **Metric Recording**
    ///
    fn record_metric(
        &self,
        name: &str,
        value: Self::MetricPoint,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **Alert Management**
    ///
    /// Configures an alert based on metric thresholds and conditions.
    /// Alerts should be reliable and avoid false positives.
    fn configure_alert(
        &self,
        name: &str,
        config: Self::AlertConfig,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **System Health Summary**
    ///
    /// Returns a comprehensive health summary of all monitored components.
    /// This provides a high-level view of system status.
    fn system_health(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, Self::Error>> + Send;
}

/// **Adapter Provider Trait**
///
/// Adapters enable `BearDog` to integrate with legacy systems and external services.
pub trait AdapterProvider: BearDogProvider {
    /// External system connection type
    type Connection: Send + Sync + Clone;

    /// Protocol message type
    type Request: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Response type
    type Response: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// **External Connection**
    ///
    /// Establishes a connection to an external system using the appropriate protocol.
    /// Connections should be resilient and support automatic reconnection.
    fn connect(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Connection, Self::Error>> + Send;

    /// **Message Translation**
    ///
    /// This enables seamless integration with heterogeneous systems.
    fn send_request(
        &self,
        connection: &Self::Connection,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send;

    /// **Protocol Support**
    ///
    /// Returns a list of external protocols supported by this adapter.
    /// Gets `supported_operations`
    fn get_supported_operations(&self) -> Vec<String>;

    /// Normalizes an outgoing request to the wire format expected by the remote system.
    fn transform_request(
        &self,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Request, Self::Error>> + Send;

    /// Maps a raw remote response back into the internal [`AdapterProvider::Response`] shape.
    fn transform_response(
        &self,
        response: Self::Response,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send;
}

/// **Workflow Provider Trait**
///
/// and business process automation.
pub trait WorkflowProvider: BearDogProvider {
    /// Workflow definition type
    type Workflow: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Workflow execution context type
    type ExecutionContext: Send + Sync + Clone;

    /// **Workflow Execution**
    ///
    /// Executes a workflow definition with the provided context.
    /// Workflows should be resilient and support error recovery.
    /// Executes workflow
    fn execute_workflow(
        &self,
        workflow: &Self::Workflow,
        context: Self::ExecutionContext,
    ) -> impl std::future::Future<Output = Result<serde_json::Value, Self::Error>> + Send;

    /// **Workflow Status**
    ///
    /// Returns the current status of a running workflow instance.
    /// This enables monitoring and management of long-running processes.
    /// Gets `workflow_status`
    fn get_workflow_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;

    /// **Available Workflows**
    ///
    fn list_workflows(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Workflow>, Self::Error>> + Send;

    /// Delete workflow
    /// Removes workflow
    fn delete_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Get execution history
    /// Gets `execution_history`
    fn get_execution_history(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<serde_json::Value>, Self::Error>> + Send;
}

// Re-export commonly used types for convenience
// Note: These are already imported at the top, so we don't need to re-export them
// pub use beardog_types::canonical::providers_unified::traits::{
//     ProviderInfo, ProviderHealth, ProviderMetrics, ProviderCapability, ProviderType
// };
