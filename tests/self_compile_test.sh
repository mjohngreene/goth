#!/bin/bash
# Self-compilation test for Goth
# Tests that the compiler can compile and run Goth programs

set -e

GOTHC="${GOTHC:-./crates/target/release/gothc}"
GOTH="${GOTH:-./crates/target/release/goth}"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

pass() {
    echo -e "${GREEN}✓ $1${NC}"
}

fail() {
    echo -e "${RED}✗ $1${NC}"
    exit 1
}

echo "=== Goth Self-Compilation Tests ==="
echo ""

# Test 1: Simple expression evaluation
echo "Test 1: Expression evaluation"
result=$($GOTH -e "1 + 2 * 3")
if [ "$result" = "7" ]; then
    pass "Expression: 1 + 2 * 3 = 7"
else
    fail "Expected 7, got $result"
fi

# Test 2: Lambda evaluation
echo "Test 2: Lambda evaluation"
result=$($GOTH -e "(λ→ ₀ + 1) 5")
if [ "$result" = "6" ]; then
    pass "Lambda: (λ→ ₀ + 1) 5 = 6"
else
    fail "Expected 6, got $result"
fi

# Test 3: Array sum
echo "Test 3: Array sum"
result=$($GOTH -e "Σ [1, 2, 3, 4, 5]")
if [ "$result" = "15" ]; then
    pass "Sum: Σ [1, 2, 3, 4, 5] = 15"
else
    fail "Expected 15, got $result"
fi

# Test 4: Map operation
echo "Test 4: Map operation"
result=$($GOTH -e "Σ ([1, 2, 3] ↦ λ→ ₀ × 2)")
if [ "$result" = "12" ]; then
    pass "Map: Σ ([1, 2, 3] ↦ λ→ ₀ × 2) = 12"
else
    fail "Expected 12, got $result"
fi

# Test 5: Compile hello world
echo "Test 5: Compile and run hello world"
cat > /tmp/hello.goth << 'EOF'
╭─ main : () → I
╰─ 42
EOF

$GOTHC /tmp/hello.goth -o /tmp/hello 2>/dev/null
result=$(/tmp/hello)
if [ "$result" = "42" ]; then
    pass "Compile: hello world returns 42"
else
    fail "Expected 42, got $result"
fi

# Test 6: Compile arithmetic
echo "Test 6: Compile arithmetic"
cat > /tmp/arith.goth << 'EOF'
╭─ main : () → I
╰─ (3 + 4) × 5
EOF

$GOTHC /tmp/arith.goth -o /tmp/arith 2>/dev/null
result=$(/tmp/arith)
if [ "$result" = "35" ]; then
    pass "Compile: (3 + 4) × 5 = 35"
else
    fail "Expected 35, got $result"
fi

# Test 7: Compile with lambda
echo "Test 7: Compile lambda"
cat > /tmp/lambda.goth << 'EOF'
╭─ main : I → I
╰─ (λ→ ₀ × ₀) ₀
EOF

$GOTHC /tmp/lambda.goth -o /tmp/lambda 2>/dev/null
result=$(/tmp/lambda 7)
if [ "$result" = "49" ]; then
    pass "Compile: lambda square 7 = 49"
else
    fail "Expected 49, got $result"
fi

# Test 8: Parse standard library files
echo "Test 8: Parse standard library"
for file in stdlib/*.goth; do
    $GOTH -p "$file" > /dev/null || fail "Failed to parse $file"
done
pass "All stdlib files parse successfully"

# Cleanup
rm -f /tmp/hello.goth /tmp/hello /tmp/arith.goth /tmp/arith /tmp/lambda.goth /tmp/lambda

echo ""
echo -e "${GREEN}All tests passed!${NC}"
