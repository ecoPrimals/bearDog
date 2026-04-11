// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::types::genetics::{BearDogGenetics, NodeCapability};
use beardog_errors::BearDogError;

#[tokio::test]
async fn test_default_genetics() -> Result<(), BearDogError> {
    let genetics = BearDogGenetics::default();
    assert!(!genetics.id.is_empty(), "genetics should have an ID");
    assert_eq!(genetics.generation, 0, "default generation should be 0");
    Ok(())
}

#[tokio::test]
async fn test_capabilities_assignment() -> Result<(), BearDogError> {
    let mut genetics = BearDogGenetics::default();
    genetics.capabilities.push(NodeCapability::HsmOperations);
    genetics
        .capabilities
        .push(NodeCapability::NetworkCommunication);

    assert_eq!(genetics.capabilities.len(), 2);
    assert!(
        genetics
            .capabilities
            .contains(&NodeCapability::HsmOperations)
    );
    Ok(())
}
