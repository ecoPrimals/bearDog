// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generic IPC Server for `BearDog`
//!
//! **Design Principle**: Primal-agnostic IPC
//! - Provides: Unix socket server
//! - Handles: Capability-based requests
//! - Does NOT know: Who the clients are (any peer primal)
//! - biomeOS routes capability requests to appropriate endpoints

use beardog_core::capabilities::{CapabilityRequest, CapabilityResponse};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

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
        /// Unique primal identifier
        primal_id: String,
        /// Capabilities offered by this primal
        capabilities: Vec<String>,
    },

    /// Heartbeat
    Ping {
        /// Sender primal identifier
        from: String,
    },

    /// Heartbeat response
    Pong {
        /// Recipient primal identifier
        to: String,
    },

    /// Generic event notification
    Event {
        /// Event type identifier
        event_type: String,
        /// Event payload
        data: serde_json::Value,
    },
}

/// IPC request handler trait
///
/// Implement this to handle capability requests in a primal-specific way
#[expect(
    async_fn_in_trait,
    reason = "concrete dispatch via IpcHandlerBackend; no dyn usage"
)]
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

// --- Test fixtures, enum dispatch, and legacy IpcServer (test-only) ---
#[cfg(test)]
mod test_fixtures {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::{UnixListener, UnixStream};
    use tokio::sync::RwLock;
    use tokio::time::Duration;
    use tracing::{debug, error, info, warn};

    const IPC_READ_TIMEOUT: Duration = Duration::from_secs(30);

    /// Cooperative IPC handler used by unit tests
    #[derive(Clone, Copy, Debug)]
    pub struct IpcTestHandler;

    /// Registration always fails (for error-path tests)
    #[derive(Clone, Copy, Debug)]
    pub struct IpcFailingRegisterHandler;

    /// Event handling always fails (for error-path tests)
    #[derive(Clone, Copy, Debug)]
    pub struct IpcFailingEventHandler;

    /// Capability requests always fail (for error-path tests)
    #[derive(Clone, Copy, Debug)]
    pub struct IpcFailingCapabilityHandler;

    /// Enum over all [`IpcHandler`] implementations used in-tree
    #[derive(Clone, Copy, Debug)]
    pub enum IpcHandlerBackend {
        /// Default successful test handler
        Test(IpcTestHandler),
        /// Registration always fails (for error-path tests)
        FailingRegister(IpcFailingRegisterHandler),
        /// Event handling always fails (for error-path tests)
        FailingEvent(IpcFailingEventHandler),
        /// Capability requests always fail (for error-path tests)
        FailingCapability(IpcFailingCapabilityHandler),
    }

    impl IpcHandler for IpcHandlerBackend {
        async fn handle_capability_request(
            &self,
            request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            match *self {
                Self::Test(h) => IpcHandler::handle_capability_request(&h, request).await,
                Self::FailingRegister(h) => {
                    IpcHandler::handle_capability_request(&h, request).await
                }
                Self::FailingEvent(h) => IpcHandler::handle_capability_request(&h, request).await,
                Self::FailingCapability(h) => {
                    IpcHandler::handle_capability_request(&h, request).await
                }
            }
        }

        async fn handle_register(
            &self,
            primal_id: String,
            capabilities: Vec<String>,
        ) -> Result<(), BearDogError> {
            match *self {
                Self::Test(h) => IpcHandler::handle_register(&h, primal_id, capabilities).await,
                Self::FailingRegister(h) => {
                    IpcHandler::handle_register(&h, primal_id, capabilities).await
                }
                Self::FailingEvent(h) => {
                    IpcHandler::handle_register(&h, primal_id, capabilities).await
                }
                Self::FailingCapability(h) => {
                    IpcHandler::handle_register(&h, primal_id, capabilities).await
                }
            }
        }

        async fn handle_event(
            &self,
            event_type: String,
            data: serde_json::Value,
        ) -> Result<(), BearDogError> {
            match *self {
                Self::Test(h) => IpcHandler::handle_event(&h, event_type, data).await,
                Self::FailingRegister(h) => IpcHandler::handle_event(&h, event_type, data).await,
                Self::FailingEvent(h) => IpcHandler::handle_event(&h, event_type, data).await,
                Self::FailingCapability(h) => IpcHandler::handle_event(&h, event_type, data).await,
            }
        }
    }

    impl IpcHandler for IpcTestHandler {
        async fn handle_capability_request(
            &self,
            request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            Ok(CapabilityResponse {
                request_id: request.request_id,
                status: beardog_core::capabilities::ResponseStatus::Success,
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

    impl IpcHandler for IpcFailingRegisterHandler {
        async fn handle_capability_request(
            &self,
            _request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            unreachable!()
        }

        async fn handle_register(
            &self,
            _primal_id: String,
            _capabilities: Vec<String>,
        ) -> Result<(), BearDogError> {
            Err(BearDogError::business("register failed".to_string()))
        }

        async fn handle_event(
            &self,
            _event_type: String,
            _data: serde_json::Value,
        ) -> Result<(), BearDogError> {
            Ok(())
        }
    }

    impl IpcHandler for IpcFailingEventHandler {
        async fn handle_capability_request(
            &self,
            _request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            unreachable!()
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
            Err(BearDogError::business("event failed".to_string()))
        }
    }

    impl IpcHandler for IpcFailingCapabilityHandler {
        async fn handle_capability_request(
            &self,
            _request: CapabilityRequest,
        ) -> Result<CapabilityResponse, BearDogError> {
            Err(BearDogError::business("capability failed".to_string()))
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

    /// Generic Unix socket IPC server
    pub struct IpcServer {
        pub(super) socket_path: PathBuf,
        handler: Arc<IpcHandlerBackend>,
        active_connections: Arc<RwLock<Vec<String>>>,
    }

    impl IpcServer {
        /// Create a new IPC server
        pub fn new(socket_path: PathBuf, handler: Arc<IpcHandlerBackend>) -> Self {
            Self {
                socket_path,
                handler,
                active_connections: Arc::new(RwLock::new(Vec::new())),
            }
        }

        /// Start the IPC server
        ///
        /// # Errors
        ///
        /// Returns an error if the existing socket file cannot be removed or the Unix listener cannot be bound.
        pub async fn serve(&self) -> Result<(), BearDogError> {
            // Remove existing socket if present
            if self.socket_path.exists() {
                std::fs::remove_file(&self.socket_path).map_err(|e| {
                    BearDogError::system(format!("Failed to remove existing socket: {e}"))
                })?;
            }

            // Create Unix listener
            let listener = UnixListener::bind(&self.socket_path)
                .map_err(|e| BearDogError::system(format!("Failed to bind Unix socket: {e}")))?;

            info!("🔌 IPC server listening on {:?}", self.socket_path);

            // Accept connections
            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        let handler = Arc::clone(&self.handler);
                        let connections = Arc::clone(&self.active_connections);

                        tokio::spawn(async move {
                            if let Err(e) =
                                Self::handle_connection(stream, handler, connections).await
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
        pub(super) async fn handle_connection(
            stream: UnixStream,
            handler: Arc<IpcHandlerBackend>,
            active_connections: Arc<RwLock<Vec<String>>>,
        ) -> Result<(), BearDogError> {
            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();

                let read_result =
                    tokio::time::timeout(IPC_READ_TIMEOUT, reader.read_line(&mut line)).await;

                let bytes_read = match read_result {
                    Err(_elapsed) => {
                        debug!(
                            timeout_secs = IPC_READ_TIMEOUT.as_secs(),
                            "IPC read timed out — closing idle connection"
                        );
                        break;
                    }
                    Ok(Err(e)) => {
                        error!("Failed to read from IPC client: {}", e);
                        break;
                    }
                    Ok(Ok(n)) => n,
                };

                if bytes_read == 0 {
                    break;
                }

                let message: IpcMessage = match serde_json::from_str(&line) {
                    Ok(msg) => msg,
                    Err(e) => {
                        warn!("Failed to parse IPC message: {}", e);
                        continue;
                    }
                };

                let response = Self::handle_message(message, &handler, &active_connections).await;

                if let Some(response_msg) = response {
                    let response_json = serde_json::to_string(&response_msg).map_err(|e| {
                        BearDogError::system(format!("Failed to serialize response: {e}"))
                    })?;

                    writer
                        .write_all(response_json.as_bytes())
                        .await
                        .map_err(|e| {
                            BearDogError::system(format!("Failed to write response: {e}"))
                        })?;
                    writer.write_all(b"\n").await.map_err(|e| {
                        BearDogError::system(format!("Failed to write newline: {e}"))
                    })?;
                }
            }

            Ok(())
        }

        /// Handle a single message
        pub(super) async fn handle_message(
            message: IpcMessage,
            handler: &Arc<IpcHandlerBackend>,
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
                        Ok(()) => {
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
} // mod test_fixtures

#[cfg(test)]
pub(crate) use test_fixtures::*;

#[cfg(test)]
#[path = "ipc_server_tests.rs"]
mod tests;
