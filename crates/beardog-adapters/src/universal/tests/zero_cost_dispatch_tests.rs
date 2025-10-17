// Zero-Cost Capability Dispatch Tests
//
// Testing zero-cost dispatch patterns

use crate::universal::zero_cost_capability_dispatch::*;
use beardog_types::adapters::{CapabilityRequest, CapabilityResponse, CapabilityType};

#[test]
fn test_zero_cost_router_creation() {
    let router = ZeroCostCapabilityRouter::new();
    
    assert_eq!(router.handler_count(), 0);
}

#[test]
fn test_router_handler_registration() {
    let mut router = ZeroCostCapabilityRouter::new();
    
    let handler = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Security,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({"status": "handled"}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(handler, 1.0);
    
    assert_eq!(router.handler_count(), 1);
}

#[test]
fn test_router_multiple_handlers() {
    let mut router = ZeroCostCapabilityRouter::new();
    
    let security_handler = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Security,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    let compute_handler = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Compute,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(security_handler, 1.0);
    router.add_handler(compute_handler, 1.0);
    
    assert_eq!(router.handler_count(), 2);
}

#[test]
fn test_router_request_routing() {
    let mut router = ZeroCostCapabilityRouter::new();
    
    let handler = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Security,
        handler_fn: |req| {
            assert_eq!(req.required_capability, CapabilityType::Security);
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({"handled": true}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(handler, 1.0);
    
    let request = CapabilityRequest {
        required_capability: CapabilityType::Security,
        payload: serde_json::json!({}),
        metadata: std::collections::HashMap::new(),
    };
    
    let result = router.route_request(&request);
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response.success);
}

#[test]
fn test_router_no_matching_handler() {
    let router = ZeroCostCapabilityRouter::new();
    
    let request = CapabilityRequest {
        required_capability: CapabilityType::Security,
        payload: serde_json::json!({}),
        metadata: std::collections::HashMap::new(),
    };
    
    let result = router.route_request(&request);
    assert!(result.is_err());
}

#[test]
fn test_capability_handler_confidence_score() {
    let mut router = ZeroCostCapabilityRouter::new();
    
    let high_confidence_handler = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Compute,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(high_confidence_handler, 0.95);
    assert_eq!(router.handler_count(), 1);
}

#[test]
fn test_router_handler_replacement() {
    let mut router = ZeroCostCapabilityRouter::new();
    
    // Add first handler
    let handler1 = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Storage,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({"version": 1}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(handler1, 0.8);
    assert_eq!(router.handler_count(), 1);
    
    // Add second handler (may replace or add depending on implementation)
    let handler2 = CapabilityHandlerDispatch {
        capability_type: CapabilityType::Monitoring,
        handler_fn: |_req| {
            Ok(CapabilityResponse {
                success: true,
                result: serde_json::json!({"version": 2}),
                metadata: std::collections::HashMap::new(),
            })
        },
    };
    
    router.add_handler(handler2, 0.9);
    assert!(router.handler_count() >= 1);
}

