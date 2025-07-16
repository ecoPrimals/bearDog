//! # Security API Endpoints
//!
//! **AI-First Security Intelligence API**
//!
//! Provides comprehensive REST API access to BearDog's security capabilities:
//! - Threat detection and analysis
//! - ML-powered anomaly detection
//! - Behavioral analysis
//! - Threat intelligence integration
//! - Incident response management
//! - Security provider interfaces
//!
//! This module is organized into focused sub-modules:
//! - `models` - Request/response types and data structures
//! - `handlers` - Endpoint handler implementations
//! - `routes` - Route definitions and configuration
//! - `utils` - Utility functions and helpers

pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

// Re-export all types for backward compatibility
pub use handlers::*;
pub use models::*;
pub use routes::create_routes;
pub use utils::*;
