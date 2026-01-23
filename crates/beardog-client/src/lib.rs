//! # BearDog Client Library (Tower Atomic Edition)
//!
//! **EVOLVED**: Now uses Tower Atomic (Unix sockets + JSON-RPC) instead of HTTP!
//!
//! Easy integration with BearDog's lineage API via **Pure Rust IPC**.
//!
//! ## Evolution
//!
//! - **Before**: HTTP client using reqwest (had ring dependency via rustls)
//! - **After**: Tower Atomic client using Unix sockets (100% Pure Rust!)
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_client::BearDogClient;
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to BearDog via Unix socket (not HTTP!)
//!     let mut client = BearDogClient::connect().await?;
//!     
//!     // Create genesis lineage
//!     let genesis = client
//!         .create_lineage("tower", None)
//!         .await?;
//!     
//!     println!("Created lineage: {}", genesis["lineage_id"]);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐                          ┌─────────────┐
//! │  Your App    │   Unix Socket JSON-RPC   │   BearDog   │
//! │  (Client)    │ ──────────────────────>  │   (Server)  │
//! └──────────────┘                          └─────────────┘
//!                                                  ↓
//!                                            Lineage API
//!                                            (create, verify, extend)
//! ```

use beardog_tower_atomic::Client as AtomicClient;
use serde_json::{json, Value};
use tracing::{debug, info};

mod error;
pub use error::{BearDogClientError, ClientResult};

use beardog_genetics::birdsong::{types::LineageMetadata, LineageProof};

/// BearDog client for lineage API (Tower Atomic edition)
///
/// Uses Unix sockets + JSON-RPC instead of HTTP for 100% Pure Rust communication.
pub struct BearDogClient {
    /// Tower Atomic client
    client: AtomicClient,
}

impl BearDogClient {
    /// Connect to BearDog via Tower Atomic (Unix socket discovery)
    ///
    /// Automatically discovers BearDog's Unix socket path.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_client::BearDogClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::connect().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect() -> ClientResult<Self> {
        info!("🔌 Connecting to BearDog via Tower Atomic");

        let client = AtomicClient::connect("beardog")
            .await
            .map_err(|e| BearDogClientError::ConnectionFailed(e.to_string()))?;

        info!("✅ Connected to BearDog via Unix socket");

        Ok(Self { client })
    }

    /// Create a new genesis lineage
    ///
    /// # Arguments
    ///
    /// * `service_type` - Type of service (e.g., "tower", "songbird")
    /// * `metadata` - Optional metadata for the root node
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let genesis = client.create_lineage("tower", None).await?;
    /// println!("Created: {}", genesis["lineage_id"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_lineage(
        &mut self,
        service_type: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<Value> {
        info!("🌱 Creating genesis lineage for: {}", service_type);

        let params = json!({
            "service_type": service_type,
            "metadata": metadata,
        });

        let response = self
            .client
            .call("lineage.create", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Created lineage: {:?}", response);
        Ok(response)
    }

    /// Verify a lineage proof
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # use beardog_genetics::birdsong::LineageProof;
    /// # async fn example(proof: LineageProof) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let verification = client.verify_lineage(&proof).await?;
    /// 
    /// if verification["valid"].as_bool().unwrap_or(false) {
    ///     println!("✅ Valid lineage!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_lineage(&mut self, proof: &LineageProof) -> ClientResult<Value> {
        info!("🔍 Verifying lineage proof");

        let params = json!({
            "proof": proof,
        });

        let response = self
            .client
            .call("lineage.verify", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Verification result: {:?}", response);
        Ok(response)
    }

    /// Extend an existing lineage
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example(lineage_id: String, parent_id: String) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let extended = client.extend_lineage(&lineage_id, &parent_id, None).await?;
    /// println!("Extended: {}", extended["node_id"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn extend_lineage(
        &mut self,
        lineage_id: &str,
        parent_id: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<Value> {
        info!("🌿 Extending lineage: {}", lineage_id);

        let params = json!({
            "lineage_id": lineage_id,
            "parent_id": parent_id,
            "metadata": metadata,
        });

        let response = self
            .client
            .call("lineage.extend", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Extended lineage: {:?}", response);
        Ok(response)
    }

    /// Get lineage history
    pub async fn get_lineage(&mut self, lineage_id: &str) -> ClientResult<Value> {
        info!("📜 Getting lineage: {}", lineage_id);

        let params = json!({
            "lineage_id": lineage_id,
        });

        let response = self
            .client
            .call("lineage.get", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Retrieved lineage: {:?}", response);
        Ok(response)
    }
}

// Legacy type aliases for backward compatibility
/// Create lineage response (now returns generic Value)
pub type CreateLineageResponse = Value;

/// Verification response (now returns generic Value)
pub type VerificationResponse = Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running BearDog instance
    async fn test_connect() {
        let client = BearDogClient::connect().await;
        // Should connect if BearDog is running
        // Otherwise will fail with connection error (expected)
        match client {
            Ok(_) => println!("✅ Connected to BearDog"),
            Err(e) => println!("⚠️  BearDog not running: {}", e),
        }
    }
}
