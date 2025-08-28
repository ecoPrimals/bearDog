

use beardog_errors::BearDogError;

pub struct DistributedTraceCollector {}

impl DistributedTraceCollector {
    pub fn new() -> Result<Self, BearDogError> { Ok(Self {}) }
    pub async fn initialize(&self) -> Result<(), BearDogError> { Ok(()) }
    pub async fn start(&self) -> Result<(), BearDogError> { Ok(()) }
    pub async fn is_healthy(&self) -> Result<bool, BearDogError> { Ok(true) }
    pub async fn shutdown(&self) -> Result<(), BearDogError> { Ok(()) }
} 