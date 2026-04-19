// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared test helpers for graph security integration tests.

use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use serde_json::json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    use beardog_genetics::EcosystemGeneticEngine;
    use beardog_tunnel::tunnel::hsm::manager::HsmManager;

    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));

    Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP provider init"),
    )
}

pub async fn send_jsonrpc_request(
    method: &str,
    params: serde_json::Value,
    socket_path: &str,
) -> serde_json::Value {
    let mut stream = UnixStream::connect(socket_path).await.unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let request_str = serde_json::to_string(&request).unwrap();
    eprintln!("Sending request: {request_str}");
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap();
    stream.flush().await.unwrap();

    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.unwrap();

    serde_json::from_str(&response_line).unwrap()
}
