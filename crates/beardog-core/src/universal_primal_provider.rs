

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
    /// The version value
    pub version: String,

    /// The primal type value
    pub primal_type: PrimalType,

    /// The description value
    pub description: String,

    /// The maintainer value
    pub maintainer: String,

    /// The ecosystem role value
    pub ecosystem_role: EcosystemRole,

    /// The ai first score value
    pub ai_first_score: f64,
}

#[derive(Debug, Clone)]
    /// The endpoint value
    pub endpoint: ServiceEndpoint,

    /// Collection of capabilities
    pub capabilities: Vec<PrimalCapability>,

    /// The health value
    pub health: ServiceHealth,

pub struct ServiceEndpoint {

    /// The protocol value
    pub protocol: String,

    /// The host value
    pub host: String,

    /// Number of port
    pub port: u16,

    /// The path value
    pub path: String,

    /// The security value
    pub security: EndpointSecurity,

pub enum ServiceHealth {


    /// Represents healthy variant
    Healthy,


    /// State indicating degraded
    Degraded,


    /// Represents unhealthy variant
    Unhealthy,


    /// Unknown or undefined state
    Unknown,

pub struct EndpointSecurity {

    /// Whether require_tls is enabled
    pub require_tls: bool,

    /// Whether require_client_cert is enabled
    pub require_client_cert: bool,

    /// Whether require_api_key is enabled
    pub require_api_key: bool,

    /// Collection of custom auth
    pub custom_auth: Vec<String>,
}

pub struct ServiceContext {


    pub request_id: Uuid,

    /// The source value
    pub source: PrimalIdentity,

    /// The security value
    pub security: SecurityContext,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,

pub struct PrimalIdentity {


    pub node_id: String,

pub struct SecurityContext {

    /// Optional auth token
    pub auth_token: Option<String>,

    /// Optional client cert
    pub client_cert: Option<String>,

    /// Number of clearance_level
    pub clearance_level: u8,

    /// Whether encryption_required is enabled
    pub encryption_required: bool,}

impl Default for PrimalMetadata {}

    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "AI-First Security and Compliance Management".to_string(),
            maintainer: "BearDog Security Team".to_string()]
/// Types of universal provider
pub enum UniversalProviderType {
    /// Represents compute variant
    Compute,
    /// Represents mesh variant
    Mesh,
    /// Represents storage variant
    Storage,
    /// Represents intelligence variant
    Intelligence,
    /// Represents security variant
    Security,

    /// Represents custom variant
    Custom(String),
}

