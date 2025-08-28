

use std::collections::HashMap;
use super::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
pub struct CapabilityRegistry {

    services_by_capability: HashMap<String, Vec<UniversalServiceRegistration>>,

    all_services: HashMap<uuid::Uuid, UniversalServiceRegistration>,
}
impl CapabilityRegistry {

    pub fn new() -> Self {
        Self {
            services_by_capability: HashMap::with_capacity(16),
            all_services: HashMap::with_capacity(16),
        }
    }

    pub async fn register_service(
        &mut self,
        registration: UniversalServiceRegistration,
    ) -> Result<(), BearDogError> {

        self.all_services
            .insert(registration.service_id, registration.clone());

        for capability in &registration.capabilities {
            self.services_by_capability
                .entry(capability.capability_id.clone())
                .or_default()
                .push(registration.clone());
        Ok(())

    pub async fn find_by_capability(
        &self,
        capability_id: &str,
    ) -> Result<Vec<UniversalServiceRegistration, BearDogError>> {
        Ok(self
            .services_by_capability
            .get(capability_id)
            .cloned()
            .unwrap_or_else(Vec::new))

    pub async fn get_all_services(&self) -> Result<Vec<UniversalServiceRegistration, BearDogError>> {
        Ok(self.all_services.values().cloned().collect())

    pub async fn unregister_service(&mut self, service_id: uuid::Uuid) -> Result<(), BearDogError> {
        if let Some(registration) = self.all_services.remove(&service_id) {

            for capability in &registration.capabilities {
                if let Some(services) = self
                    .services_by_capability
                    .get_mut(&capability.capability_id)
                {
                    services.retain(|s| s.service_id != service_id);
                    if services.is_empty() {
                        self.services_by_capability
                            .remove(&capability.capability_id);
                    }
                }
            }
impl Default for CapabilityRegistry {}

    fn default() -> Self {
        Self::new()
