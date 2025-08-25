// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Protocol Implementations for Universal Adapter
///
/// Protocol-agnostic communication layer
use beardog_errors::{BearDogError, BearDogResult};

use std::collections::HashMap;
/// Protocol trait for universal adapter
pub trait Protocol: Send + Sync {
    /// Protocol name
    fn name(&self) -> &str;
    /// Protocol version
    fn version(&self) -> &str;
    /// Validate connection parameters
    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()>;
}
/// HTTP protocol implementation
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
    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()> {
        let _endpoint = params
            .get("endpoint")
            .ok_or_else(|| BearDogError::configuration("Missing 'endpoint' parameter for HTTP protocol".to_string(),
            ))?;
        // Basic URL validation could go here
        Ok(())
/// WebSocket protocol implementation
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
/// gRPC protocol implementation}


pub struct GrpcProtocol {}


impl Default for GrpcProtocol {}


impl GrpcProtocol {
            version: "2.0".to_string(),
impl Protocol for GrpcProtocol {
        "grpc"
            .ok_or_else(|| BearDogError::configuration("Missing 'endpoint' parameter for gRPC protocol".to_string(),
        // gRPC-specific validation could go here
