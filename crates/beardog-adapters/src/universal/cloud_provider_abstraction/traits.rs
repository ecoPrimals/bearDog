

// async_trait no longer needed - using native fn
use beardog_errors::BearDogError;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

use super::types::*;


pub trait CloudProvider: Send + Sync {


    fn provider_name(&self) -> &'static str;
    fn provider_type(&self) -> CloudProviderType;

    /// Initializes componentialize
    fn initialize(&self, config: &CloudConfig) -> Result<(), BearDogError>;
    fn health_check(Send + Sync {
    /// Creates key
    fn create_key(&self, key_spec: &KeySpecification) -> Result<CloudKey, BearDogError>;
    fn encrypt_data(&str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> + Send;
    fn decrypt_data(&str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> + Send;
    fn rotate_key(&self, key_id: &str) -> Result<String, BearDogError>;
    fn list_keys(&self) -> Result<Vec<CloudKey>, BearDogError>> + Send;
    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError>;
}


pub trait ComputeService: Send + Sync {
    /// Creates instance
    fn create_instance(&self, spec: &InstanceSpecification) -> Result<ComputeInstance, BearDogError>;
    fn list_instances(&self) -> Result<Vec<ComputeInstance>, BearDogError>> + Send;
    fn terminate_instance(&self, instance_id: &str) -> Result<(), BearDogError>;
    /// Gets instance_status
    fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus, BearDogError>;
}

pub trait StorageService: Send + Sync {
    /// Creates bucket
    fn create_bucket(&str, config: &StorageConfig) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
    /// Removes bucket
    fn delete_bucket(&self, name: &str) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
    fn upload_object(&str, key: &str, data: &[u8]) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
    fn download_object(&str, key: &str) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;
    /// Removes object
    fn delete_object(&str, key: &str) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
    fn list_objects(&str, prefix: Option<&str>) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;
}

pub trait NetworkService: Send + Sync {
    /// Creates vpc
    fn create_vpc(&self, config: &NetworkConfig) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;
    /// Removes vpc
    fn delete_vpc(&self, vpc_id: &str) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
    /// Creates subnet
    fn create_subnet(&str, config: &SubnetConfig) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;
    /// Creates security_group
    fn create_security_group(&str, rules: &[SecurityRule]) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;
    /// Creates load_balancer
    fn create_load_balancer(&self, config: &LoadBalancerConfig) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;
}
