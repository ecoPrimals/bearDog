

use beardog_errors::BearDogResult;
use beardog_types::canonical::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use beardog_types::canonical::crypto::KeyType;
pub use beardog_types::canonical::hsm::{HsmCapabilities, HsmKey as CanonicalHsmKey};
pub use beardog_traits::canonical::HsmProvider;
pub use beardog_types::canonical::health_status::{ComponentStatus as CanonicalComponentStatus, HealthStatus as CanonicalHealthStatus};
pub use beardog_types::canonical::hsm::tiers::HsmTier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreState {
    pub components: HashMap<String, CanonicalComponentStatus>,
    pub overall_health: HealthStatus,
    pub startup_time: DateTime<Utc>,

    pub health_status: HealthStatus,
    pub component_status: HashMap<String, ComponentStatus>,
    pub start_time: DateTime<Utc>,
}

pub struct HealthCheck {
    pub component_name: String,
    pub status: ComponentStatus,
    pub last_check: DateTime<Utc>,
    pub details: HashMap<String, String>,
    pub uptime: Option<chrono::Duration>,

pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_activity: u64,
    pub timestamp: DateTime<Utc>,

impl Default for HsmHealthStatus {}

    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
        }
    }
impl Default for HealthStatus {}

impl Default for ComponentStatus {
        ComponentStatus::Stopped
impl Default for CoreState {
        let now = chrono::Utc::now();
            components: ahash::HashMap::default(),
            overall_health: HealthStatus::default(),
            startup_time: now,
            health_status: HealthStatus::default(),
            component_status: ahash::HashMap::default(),
            start_time: now,}

impl HsmHealthStatus {
    pub fn healthy() -> Self {
        Self::default()}

    pub fn unhealthy(error: &str) -> Self {
            is_healthy: false,
            error_message: Some(error),
impl HealthStatus {}}

impl Default for HsmCapabilities {
            supported_key_types: vec![KeyType::Ed25519, KeyType::EccP256R1],
            max_key_size: 4096,
            supports_attestation: false,
            hardware_backed: false,

#[derive(Debug, Clone)]
pub struct SystemMonitor {
    pub active: bool,}

impl SystemMonitor {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self { active: true })
    pub async fn start(&self) -> Result<(), BearDogError> {
        Ok(())
pub struct GeneticOptimizer {}

impl GeneticOptimizer {
    pub async fn initialize(&self) -> Result<(), BearDogError> {

#[derive(Debug)]
pub struct BearDogSecurityProvider {}

impl BearDogSecurityProvider {

use beardog_errors::BearDogError;
