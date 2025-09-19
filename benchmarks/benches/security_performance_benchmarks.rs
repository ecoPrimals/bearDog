use beardog_errors::BearDogError;
use beardog_security::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;

use tokio::runtime::Runtime;

fn benchmark_security_provider_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("security_provider_init");

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;

    let configs = vec![
        ("default", SecurityProviderConfig::default(true,
                audit_logging_enabled: true,
                threat_detection_enabled: true,
                compliance_enabled: true,
                ..Default::default()
            },
        ),
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("provider_creation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    rt.block_on(async {
                        let _provider = BearDogSecurityProvider::new_with_config(config.clone())
                            .await
                            .map_err(|e| {
                                tracing::error!("Operation failed: {:?}", e);
                                beardog_errors::BearDogError::internal({:?}", e))
                            })?;
                    })
                });
            },
        );
    }

    group.finish();
}

fn benchmark_authentication(c: &mut Criterion) {
    let mut group = c.benchmark_group("authentication");
    group.throughput(Throughput::Elements(1));

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let config = SecurityProviderConfig::default();
    let provider = rt.block_on(async {
        BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    });

    let test_cases = vec![
        ("valid_credentials", ("admin", "admin123", true)),
        ("invalid_credentials", ("admin", "wrong_password", false)),
        ("unknown_user", ("unknown", "password", false)),
    ];

    for (name, (username, password, _expected)) in test_cases {
        let mut credentials = HashMap::with_capacity(16);
        credentials.insert("username".to_string(), username);
        credentials.insert("password".to_string(), password);

        group.bench_with_input(
            BenchmarkId::new("authenticate", name),
            &credentials,
            |b, creds| {
                b.iter(|| {
                    rt.block_on(async {
                        let _result = provider.authenticate(creds).await.map_err(|e| {
                            tracing::error!("Operation failed: {:?}", e);
                            beardog_errors::BearDogError::internal({:?}", e))
                        })?;
                    })
                });
            },
        );
    }

    group.finish();
}

fn benchmark_authorization(c: &mut Criterion) {
    let mut group = c.benchmark_group("authorization");
    group.throughput(Throughput::Elements(1));

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let config = SecurityProviderConfig::default();
    let provider = rt.block_on(async {
        BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    });

    let subject = Subject {
        id: "user1".to_string(),
        name: "Test User".to_string(SubjectType::User,
        roles: vec!["user".to_string()],
        clearance_level: Some(1),
        metadata: HashMap::with_capacity(16),
    };

    let resource = Resource {
        id: "document1".to_string(),
        name: "Test Document".to_string(ResourceClassification::Internal,
        metadata: HashMap::with_capacity(ActionType::Read,
        description: "Read access".to_string(RiskLevel::Low,
        timestamp: chrono::Utc::now(),
    };

    group.bench_function("authorize", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _result = provider
                    .authorize(&subject, &action, &resource)
                    .await
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal({:?}", e))
                    })?;
            })
        });
    });

    group.finish();
}

fn benchmark_rate_limiting(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_limiting");
    group.throughput(Throughput::Elements(1));

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let mut config = SecurityProviderConfig::default();
    config.rate_limit_config.max_operations = 1000; // High limit for benchmarking

    let mut provider = rt.block_on(async {
        BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    });

    group.bench_function("rate_limit_check", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _result = provider
                    .check_rate_limit("benchmark_user")
                    .await
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal({:?}", e))
                    })?;
            })
        });
    });

    group.finish();
}

fn benchmark_crypto_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto_operations");

    let data_sizes = vec![
        ("small", 64),    // 64 bytes
        ("medium", 1024), // 1 KB
        ("large", 65536), // 64 KB
    ];

    for (name, size) in data_sizes {
        let data = vec![0x42u8; size];

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("ed25519_sign", name), &data, |b, data| {
            b.iter(|| {
                let (private_key, _) = BearDogCrypto::generate_ed25519_keypair().map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
                let _signature = BearDogCrypto::sign_ed25519(&private_key, data).map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            });
        });

        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        let signature = BearDogCrypto::sign_ed25519(&private_key, &data).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

        group.bench_with_input(
            BenchmarkId::new("ed25519_verify", name),
            &(data.clone(), public_key.clone(), signature.clone()),
            |b, (data, pub_key, sig)| {
                b.iter(|| {
                    let _valid = BearDogCrypto::verify_ed25519_signature(pub_key, data, sig)
                        .map_err(|e| {
                            tracing::error!("Operation failed: {:?}", e);
                            beardog_errors::BearDogError::internal({:?}", e))
                        })?;
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("sha256_hash", name), &data, |b, data| {
            b.iter(|| {
                let _hash = BearDogCrypto::sha256_hash(data).map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            });
        });
    }

    group.finish();
}

fn benchmark_password_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_operations");

    let passwords = vec![
        "password123",
        "very_secure_password_with_lots_of_characters",
        "P@ssw0rd!#$%^&*()",
    ];

    for password in &passwords {
        group.bench_with_input(
            BenchmarkId::new("argon2_hash", password.len()),
            password,
            |b, pwd| {
                b.iter(|| {
                    let _hash = BearDogCrypto::hash_password_argon2(pwd).map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal({:?}", e))
                    })?;
                });
            },
        );

        let hash = BearDogCrypto::hash_password_argon2(password).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        group.bench_with_input(
            BenchmarkId::new("argon2_verify", password.len()),
            &(password, &hash),
            |b, (pwd, hash)| {
                b.iter(|| {
                    let _valid = BearDogCrypto::verify_password_argon2(pwd, hash).map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal({:?}", e))
                    })?;
                });
            },
        );
    }

    group.finish();
}

fn benchmark_session_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_management");

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let config = SecurityProviderConfig::default();
    let provider = rt.block_on(async {
        BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    });

    let user_info = UserInfo {
        id: "test_user".to_string(),
        user_id: "test_user".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        full_name: "Test User".to_string(),
        roles: vec!["user".to_string()],
        permissions: vec!["read".to_string(AccountStatus::Active,
        last_login: None,
    };

    group.bench_function("create_session", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _session = provider
                    .create_session(
                        &user_info,
                        "127.0.0.1".to_string(),
                        "benchmark/1.0")
                    .await
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal({:?}", e))
                    })?;
            })
        });
    });

    let session = rt.block_on(async {
        provider
            .create_session(
                &user_info,
                "127.0.0.1".to_string(),
                "benchmark/1.0")
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    });

    group.bench_function("validate_session", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _result = provider.validate_session(&session.id).await.map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            })
        });
    });

    group.finish();
}

fn benchmark_concurrent_auth(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_auth");

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let config = SecurityProviderConfig::default();
    let provider = std::sync::Arc::new(rt.block_on(async {
        BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?
    }));

    let concurrency_levels = vec![1, 5, 10, 25, 50];

    for concurrency in concurrency_levels {
        group.throughput(Throughput::Elements(concurrency as u64));

        group.bench_with_input(
            BenchmarkId::new("concurrent_requests", concurrency),
            &concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on({
                        let provider = provider.clone();
                        async move {
                            let mut handles = Vec::new();

                            for i in 0..concurrency {
                                let provider = provider.clone();
                                let handle = tokio::spawn(async move {
                                    let mut credentials = HashMap::with_capacity(16);
                                    credentials.insert("username".to_string(), format!("user{i}"));
                                    credentials
                                        .insert("password".to_string(), "password");

                                    let _result =
                                        provider.authenticate(&credentials).await.map_err(|e| {
                                            tracing::error!("Operation failed: {:?}", e);
                                            beardog_errors::BearDogError::internal({:?}", e)
                                                    )
                                        })?;
                                });
                                handles.push(handle);
                            }

                            for handle in handles {
                                handle.await.map_err(|e| {
                                    tracing::error!("Operation failed: {:?}", e);
                                    beardog_errors::BearDogError::internal({:?}", e))
                                })?;
                            }
                        }
                    })
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    security_benches,
    benchmark_security_provider_init,
    benchmark_authentication,
    benchmark_authorization,
    benchmark_rate_limiting,
    benchmark_crypto_operations,
    benchmark_password_operations,
    benchmark_session_management,
    benchmark_concurrent_auth
);

criterion_main!(security_benches);
