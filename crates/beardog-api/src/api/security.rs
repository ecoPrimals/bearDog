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


/// # Security API Endpoints
///
/// **AI-First Security Intelligence API**
/// Provides comprehensive REST API access to BearDog's security capabilities:
/// - Threat detection and analysis
/// - ML-powered anomaly detection
/// - Behavioral analysis
/// - Threat intelligence integration
/// - Incident response management
/// - Security provider interfaces
/// This module is organized into focused sub-modules:
/// - `models` - Request/response types and data structures
/// - `handlers` - Endpoint handler implementations
/// - `routes` - Route definitions and configuration
/// - `utils` - Utility functions and helpers

pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;
// Re-export all types for backward compatibility
pub use handlers::*;
pub use models::*;
pub use routes::create_routes;
pub use utils::*;
