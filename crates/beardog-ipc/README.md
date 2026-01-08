# beardog-ipc

Inter-process communication primitives for BearDog primal coordination.

## Features

- Async IPC client/server primitives
- JSON-RPC 2.0 protocol support
- Unix socket and HTTP transport
- Type-safe message passing

## Usage

```rust
use beardog_ipc::*;

// Create IPC client
let client = IpcClient::new("http://localhost:9000").await?;

// Make requests
let response = client.call("method_name", params).await?;
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE) or http://opensource.org/licenses/MIT)

at your option.

