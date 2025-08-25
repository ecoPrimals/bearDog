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


/// Individual Sovereignty & Peer-to-Peer Sharing API
///
/// **Empowering individuals to control their compute, data, and identity**
/// This module provides APIs that embody BearDog's core values:
/// - 🏠 Individual sovereignty and self-determination
/// - 🤝 Consent-based peer-to-peer resource sharing
/// - 🔒 Privacy-preserving anti-surveillance capabilities
/// - 💫 Human dignity through technology empowerment
/// ## API Organization
/// * **`/api/v1/sovereignty/sharing/*`** - Peer-to-peer resource sharing with consent
/// * **`/api/v1/sovereignty/recovery/*`** - Friend-based account recovery workflows
/// * **`/api/v1/sovereignty/identity/*`** - Self-sovereign identity management  
/// * **`/api/v1/sovereignty/privacy/*`** - Anti-surveillance privacy protections
/// * **`/api/v1/sovereignty/consent/*`** - Consent management for all interactions
/// ## Core Principles
/// - **Individual Control**: You own and control your resources completely
/// - **Consent-Based**: All sharing requires explicit, informed consent
/// - **Friend-to-Friend**: Direct peer relationships without intermediaries
/// - **Privacy-First**: Anti-surveillance by design
/// - **Human Dignity**: Technology serves individuals, not corporations

pub mod consent;
pub mod handlers;
pub mod identity;
pub mod models;
pub mod privacy;
pub mod recovery;
pub mod routes;
pub mod sharing;
pub use routes::create_sovereignty_routes as create_routes;
