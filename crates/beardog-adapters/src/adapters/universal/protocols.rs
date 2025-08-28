use beardog_errors::BearDogError;

use std::collections::HashMap;

pub trait Protocol: Send + Sync {
    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn validate_connection(&self, params: &HashMap<&str, &str>) -> Result<(), BearDogError>;
}

pub struct HttpProtocol {
    version: String,
}

impl Default for HttpProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProtocol {
    pub fn new() -> Self {
        Self {
            version: "1.1".to_string(),
        }
    }
}

impl Protocol for HttpProtocol {
    fn name(&self) -> &str {
        "http"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_connection(&self, params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        let _endpoint = params.get("endpoint").ok_or_else(|| {
            BearDogError::configuration(
                "Missing 'endpoint' parameter for HTTP protocol".to_string(),
            )
        })?;

        Ok(())
    }
}

pub struct WebSocketProtocol {
    #[allow(dead_code)] // Future protocol versioning functionality
    version: String,
}

impl Default for WebSocketProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketProtocol {
    pub fn new() -> Self {
        Self {
            version: "13".to_string(),
        }
    }
}

pub struct GrpcProtocol {
    version: String,
}

impl Default for GrpcProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcProtocol {
    pub fn new() -> Self {
        Self {
            version: "2.0".to_string(),
        }
    }
}

impl Protocol for GrpcProtocol {
    fn name(&self) -> &str {
        "grpc"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_connection(&self, params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        let _endpoint = params.get("endpoint").ok_or_else(|| {
            BearDogError::configuration(
                "Missing 'endpoint' parameter for gRPC protocol".to_string(),
            )
        })?;
        Ok(())
    }
}
