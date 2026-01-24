# Benchmark Metrics

This document defines the metrics collected when benchmarking LLM code generation for Goth.

## Primary Metrics

### 1. First-Attempt Success Rate

**Definition**: Percentage of prompts where the LLM generates code that:
- Parses without syntax errors
- Produces correct output on all test cases

**Formula**: `successful_first_attempts / total_prompts × 100%`

**Why it matters**: Measures raw generation accuracy without iteration.

### 2. Parse Success Rate

**Definition**: Percentage of generated code that parses successfully.

**Formula**: `parseable_outputs / total_prompts × 100%`

**Why it matters**: Separates syntax errors from semantic errors.

### 3. Semantic Correctness Rate

**Definition**: Of code that parses, percentage that produces correct output.

**Formula**: `correct_outputs / parseable_outputs × 100%`

**Why it matters**: Measures understanding of the problem vs. language mechanics.

### 4. Iterations to Correct

**Definition**: Number of attempts needed to get correct code.

**Tracking**:
- 1 = first attempt correct
- 2+ = required error feedback and retry
- MAX = never achieved correctness

**Why it matters**: Measures ability to learn from feedback.

## Secondary Metrics

### 5. Token Count

**Definition**: Number of tokens in generated code.

**Comparison**: Compare to Python baseline for same function.

**Why it matters**: Goth should be more concise due to De Bruijn indices.

### 6. Error Types

**Categories**:
- `PARSE_ERROR`: Syntax/parsing failure
- `TYPE_ERROR`: Type mismatch (if type checking enabled)
- `RUNTIME_ERROR`: Evaluation error (division by zero, etc.)
- `WRONG_OUTPUT`: Code runs but produces incorrect result
- `TIMEOUT`: Execution exceeded time limit

**Why it matters**: Identifies systematic weaknesses.

### 7. De Bruijn Index Errors

**Definition**: Frequency of incorrect index usage.

**Common errors**:
- Wrong index for parameter count (using ₀ instead of ₁)
- Off-by-one in multi-arg functions
- Confusion after let bindings (indices shift)

**Why it matters**: Core hypothesis test for De Bruijn clarity.

## Comparison Metrics

### 8. Goth Syntax vs JSON AST

Compare first-attempt success rate between:
- Generating Goth syntax directly
- Generating JSON AST

**Hypothesis**: JSON AST should have lower syntax error rate.

### 9. Goth vs Python

Compare for same functions:
- Token count
- First-attempt success rate
- Error types

**Hypothesis**: Goth should have fewer variable scoping errors.

## Data Collection Format

```json
{
  "model": "claude-3-opus",
  "timestamp": "2024-01-24T12:00:00Z",
  "prompt_type": "syntax",
  "results": [
    {
      "test": "factorial",
      "attempt": 1,
      "parse_success": true,
      "correct": true,
      "output": "120",
      "expected": "120",
      "tokens": 45,
      "error_type": null,
      "raw_output": "╭─ main : I64 → I64\n╰─ ..."
    }
  ],
  "summary": {
    "total": 10,
    "first_attempt_correct": 8,
    "parse_failures": 1,
    "semantic_failures": 1
  }
}
```

## Benchmark Protocol

### Phase 1: Baseline Collection
1. Run each prompt once per model
2. Record raw outputs and metrics
3. No feedback or iteration

### Phase 2: Iterative Correction
1. For failed attempts, provide error message
2. Allow up to 3 retry attempts
3. Record iterations to correct

### Phase 3: Comparative Analysis
1. Compare Goth syntax vs JSON AST
2. Compare Goth vs Python baselines
3. Identify systematic patterns

## Success Criteria

The Goth hypothesis is supported if:
1. JSON AST generation has >20% lower syntax error rate than direct syntax
2. De Bruijn index errors are <10% of semantic errors
3. Goth has comparable or better first-attempt success vs Python
4. Token count is <80% of equivalent Python code
