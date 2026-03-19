// SPDX-License-Identifier: AGPL-3.0-only

//! # Enhanced Unified Error System - Modular Architecture
//!
//! This module provides the **ultimate unified error system** split into
//! manageable, modular components while maintaining comprehensive error handling.
//!
//! ## 🎯 **Modular Structure**
//!
//! The unified error system is organized into logical modules:
//!
//! - **`enhanced_error`**: EnhancedBearDogError and core error types
//! - **`context`**: ErrorContext, ErrorLocation, and contextual information
//! - **`recovery`**: ErrorRecovery, ErrorRemediation, and recovery mechanisms
//! - **`analytics`**: ErrorAnalytics, ErrorPattern, and error analysis
//!
//! ## 🏗️ **Architecture Benefits**
//!
//! - **Maintainability**: Each module under 300 lines
//! - **Single Source of Truth**: All error handling unified in one system
//! - **Comprehensive**: Rich context, recovery, and analytics
//! - **Performance**: Efficient error handling and correlation
//! - **Modularity**: Clear separation of error handling concerns

// Import all modular components
pub mod analytics;
pub mod context;
pub mod enhanced_error;
pub mod recovery;

// Re-export all error types
pub use analytics::*;
pub use context::*;
pub use enhanced_error::*;
pub use recovery::*;
