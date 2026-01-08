//! BearDog IPC - Inter-Primal Communication
//!
//! Universal adapters for primal-to-primal communication
//!
//! # Zero Vendor Hardcoding
//!
//! This crate provides GENERIC adapters that work with ANY system:
//! - Any primal registry (not just Songbird)
//! - Any service mesh (not just Consul)
//! - Any discovery system
//!
//! # Design Principle
//!
//! Each primal only knows itself. Discovery happens via universal adapters.

pub mod registry_client;

pub use registry_client::{JsonRpcRequest, JsonRpcResponse, PrimalInfo, PrimalRegistryClient};
