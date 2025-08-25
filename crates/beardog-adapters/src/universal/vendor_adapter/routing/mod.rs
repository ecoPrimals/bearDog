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


/// # Universal Vendor Adapter Routing System
///
/// **MODULAR ROUTING ARCHITECTURE** - Replaces monolithic strategies.rs (950 lines)
/// This module breaks down routing logic into focused, maintainable components:
/// - `traits.rs` - Core routing traits and interfaces (~150 lines)
/// - `performance.rs` - Performance-based routing strategies (~250 lines)
/// - `multi_criteria.rs` - Multi-criteria routing strategies (~250 lines)
/// - `circuit_breaker.rs` - Circuit breaker routing strategies (~200 lines)
/// - `adaptive.rs` - Adaptive and learning-based routing (~200 lines)
/// **Benefits:**
/// - ✅ **Focused modules** - Each <300 lines, single responsibility
/// - ✅ **Better testability** - Isolated strategy implementations
/// - ✅ **Easier extension** - Clear plugin points for new strategies
/// - ✅ **Reduced complexity** - Simpler mental model for each strategy type

pub mod adaptive;
pub mod circuit_breaker;
pub mod multi_criteria;
pub mod performance;
pub mod router;
pub mod traits;
// Re-export router components
pub use router::{RouterConfig, UniversalRequestRouter};
// Re-export core routing traits
pub use traits::{RoutingContext, RoutingDecision, RoutingStrategy};
// Re-export all strategy implementations
pub use adaptive::{AdaptiveRouting, LearningRouting, RoutingHistory};
pub use circuit_breaker::{CircuitBreakerRouting, CircuitConfig, CircuitState};
pub use multi_criteria::{ExecutionMetric, MultiCriteriaRouting, RoutingWeights};
pub use performance::{PerformanceFirstRouting, PerformanceMetric};
// Re-export commonly used functionality
pub use adaptive::create_adaptive_routing;
pub use circuit_breaker::create_circuit_breaker_routing;
pub use multi_criteria::create_balanced_routing;
pub use performance::create_performance_routing;
