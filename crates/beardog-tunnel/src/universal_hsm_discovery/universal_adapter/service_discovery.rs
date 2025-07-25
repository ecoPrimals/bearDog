//! Service Discovery
//!
//! Network service discovery and registration

use super::core_types::*;
use beardog_errors::BearDogResult;

pub struct ServiceDiscoveryClient;

impl ServiceDiscoveryClient {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
}

pub struct NetworkService;
pub struct BearDogServiceRegistration;

// TODO: Extract complete service discovery from original file 