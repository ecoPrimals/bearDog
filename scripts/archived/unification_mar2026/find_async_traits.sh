#!/bin/bash
# Find and analyze async_trait usage
# Part of Phase 1: Type System Unification

echo "🔍 Analyzing async_trait Usage"
echo "=============================="
echo ""

echo "📊 Summary:"
TOTAL=$(grep -r "#\[async_trait\]" crates --include="*.rs" | wc -l)
echo "   Total async_trait usages: $TOTAL"

if [ "$TOTAL" -eq 0 ]; then
    echo "   ✅ No async_trait found - already migrated!"
    exit 0
fi

echo ""
echo "📍 Locations:"
grep -rn "#\[async_trait\]" crates --include="*.rs" | head -20

echo ""
echo "🎯 Traits to migrate:"
grep -A 2 "#\[async_trait\]" crates --include="*.rs" | \
    grep "pub trait" | \
    sed 's/.*pub trait/   -/' | \
    sort | uniq

echo ""
echo "📋 Migration Pattern:"
cat << 'EOF'
BEFORE (with overhead):
#[async_trait]
pub trait MyProvider {
    async fn process(&self, data: Data) -> Result<Output, Error>;
}

AFTER (zero-cost):
pub trait MyProvider {
    fn process(&self, data: Data) 
        -> impl Future<Output = Result<Output, Error>> + Send;
}

Implementation:
impl MyProvider for MyStruct {
    fn process(&self, data: Data) 
        -> impl Future<Output = Result<Output, Error>> + Send 
    {
        async move {
            // Your async code here
            Ok(output)
        }
    }
}
EOF

echo ""
echo "⚡ Performance Gain: 15-30% for async operations"
echo ""
echo "📝 To migrate manually:"
echo "   1. Remove #[async_trait] attribute"
echo "   2. Change: async fn method(...) -> Result<T>"
echo "   3. To: fn method(...) -> impl Future<Output = Result<T>> + Send"
echo "   4. Wrap implementation in: async move { ... }"
echo "   5. Test: cargo check && cargo test"

