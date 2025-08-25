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


/// Monitoring API Models
///
/// Response types for monitoring API endpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// System health response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthResponse {
    /// Overall system status
    pub status: String,
    /// Timestamp of health check
    pub timestamp: String,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// BearDog version
    pub version: String,
    /// Component health statuses
    pub components: Vec<ComponentStatus>,
    /// Performance overview
    pub performance: PerformanceOverview,
}
/// Individual component status
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Component status
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Last check timestamp
    pub last_check: String,
/// Performance overview
pub struct PerformanceOverview {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Active connections
    pub active_connections: u32,
    /// Request rate (per minute)
    pub request_rate: f64,
/// Metrics response structure for system performance data
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    /// Timestamp of the metrics collection
    /// Performance metrics
    pub performance: beardog_monitoring::PerformanceMetrics,
    /// Resource usage metrics
    pub resources: beardog_monitoring::ResourceMetrics,
    /// Custom metrics
    pub custom: HashMap<String, f64>,
/// Alert information structure
pub struct Alert {
    /// Unique alert identifier
    pub id: String,
    /// Alert severity level
    pub severity: String,
    /// Alert title/summary
    pub title: String,
    /// Detailed alert description
    pub description: String,
    /// Timestamp when alert was created
    pub created_at: String,
    /// Current alert status
    /// Source component that triggered the alert
    pub source: String,
    /// Alert tags for categorization
    pub tags: Vec<String>,
/// Alert rule configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule identifier
    /// Rule name
    /// Rule condition expression
    pub condition: String,
    /// Alert severity when triggered
    /// Rule description
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Notification channels
    pub notifications: Vec<String>,
/// Log entry structure
pub struct LogEntry {
    /// Log entry timestamp
    /// Log level
    pub level: String,
    /// Source component
    /// Log message
    pub message: String,
    /// Additional structured data
    pub fields: HashMap<String, String>,
/// Trace information structure
pub struct TraceInfo {
    /// Trace identifier
    pub trace_id: String,
    /// Trace spans
    pub spans: Vec<SpanInfo>,
    /// Trace duration in microseconds
    pub duration_us: u64,
    /// Trace status
/// Individual span information
pub struct SpanInfo {
    /// Span identifier
    pub span_id: String,
    /// Parent span identifier
    pub parent_span_id: Option<String>,
    /// Operation name
    pub operation_name: String,
    /// Span start time
    pub start_time: String,
    /// Span duration in microseconds
    /// Span tags
    pub tags: HashMap<String, String>,
/// Dashboard configuration
pub struct Dashboard {
    /// Dashboard identifier
    /// Dashboard title
    /// Dashboard description
    /// Dashboard widgets configuration
    pub widgets: Vec<DashboardWidget>,
    /// Dashboard refresh interval in seconds
    pub refresh_interval: u32,
/// Dashboard widget configuration
pub struct DashboardWidget {
    /// Widget identifier
    /// Widget type
    pub widget_type: String,
    /// Widget title
    /// Widget configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Widget position
    pub position: WidgetPosition,
/// Widget position on dashboard
pub struct WidgetPosition {
    /// X coordinate
    pub x: u32,
    /// Y coordinate
    pub y: u32,
    /// Widget width
    pub width: u32,
    /// Widget height
    pub height: u32,
/// Report configuration and metadata
pub struct Report {
    /// Report identifier
    /// Report title
    /// Report type
    pub report_type: String,
    /// Report status
    /// Report creation timestamp
    /// Report completion timestamp
    pub completed_at: Option<String>,
    /// Report file URL (if available)
    pub file_url: Option<String>,
    /// Report parameters
    pub parameters: HashMap<String, serde_json::Value>,
// Request models for various operations
/// Request structure for log search
#[derive(Debug, Deserialize)]
pub struct LogSearchRequest {
    /// Search query
    pub query: String,
    /// Start timestamp
    pub start_time: Option<String>,
    /// End timestamp
    pub end_time: Option<String>,
    /// Maximum number of results
    pub limit: Option<u32>,
    /// Log levels to include
    pub levels: Option<Vec<String>>,
    /// Components to include
    pub sources: Option<Vec<String>>,
/// Request structure for report generation
pub struct GenerateReportRequest {
    /// Report format (pdf, html, json, etc.)
    pub format: String,
    /// Start time for report data
    /// End time for report data
