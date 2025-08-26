

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthResponse {

    pub status: String,

    pub timestamp: String,

    pub uptime_seconds: u64,

    pub version: String,

    pub components: Vec<ComponentStatus>,

    pub performance: PerformanceOverview,
}

pub struct ComponentStatus {

    pub name: String,

    pub response_time_ms: u64,

    pub last_check: String,

pub struct PerformanceOverview {

    pub cpu_usage_percent: f64,

    pub memory_usage_percent: f64,

    pub disk_usage_percent: f64,

    pub active_connections: u32,

    pub request_rate: f64,

#[derive(Debug, Serialize)]
pub struct MetricsResponse {

    pub performance: beardog_monitoring::PerformanceMetrics,

    pub resources: beardog_monitoring::ResourceMetrics,

    pub custom: HashMap<String, f64>,

pub struct Alert {

    pub id: String,

    pub severity: String,

    pub title: String,

    pub description: String,

    pub created_at: String,

    pub source: String,

    pub tags: Vec<String>,

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertRule {

    pub condition: String,

    pub enabled: bool,

    pub notifications: Vec<String>,

pub struct LogEntry {

    pub level: String,

    pub message: String,

    pub fields: HashMap<String, String>,

pub struct TraceInfo {

    pub trace_id: String,

    pub spans: Vec<SpanInfo>,

    pub duration_us: u64,

pub struct SpanInfo {

    pub span_id: String,

    pub parent_span_id: Option<String>,

    pub operation_name: String,

    pub start_time: String,

    pub tags: HashMap<String, String>,

pub struct Dashboard {

    pub widgets: Vec<DashboardWidget>,

    pub refresh_interval: u32,

pub struct DashboardWidget {

    pub widget_type: String,

    pub config: HashMap<String, serde_json::Value>,

    pub position: WidgetPosition,

pub struct WidgetPosition {

    pub x: u32,

    pub y: u32,

    pub width: u32,

    pub height: u32,

pub struct Report {

    pub report_type: String,

    pub completed_at: Option<String>,

    pub file_url: Option<String>,

    pub parameters: HashMap<String, serde_json::Value>,

#[derive(Debug, Deserialize)]
pub struct LogSearchRequest {

    pub query: String,

    pub start_time: Option<String>,

    pub end_time: Option<String>,

    pub limit: Option<u32>,

    pub levels: Option<Vec<String>>,

    pub sources: Option<Vec<String>>,

pub struct GenerateReportRequest {

    pub format: String,

