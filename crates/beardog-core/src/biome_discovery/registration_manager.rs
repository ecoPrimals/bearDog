

use super::types::*;
use super::protocol::RegistrationRequest;
use beardog_errors::BearDogError;
use uuid::Uuid;

#[derive(Debug, Clone)]
        _assessment: BiomeAssessment,
        _request: RegistrationRequest,
    ) -> Result<String, BearDogError> {

        Ok(Uuid::new_v4().to_string())
    }
} 
