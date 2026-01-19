//! Generic IPC Server for BearDog
//!
//! **Design Principle**: Primal-agnostic IPC
//! - Provides: Unix socket server
//! - Handles: Capability-based requests
//! - Does NOT know: Who the clients are (Songbird, ToadStool, etc.)
//! - biomeOS routes capability requests to appropriate endpoints

use beardog_core::capabilities::{CapabilityRequest, CapabilityResponse};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Generic IPC message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcMessage {
    /// Capability request
    CapabilityRequest(CapabilityRequest),

    /// Capability response
    CapabilityResponse(CapabilityResponse),

    /// Register client
    Register {
        primal_id: String,
        capabilities: Vec<String>,
    },

    /// Heartbeat
    Ping { from: String },

    /// Heartbeat response
    Pong { to: String },

    /// Generic event notification
    Event {
        event_type: String,
        data: serde_json::Value,
    },
}

/// IPC request handler trait
///
/// Implement this to handle capability requests in a primal-specific way
#[async_trait::async_trait]
pub trait IpcHandler: Send + Sync {
    /// Handle a capability request
    async fn handle_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError>;

    /// Handle a registration request
    async fn handle_register(
        &self,
        primal_id: String,
        capabilities: Vec<String>,
    ) -> Result<(), BearDogError>;

    /// Handle custom events
    async fn handle_event(
        &self,
        event_type: String,
        data: serde_json::Value,
    ) -> Result<(), BearDogError>;
}

/// Generic Unix socket IPC server
pub struct IpcServer {
    socket_path: PathBuf,
    handler: Arc<dyn IpcHandler>,
    active_connections: Arc<RwLock<Vec<String>>>,
}

impl IpcServer {
    /// Create a new IPC server
    pub fn new(socket_path: PathBuf, handler: Arc<dyn IpcHandler>) -> Self {
        Self {
            socket_path,
            handler,
            active_connections: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Start the IPC server
    pub async fn serve(&self) -> Result<(), BearDogError> {
        // Remove existing socket if present
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path).map_err(|e| {
                BearDogError::system(format!("Failed to remove existing socket: {}", e))
            })?;
        }

        // Create Unix listener
        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| BearDogError::system(format!("Failed to bind Unix socket: {}", e)))?;

        info!("🔌 IPC server listening on {:?}", self.socket_path);

        // Accept connections
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let handler = Arc::clone(&self.handler);
                    let connections = Arc::clone(&self.active_connections);

                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, handler, connections).await
                        {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Handle a single connection
    async fn handle_connection(
        stream: UnixStream,
        handler: Arc<dyn IpcHandler>,
        active_connections: Arc<RwLock<Vec<String>>>,
    ) -> Result<(), BearDogError> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    // Connection closed
                    break;
                }
                Ok(_) => {
                    // Parse message
                    let message: IpcMessage = match serde_json::from_str(&line) {
                        Ok(msg) => msg,
                        Err(e) => {
                            warn!("Failed to parse IPC message: {}", e);
                            continue;
                        }
                    };

                    // Handle message
                    let response =
                        Self::handle_message(message, &handler, &active_connections).await;

                    // Send response
                    if let Some(response_msg) = response {
                        let response_json = serde_json::to_string(&response_msg).map_err(|e| {
                            BearDogError::system(format!("Failed to serialize response: {}", e))
                        })?;

                        writer
                            .write_all(response_json.as_bytes())
                            .await
                            .map_err(|e| {
                                BearDogError::system(format!("Failed to write response: {}", e))
                            })?;
                        writer.write_all(b"\n").await.map_err(|e| {
                            BearDogError::system(format!("Failed to write newline: {}", e))
                        })?;
                    }
                }
                Err(e) => {
                    error!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle a single message
    async fn handle_message(
        message: IpcMessage,
        handler: &Arc<dyn IpcHandler>,
        active_connections: &Arc<RwLock<Vec<String>>>,
    ) -> Option<IpcMessage> {
        match message {
            IpcMessage::CapabilityRequest(req) => {
                match handler.handle_capability_request(req).await {
                    Ok(response) => Some(IpcMessage::CapabilityResponse(response)),
                    Err(e) => {
                        error!("Capability request failed: {}", e);
                        None
                    }
                }
            }

            IpcMessage::Register {
                primal_id,
                capabilities,
            } => {
                match handler
                    .handle_register(primal_id.clone(), capabilities)
                    .await
                {
                    Ok(_) => {
                        active_connections.write().await.push(primal_id.clone());
                        info!("✅ Registered primal: {}", primal_id);
                        Some(IpcMessage::Pong { to: primal_id })
                    }
                    Err(e) => {
                        error!("Registration failed: {}", e);
                        None
                    }
                }
            }

            IpcMessage::Ping { from } => Some(IpcMessage::Pong { to: from }),

            IpcMessage::Event { event_type, data } => {
                if let Err(e) = handler.handle_event(event_type, data).await {
                    error!("Event handling failed: {}", e);
                }
                None
            }

            IpcMessage::Pong { .. } | IpcMessage::CapabilityResponse(_) => {
                // These are responses, not requests
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_core::capabilities::{Capability, CapabilityRequest};

    struct TestHandler;

    #[async_trait::async_trait]
    impl IpcHandler for TestHandler {
        async fn handle_capability_request(
            &self,
            request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            Ok(CapabilityResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(serde_json::json!({"message": "test"})),
                error: None,
            })
        }

        async fn handle_register(
            &self,
            _primal_id: String,
            _capabilities: Vec<String>,
        ) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn handle_event(
            &self,
            _event_type: String,
            _data: serde_json::Value,
        ) -> Result<(), BearDogError> {
            Ok(())
        }
    }

    #[test]
    fn test_ipc_message_serialization() {
        let msg = IpcMessage::Ping {
            from: "test".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("ping"));

        let deserialized: IpcMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            IpcMessage::Ping { from } => assert_eq!(from, "test"),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_capability_request_serialization() {
        let req = CapabilityRequest {
            from_primal: "test_primal".to_string(),
            capability: Capability::Encryption {
                algorithms: vec!["ChaCha20".to_string()],
                key_types: vec!["X25519".to_string()],
            },
            params: std::collections::HashMap::new(),
            request_id: "req_123".to_string(),
        };

        let msg = IpcMessage::CapabilityRequest(req);
        let json = serde_json::to_string(&msg).unwrap();

        let deserialized: IpcMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            IpcMessage::CapabilityRequest(req) => {
                assert_eq!(req.from_primal, "test_primal");
                assert_eq!(req.request_id, "req_123");
            }
            _ => panic!("Wrong message type"),
        }
    }
}
