# Goth Code Generation Prompt (JSON AST)

Use this prompt to ask LLMs to generate Goth code as JSON AST.

---

## System Prompt

You are generating code in Goth as a JSON Abstract Syntax Tree. This is more reliable than syntax generation because the structure is explicit.

### JSON AST Structure:

```json
{
  "decls": [
    {
      "Fn": {
        "name": "functionName",
        "signature": {"Arrow": [{"Base": "I64"}, {"Base": "I64"}]},
        "body": { ... expression ... }
      }
    }
  ]
}
```

### Expression Types:

- **Literal**: `{"Lit": {"Int": 42}}` or `{"Lit": {"Float": 3.14}}` or `{"Lit": {"Bool": true}}`
- **De Bruijn index**: `{"Idx": 0}` (₀), `{"Idx": 1}` (₁), etc.
- **Function call**: `{"App": [function_expr, arg_expr]}`
- **Named reference**: `{"Name": "functionName"}`
- **Binary op**: `{"BinOp": ["Add", left_expr, right_expr]}`
- **Unary op**: `{"UnaryOp": ["Neg", expr]}`
- **Conditional**: `{"If": [condition, then_expr, else_expr]}`
- **Let binding**: `{"Let": {"bindings": [["x", expr]], "body": body_expr}}`

### Binary Operators:
`Add`, `Sub`, `Mul`, `Div`, `Mod`, `Eq`, `Lt`, `Gt`, `Le`, `Ge`, `And`, `Or`

### Type Syntax:
- Base type: `{"Base": "I64"}`, `{"Base": "F64"}`, `{"Base": "Bool"}`
- Function: `{"Arrow": [arg_type, return_type]}`
- Multi-arg: `{"Arrow": [t1, {"Arrow": [t2, t3]}]}`

### De Bruijn Convention:
For `f : A → B → C` (two parameters):
- `{"Idx": 1}` = first parameter (type A)
- `{"Idx": 0}` = second parameter (type B)

---

## User Prompt Template

Generate a Goth JSON AST for the following function:

**Function**: {function_name}
**Signature**: {signature}
**Description**: {description}
**Examples**:
{examples}

Output only valid JSON, no explanation.

---

## Example: Factorial

Prompt:
```
Generate a Goth JSON AST for the following function:

Function: factorial
Signature: I64 → I64
Description: Compute n! (n factorial). Return 1 for n ≤ 1.
Examples:
  factorial(0) = 1
  factorial(5) = 120

Output only valid JSON, no explanation.
```

Expected output:
```json
{
  "decls": [
    {
      "Fn": {
        "name": "main",
        "signature": {"Arrow": [{"Base": "I64"}, {"Base": "I64"}]},
        "body": {
          "If": [
            {"BinOp": ["Lt", {"Idx": 0}, {"Lit": {"Int": 2}}]},
            {"Lit": {"Int": 1}},
            {"BinOp": ["Mul",
              {"Idx": 0},
              {"App": [{"Name": "main"}, {"BinOp": ["Sub", {"Idx": 0}, {"Lit": {"Int": 1}}]}]}
            ]}
          ]
        }
      }
    }
  ]
}
```

---

## Example: GCD

```json
{
  "decls": [
    {
      "Fn": {
        "name": "main",
        "signature": {"Arrow": [{"Base": "I64"}, {"Arrow": [{"Base": "I64"}, {"Base": "I64"}]}]},
        "body": {
          "If": [
            {"BinOp": ["Eq", {"Idx": 0}, {"Lit": {"Int": 0}}]},
            {"Idx": 1},
            {"App": [
              {"App": [{"Name": "main"}, {"Idx": 0}]},
              {"BinOp": ["Mod", {"Idx": 1}, {"Idx": 0}]}
            ]}
          ]
        }
      }
    }
  ]
}
```
