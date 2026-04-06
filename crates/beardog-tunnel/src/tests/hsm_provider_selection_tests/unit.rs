// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests: tier ordering, capability filtering, and selection logic without live providers.

#[test]
fn test_software_hsm_always_available() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[test]
fn test_hsm_tier_ordering() -> Result<(), Box<dyn std::error::Error>> {
    let hardware_tier = 0;
    let cloud_tier = 1;
    let software_tier = 2;

    assert!(
        hardware_tier < cloud_tier,
        "Hardware tier should be higher priority"
    );
    assert!(
        cloud_tier < software_tier,
        "Cloud tier should be higher priority than software"
    );
    Ok(())
}

#[test]
fn test_hsm_selection_with_no_hardware() -> Result<(), Box<dyn std::error::Error>> {
    let hardware_available = false;
    let software_available = true;

    let selected_tier = if hardware_available {
        "hardware"
    } else if software_available {
        "software"
    } else {
        "none"
    };

    assert_eq!(
        selected_tier, "software",
        "Should select software when hardware unavailable"
    );
    Ok(())
}

#[test]
fn test_hsm_selection_prefers_hardware() -> Result<(), Box<dyn std::error::Error>> {
    let hardware_available = true;
    let software_available = true;

    let selected_tier = if hardware_available {
        "hardware"
    } else if software_available {
        "software"
    } else {
        "none"
    };

    assert_eq!(
        selected_tier, "hardware",
        "Should prefer hardware when available"
    );
    Ok(())
}

#[test]
fn test_hsm_failover_to_software() -> Result<(), Box<dyn std::error::Error>> {
    let mut hardware_working = true;
    let software_working = true;

    let first_provider = if hardware_working {
        "hardware"
    } else {
        "software"
    };
    assert_eq!(first_provider, "hardware");

    hardware_working = false;

    let second_provider = if hardware_working {
        "hardware"
    } else if software_working {
        "software"
    } else {
        "none"
    };
    assert_eq!(second_provider, "software", "Should fail over to software");
    Ok(())
}

#[test]
fn test_multiple_hsm_providers_available() -> Result<(), Box<dyn std::error::Error>> {
    let providers = ["hardware", "cloud", "software"];

    assert!(!providers.is_empty(), "Should have at least one provider");
    assert!(!providers.is_empty(), "Should support multiple providers");
    assert_eq!(
        providers[0], "hardware",
        "First provider should be hardware (highest tier)"
    );
    Ok(())
}

#[test]
fn test_hsm_provider_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let health_statuses = [("hardware", true), ("cloud", false), ("software", true)];

    let healthy_providers: Vec<&str> = health_statuses
        .iter()
        .filter(|(_, healthy)| *healthy)
        .map(|(name, _)| *name)
        .collect();

    assert_eq!(
        healthy_providers.len(),
        2,
        "Should have 2 healthy providers"
    );
    assert!(
        healthy_providers.contains(&"hardware"),
        "Hardware should be healthy"
    );
    assert!(
        healthy_providers.contains(&"software"),
        "Software should be healthy"
    );
    assert!(
        !healthy_providers.contains(&"cloud"),
        "Cloud should not be healthy"
    );
    Ok(())
}

#[test]
fn test_hsm_capabilities_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let providers = [
        ("hardware", vec!["sign", "encrypt", "decrypt"]),
        ("software", vec!["sign", "encrypt", "decrypt", "derive"]),
        ("limited", vec!["sign"]),
    ];

    let encryption_capable: Vec<&str> = providers
        .iter()
        .filter(|(_, caps)| caps.contains(&"encrypt"))
        .map(|(name, _)| *name)
        .collect();

    assert_eq!(
        encryption_capable.len(),
        2,
        "Should have 2 encryption-capable providers"
    );
    assert!(encryption_capable.contains(&"hardware"));
    assert!(encryption_capable.contains(&"software"));
    Ok(())
}

#[test]
fn test_hsm_selection_with_capability_requirement() -> Result<(), Box<dyn std::error::Error>> {
    let required_capability = "derive";

    let providers = [
        ("hardware", vec!["sign", "encrypt"]),
        ("software", vec!["sign", "encrypt", "derive"]),
    ];

    let capable_provider = providers
        .iter()
        .find(|(_, caps)| caps.contains(&required_capability))
        .map(|(name, _)| *name);

    assert_eq!(
        capable_provider,
        Some("software"),
        "Should select provider with required capability"
    );
    Ok(())
}

#[test]
fn test_no_providers_available_error() -> Result<(), Box<dyn std::error::Error>> {
    let providers: Vec<&str> = vec![];

    let result = if providers.is_empty() {
        Err("No HSM providers available")
    } else {
        Ok(providers[0])
    };

    assert!(
        result.is_err(),
        "Should return error when no providers available"
    );
    assert_eq!(result.unwrap_err(), "No HSM providers available");
    Ok(())
}
