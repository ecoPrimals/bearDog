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


/// Service Types
///
/// Type definitions for ecosystem services

use super::super::traits::*;
use std::collections::HashMap;
/// Ecosystem service information
#[derive(Debug, Clone)]
pub struct EcosystemService {
    /// Unique identifier for the service
    pub service_id: String,
    /// Identifier of the ecosystem this service belongs to
    pub ecosystem_id: String,
    /// Unique identifier for this service instance
    pub instance_id: String,
    /// Type of service provided
    pub service_type: EcosystemServiceType,
    /// Network endpoints for accessing the service
    pub endpoints: ServiceEndpoints,
    /// List of capabilities provided by this service
    pub capabilities: Vec<Capability>,
    /// When this service was first discovered
    pub discovery_time: chrono::DateTime<chrono::Utc>,
    /// Last time this service was seen as active
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Additional metadata about the service
    pub metadata: HashMap<String, String>,
}
/// Types of ecosystem services
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemServiceType {
    /// Compute services (ToadStool)
    Compute,
    /// Storage services (NestGate)
    Storage,
    /// Communication services (SongBird)
    Communication,
    /// AI services (Squirrel)
    AI,
    /// BioMe services (biomeOS)
    BioMe,
    /// Security services (BearDog)
    Security,
    /// Core services (BearDog Core)
    Core,
    /// Custom service type with name
    Custom(String),
} 
