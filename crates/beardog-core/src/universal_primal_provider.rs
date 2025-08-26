

use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SecurityResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalMetadata {

    pub name: String,

    pub version: String,

    pub primal_type: PrimalType,

    pub description: String,

    pub maintainer: String,

    pub ecosystem_role: EcosystemRole,

    pub ai_first_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimalType {

    BearDog,

    Songbird,

    NestGate,

    UniversalCompute,

    AIFirstCitizen,

    Custom(String),

pub enum EcosystemRole {

    SecurityProvider,

    NetworkOrchestrator,

    StorageProvider,

    ComputeOrchestrator,

    AIProvider,

    ServiceConsumer,

    ExternalBridge,

pub enum PrimalCapability {

    Security,

    AI,

    Monitoring,

    Storage,

    Networking,

    Compliance,

    Workflow,

    ThreatDetection,

    KeyManagement,

pub struct PrimalService {

    pub id: String,

    pub endpoint: ServiceEndpoint,

    pub capabilities: Vec<PrimalCapability>,

    pub health: ServiceHealth,

pub struct ServiceEndpoint {

    pub protocol: String,

    pub host: String,

    pub port: u16,

    pub path: String,

    pub security: EndpointSecurity,

pub enum ServiceHealth {

    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

pub struct EndpointSecurity {

    pub require_tls: bool,

    pub require_client_cert: bool,

    pub require_api_key: bool,

    pub custom_auth: Vec<String>,
}

pub struct ServiceContext {

    pub request_id: Uuid,

    pub source: PrimalIdentity,

    pub security: SecurityContext,

    pub metadata: HashMap<String, String>,

pub struct PrimalIdentity {

    pub node_id: String,

pub struct SecurityContext {

    pub auth_token: Option<String>,

    pub client_cert: Option<String>,

    pub clearance_level: u8,

    pub encryption_required: bool,}

impl Default for PrimalMetadata {}

    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            primal_type: PrimalType::BearDog,
            description: "AI-First Security and Compliance Management".to_string(),
            maintainer: "BearDog Security Team".to_string(),
            ecosystem_role: EcosystemRole::SecurityProvider,
            ai_first_score: 0.95, // Gold standard compliance
        }
    }
impl Default for ServiceEndpoint {
            protocol: "https".to_string(),
            host: beardog_types::config::constants::network::get_default_host(),
            port: 8443,
            path: "/api/v1".to_string(),
            security: EndpointSecurity {
                require_tls: true,
                require_client_cert: false,
                require_api_key: true,
                custom_auth: vec!["beardog-auth".to_string()],
            },
