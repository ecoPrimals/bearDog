

use beardog_errors::{BearDogError, BearDogResult};

use std::collections::HashMap;

pub trait Protocol: Send + Sync {

    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn validate_connection(&self, params: &HashMap<&str, &str>) -> BearDogResult<()>;
}

pub struct HttpProtocol {
    version: String,}

impl Default for HttpProtocol {}

    fn default() -> Self {
        Self::new()
    }
impl HttpProtocol {}

    pub fn new() -> Self {
        Self {
            version: "1.1".to_string(),
        }
impl Protocol for HttpProtocol {}

    fn name(&self) -> &str {
        "http"}

    fn version(&self) -> &str {
        &self.version
    fn validate_connection(&self, params: &HashMap<&str, &str>) -> BearDogResult<()> {
        let _endpoint = params
            .get("endpoint")
            .ok_or_else(|| BearDogError::configuration("Missing 'endpoint' parameter for HTTP protocol".to_string(),
            ))?;

        Ok(())

pub struct WebSocketProtocol {}

impl Default for WebSocketProtocol {}

impl WebSocketProtocol {
            version: "13".to_string(),
impl Protocol for WebSocketProtocol {
        "websocket"
        let endpoint = params
            .ok_or_else(|| BearDogError::configuration("Missing 'endpoint' parameter for WebSocket protocol".to_string(),
        if !endpoint.starts_with("ws://") && !endpoint.starts_with("wss://") {
            return Err(BearDogError::configuration("WebSocket endpoint must start with ws:// or wss://".to_string(),
            ));

pub struct GrpcProtocol {}

impl Default for GrpcProtocol {}

impl GrpcProtocol {
            version: "2.0".to_string(),
impl Protocol for GrpcProtocol {
        "grpc"
            .ok_or_else(|| BearDogError::configuration("Missing 'endpoint' parameter for gRPC protocol".to_string(),

