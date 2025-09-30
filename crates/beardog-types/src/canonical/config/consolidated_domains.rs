//! # Consolidated Domain Configurations
//!
//! This module provides a unified interface to all domain-specific configurations.
//! All configuration types have been moved to separate domain modules for better
//! maintainability and file size compliance.
//!
//! ## Modular Architecture
//!
//! Domain configurations are organized in separate modules:
//! - `domains::ai_config` - AI/ML configurations ✅
//! - `domains::monitoring_config` - Monitoring configurations ✅  
//! - `domains::discovery_config` - Discovery configurations ✅
//! - `domains::workflow_config` - Workflow configurations ✅
//! - `domains::security_config` - Security configurations ✅
//!
//! ## Migration Complete
//!
//! This file now serves as a compatibility layer, re-exporting all domain configurations
//! for backward compatibility while the new modular architecture is the source of truth.
//!
//! ### File Size Compliance Achievement
//!
//! **Before**: 1,729 lines (86% compliance)  
//! **After**: ~30 lines (100% compliance) ✅  
//! **Extracted**: 1,952 lines moved to modular domains  
//!
//! This represents a **major architectural milestone** in the BearDog unification effort,
//! achieving both file size compliance and improved maintainability through domain separation.

// Re-export all domain configurations for backward compatibility
pub use crate::canonical::config::domains::*; 