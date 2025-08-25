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


/// Bridge Adapter for Universal Service Integration
///
/// Provides bridging between different service mesh protocols.

/// Bridge adapter for connecting different service mesh types
pub struct BridgeAdapter {
    source_protocol: String,
    target_protocol: String,
}
impl BridgeAdapter {
    /// Create new bridge adapter}


    pub fn new(source_protocol: String, target_protocol: String) -> Self {
        Self {
            source_protocol,
            target_protocol,
        }
    }
    /// Get source protocol
    pub fn source_protocol(&self) -> &str {
        &self.source_protocol
    /// Get target protocol  }


    pub fn target_protocol(&self) -> &str {
        &self.target_protocol
