// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn jsonrpc_discover_returns_capabilities_and_echoes_id() -> Result<(), serde_json::Error> {
    let line = r#"{"jsonrpc":"2.0","method":"rpc.discover","id":42}"#;
    let s = handle_jsonrpc_request_line(line).expect("response");
    let v: serde_json::Value = serde_json::from_str(&s)?;
    assert_eq!(v["jsonrpc"], "2.0");
    assert_eq!(v["id"], 42);
    assert!(v["result"]["capabilities"].is_array());
    assert_eq!(v["result"]["name"], "BearDog");
    Ok(())
}

#[test]
fn jsonrpc_unknown_method_returns_error() -> Result<(), serde_json::Error> {
    let line = r#"{"jsonrpc":"2.0","method":"crypto.sign","id":"a"}"#;
    let s = handle_jsonrpc_request_line(line).expect("response");
    let v: serde_json::Value = serde_json::from_str(&s)?;
    assert_eq!(v["error"]["code"], -32601);
    assert_eq!(v["id"], "a");
    Ok(())
}

#[test]
fn jsonrpc_notification_produces_no_response() {
    let line = r#"{"jsonrpc":"2.0","method":"rpc.discover"}"#;
    assert!(handle_jsonrpc_request_line(line).is_none());
}

#[test]
fn jsonrpc_invalid_json_returns_parse_error() -> Result<(), serde_json::Error> {
    let s = handle_jsonrpc_request_line("not json").expect("response");
    let v: serde_json::Value = serde_json::from_str(&s)?;
    assert_eq!(v["error"]["code"], -32700);
    Ok(())
}

#[test]
fn test_config_from_env() {
    // Clear any existing env vars
    beardog_errors::process_env::remove_var("BEARDOG_TARPC_ADDR");
    beardog_errors::process_env::remove_var("BEARDOG_JSONRPC_ADDR");

    let config = MultiTransportConfig::from_env();
    assert!(config.enable_tarpc);
    assert!(config.enable_jsonrpc);
}

#[test]
fn test_config_capabilities() {
    let config = MultiTransportConfig::default();
    let caps = config.capabilities();

    assert!(caps.supported.contains(&"tarpc".to_string()));
    assert!(caps.supported.contains(&"json-rpc".to_string()));
    assert_eq!(caps.recommended, "tarpc");
}

#[test]
fn test_protocol_selector_throughput() {
    let available = vec![Protocol::Tarpc, Protocol::JsonRpc];
    assert_eq!(
        ProtocolSelector::for_throughput(&available),
        Protocol::Tarpc
    );
}

#[test]
fn test_protocol_selector_debugging() {
    let available = vec![Protocol::Tarpc, Protocol::JsonRpc];
    assert_eq!(
        ProtocolSelector::for_debugging(&available),
        Protocol::JsonRpc
    );
}

#[test]
fn test_protocol_selector_frequency() {
    let available = vec![Protocol::Tarpc, Protocol::JsonRpc];

    // High frequency should use tarpc
    assert_eq!(
        ProtocolSelector::for_frequency(200, &available),
        Protocol::Tarpc
    );

    // Low frequency should use JSON-RPC
    assert_eq!(
        ProtocolSelector::for_frequency(5, &available),
        Protocol::JsonRpc
    );
}

#[test]
fn test_protocol_selector_empty_falls_back_to_http() {
    let empty: Vec<Protocol> = vec![];
    assert_eq!(ProtocolSelector::for_throughput(&empty), Protocol::Http);
    assert_eq!(ProtocolSelector::for_debugging(&empty), Protocol::Http);
    assert_eq!(ProtocolSelector::for_occasional(&empty), Protocol::Http);
    assert_eq!(ProtocolSelector::for_frequency(50, &empty), Protocol::Http);
}

#[test]
fn test_protocol_selector_tarpc_only_and_jsonrpc_only() {
    assert_eq!(
        ProtocolSelector::for_throughput(&[Protocol::Tarpc]),
        Protocol::Tarpc
    );
    assert_eq!(
        ProtocolSelector::for_debugging(&[Protocol::Tarpc]),
        Protocol::Tarpc
    );
    assert_eq!(
        ProtocolSelector::for_throughput(&[Protocol::JsonRpc]),
        Protocol::JsonRpc
    );
    assert_eq!(
        ProtocolSelector::for_debugging(&[Protocol::JsonRpc]),
        Protocol::JsonRpc
    );
}

#[test]
fn test_from_router_config_and_capabilities_jsonrpc_preferred() {
    let router = RouterConfig::jsonrpc_only();
    let cfg = MultiTransportConfig::from_router_config(&router, 9950);
    assert!(!cfg.enable_tarpc);
    assert!(cfg.enable_jsonrpc);
    let caps = cfg.capabilities();
    assert_eq!(caps.recommended, "json-rpc");
}

#[test]
fn test_default_config_invalid_bind_addr_fallback() {
    let cfg = MultiTransportConfig::from_bind_and_ports(
        "not-a-valid-host!!!",
        DEFAULT_TARPC_PORT,
        DEFAULT_JSONRPC_PORT,
        DEFAULT_SHUTDOWN_TIMEOUT_SECS,
    );
    assert_eq!(cfg.tarpc_addr.port(), DEFAULT_TARPC_PORT);
    assert_eq!(cfg.jsonrpc_addr.port(), DEFAULT_JSONRPC_PORT);
}

#[test]
fn test_from_env_enable_flags() {
    let mut cfg = MultiTransportConfig::from_bind_and_ports(
        DEFAULT_BIND_ADDRESS,
        DEFAULT_TARPC_PORT,
        DEFAULT_JSONRPC_PORT,
        DEFAULT_SHUTDOWN_TIMEOUT_SECS,
    );
    cfg.enable_tarpc = false;
    cfg.enable_jsonrpc = false;
    assert!(!cfg.enable_tarpc);
    assert!(!cfg.enable_jsonrpc);
}

#[tokio::test]
async fn test_multi_transport_server_jsonrpc_only_ephemeral_port() {
    let addr: std::net::SocketAddr = "127.0.0.1:0"
        .parse()
        .expect("127.0.0.1:0 parses as SocketAddr");
    let config = MultiTransportConfig {
        tarpc_addr: addr,
        jsonrpc_addr: addr,
        enable_tarpc: false,
        enable_jsonrpc: true,
        shutdown_timeout: std::time::Duration::from_millis(200),
    };
    let server = MultiTransportServer::new(config);
    let handle = server.start().await.expect("start");
    assert!(handle.jsonrpc_addr().is_some());
    assert!(handle.tarpc_addr().is_none());
    handle.shutdown();
    handle
        .join_servers_with_timeout(std::time::Duration::from_millis(200))
        .await;
}

#[tokio::test]
async fn test_multi_transport_handle_shutdown_no_receivers() {
    let config = MultiTransportConfig {
        tarpc_addr: "127.0.0.1:0"
            .parse()
            .expect("127.0.0.1:0 parses as SocketAddr"),
        jsonrpc_addr: "127.0.0.1:0"
            .parse()
            .expect("127.0.0.1:0 parses as SocketAddr"),
        enable_tarpc: false,
        enable_jsonrpc: false,
        shutdown_timeout: std::time::Duration::from_millis(50),
    };
    let handle = MultiTransportServer::new(config)
        .start()
        .await
        .expect("start");
    handle.shutdown();
    handle.shutdown();
}

#[test]
fn test_multi_transport_server_constructors() {
    let _ = MultiTransportServer::with_defaults();
    let _ = MultiTransportServer::from_env();
    let _ = MultiTransportServer::new(MultiTransportConfig::default());
}
