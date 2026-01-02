# beardog-discovery

**Capability-based service discovery for BearDog - zero hardcoded knowledge**

Part of the BearDog secure mesh networking system, this crate provides runtime service discovery without any hardcoded service names or locations.

## Features

- **mDNS Discovery** - Local network service discovery (RFC 6762)
- **DNS-SD** - Service discovery via DNS (RFC 6763)
- **Service Registry** - Consul/etcd integration for distributed discovery
- **Capability-Based** - Discover services by capabilities, not names
- **Zero Hardcoding** - No hardcoded service locations or ports
- **Production-Ready** - 56 tests, ~95% coverage

## Usage

```rust
use beardog_discovery::{DiscoveryConfig, UniversalDiscovery};

let config = DiscoveryConfig::default();
let discovery = UniversalDiscovery::new(config).await?;

// Discover services by capability
let services = discovery
    .discover_by_capability("secure-tunnel", Duration::from_secs(5))
    .await?;
```

## License

AGPL-3.0

