//! BearDog HTTP Client Library
//!
//! Easy integration with BearDog's lineage API for Songbird and other primals.
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_client::BearDogClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client
//!     let client = BearDogClient::new("http://localhost:9000");
//!     
//!     // Create genesis lineage
//!     let genesis = client
//!         .create_lineage("tower", None)
//!         .await?;
//!     
//!     println!("Created lineage: {}", genesis.lineage_id);
//!     
//!     // Verify a lineage proof
//!     let verification = client.verify_lineage(&proof).await?;
//!     
//!     if verification.valid && verification.same_genesis {
//!         println!("✅ Same family - auto-accept!");
//!     }
//!     
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

use beardog_genetics::birdsong::{types::LineageMetadata, LineageProof};

mod error;
pub use error::{BearDogClientError, ClientResult};

/// BearDog HTTP client for lineage API
#[derive(Clone)]
pub struct BearDogClient {
    /// Base URL for BearDog API
    base_url: String,
    /// HTTP client
    client: Client,
}

impl BearDogClient {
    /// Create a new BearDog client
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL for BearDog API (e.g., "http://localhost:9000")
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_client::BearDogClient;
    ///
    /// let client = BearDogClient::new("http://localhost:9000");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: base_url.into(),
            client,
        }
    }

    /// Create a new BearDog client with custom HTTP client
    pub fn with_client(base_url: impl Into<String>, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    /// Create a new genesis lineage
    ///
    /// # Arguments
    ///
    /// * `service_type` - Type of service (e.g., "tower", "songbird")
    /// * `metadata` - Optional metadata for the root node
    ///
    /// # Returns
    ///
    /// * `CreateLineageResponse` with the new lineage ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::new("http://localhost:9000");
    /// let genesis = client.create_lineage("tower", None).await?;
    /// println!("Created: {}", genesis.lineage_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_lineage(
        &self,
        service_type: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<CreateLineageResponse> {
        info!("🌱 Creating genesis lineage for: {}", service_type);

        let request = CreateLineageRequest {
            service_type: service_type.to_string(),
            metadata,
        };

        let response = self
            .client
            .post(format!("{}/api/v1/lineage/create", self.base_url))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<CreateLineageResponse> = response.json().await?;

        if api_response.success {
            debug!("✅ Lineage created: {}", api_response.data.lineage_id);
            Ok(api_response.data)
        } else {
            Err(BearDogClientError::ApiError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Spawn a child lineage from parent
    ///
    /// # Arguments
    ///
    /// * `parent_lineage` - Parent lineage ID
    /// * `service_type` - Type of child service
    /// * `metadata` - Optional metadata for child
    ///
    /// # Returns
    ///
    /// * `SpawnLineageResponse` with new lineage ID and proof
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::new("http://localhost:9000");
    /// let child = client
    ///     .spawn_lineage("lineage:tower:...", "songbird", None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn spawn_lineage(
        &self,
        parent_lineage: &str,
        service_type: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<SpawnLineageResponse> {
        info!(
            "👶 Spawning {} from parent {}",
            service_type, parent_lineage
        );

        let request = SpawnLineageRequest {
            parent_lineage: parent_lineage.to_string(),
            service_type: service_type.to_string(),
            metadata,
        };

        let response = self
            .client
            .post(format!("{}/api/v1/lineage/spawn", self.base_url))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<SpawnLineageResponse> = response.json().await?;

        if api_response.success {
            debug!("✅ Child lineage spawned: {}", api_response.data.lineage_id);
            Ok(api_response.data)
        } else {
            Err(BearDogClientError::ApiError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Verify a lineage proof
    ///
    /// # Arguments
    ///
    /// * `proof` - Lineage proof to verify
    ///
    /// # Returns
    ///
    /// * `VerificationResult` with `valid` and `same_genesis` fields
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # use beardog_genetics::birdsong::LineageProof;
    /// # async fn example(proof: LineageProof) -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::new("http://localhost:9000");
    /// let result = client.verify_lineage(&proof).await?;
    ///
    /// if result.valid && result.same_genesis {
    ///     println!("✅ Auto-accept - same family!");
    /// } else if result.valid {
    ///     println!("⚠️  Different family - prompt user");
    /// } else {
    ///     println!("❌ Invalid proof");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_lineage(&self, proof: &LineageProof) -> ClientResult<VerificationResult> {
        debug!("🔍 Verifying lineage proof for: {}", proof.node_id);

        let request = VerifyProofRequest {
            proof: proof.clone(),
        };

        let response = self
            .client
            .post(format!("{}/api/v1/lineage/proof/verify", self.base_url))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<VerificationResult> = response.json().await?;

        if api_response.success {
            let result = api_response.data;
            if result.valid {
                debug!("✅ Proof verified (same_genesis: {})", result.same_genesis);
            } else {
                warn!("❌ Proof verification failed");
            }
            Ok(result)
        } else {
            Err(BearDogClientError::ApiError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Check if two lineages share the same family
    ///
    /// # Arguments
    ///
    /// * `lineage_a` - First lineage ID
    /// * `lineage_b` - Second lineage ID
    ///
    /// # Returns
    ///
    /// * `SameFamilyResponse` with `same_family` boolean and optional `common_ancestor`
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::new("http://localhost:9000");
    /// let result = client
    ///     .same_family("lineage:tower:...", "lineage:songbird:...")
    ///     .await?;
    ///
    /// if result.same_family {
    ///     println!("Same family! Ancestor: {:?}", result.common_ancestor);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn same_family(
        &self,
        lineage_a: &str,
        lineage_b: &str,
    ) -> ClientResult<SameFamilyResponse> {
        debug!("🔍 Checking family relationship");

        let request = SameFamilyRequest {
            lineage_a: lineage_a.to_string(),
            lineage_b: lineage_b.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/api/v1/lineage/same_family", self.base_url))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<SameFamilyResponse> = response.json().await?;

        if api_response.success {
            Ok(api_response.data)
        } else {
            Err(BearDogClientError::ApiError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Get current lineage state
    ///
    /// # Returns
    ///
    /// * `CurrentLineageResponse` with current lineage info
    pub async fn get_current_lineage(&self) -> ClientResult<CurrentLineageResponse> {
        debug!("📊 Getting current lineage state");

        let response = self
            .client
            .get(format!("{}/api/v1/lineage/current", self.base_url))
            .send()
            .await?;

        let api_response: ApiResponse<CurrentLineageResponse> = response.json().await?;

        if api_response.success {
            Ok(api_response.data)
        } else {
            Err(BearDogClientError::ApiError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Health check
    ///
    /// # Returns
    ///
    /// * `true` if BearDog API is reachable and healthy
    pub async fn health_check(&self) -> bool {
        match self
            .client
            .get(format!("{}/api/v1/health", self.base_url))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

// ====================================================================================
// Request/Response Types
// ====================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateLineageRequest {
    service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<LineageMetadata>,
}

/// Response from lineage creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLineageResponse {
    /// Lineage ID
    pub lineage_id: String,
    /// Creation timestamp
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpawnLineageRequest {
    parent_lineage: String,
    service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<LineageMetadata>,
}

/// Response from lineage spawn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnLineageResponse {
    /// New child lineage ID
    pub lineage_id: String,
    /// Lineage proof for the child
    pub proof: LineageProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyProofRequest {
    proof: LineageProof,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether proof is valid
    pub valid: bool,
    /// Whether they share the same genesis (KEY for auto-accept)
    pub same_genesis: bool,
    /// Optional verification message
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SameFamilyRequest {
    lineage_a: String,
    lineage_b: String,
}

/// Response from same family check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SameFamilyResponse {
    /// Whether they share the same genesis
    pub same_family: bool,
    /// Common ancestor node ID (if they're in the same family)
    pub common_ancestor: Option<String>,
}

/// Response from current lineage query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentLineageResponse {
    /// Current lineage ID
    pub lineage_id: String,
    /// Genesis (root) node ID
    pub genesis: String,
    /// Children of this node
    pub children: Vec<String>,
    /// Depth in lineage tree
    pub depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BearDogClient::new("http://localhost:9000");
        assert_eq!(client.base_url, "http://localhost:9000");
    }

    #[test]
    fn test_verification_result() {
        let result = VerificationResult {
            valid: true,
            same_genesis: true,
            message: None,
        };

        assert!(result.valid);
        assert!(result.same_genesis);
    }

    #[test]
    fn test_same_family_response() {
        let response = SameFamilyResponse {
            same_family: true,
            common_ancestor: Some("root-node".to_string()),
        };

        assert!(response.same_family);
        assert_eq!(response.common_ancestor, Some("root-node".to_string()));
    }
}
