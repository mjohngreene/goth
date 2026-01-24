# Blind Test Protocol

This document describes how to run a fair blind test of LLM code generation for Goth.

## Documents to Include

For a blind test, provide the LLM with **only these documents**:

### Minimal Set (Recommended)
1. **`docs/CLAUDE-SKILLS.md`** - Language syntax and AST structure
2. **One example file** - e.g., `examples/recursion/factorial.goth` as reference

### Extended Set (If Needed)
3. **`benchmark/prompts/goth_syntax.md`** - Syntax generation guide
4. **`benchmark/prompts/goth_json.md`** - JSON AST generation guide

## Documents to EXCLUDE

Do NOT provide these during blind testing:
- Test case files (`benchmark/tests/*.json`)
- Expected outputs
- Python baselines
- Other example implementations

## Test Protocol

### Phase 1: Cold Start (No Examples)

1. Provide only the CLAUDE-SKILLS.md document
2. Give prompts for each test function
3. Record first-attempt outputs
4. No feedback, no retries

### Phase 2: With Reference Example

1. Provide CLAUDE-SKILLS.md + one example (factorial)
2. Give prompts for remaining functions
3. Record first-attempt outputs
4. Compare to Phase 1

### Phase 3: Iterative Correction

1. For Phase 1 failures, provide error message
2. Allow up to 3 retries
3. Record iterations to success

## Prompt Format

Use consistent prompts from `benchmark/prompts/prompt_generator.py`:

```bash
# Generate syntax prompt
python benchmark/prompts/prompt_generator.py factorial

# Generate JSON prompt
python benchmark/prompts/prompt_generator.py --format json factorial
```

## Validation

After collecting LLM output:

```bash
# For syntax output - save to file and run
echo "$LLM_OUTPUT" > /tmp/test.goth
./goth /tmp/test.goth <test_args>

# For JSON output
echo "$LLM_OUTPUT" > /tmp/test.json
./goth --from-json /tmp/test.json <test_args>
```

## Recording Results

For each test, record:

```json
{
  "model": "model-name",
  "test": "factorial",
  "prompt_type": "syntax",
  "phase": 1,
  "attempt": 1,
  "raw_output": "╭─ main : I64 → I64\n╰─ ...",
  "parse_success": true,
  "runtime_error": null,
  "actual_output": "120",
  "expected_output": "120",
  "correct": true,
  "tokens_used": 45
}
```

## Test Order

Run tests in this order (increasing complexity):

1. **basic**: identity, add_one, double, square
2. **basic**: abs, sign, max_two, min_two
3. **basic**: is_even, is_positive
4. **recursion**: factorial, sum_to_n
5. **recursion**: fibonacci, power
6. **recursion**: gcd, lcm
7. **algorithms**: isPrime, isqrt
8. **recursion**: ackermann, mccarthy91

## Comparison Matrix

| Model | Syntax Success | JSON Success | Avg Tokens |
|-------|----------------|--------------|------------|
| Claude Opus | | | |
| Claude Sonnet | | | |
| GPT-4 | | | |
| Gemini | | | |

## Hypothesis Validation

The Goth hypothesis is validated if:

1. **JSON AST > Syntax**: JSON generation has >20% lower error rate
2. **De Bruijn clarity**: <10% of errors are index-related
3. **Competitive with Python**: First-attempt success within 10% of Python
4. **Conciseness**: Goth code uses <80% of Python token count
