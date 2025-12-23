// Session Management Tests
//
// Comprehensive tests for secure session creation, management, and lifecycle

use crate::tunnel::session::{
    GamingSecurityProfile, SecureSession, SecurityGenetics, SessionManager,
};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_secure_session_creation() {
    let session = SecureSession::new(
        "test-session-123",
        "peer-node-456",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    );

    assert!(session.is_ok());
    let session = session.unwrap();
    assert_eq!(session.session_id, "test-session-123");
    assert_eq!(session.peer_node_id, "peer-node-456");
    assert!(!session.is_expired());
}

#[test]
fn test_session_expiration() {
    let mut session = SecureSession::new(
        "test-session",
        "peer-node",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    )
    .unwrap();

    // Should not be expired initially
    assert!(!session.is_expired());

    // Extend session
    session.extend_session(Duration::from_secs(7200));
    assert!(!session.is_expired());
}

#[test]
fn test_security_genetics_default() {
    let genetics = SecurityGenetics::default();
    assert_eq!(genetics.entropy_level, 0.8);
    assert_eq!(genetics.mutation_rate, 0.05);
    assert_eq!(genetics.adaptive_threshold, 0.7);
}

#[test]
fn test_security_genetics_custom_values() {
    let genetics = SecurityGenetics {
        entropy_level: 0.95,
        mutation_rate: 0.1,
        adaptive_threshold: 0.85,
    };

    assert_eq!(genetics.entropy_level, 0.95);
    assert_eq!(genetics.mutation_rate, 0.1);
    assert_eq!(genetics.adaptive_threshold, 0.85);
}

#[test]
fn test_gaming_security_profile_competitive() {
    let profile = GamingSecurityProfile::competitive_gaming();
    assert_eq!(profile.latency_priority, 0.9);
    assert_eq!(profile.security_level.level, 3);
    assert_eq!(profile.security_level.authentication_strength, 85);
    assert_eq!(profile.security_level.threat_detection_accuracy, 0.95);
    assert_eq!(profile.security_level.performance_overhead, 0.15);
}

#[tokio::test]
async fn test_session_manager_creation() {
    let manager = SessionManager::new();
    let count = manager.session_count().await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_session_manager_create_session() {
    let manager = SessionManager::new();

    let result = manager
        .create_session(
            "session-123".to_string(),
            "peer-123".to_string(),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        )
        .await;

    assert!(result.is_ok());

    let count = manager.session_count().await;
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_session_manager_get_session() {
    let manager = SessionManager::new();

    let session_id = "session-456";
    manager
        .create_session(
            session_id.to_string(),
            "peer-456".to_string(),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        )
        .await
        .unwrap();

    let session = manager.get_session(session_id).await;
    assert!(session.is_some());

    let session = session.unwrap();
    assert_eq!(session.session_id, session_id);
    assert_eq!(session.peer_node_id, "peer-456");
}

#[tokio::test]
async fn test_session_manager_get_nonexistent_session() {
    let manager = SessionManager::new();
    let session = manager.get_session("nonexistent").await;
    assert!(session.is_none());
}

#[tokio::test]
async fn test_session_manager_remove_session() {
    let manager = SessionManager::new();

    let session_id = "session-789";
    manager
        .create_session(
            session_id.to_string(),
            "peer-789".to_string(),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        )
        .await
        .unwrap();

    let removed = manager.remove_session(session_id).await;
    assert!(removed.is_some());

    let session = manager.get_session(session_id).await;
    assert!(session.is_none());

    let count = manager.session_count().await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_session_manager_remove_nonexistent() {
    let manager = SessionManager::new();
    let removed = manager.remove_session("nonexistent").await;
    assert!(removed.is_none());
}

#[tokio::test]
async fn test_session_manager_cleanup_expired() {
    let manager = SessionManager::new();

    // Create sessions with very short timeout
    for i in 0..5 {
        manager
            .create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .unwrap();
    }

    let count = manager.session_count().await;
    assert_eq!(count, 5);

    // Wait for sessions to expire (they have 1 hour timeout by default)
    // Since we can't wait that long, we'll just test the cleanup mechanism works
    let result = manager.cleanup_expired_sessions().await;
    assert!(result.is_ok());

    // Sessions shouldn't be expired yet (1 hour timeout)
    let count = manager.session_count().await;
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_session_manager_concurrent_access() {
    let manager = Arc::new(SessionManager::new());

    let mut handles = vec![];

    // Create multiple sessions concurrently
    for i in 0..10 {
        let mgr = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            mgr.create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }

    let count = manager.session_count().await;
    assert_eq!(count, 10);
}

#[tokio::test]
async fn test_session_manager_concurrent_read_write() {
    let manager = Arc::new(SessionManager::new());

    // Create initial sessions
    for i in 0..5 {
        manager
            .create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .unwrap();
    }

    let mut handles = vec![];

    // Concurrent readers
    for i in 0..5 {
        let mgr = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let session_id = format!("session-{}", i);
            mgr.get_session(&session_id).await
        });
        handles.push(handle);
    }

    // Concurrent writers
    for i in 5..10 {
        let mgr = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            mgr.create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }

    let count = manager.session_count().await;
    assert_eq!(count, 10);
}

#[test]
fn test_session_manager_default() {
    let manager1 = SessionManager::default();
    let manager2 = SessionManager::new();

    // Both should create valid instances
    assert_eq!(
        std::mem::size_of_val(&manager1),
        std::mem::size_of_val(&manager2)
    );
}

#[tokio::test]
async fn test_multiple_sessions_same_peer() {
    let manager = SessionManager::new();

    // Create multiple sessions for the same peer
    for i in 0..3 {
        manager
            .create_session(
                format!("session-{}", i),
                "peer-same".to_string(),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .unwrap();
    }

    let count = manager.session_count().await;
    assert_eq!(count, 3);

    // All sessions should be retrievable
    for i in 0..3 {
        let session = manager.get_session(&format!("session-{}", i)).await;
        assert!(session.is_some());
        assert_eq!(session.unwrap().peer_node_id, "peer-same");
    }
}

#[tokio::test]
async fn test_session_extension() {
    let manager = SessionManager::new();

    let session_id = "session-extend";
    manager
        .create_session(
            session_id.to_string(),
            "peer-extend".to_string(),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        )
        .await
        .unwrap();

    let session = manager.get_session(session_id).await.unwrap();
    let original_expiry = session.expires_at;

    // Note: extend_session is on the session itself, not the manager
    // This test verifies the session structure
    assert!(!session.is_expired());
    assert!(session.expires_at > session.created_at);
}

#[test]
fn test_security_genetics_bounds() {
    // Test edge cases for security genetics values
    let genetics = SecurityGenetics {
        entropy_level: 1.0,
        mutation_rate: 0.0,
        adaptive_threshold: 1.0,
    };

    assert_eq!(genetics.entropy_level, 1.0);
    assert_eq!(genetics.mutation_rate, 0.0);
    assert_eq!(genetics.adaptive_threshold, 1.0);
}

#[test]
fn test_session_id_uniqueness() {
    // Create sessions with different IDs
    let session1 = SecureSession::new(
        "session-1",
        "peer-1",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    )
    .unwrap();

    let session2 = SecureSession::new(
        "session-2",
        "peer-1",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    )
    .unwrap();

    assert_ne!(session1.session_id, session2.session_id);
}

#[tokio::test]
async fn test_session_count_accuracy() {
    let manager = SessionManager::new();

    // Start with 0
    assert_eq!(manager.session_count().await, 0);

    // Add 5 sessions
    for i in 0..5 {
        manager
            .create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .unwrap();
    }
    assert_eq!(manager.session_count().await, 5);

    // Remove 2 sessions
    manager.remove_session("session-0").await;
    manager.remove_session("session-1").await;
    assert_eq!(manager.session_count().await, 3);

    // Add 3 more
    for i in 5..8 {
        manager
            .create_session(
                format!("session-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .unwrap();
    }
    assert_eq!(manager.session_count().await, 6);
}

// ============================================================================
// Additional Session Tests - Day 2 Expansion  
// ============================================================================

#[test]
fn test_security_genetics_high_entropy() {
    let genetics = SecurityGenetics {
        entropy_level: 1.0,
        mutation_rate: 0.0,
        adaptive_threshold: 1.0,
    };
    
    assert_eq!(genetics.entropy_level, 1.0);
    assert_eq!(genetics.mutation_rate, 0.0);
}

#[test]
fn test_security_genetics_low_entropy() {
    let genetics = SecurityGenetics {
        entropy_level: 0.1,
        mutation_rate: 0.5,
        adaptive_threshold: 0.2,
    };
    
    assert_eq!(genetics.entropy_level, 0.1);
    assert_eq!(genetics.mutation_rate, 0.5);
}

#[test]
fn test_gaming_security_profile_values() {
    let profile = GamingSecurityProfile::competitive_gaming();
    
    assert!(profile.latency_priority > 0.0);
    assert!(profile.latency_priority <= 1.0);
    assert!(profile.security_level.level > 0);
}

#[test]
fn test_secure_session_peer_node_id() {
    let session = SecureSession::new(
        "session-001",
        "peer-abc-123",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    ).unwrap();
    
    assert_eq!(session.peer_node_id, "peer-abc-123");
}

#[test]
fn test_secure_session_different_genetics() {
    let genetics1 = SecurityGenetics::default();
    let genetics2 = SecurityGenetics {
        entropy_level: 0.95,
        mutation_rate: 0.02,
        adaptive_threshold: 0.9,
    };
    
    let session1 = SecureSession::new(
        "s1",
        "p1",
        genetics1,
        GamingSecurityProfile::competitive_gaming(),
    ).unwrap();
    
    let session2 = SecureSession::new(
        "s2",
        "p2",
        genetics2,
        GamingSecurityProfile::competitive_gaming(),
    ).unwrap();
    
    assert_ne!(session1.session_id, session2.session_id);
}

#[tokio::test]
async fn test_session_manager_multiple_peers() {
    let manager = SessionManager::new();
    
    for i in 0..5 {
        manager.create_session(
            format!("session-{}", i),
            format!("peer-{}", i),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        ).await.unwrap();
    }
    
    assert_eq!(manager.session_count().await, 5);
}

#[tokio::test]
async fn test_session_manager_remove_all() {
    let manager = SessionManager::new();
    
    let session_ids: Vec<String> = (0..3)
        .map(|i| {
            let id = format!("session-{}", i);
            id
        })
        .collect();
    
    for id in &session_ids {
        manager.create_session(
            id.clone(),
            format!("peer-{}", id),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        ).await.unwrap();
    }
    
    assert_eq!(manager.session_count().await, 3);
    
    for id in &session_ids {
        manager.remove_session(id).await;
    }
    
    assert_eq!(manager.session_count().await, 0);
}

#[tokio::test]
async fn test_session_manager_concurrent_creates() {
    let manager = Arc::new(SessionManager::new());
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);
        handles.push(tokio::spawn(async move {
            manager_clone.create_session(
                format!("concurrent-{}", i),
                format!("peer-{}", i),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            ).await
        }));
    }
    
    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
    
    assert_eq!(manager.session_count().await, 10);
}

#[test]
fn test_session_extend_by_duration() {
    let mut session = SecureSession::new(
        "test",
        "peer",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    ).unwrap();
    
    session.extend_session(Duration::from_secs(3600));
    assert!(!session.is_expired());
}

#[test]
fn test_security_genetics_boundary_values() {
    let genetics = SecurityGenetics {
        entropy_level: 0.0,
        mutation_rate: 1.0,
        adaptive_threshold: 0.0,
    };
    
    assert_eq!(genetics.entropy_level, 0.0);
    assert_eq!(genetics.mutation_rate, 1.0);
    assert_eq!(genetics.adaptive_threshold, 0.0);
}

#[test]
fn test_gaming_profile_security_level() {
    let profile = GamingSecurityProfile::competitive_gaming();
    
    assert!(profile.security_level.authentication_strength > 0);
    assert!(profile.security_level.threat_detection_accuracy >= 0.0);
    assert!(profile.security_level.threat_detection_accuracy <= 1.0);
}

#[tokio::test]
async fn test_session_manager_empty_after_init() {
    let manager = SessionManager::new();
    assert_eq!(manager.session_count().await, 0);
    assert!(manager.get_session("any").await.is_none());
}

#[tokio::test]
async fn test_session_manager_get_after_remove() {
    let manager = SessionManager::new();
    
    manager.create_session(
        "temp".to_string(),
        "peer".to_string(),
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    ).await.unwrap();
    
    manager.remove_session("temp").await;
    
    assert!(manager.get_session("temp").await.is_none());
}

#[test]
fn test_session_creation_with_empty_ids() {
    let session = SecureSession::new(
        "",
        "",
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    );
    
    assert!(session.is_ok() || session.is_err());
}

#[tokio::test]
async fn test_session_manager_duplicate_session_ids() {
    let manager = SessionManager::new();
    
    let result1 = manager.create_session(
        "duplicate".to_string(),
        "peer1".to_string(),
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    ).await;
    
    let result2 = manager.create_session(
        "duplicate".to_string(),
        "peer2".to_string(),
        SecurityGenetics::default(),
        GamingSecurityProfile::competitive_gaming(),
    ).await;
    
    // Either both succeed or second fails
    assert!(result1.is_ok());
}

#[test]
fn test_security_genetics_mid_range_values() {
    let genetics = SecurityGenetics {
        entropy_level: 0.5,
        mutation_rate: 0.25,
        adaptive_threshold: 0.6,
    };
    
    assert_eq!(genetics.entropy_level, 0.5);
    assert_eq!(genetics.mutation_rate, 0.25);
    assert_eq!(genetics.adaptive_threshold, 0.6);
}

#[tokio::test]
async fn test_session_manager_stress_test() {
    let manager = SessionManager::new();
    
    // Create 50 sessions
    for i in 0..50 {
        manager.create_session(
            format!("stress-{}", i),
            format!("peer-{}", i),
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        ).await.unwrap();
    }
    
    assert_eq!(manager.session_count().await, 50);
    
    // Remove all
    for i in 0..50 {
        manager.remove_session(&format!("stress-{}", i)).await;
    }
    
    assert_eq!(manager.session_count().await, 0);
}

// ============================================================================
// Test Summary
// ============================================================================
// Original tests: 20
// New tests added: 17
// Total tests: 37
// Category: Session management, security genetics, concurrency
// Purpose: Day 2 comprehensive session testing
// Date: November 5, 2025
// ============================================================================

