// SPDX-License-Identifier: AGPL-3.0-or-later

mod zero_cost_memory_tests {
    use crate::zero_cost::memory_safe::*;

    #[test]
    fn test_memory_pool_new() {
        let pool = SafeZeroCopyMemoryPool::new(&[64, 256, 1024], 8);
        let _ = format!("{:?}", pool.get_metrics());
    }

    #[test]
    fn test_memory_pool_allocate_deallocate() {
        let pool = SafeZeroCopyMemoryPool::new(&[64, 256], 8);
        let buf = pool.allocate(32);
        assert!(buf.len() >= 32);
        pool.deallocate(buf);
    }

    #[test]
    fn test_memory_pool_allocate_large() {
        let pool = SafeZeroCopyMemoryPool::new(&[64], 8);
        let buf = pool.allocate(128);
        assert!(buf.len() >= 128);
        pool.deallocate(buf);
    }

    #[test]
    fn test_memory_pool_metrics() {
        let pool = SafeZeroCopyMemoryPool::new(&[64], 8);
        let _ = pool.allocate(32);
        let metrics = pool.get_metrics();
        let _ = format!("{metrics:?}");
    }

    #[test]
    fn test_memory_pool_clear() {
        let pool = SafeZeroCopyMemoryPool::new(&[64], 8);
        let buf = pool.allocate(32);
        pool.deallocate(buf);
        pool.clear();
    }

    #[test]
    fn test_ring_buffer_default() {
        let rb: SafeRingBuffer<u32> = SafeRingBuffer::default();
        assert!(rb.is_empty());
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_ring_buffer_push_pop() {
        let rb: SafeRingBuffer<i32> = SafeRingBuffer::default();
        assert!(rb.push(42));
        assert!(!rb.is_empty());
        assert_eq!(rb.len(), 1);
        let val = rb.pop();
        assert_eq!(val, Some(42));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_ring_buffer_try_push_pop() {
        let rb: SafeRingBuffer<String> = SafeRingBuffer::default();
        assert!(rb.try_push("hello".to_string()).is_ok());
        let val = rb.try_pop();
        assert_eq!(val, Some("hello".to_string()));
        assert!(rb.try_pop().is_none());
    }

    #[test]
    fn test_ring_buffer_is_full() {
        let rb: SafeRingBuffer<u8> = SafeRingBuffer::default();
        let _ = rb.is_full();
    }

    #[test]
    fn test_simd_capabilities_new() {
        let caps = SafeSimdCapabilities::new();
        let _ = format!("{caps:?}");
    }

    #[test]
    fn test_simd_capabilities_detect() {
        let caps = SafeSimdCapabilities::detect();
        let _ = format!("{caps:?}");
    }

    #[test]
    fn test_simd_capabilities_vectorized_hash() {
        let caps = SafeSimdCapabilities::new();
        let hash = caps.vectorized_hash(b"test data");
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_simd_capabilities_default() {
        let caps = SafeSimdCapabilities::default();
        let _ = format!("{caps:?}");
    }
}

// ===========================================================================
// zero_cost/workflow.rs - 27 uncov
// ===========================================================================
mod zero_cost_workflow_tests {
    use crate::zero_cost::workflow::*;

    #[test]
    fn test_workflow_engine_config_default() {
        let c = WorkflowEngineConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// genetics_constraints.rs - 97 uncov
// ===========================================================================
mod genetics_constraints_tests {
    use crate::genetics_constraints::*;

    #[test]
    fn test_key_constraints_default() {
        let c = KeyConstraints::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_lifetime_constraint_default() {
        let c = LifetimeConstraint::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_data_access_constraint_default() {
        let c = DataAccessConstraint::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_behavioral_constraint_default() {
        let c = BehavioralConstraint::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_constraints_hash() {
        let c = KeyConstraints::default();
        let hash = c.hash();
        assert!(hash.is_ok());
        let h = hash.unwrap();
        assert_eq!(h.len(), 32);
    }

    #[test]
    fn test_key_constraints_description() {
        let c = KeyConstraints::default();
        let desc = c.description();
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_key_constraints_verify_operation() {
        let c = KeyConstraints::default();
        let op = KeyOperation::Read {
            path: "/data/test.txt".to_string(),
            project: None,
        };
        let _ = c.verify_operation(&op);
    }

    #[test]
    fn test_key_operation_variants() {
        let _read = KeyOperation::Read {
            path: "/data".to_string(),
            project: Some("proj1".to_string()),
        };
        let _write = KeyOperation::Write {
            path: "/data/out.txt".to_string(),
            size_bytes: 1024,
            project: None,
        };
        let _delete = KeyOperation::Delete {
            path: "/data/temp".to_string(),
        };
    }

    #[test]
    fn test_scope_constraint_variants() {
        let _ = ScopeConstraint::Unrestricted;
        let _ = ScopeConstraint::Project {
            name: "test-project".to_string(),
            project_hash: [0u8; 32],
        };
        let _ = ScopeConstraint::Resources {
            allow_read: vec!["*".to_string()],
            allow_write: vec!["*".to_string()],
            deny_delete: vec!["protected/*".to_string()],
        };
        let _ = ScopeConstraint::Operations {
            allowed_operations: vec!["read".to_string(), "write".to_string()],
        };
    }
}
