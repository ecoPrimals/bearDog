// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network utilities for BearDog

pub mod port_discovery;

pub use port_discovery::{discover_port, discover_ports, DiscoveredPort, PortDiscoveryConfig, PortSource};

