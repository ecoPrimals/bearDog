// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health Check System
//!
//! Provides comprehensive health checking capabilities for all system components.
//!
//! # Architecture
//!
//! This module is organized into three sub-modules:
//!
//! - **traits**: Core `HealthChecker` trait definition
//! - **checkers**: Concrete health checker implementations
//! - **aggregator**: System-wide health aggregation
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_monitoring::monitoring::health::{
//!     aggregator::HealthCheckAggregator,
//!     checkers::{DatabaseHealthChecker, HealthCheckerType},
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut aggregator = HealthCheckAggregator::new();
//! aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
//!
//! let overall_status = aggregator.get_overall_status().await?;
//! # Ok(())
//! # }
//! ```

pub mod aggregator;
pub mod checkers;
pub mod traits;

// Re-export commonly used types
pub use aggregator::HealthCheckAggregator;
pub use checkers::{
    CacheHealthChecker, DatabaseHealthChecker, ExternalApiHealthChecker, HealthCheckerType,
    HsmHealthChecker,
};
pub use traits::HealthChecker;
