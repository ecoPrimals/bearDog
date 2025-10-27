#!/bin/bash
# Fix test function signatures to return Result<(), Box<dyn std::error::Error>>
# for tests that use the ? operator

echo "Fixing test function signatures in beardog-core..."

# Fix self_discovery.rs tests
sed -i 's/async fn test_self_discovery_engine() {/async fn test_self_discovery_engine() -> Result<(), Box<dyn std::error::Error>> {/' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs
sed -i 's/async fn test_zero_hardcoded_knowledge() {/async fn test_zero_hardcoded_knowledge() -> Result<(), Box<dyn std::error::Error>> {/' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs  
sed -i 's/async fn test_capability_auto_detection() {/async fn test_capability_auto_detection() -> Result<(), Box<dyn std::error::Error>> {/' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs

# Fix mod.rs tests  
sed -i 's/async fn test_zero_knowledge_bootstrap() {/async fn test_zero_knowledge_bootstrap() -> Result<(), Box<dyn std::error::Error>> {/' crates/beardog-core/src/zero_knowledge_bootstrap/mod.rs
sed -i 's/async fn test_infant_learning_pattern() {/async fn test_infant_learning_pattern() -> Result<(), Box<dyn std::error::Error>> {/' crates/beardog-core/src/zero_knowledge_bootstrap/mod.rs

# Add Ok(()) returns at end of test functions - more complex, using perl
perl -i -0pe 's/(async fn test_self_discovery_engine\(\) -> Result<[^>]+> \{(?:(?!^\}$).)*?)(\n\})/\1\n        Ok(())\2/gsm' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs
perl -i -0pe 's/(async fn test_zero_hardcoded_knowledge\(\) -> Result<[^>]+> \{(?:(?!^\}$).)*?)(\n\})/\1\n        Ok(())\2/gsm' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs  
perl -i -0pe 's/(async fn test_capability_auto_detection\(\) -> Result<[^>]+> \{(?:(?!^\}$).)*?)(\n\})/\1\n        Ok(())\2/gsm' crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs
perl -i -0pe 's/(async fn test_zero_knowledge_bootstrap\(\) -> Result<[^>]+> \{(?:(?!^\}$).)*?)(\n\})/\1\n        Ok(())\2/gsm' crates/beardog-core/src/zero_knowledge_bootstrap/mod.rs
perl -i -0pe 's/(async fn test_infant_learning_pattern\(\) -> Result<[^>]+> \{(?:(?!^\}$).)*?)(\n\})/\1\n        Ok(())\2/gsm' crates/beardog-core/src/zero_knowledge_bootstrap/mod.rs

echo "Done! Now testing compilation..."
cargo test --package beardog-core --lib --no-run 2>&1 | tail -20

