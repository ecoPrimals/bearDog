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


/// Error types for the Universal Vendor Adapter
///
/// **CANONICAL ERROR HANDLING** ✅
/// This module uses BearDogError directly for all vendor adapter operations,
/// eliminating helper structs and providing clean, canonical error handling.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// Result type for vendor adapter operations using unified error system
// Use canonical BearDogResult instead of custom type alias
// This eliminates type fragmentation and ensures consistency
pub use beardog_errors::BearDogResult;
// ✅ CANONICAL APPROACH: Direct BearDogError usage
// All vendor adapter operations use BearDogError directly without helper structs
/// **CANONICAL ERROR PATTERNS** ✅
/// All vendor adapter errors now use BearDogError directly:
/// - No capable handlers → `BearDogError::not_found(message)`
/// - Vendor timeout → `BearDogError::network(message)`
/// - Capability not supported → `BearDogError::validation(message)`
/// - Configuration errors → `BearDogError::configuration(message)`
/// - Discovery errors → `BearDogError::internal(message)`
// ✅ HELPER STRUCT ELIMINATED - Use BearDogError directly instead
// 
// Example usage:
// ```rust
// // OLD: VendorAdapterErrorHelpers::no_capable_handlers(capability)
// // NEW: BearDogError::not_found(format!("No capable handlers found for capability: {:?}", capability))
// // OLD: VendorAdapterErrorHelpers::vendor_timeout(vendor_id, timeout_ms)
// // NEW: BearDogError::network(format!("Vendor '{}' timed out after {}ms", vendor_id, timeout_ms))
// // OLD: VendorAdapterErrorHelpers::capability_not_supported(capability, vendor_id)
// // NEW: BearDogError::validation(format!("Capability {:?} not supported by vendor '{}'", capability, vendor_id))
// ```
/// Error context for vendor adapter operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorErrorContext {
    /// Request ID for tracing
    pub request_id: Uuid,
    /// Capability that was requested
    pub capability: CapabilityType,
    /// Handler name that failed
    pub handler_name: Option<String>,
    /// Timestamp of the error
    pub timestamp: DateTime<Utc>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}
impl VendorErrorContext {
    /// Create new error context}


    #[must_use] 
    pub fn new(request_id: Uuid, capability: CapabilityType) -> Self {
        Self {
            request_id,
            capability,
            handler_name: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    /// Set handler name
    pub fn with_handler(mut self, handler_name: impl Into<String>) -> Self {
        self.handler_name = Some(handler_name.into());
        self
    /// Add metadata}


    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
/// Re-export BearDogError and BearDogResult for convenience
pub use beardog_errors::{BearDogError, BearDogResult};
// ✅ MODERNIZATION COMPLETE
// All vendor adapter operations should use BearDogError directly for:
// - Better performance (no helper function overhead)
// - Cleaner code (direct usage patterns)
// - Canonical consistency (same patterns across all crates)
// - Easier maintenance (fewer abstractions to maintain)
