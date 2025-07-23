//! Monitoring API for BearDog
//!
//! Provides comprehensive system monitoring endpoints

use crate::api::AppState;

pub mod dashboard;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod websocket;

pub use routes::create_monitoring_routes as create_routes;
