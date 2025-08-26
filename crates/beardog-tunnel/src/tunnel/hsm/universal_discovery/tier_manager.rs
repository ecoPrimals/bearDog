

use beardog_errors::{BearDogError, BearDogResult};

pub struct TierManager {

}
impl TierManager {

    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {

        })
    }

    pub fn assign_tier(&self, _capabilities: &str) -> String {

        "default".to_string()

    pub fn select_best_hsm_for_operation(
        &self,
        _operation: &str,
        _requirements: &str,
    ) -> Option<String> {

        Some("default_hsm".to_string())
