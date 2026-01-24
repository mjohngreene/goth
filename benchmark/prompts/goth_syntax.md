# Goth Code Generation Prompt (Syntax)

Use this prompt to ask LLMs to generate Goth code in syntax form.

---

## System Prompt

You are generating code in Goth, a functional programming language with De Bruijn indices.

### Goth Syntax Rules:

1. **Function definition**:
   ```
   ╭─ functionName : Type → Type
   ╰─ body
   ```

2. **De Bruijn indices** (instead of named parameters):
   - `₀` = most recent binding (last parameter)
   - `₁` = second most recent
   - `₂` = third most recent
   - For `f : A → B → C`: ₁ = first arg, ₀ = second arg

3. **Types**: `I64` (integer), `F64` (float), `Bool`, `T` (generic)

4. **Operators**:
   - Arithmetic: `+`, `-`, `×`, `/`, `%`
   - Comparison: `=`, `<`, `>`, `≤`, `≥`
   - Boolean: `∧` (and), `∨` (or), `¬` (not)
   - Boolean literals: `⊤` (true), `⊥` (false)

5. **Conditionals**:
   ```
   if condition then expr1 else expr2
   ```

6. **Recursion**: Functions can call themselves by name

### ASCII Alternatives:
- `×` → `*`
- `→` → `->`
- `≤` → `<=`
- `≥` → `>=`
- `∧` → `&&`
- `∨` → `||`
- `¬` → `!`
- `⊤` → `true`
- `⊥` → `false`
- `╭─` → `fn`
- `╰─` → `=`

---

## User Prompt Template

Implement the following function in Goth:

**Function**: {function_name}
**Signature**: {signature}
**Description**: {description}
**Examples**:
{examples}

Generate only the Goth code, no explanation.

---

## Example Prompts

### Factorial
```
Implement the following function in Goth:

Function: factorial
Signature: I64 → I64
Description: Compute n! (n factorial). Return 1 for n ≤ 1.
Examples:
  factorial(0) = 1
  factorial(5) = 120
  factorial(10) = 3628800

Generate only the Goth code, no explanation.
```

### GCD
```
Implement the following function in Goth:

Function: gcd
Signature: I64 → I64 → I64
Description: Compute the greatest common divisor using Euclidean algorithm.
Examples:
  gcd(48, 18) = 6
  gcd(17, 13) = 1

Generate only the Goth code, no explanation.
```

### Is Prime
```
Implement the following function in Goth:

Function: isPrime
Signature: I64 → Bool
Description: Return true if n is prime, false otherwise.
Examples:
  isPrime(2) = ⊤
  isPrime(17) = ⊤
  isPrime(15) = ⊥

Generate only the Goth code, no explanation.
```
