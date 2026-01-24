# Tail Call Optimization (TCO) Examples

This directory demonstrates the difference between naive recursive functions and tail-recursive versions that benefit from TCO.

## What is TCO?

Tail Call Optimization eliminates stack growth for functions where the recursive call is the **last operation** (in "tail position"). Instead of pushing new stack frames, the interpreter reuses the current frame.

## The Pattern

**NOT tail recursive** (stack grows):
```goth
╭─ sum : I64 → I64
╰─ if ₀ < 1 then 0
   else ₀ + sum (₀ - 1)    # ← addition happens AFTER recursive call
```

**IS tail recursive** (constant stack):
```goth
╭─ sumAcc : I64 → I64 → I64
╰─ if ₁ < 1 then ₀
   else sumAcc (₁ - 1) (₀ + ₁)    # ← recursive call IS the last operation
```

## Accumulator Transformation

The key insight: move the "pending work" into an accumulator parameter.

| Naive | TCO |
|-------|-----|
| `n + rec(n-1)` | `rec(n-1, acc+n)` |
| `n × rec(n-1)` | `rec(n-1, acc×n)` |
| `1 + rec(...)` | `rec(..., acc+1)` |

## Examples

| Function | Naive | TCO | Notes |
|----------|-------|-----|-------|
| Factorial | `factorial_naive.goth` | `factorial_tco.goth` | Product accumulator |
| Fibonacci | `fibonacci_naive.goth` | `fibonacci_tco.goth` | Two accumulators (a,b) |
| Sum 1..n | `sum_naive.goth` | `sum_tco.goth` | Sum accumulator |
| Collatz | `collatz_naive.goth` | `collatz_tco.goth` | Step counter |
| Length | `length_naive.goth` | `length_tco.goth` | Count accumulator |

## Testing the Difference

```bash
# TCO version handles deep recursion
cargo run --package goth-cli -- examples/tco/collatz_tco.goth 27
# Output: 111

# Naive version overflows
cargo run --package goth-cli -- examples/tco/collatz_naive.goth 27
# Output: (stack overflow)

# TCO handles 5000 iterations
cargo run --package goth-cli -- examples/tco/sum_tco.goth 5000
# Output: 12502500
```

## De Bruijn Index Convention

For accumulator functions with 2 parameters `f(n, acc)`:
- `₁` = first parameter (n, the value being processed)
- `₀` = second parameter (acc, the accumulator)

For 3 parameters `f(n, a, b)`:
- `₂` = first parameter
- `₁` = second parameter
- `₀` = third parameter

## When to Use TCO

Use tail recursion when:
- Processing potentially large inputs
- Recursion depth is unbounded
- You need predictable memory usage

The naive version is fine when:
- Input size is bounded and small
- Code clarity matters more than efficiency
- You're demonstrating the algorithm (not running it at scale)
