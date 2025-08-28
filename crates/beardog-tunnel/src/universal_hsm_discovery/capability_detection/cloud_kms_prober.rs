

use super::super::*;
use beardog_core::HsmCapabilities;
use beardog_errors::BearDogError;
use tracing::debug;
#[derive(Debug)]
pub struct CloudKmsCapabilityProber;
impl CloudKmsCapabilityProber {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn probe_aws_kms_capabilities(&self, region: &str) -> Result<HsmCapabilities, BearDogError> {
        debug!("☁️ Probing AWS KMS capabilities in region: {}", region);

        Ok(HsmCapabilities::default())
    pub async fn probe_azure_kv_capabilities(
        &self,
        vault_url: &str,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("☁️ Probing Azure Key Vault: {}", vault_url);

    pub async fn probe_gcp_kms_capabilities(
        project_id: &str,
        location: &str,
        debug!("☁️ Probing GCP KMS: {} in {}", project_id, location);

}
