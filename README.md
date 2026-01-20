# 🦇 𝔊𝔬𝔱𝔥

A functional programming language with Unicode syntax, dependent types, and tensor operations.

## Quick Start

```sh
cd crates
cargo build --release
```

### Interpreter

```sh
# REPL
./target/release/goth

# Run a file
./target/release/goth ../examples/factorial.goth

# Evaluate expression
./target/release/goth -e "Σ [1, 2, 3, 4, 5]"
```

### Compiler

```sh
# Compile to native executable
./target/release/gothic ../examples/hello_main.goth -o hello
./hello

# Emit LLVM IR
./target/release/gothic program.goth --emit-llvm

# Emit MIR
./target/release/gothic program.goth --emit-mir
```

### Tests

```sh
# Unit tests
cargo test

# Integration tests (interpreter + compiler)
cd .. && bash tests/self_compile_test.sh
```

## Example

```goth
╭─ factorial : I64 → I64
╰─ if ₀ ≤ 1 then 1 else ₀ × factorial (₀ - 1)

╭─ main : () → I64
╰─ factorial 10
```

## Documentation

- [Language Specification](./LANGUAGE.md) — Full syntax and semantics
- [Philosophy](./docs/PHILOSOPHY.md) — Design rationale

## License

MIT © 2026 Sigilante
