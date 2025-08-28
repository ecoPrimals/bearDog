

use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct SystemHealthStatus {

    pub overall_status: HealthStatus,

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub disk_utilization: f64,

    pub network_connectivity: bool,

pub struct ComponentHealthStatus {

    pub components: HashMap<String, ComponentStatus>,

pub struct ComponentStatus {

    pub health: ComponentHealth,

    pub uptime_seconds: u64,

    pub last_check: SystemTime,

pub enum ComponentHealth {

    Down,

pub struct HealthEndpointTest {

    pub status_code: u16,

    pub response_time_ms: u64,

    pub response_valid: bool,

pub struct LivenessProbe {

    pub is_alive: bool,

    pub core_responding: bool,

    pub critical_processes_running: bool,

pub struct ReadinessProbe {

    pub is_ready: bool,

    pub accepting_requests: bool,

    pub dependencies_available: bool,

pub struct MonitoringConfiguration {

    pub cpu_threshold: f64,

    pub memory_threshold: f64,

    pub disk_threshold: f64,

    pub response_time_threshold_ms: u64,

    pub error_rate_threshold: f64,

    pub alert_cooldown_seconds: u64,

pub struct AlertSystem {

    pub alerts_configured: bool,

    pub notification_channels: Vec<String>,

    pub escalation_procedures_defined: bool,}

impl SystemHealthStatus {

    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            disk_utilization: 0.0,
            network_connectivity: false,
        }
    }

    pub fn update_overall_status(&mut self) {

        if self.cpu_utilization > 0.9
            || self.memory_utilization > 0.9
            || self.disk_utilization > 0.95
        {
            self.overall_status = HealthStatus::Critical;
        } else if self.cpu_utilization > 0.8
            || self.memory_utilization > 0.8
            || self.disk_utilization > 0.9
            self.overall_status = HealthStatus::Warning;
        } else if self.network_connectivity {
            self.overall_status = HealthStatus::Healthy;
        } else {

    pub fn is_healthy(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Healthy)

    pub fn has_warnings(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Warning)

    pub fn is_critical(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Critical)

    pub fn resource_summary(&self) -> String {
        format!(
            "CPU: {:.1}%, Memory: {:.1}%, Disk: {:.1}%, Network: {}",
            self.cpu_utilization * 100.0,
            self.memory_utilization * 100.0,
            self.disk_utilization * 100.0,
            if self.network_connectivity {
                "OK"
            } else {
                "DOWN"
            }
        )
impl ComponentHealthStatus {

            components: HashMap::with_capacity(16),

    pub fn update_component(&mut self, name: &str, status: ComponentStatus) {
        self.components.insert(name, status);

    pub fn get_component(&self, name: &str) -> Option<&ComponentStatus> {
        self.components.get(name)

    pub fn healthy_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Healthy))
            .collect()

    pub fn warning_components(&self) -> Vec<(&String, &ComponentStatus)> {
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Warning))

    pub fn critical_components(&self) -> Vec<(&String, &ComponentStatus)> {
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Critical))

    pub fn down_components(&self) -> Vec<(&String, &ComponentStatus)> {
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Down))

    pub fn all_healthy(&self) -> bool {
            .values()
            .all(|status| matches!(status.health, ComponentHealth::Healthy))

    pub fn overall_health(&self) -> ComponentHealth {
        if self
            .components
            .any(|s| matches!(s.health, ComponentHealth::Down))
            ComponentHealth::Down
        } else if self
            .any(|s| matches!(s.health, ComponentHealth::Critical))
            ComponentHealth::Critical
            .any(|s| matches!(s.health, ComponentHealth::Warning))
            ComponentHealth::Warning
            ComponentHealth::Healthy
impl ComponentStatus {

    pub fn new(health: ComponentHealth) -> Self {
            health,
            uptime_seconds: 0,
            last_check: SystemTime::now(),

    pub fn healthy() -> Self {
        Self::new(ComponentHealth::Healthy)

    pub fn warning() -> Self {
        Self::new(ComponentHealth::Warning)

    pub fn critical() -> Self {
        Self::new(ComponentHealth::Critical)

    pub fn down() -> Self {
        Self::new(ComponentHealth::Down)

    pub fn update_check_time(&mut self) {
        self.last_check = SystemTime::now();

        matches!(self.health, ComponentHealth::Healthy)

    pub fn is_down(&self) -> bool {
        matches!(self.health, ComponentHealth::Down)

    pub fn seconds_since_last_check(&self) -> u64 {
        self.last_check.elapsed().unwrap_or_default().as_secs()
impl LivenessProbe {

            is_alive: false,
            core_responding: false,
            critical_processes_running: false,

    pub fn is_alive(&self) -> bool {
        self.is_alive && self.core_responding && self.critical_processes_running

    pub fn update(
        &mut self,
        is_alive: bool,
        core_responding: bool,
        critical_processes_running: bool,
    ) {
        self.is_alive = is_alive;
        self.core_responding = core_responding;
        self.critical_processes_running = critical_processes_running;
impl ReadinessProbe {

            is_ready: false,
            accepting_requests: false,
            dependencies_available: false,

    pub fn is_ready(&self) -> bool {
        self.is_ready && self.accepting_requests && self.dependencies_available

        is_ready: bool,
        accepting_requests: bool,
        dependencies_available: bool,
        self.is_ready = is_ready;
        self.accepting_requests = accepting_requests;
        self.dependencies_available = dependencies_available;
impl HealthEndpointTest {

    pub fn new(status_code: u16, response_time_ms: u64, response_valid: bool) -> Self {
            status_code,
            response_time_ms,
            response_valid,

        self.status_code == 200 && self.response_valid

    pub fn is_fast_enough(&self, threshold_ms: u64) -> bool {
        self.response_time_ms <= threshold_ms
impl MonitoringConfiguration {

    pub fn default() -> Self {
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            disk_threshold: 0.9,
            response_time_threshold_ms: 1000,
            error_rate_threshold: 0.05,
            alert_cooldown_seconds: 300,

    pub fn production() -> Self {
            cpu_threshold: 0.7,
            memory_threshold: 0.7,
            disk_threshold: 0.85,
            response_time_threshold_ms: 500,
            error_rate_threshold: 0.01,
            alert_cooldown_seconds: 600,

    pub fn development() -> Self {
            cpu_threshold: 0.9,
            memory_threshold: 0.9,
            disk_threshold: 0.95,
            response_time_threshold_ms: 2000,
            error_rate_threshold: 0.1,
            alert_cooldown_seconds: 60,

    pub fn cpu_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.cpu_threshold

    pub fn memory_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.memory_threshold

    pub fn disk_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.disk_threshold

    pub fn response_time_exceeds_threshold(&self, response_time_ms: u64) -> bool {
        response_time_ms > self.response_time_threshold_ms

    pub fn error_rate_exceeds_threshold(&self, error_rate: f64) -> bool {
        error_rate > self.error_rate_threshold
impl AlertSystem {

            alerts_configured: false,
            notification_channels: Vec::new(),
            escalation_procedures_defined: false,

    pub fn configure(&mut self, notification_channels: Vec<&str>, escalation_procedures: bool) {
        self.notification_channels = notification_channels;
        self.escalation_procedures_defined = escalation_procedures;
        self.alerts_configured = !self.notification_channels.is_empty();

    pub fn is_configured(&self) -> bool {
        self.alerts_configured && !self.notification_channels.is_empty()

    pub fn add_notification_channel(&mut self, channel: &str) {
        if !self.notification_channels.contains(&channel) {
            self.notification_channels.push(channel);
            self.alerts_configured = true;

    pub fn remove_notification_channel(&mut self, channel: &str) {
        self.notification_channels.retain(|c| c != channel);
impl Default for SystemHealthStatus {}

    fn default() -> Self {
        Self::new()
impl Default for ComponentHealthStatus {}

impl Default for LivenessProbe {
impl Default for ReadinessProbe {}

impl Default for MonitoringConfiguration {
        Self::default()
impl Default for AlertSystem {
