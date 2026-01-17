# goth-ast

Abstract Syntax Tree for the **Goth** programming language.

Goth is an LLM-native programming language designed for efficient code generation by large language models.

## Features

- **De Bruijn indices** — No name management, positional variable binding
- **Shape-indexed types** — Tensor dimensions tracked in the type system
- **Effect tracking** — IO, mutation, randomness as type-level capabilities
- **Interval types** — Value ranges proven at compile time
- **Refinement types** — Pre/postconditions as executable specifications
- **Homoiconic** — Code is data, enabling powerful metaprogramming

## Source Formats

The AST can be serialized to three isomorphic formats:

| Format | Extension | Use Case |
|--------|-----------|----------|
| Text | `.goth` | Human authoring/reading |
| JSON | `.gast` | Tooling, diffs, transforms |
| Binary | `.gbin` | LLM I/O, compilation |

## Quick Example

```rust
use goth_ast::prelude::*;

// Define: ╭─ dot : [n]F64 → [n]F64 → F64
//         ╰─ ₀ ⊗ ₁ Σ
let dot_product = FnDecl::simple(
    "dot",
    Type::func_n(
        [
            Type::tensor(Shape::symbolic(&["n"]), Type::f64()),
            Type::tensor(Shape::symbolic(&["n"]), Type::f64()),
        ],
        Type::f64(),
    ),
    Expr::sum(Expr::zip_with(Expr::idx(1), Expr::idx(0))),
);

// Pretty print
println!("{}", pretty::print_fn(&dot_product));

// Serialize to JSON
let module = Module::new(vec![dot_product.into()]);
let json = ser::to_json(&module).unwrap();
```

Output:
```
╭─ dot : [n]F64 → [n]F64 → F64
╰─ Σ(₁ ⊗ ₀)
```

## Modules

- `op` — Binary and unary operators
- `literal` — Literal values
- `shape` — Tensor shapes and dimensions
- `effect` — Effect system (□ pure, ◇io, ◇mut, etc.)
- `interval` — Interval types for value ranges
- `types` — Full type system
- `pattern` — Pattern matching
- `expr` — Core expression AST
- `decl` — Top-level declarations
- `pretty` — Pretty printing to text
- `ser` — Serialization (JSON, binary)

## Operator Glyphs

| Glyph | ASCII | Name |
|-------|-------|------|
| `↦` | `-:` | map |
| `▸` | `\|>` | filter |
| `⤇` | `=>` | bind |
| `⊗` | `*:` | zip-with |
| `⊕` | `+:` | concat |
| `Σ` | `+/` | sum |
| `Π` | `*/` | product |
| `∘` | `.:` | compose |
| `λ→` | `\->` | lambda |
| `⊢` | `\|-` | precondition |
| `⊨` | `\|=` | postcondition |

## De Bruijn Indices

Variables are referenced by position, not name:

```
₀  — innermost binding
₁  — one level out
₂  — two levels out
```

Inside a lambda, all outer indices shift up by 1.

## License

MIT
