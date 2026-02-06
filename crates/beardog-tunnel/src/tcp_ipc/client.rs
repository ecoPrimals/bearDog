//! TCP IPC Client for BearDog
//!
//! Universal JSON-RPC client over TCP.

use beardog_errors::BearDogError;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tracing::debug;

/// TCP IPC Client
pub struct TcpIpcClient {
    server_addr: SocketAddr,
}

impl TcpIpcClient {
    /// Create new TCP client
    pub fn new(server_addr: SocketAddr) -> Self {
        Self { server_addr }
    }

    /// Call a JSON-RPC method
    pub async fn call(&self, method: &str, params: Option<Value>) -> Result<Value, BearDogError> {
        debug!("📞 Calling {}", method);

        // Connect to server
        let stream = TcpStream::connect(self.server_addr).await.map_err(|e| {
            BearDogError::system(format!("Failed to connect to {}: {}", self.server_addr, e))
        })?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Build JSON-RPC request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        // Send request (serialization of valid json! macro value is infallible, but handle gracefully)
        let request_str = serde_json::to_string(&request)
            .map_err(|e| BearDogError::system(format!("Failed to serialize request: {}", e)))?
            + "\n";
        writer
            .write_all(request_str.as_bytes())
            .await
            .map_err(|e| BearDogError::system(format!("Failed to send request: {}", e)))?;

        // Read response
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read response: {}", e)))?;

        // Parse response
        let response: Value = serde_json::from_str(&response_line)
            .map_err(|e| BearDogError::system(format!("Invalid JSON response: {}", e)))?;

        // Check for error
        if let Some(error) = response.get("error") {
            return Err(BearDogError::system(format!("RPC error: {}", error)));
        }

        // Return result
        response
            .get("result")
            .cloned()
            .ok_or_else(|| BearDogError::system("No result in response".to_string()))
    }
}
