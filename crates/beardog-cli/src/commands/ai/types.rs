

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliResponse<T> {

    pub success: bool,

    pub data: Option<T>,

    pub error: Option<CliError>,

    pub execution_time_ms: u64,

    pub metadata: HashMap<String, serde_json::Value>,
}

pub struct CliError {

    pub code: String,

    pub message: String,

    pub exit_code: i32,

    pub context: Option<serde_json::Value>,

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[default]
    Json,
    Yaml,
    Table,
    Raw,}

impl std::fmt::Display for OutputFormat {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
            Self::Table => write!(f, "table"),
            Self::Raw => write!(f, "raw"),
        }
    }

pub enum StreamType {

    JsonLines,

    Sse,

    Binary,

    Text,

pub struct BatchStatus {

    pub total: usize,

    pub completed: usize,

    pub failed: usize,

    pub results: Vec<BatchResult>,

pub struct BatchResult {

    pub id: String,

    pub data: Option<serde_json::Value>,
    pub error: Option<String>,

    pub processing_time_ms: u64,

pub struct OperationContext {

    pub operation_type: String,

    pub user_id: Option<String>,

    pub request_id: String,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub metadata: HashMap<String, String>,
}

impl<T> CliResponse<T> {

    pub fn success(data: T, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms,
            metadata: HashMap::with_capacity(16),

    pub fn error(error: CliError, execution_time_ms: u64) -> Self {
            success: false,
            data: None,
            error: Some(error),

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self}

impl CliError {

    pub const fn new(code: &str, message: &str, exit_code: i32) -> Self {
            code,
            message,
            exit_code,
            context: None,

    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);

    pub fn validation_error(message: &str) -> Self {
        Self::new("VALIDATION_ERROR".to_string(), message, 1)

    pub fn config_error(message: &str) -> Self {
        Self::new("CONFIG_ERROR".to_string(), message, 2)

    pub fn auth_error(message: &str) -> Self {
        Self::new("AUTH_ERROR".to_string(), message, 3)

    pub fn network_error(message: &str) -> Self {
        Self::new("NETWORK_ERROR".to_string(), message, 4)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]}

    fn test_cli_response_success() {
        let data = "test data";
        let response = CliResponse::success(data, 150);
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert!(response.error.is_none());
        assert_eq!(response.execution_time_ms, 150);
        assert!(response.metadata.is_empty());

    fn test_cli_response_error() {
        let error = CliError::new(
            "TEST_ERROR".to_string(),
            "Test error message".to_string(),
            1,
        );
        let response = CliResponse::<String>::error(error, 75);
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.execution_time_ms, 75);
        let cli_error = response.error.expect("Expected error in response for this test case");
        assert_eq!(cli_error.code, "TEST_ERROR");
        assert_eq!(cli_error.message, "Test error message");
        assert_eq!(cli_error.exit_code, 1);

    fn test_cli_response_with_metadata() {
        let response = CliResponse::success("data", 100)
            .with_metadata("key1".to_string(), json!("value1"))
            .with_metadata("key2".to_string(), json!(42));
        assert_eq!(response.metadata.len(), 2);
        assert_eq!(response.metadata.get("key1"), Some(&json!("value1")));
        assert_eq!(response.metadata.get("key2"), Some(&json!(42)));

    fn test_cli_error_creation() {
        let error = CliError::new("CUSTOM_ERROR".to_string(), "Custom message".to_string(), 5);
        assert_eq!(error.code, "CUSTOM_ERROR");
        assert_eq!(error.message, "Custom message");
        assert_eq!(error.exit_code, 5);
        assert!(error.context.is_none());

    fn test_cli_error_with_context() {
        let context = json!({"field": "username", "value": "invalid"});
        let error = CliError::new("VALIDATION".to_string(), "Invalid input".to_string(), 1)
            .with_context(context.clone());
        assert_eq!(error.context, Some(context));

    fn test_cli_error_convenience_methods() {
        let validation_error = CliError::validation_error("Invalid data".to_string());
        assert_eq!(validation_error.code, "VALIDATION_ERROR");
        assert_eq!(validation_error.exit_code, 1);
        let config_error = CliError::config_error("Config missing".to_string());
        assert_eq!(config_error.code, "CONFIG_ERROR");
        assert_eq!(config_error.exit_code, 2);
        let auth_error = CliError::auth_error("Unauthorized".to_string());
        assert_eq!(auth_error.code, "AUTH_ERROR");
        assert_eq!(auth_error.exit_code, 3);
        let network_error = CliError::network_error("Connection failed".to_string());
        assert_eq!(network_error.code, "NETWORK_ERROR");
        assert_eq!(network_error.exit_code, 4);

    fn test_output_format() -> Result<(), Box<dyn std::error::Error>> {
        let formats = vec![
            OutputFormat::Json,
            OutputFormat::Yaml,
            OutputFormat::Table,
            OutputFormat::Raw,
        ];
        for format in formats {
            let serialized = serde_json::to_string(&format).map_err(|e| {
                tracing::error!("JSON serialization failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON serialization error: {e}"),
                )
            })?;
            let deserialized: OutputFormat = serde_json::from_str(&serialized).map_err(|e| {
                tracing::error!("JSON parsing failed: {}", e);
                    format!("JSON parsing error: {e}"),
            assert_eq!(format.to_string(), deserialized.to_string());

        assert_eq!(format_args!("{:?}", OutputFormat::default().to_string()), "Json");
        Ok(())

    fn test_stream_type() -> Result<(), Box<dyn std::error::Error>> {
        let stream_types = vec![
            StreamType::JsonLines,
            StreamType::Sse,
            StreamType::Binary,
            StreamType::Text,
        for stream_type in stream_types {
            let serialized = serde_json::to_string(&stream_type).map_err(|e| {
            let deserialized: StreamType = serde_json::from_str(&serialized).map_err(|e| {
            assert_eq!(format!("{stream_type:?}"), format_args!("{:?}", deserialized).to_string());
        assert_eq!(format_args!("{:?}", StreamType::default().to_string()), "JsonLines");

    fn test_batch_status() {
        let results = vec![
            BatchResult {
                id: "op1".to_string(),
                success: true,
                data: Some(json!("result1")),
                error: None,
                processing_time_ms: 50,
            },
                id: "op2".to_string(),
                success: false,
                data: None,
                error: Some("Failed to process".to_string()),
                processing_time_ms: 25,
        let batch_status = BatchStatus {
            total: 2,
            completed: 1,
            failed: 1,
            results,
        };
        assert_eq!(batch_status.total, 2);
        assert_eq!(batch_status.completed, 1);
        assert_eq!(batch_status.failed, 1);
        assert_eq!(batch_status.results.len(), 2);
        assert!(batch_status.results[0].success);
        assert!(!batch_status.results[1].success);
        assert_eq!(batch_status.results[0].id, "op1");
        assert_eq!(batch_status.results[1].id, "op2");

    fn test_batch_result() -> Result<(), Box<dyn std::error::Error>> {
        let result = BatchResult {
            id: "test-batch-1".to_string(),
            data: Some(serde_json::json!({"processed": 5, "failed": 2})),
            processing_time_ms: 1500,
        let json = serde_json::to_string(&result)?;
        assert!(json.contains("test-batch-1"));
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("processing_time_ms"));

    fn test_operation_context() {
        let mut metadata = HashMap::with_capacity(16);
        metadata.insert("source".to_string(), "cli".to_string());
        metadata.insert("version".to_string(), "1.0.0".to_string());
        let context = OperationContext {
            operation_type: "security_scan".to_string(),
            user_id: Some("user123".to_string()),
            request_id: "req-456".to_string(),
            timestamp: Utc::now(),
            metadata,
        assert_eq!(context.operation_type, "security_scan");
        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.request_id, "req-456");
        assert_eq!(context.metadata.len(), 2);
        assert_eq!(context.metadata.get("source"), Some(&"cli".to_string()));
        assert_eq!(context.metadata.get("version"), Some(&"1.0.0".to_string()));

    fn test_cli_response_json_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        metadata.insert("command".to_string(), json!("test"));
        metadata.insert("version".to_string(), json!("1.0"));
        let original = CliResponse {
            data: Some("test response data".to_string()),
            execution_time_ms: 250,
        let json = serde_json::to_string(&original).map_err(|e| {
            tracing::error!("JSON serialization failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON serialization error: {e}"),
            )
        })?;
        let deserialized: CliResponse<String> = serde_json::from_str(&json).map_err(|e| {
            tracing::error!("JSON parsing failed: {}", e);
                format!("JSON parsing error: {e}"),
        assert_eq!(original.success, deserialized.success);
        assert_eq!(original.data, deserialized.data);
        assert_eq!(original.execution_time_ms, deserialized.execution_time_ms);
        assert_eq!(original.metadata.len(), deserialized.metadata.len());

    fn test_error_response_json() -> Result<(), Box<dyn std::error::Error>> {
        let error = CliError::validation_error("Field 'name' is required".to_string())
            .with_context(json!({"field": "name"}));
        let response = CliResponse::<String>::error(error, 50);
        let json = serde_json::to_string(&response).map_err(|e| {
        assert!(json.contains("VALIDATION_ERROR"));
        assert!(json.contains("Field 'name' is required"));
        assert!(json.contains("\"success\":false"));
        assert!(!deserialized.success);
        assert!(deserialized.error.is_some());
        let cli_error = deserialized.error.expect("Expected error in deserialized response for this test case");
        assert_eq!(cli_error.code, "VALIDATION_ERROR");

    fn test_integrated_cli_types() -> Result<(), Box<dyn std::error::Error>> {

            operation_type: "batch_operation".to_string(),
            user_id: Some("admin".to_string()),
            request_id: "batch-123".to_string(),

            total: 3,
            completed: 2,
            results: vec![
                BatchResult {
                    id: "1".to_string(),
                    success: true,
                    data: Some(json!("success")),
                    error: None,
                    processing_time_ms: 100,
                },
                    id: "2".to_string(),
                    processing_time_ms: 150,
                    id: "3".to_string(),
                    success: false,
                    data: None,
                    error: Some("Processing failed".to_string()),
                    processing_time_ms: 75,
            ],

        let response = CliResponse::success(batch_status, 325)
            .with_metadata(
                "context".to_string(),
                serde_json::to_value(context).unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    serde_json::Value::Null
                }),
                "format".to_string(),
                serde_json::to_value(OutputFormat::Json).unwrap_or_else(|e| {
                    serde_json::Value::String("json".to_string())
            );
        assert!(response.data.is_some());

        let _deserialized: CliResponse<BatchStatus> = serde_json::from_str(&json).map_err(|e| {
