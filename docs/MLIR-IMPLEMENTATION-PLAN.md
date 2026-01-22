# Goth MLIR Backend - Detailed Implementation Plan

## Executive Summary

This document outlines a comprehensive plan to upgrade the Goth compiler's MLIR backend from text-based string generation to proper MLIR bindings using the `melior` crate, enabling proper dialect support, optimization passes, and verification.

---

## Part 1: Current State Analysis

### 1.1 What Currently Exists

| Component | Status | Location |
|-----------|--------|----------|
| MLIR Context | Text-based SSA management | `goth-mlir/src/emit.rs:15-72` |
| Type Emission | String generation | `goth-mlir/src/emit.rs:75-129` |
| Binary Ops | arith dialect (text) | `goth-mlir/src/emit.rs:212-252` |
| Unary Ops | arith/math dialect (text) | `goth-mlir/src/emit.rs:254-310` |
| Control Flow | cf dialect (text) | `goth-mlir/src/emit.rs:509-539` |
| Tensor Ops | tensor dialect (text) | `goth-mlir/src/emit.rs:367-481` |
| Functions | func dialect (text) | `goth-mlir/src/emit.rs:549-598` |
| Program | module wrapper (text) | `goth-mlir/src/emit.rs:600-616` |

### 1.2 Supported Dialects (Text-Based)

```
Dialect         Operations Used                           Purpose
─────────────────────────────────────────────────────────────────────
func            func.func, func.return, func.call_indirect   Functions
arith           addi, subi, muli, divsi, cmpi, etc.         Arithmetic
cf              br, cond_br, switch                          Control flow
tensor          extract, from_elements                       Arrays
math            sqrt, floor, ceil                            Math
builtin         unrealized_conversion_cast                   Tuples (hack)
goth.*          reduce_sum/prod/min/max, map, filter, iota   Custom ops
```

### 1.3 Critical Gaps

| Gap | Impact | Risk |
|-----|--------|------|
| Text-based emission (not using MLIR C API) | No verification, fragile | HIGH |
| Missing `linalg` dialect | Can't do tensor math properly | HIGH |
| Missing `scf` dialect | No structured control flow | MEDIUM |
| Missing `memref` dialect | No memory management | HIGH |
| No bufferization passes | Can't lower to executable | CRITICAL |
| No optimization passes | Poor performance | MEDIUM |
| No MLIR verification | Invalid IR goes undetected | HIGH |

---

## Part 2: Target Architecture

### 2.1 Proposed Structure

```
goth-mlir/
├── Cargo.toml              # Add melior dependency
├── src/
│   ├── lib.rs              # Public API
│   ├── context.rs          # MLIR context wrapper
│   ├── types.rs            # Type conversion (Goth → MLIR)
│   ├── dialects/
│   │   ├── mod.rs          # Dialect registry
│   │   ├── arith.rs        # Arithmetic operations
│   │   ├── func.rs         # Function handling
│   │   ├── scf.rs          # Structured control flow
│   │   ├── linalg.rs       # Tensor operations
│   │   ├── tensor.rs       # Tensor types and ops
│   │   ├── memref.rs       # Memory references
│   │   └── goth.rs         # Custom Goth dialect
│   ├── passes/
│   │   ├── mod.rs          # Pass manager
│   │   ├── bufferize.rs    # Tensor → MemRef conversion
│   │   ├── lower_goth.rs   # Goth dialect → standard
│   │   └── optimize.rs     # Optimization passes
│   ├── builder.rs          # High-level IR builder
│   ├── emit.rs             # MIR → MLIR conversion
│   └── error.rs            # Error types
└── tests/
    ├── integration.rs
    └── dialect_tests.rs
```

### 2.2 Dependencies

```toml
[dependencies]
melior = "0.18"              # Main MLIR bindings
mlir-sys = "0.2"             # Low-level FFI (if needed)
goth-ast = { path = "../goth-ast" }
goth-mir = { path = "../goth-mir" }
thiserror = "1.0"

[build-dependencies]
# May need LLVM/MLIR headers
```

---

## Part 3: Implementation Phases

### Phase 1: Foundation (Core MLIR Integration)

**Goal:** Replace text-based emission with proper `melior` bindings

#### Task 1.1: Set Up Melior Integration
- [ ] Add `melior` to Cargo.toml
- [ ] Verify LLVM/MLIR system dependencies
- [ ] Create basic context wrapper
- [ ] Write "hello world" test using melior

```rust
// Target: src/context.rs
use melior::{
    Context,
    ir::{Module, Location, Block, Region, Operation, Type, Value},
    dialect::DialectRegistry,
};

pub struct GothMlirContext {
    ctx: Context,
    module: Module,
    // Location tracking for debugging
    current_location: Location,
}

impl GothMlirContext {
    pub fn new() -> Self {
        let ctx = Context::new();
        ctx.load_all_available_dialects();

        let module = Module::new(Location::unknown(&ctx));

        Self {
            ctx,
            module,
            current_location: Location::unknown(&ctx),
        }
    }
}
```

#### Task 1.2: Implement Type Mapping
- [ ] Map Goth primitive types to MLIR types
- [ ] Map Goth tensor types to MLIR tensor types
- [ ] Map Goth function types to MLIR function types
- [ ] Handle type variables

```rust
// Target: src/types.rs
use melior::ir::Type as MlirType;
use goth_ast::types::{Type, PrimType};

pub fn convert_type(ctx: &Context, ty: &Type) -> Result<MlirType> {
    match ty {
        Type::Prim(PrimType::I64) => Ok(Type::integer(ctx, 64)),
        Type::Prim(PrimType::F64) => Ok(Type::float64(ctx)),
        Type::Prim(PrimType::Bool) => Ok(Type::integer(ctx, 1)),

        Type::Tensor(shape, elem) => {
            let elem_ty = convert_type(ctx, elem)?;
            let dims = convert_shape(shape)?;
            Ok(Type::ranked_tensor(&dims, elem_ty))
        }

        Type::Fn(arg, ret) => {
            let arg_ty = convert_type(ctx, arg)?;
            let ret_ty = convert_type(ctx, ret)?;
            Ok(Type::function(&[arg_ty], &[ret_ty]))
        }

        // ...
    }
}
```

#### Task 1.3: Implement Basic Operations
- [ ] Arithmetic operations (arith dialect)
- [ ] Comparison operations
- [ ] Constants
- [ ] Unit tests for each

```rust
// Target: src/dialects/arith.rs
use melior::dialect::arith;

pub fn emit_add(
    builder: &mut OpBuilder,
    lhs: Value,
    rhs: Value,
    loc: Location,
) -> Value {
    if is_integer_type(lhs.r#type()) {
        arith::addi(builder, lhs, rhs, loc)
    } else {
        arith::addf(builder, lhs, rhs, loc)
    }
}

pub fn emit_constant_int(
    builder: &mut OpBuilder,
    value: i64,
    loc: Location,
) -> Value {
    arith::constant(
        builder,
        IntegerAttr::new(Type::integer(builder.context(), 64), value),
        loc,
    )
}
```

#### Task 1.4: Implement Function Emission
- [ ] Function signatures
- [ ] Function bodies
- [ ] Entry blocks
- [ ] Return statements

```rust
// Target: src/dialects/func.rs
use melior::dialect::func;

pub fn emit_function(
    ctx: &mut GothMlirContext,
    func: &goth_mir::mir::Function,
) -> Result<()> {
    let func_type = build_function_type(ctx, &func.params, &func.ret_ty)?;

    let func_op = func::func(
        ctx.context(),
        StringAttr::new(ctx.context(), &func.name),
        func_type,
        // ... attributes
    );

    // Build function body
    let region = Region::new();
    let entry_block = Block::new(&build_block_args(&func.params)?);

    // Emit statements
    for stmt in &func.body.stmts {
        emit_stmt(ctx, stmt, &entry_block)?;
    }

    // Emit terminator
    emit_terminator(ctx, &func.body.term, &entry_block)?;

    region.append_block(entry_block);
    func_op.set_body(region);

    ctx.module().body().append_operation(func_op);
    Ok(())
}
```

### Phase 2: Control Flow & Complex Operations

**Goal:** Support all MIR control flow and tensor operations

#### Task 2.1: Implement SCF Dialect (Structured Control Flow)
- [ ] `scf.if` for conditionals
- [ ] `scf.for` for loops (if needed)
- [ ] `scf.while` for general loops
- [ ] Proper block arguments

```rust
// Target: src/dialects/scf.rs
use melior::dialect::scf;

pub fn emit_if(
    builder: &mut OpBuilder,
    condition: Value,
    then_region: Region,
    else_region: Region,
    result_types: &[Type],
    loc: Location,
) -> Operation {
    scf::if_(
        builder,
        condition,
        result_types,
        then_region,
        else_region,
        loc,
    )
}
```

#### Task 2.2: Implement CF Dialect (Unstructured Control Flow)
- [ ] `cf.br` unconditional branch
- [ ] `cf.cond_br` conditional branch
- [ ] `cf.switch` for match expressions
- [ ] Block argument passing

#### Task 2.3: Implement Tensor Operations
- [ ] `tensor.extract` for indexing
- [ ] `tensor.from_elements` for array literals
- [ ] `tensor.generate` for computed arrays
- [ ] `tensor.reshape` for shape changes

#### Task 2.4: Implement Linalg Operations (Critical for Goth)
- [ ] `linalg.generic` for map operations
- [ ] `linalg.reduce` for reductions
- [ ] `linalg.matmul` for matrix multiply
- [ ] `linalg.dot` for dot products

```rust
// Target: src/dialects/linalg.rs
use melior::dialect::linalg;

pub fn emit_map(
    builder: &mut OpBuilder,
    input: Value,
    func: Value,
    output_type: Type,
    loc: Location,
) -> Value {
    // Use linalg.generic with appropriate indexing maps
    linalg::generic(
        builder,
        &[input],
        &[output],
        indexing_maps,
        iterator_types,
        |block_builder| {
            // Apply function to each element
        },
        loc,
    )
}

pub fn emit_reduce(
    builder: &mut OpBuilder,
    input: Value,
    op: ReduceOp,
    axis: i64,
    loc: Location,
) -> Value {
    match op {
        ReduceOp::Sum => linalg::reduce_add(builder, input, axis, loc),
        ReduceOp::Prod => linalg::reduce_mul(builder, input, axis, loc),
        ReduceOp::Max => linalg::reduce_max(builder, input, axis, loc),
        ReduceOp::Min => linalg::reduce_min(builder, input, axis, loc),
    }
}
```

### Phase 3: Custom Goth Dialect

**Goal:** Define custom operations for Goth-specific semantics

#### Task 3.1: Define Goth Dialect ODS
- [ ] Register dialect with MLIR
- [ ] Define operation interfaces
- [ ] Define type constraints

```tablegen
// goth_dialect.td (if using TableGen)
def Goth_Dialect : Dialect {
  let name = "goth";
  let cppNamespace = "::goth";
  let summary = "Goth tensor language dialect";
}

def Goth_IotaOp : Goth_Op<"iota", [NoSideEffect]> {
  let summary = "Generate tensor [0, 1, 2, ..., n-1]";
  let arguments = (ins I64:$size);
  let results = (outs AnyTensor:$result);
}

def Goth_MapOp : Goth_Op<"map", [NoSideEffect]> {
  let summary = "Apply function elementwise";
  let arguments = (ins AnyTensor:$input, FunctionType:$func);
  let results = (outs AnyTensor:$output);
}
```

#### Task 3.2: Implement Goth Dialect in Rust
- [ ] Create dialect using melior APIs
- [ ] Register custom operations
- [ ] Implement operation builders

```rust
// Target: src/dialects/goth.rs

pub fn register_goth_dialect(ctx: &Context) {
    // Register custom dialect
    let registry = ctx.dialect_registry();
    // ... registration
}

pub fn emit_iota(
    builder: &mut OpBuilder,
    size: Value,
    elem_type: Type,
    loc: Location,
) -> Value {
    // Custom goth.iota operation
    let op = builder.create_operation(
        "goth.iota",
        loc,
        &[("size", size)],
        &[Type::ranked_tensor(&[size], elem_type)],
    );
    op.result(0)
}
```

### Phase 4: Passes and Lowering

**Goal:** Enable full compilation pipeline to LLVM

#### Task 4.1: Implement Bufferization Pass
- [ ] Convert tensor types to memref types
- [ ] Insert allocation operations
- [ ] Handle tensor copies

```rust
// Target: src/passes/bufferize.rs

pub fn bufferize_pass(module: &mut Module) -> Result<()> {
    let pm = PassManager::new(module.context());

    // Standard bufferization pipeline
    pm.add_pass(bufferization::one_shot_bufferize());
    pm.add_pass(bufferization::buffer_deallocation());

    pm.run(module)?;
    Ok(())
}
```

#### Task 4.2: Implement Goth → Standard Lowering
- [ ] Lower `goth.iota` to `linalg.generic`
- [ ] Lower `goth.map` to `linalg.generic`
- [ ] Lower `goth.reduce_*` to `linalg.reduce`
- [ ] Handle closures properly

```rust
// Target: src/passes/lower_goth.rs

pub fn lower_goth_dialect(module: &mut Module) -> Result<()> {
    let pm = PassManager::new(module.context());
    pm.add_pass(create_lower_goth_to_linalg());
    pm.run(module)?;
    Ok(())
}

fn lower_iota(op: &Operation, rewriter: &mut PatternRewriter) {
    // Convert goth.iota to linalg.generic with index-based computation
}
```

#### Task 4.3: Implement Optimization Passes
- [ ] Canonicalization
- [ ] Common subexpression elimination
- [ ] Dead code elimination
- [ ] Loop fusion (for tensor operations)

```rust
// Target: src/passes/optimize.rs

pub fn optimize_module(module: &mut Module, level: OptLevel) -> Result<()> {
    let pm = PassManager::new(module.context());

    // Canonicalization
    pm.add_pass(canonicalizer());

    // CSE
    pm.add_pass(cse());

    // Linalg-specific optimizations
    if level >= OptLevel::O2 {
        pm.add_pass(linalg::fusion());
        pm.add_pass(linalg::tiling());
    }

    pm.run(module)?;
    Ok(())
}
```

#### Task 4.4: LLVM Lowering
- [ ] Lower to LLVM dialect
- [ ] Generate LLVM IR
- [ ] Integration with existing goth-llvm crate

```rust
// Target: src/passes/to_llvm.rs

pub fn lower_to_llvm(module: &mut Module) -> Result<String> {
    let pm = PassManager::new(module.context());

    // Standard lowering pipeline
    pm.add_pass(convert_func_to_llvm());
    pm.add_pass(convert_arith_to_llvm());
    pm.add_pass(convert_cf_to_llvm());
    pm.add_pass(convert_memref_to_llvm());

    pm.run(module)?;

    // Extract LLVM IR
    module.translate_to_llvm_ir()
}
```

### Phase 5: Integration & Testing

**Goal:** Full end-to-end compilation working

#### Task 5.1: Update Compiler Pipeline
- [ ] Update `goth-cli` to use new MLIR backend
- [ ] Add `--mlir-opt` flag for optimization level
- [ ] Add `--emit-mlir` to dump MLIR at various stages

#### Task 5.2: Comprehensive Testing
- [ ] Unit tests for each dialect operation
- [ ] Integration tests with MIR
- [ ] End-to-end tests compiling to executable
- [ ] Regression tests against current interpreter output

#### Task 5.3: Error Handling & Diagnostics
- [ ] MLIR verification errors
- [ ] Source location tracking
- [ ] Helpful error messages

---

## Part 4: Implementation Schedule

### Milestone 1: Basic Working Backend
- Phase 1 complete
- Can emit valid MLIR for simple programs
- All existing tests pass

### Milestone 2: Tensor Operations
- Phase 2 and 3 complete
- Can handle tensor operations via linalg
- Custom dialect working

### Milestone 3: Full Compilation
- Phase 4 complete
- Can compile to native executables
- Optimization passes working

### Milestone 4: Production Ready
- Phase 5 complete
- Full test coverage
- Documentation complete

---

## Part 5: Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Melior version compatibility | Medium | High | Pin version, test thoroughly |
| LLVM system dependency issues | High | High | Document build requirements |
| Complex tensor lowering | Medium | Medium | Start with simple cases |
| Performance regressions | Low | Medium | Benchmark against interpreter |

---

## Part 6: Testing Strategy

### Unit Tests (Per Phase)
```rust
#[test]
fn test_emit_add_integer() {
    let ctx = GothMlirContext::new();
    let lhs = emit_constant_int(&ctx, 1);
    let rhs = emit_constant_int(&ctx, 2);
    let result = emit_add(&ctx, lhs, rhs);

    assert!(ctx.verify_module());
    // Check the operation type
}
```

### Integration Tests
```rust
#[test]
fn test_mir_to_mlir_simple() {
    let mir = lower_expr(&parse("1 + 2").unwrap()).unwrap();
    let mlir = emit_program(&mir).unwrap();

    assert!(mlir.verify());
}
```

### End-to-End Tests
```rust
#[test]
fn test_compile_and_run() {
    let result = compile_and_run("let x ← 5 in x + 1");
    assert_eq!(result, "6");
}
```

---

## Part 7: Success Criteria

### Phase 1 Complete When:
- [ ] `melior` dependency integrated
- [ ] Type conversion working
- [ ] All arith operations emit valid MLIR
- [ ] Functions emit correctly
- [ ] MLIR verifier passes

### Phase 2 Complete When:
- [ ] All MIR statements emit valid MLIR
- [ ] Control flow (if/switch) working
- [ ] Basic tensor operations working
- [ ] Closures handled

### Phase 3 Complete When:
- [ ] Custom goth dialect defined
- [ ] All goth-specific operations implemented
- [ ] Lowering patterns to standard dialects

### Phase 4 Complete When:
- [ ] Bufferization working
- [ ] Can lower to LLVM dialect
- [ ] Can generate LLVM IR
- [ ] Optimization passes improve output

### Phase 5 Complete When:
- [ ] Full compiler pipeline working
- [ ] All interpreter test cases compile and run correctly
- [ ] Performance acceptable
- [ ] Documentation complete

---

## Appendix A: Key Melior APIs

```rust
// Creating context
let ctx = Context::new();
ctx.load_all_available_dialects();

// Creating module
let module = Module::new(Location::unknown(&ctx));

// Creating operations
let op = OperationBuilder::new("arith.addi", location)
    .add_operands(&[lhs, rhs])
    .add_results(&[Type::integer(&ctx, 64)])
    .build();

// Running passes
let pm = PassManager::new(&ctx);
pm.add_pass(pass);
pm.run(&mut module)?;

// Verification
module.verify()?;

// LLVM translation
let llvm_ir = module.to_llvm_ir()?;
```

---

## Appendix B: Dialect Reference

### Arith Dialect (Arithmetic)
| Goth Op | MLIR Op | Notes |
|---------|---------|-------|
| `+` (int) | `arith.addi` | |
| `+` (float) | `arith.addf` | |
| `-` (int) | `arith.subi` | |
| `*` (int) | `arith.muli` | |
| `/` (int) | `arith.divsi` | Signed |
| `<` (int) | `arith.cmpi slt` | |
| `==` | `arith.cmpi eq` | |

### Linalg Dialect (Tensor Operations)
| Goth Op | MLIR Op | Notes |
|---------|---------|-------|
| `↦` (map) | `linalg.generic` | Elementwise |
| `Σ` (sum) | `linalg.reduce` | With add |
| `Π` (prod) | `linalg.reduce` | With mul |
| `@` (matmul) | `linalg.matmul` | |

### SCF Dialect (Control Flow)
| Goth Construct | MLIR Op | Notes |
|----------------|---------|-------|
| if/then/else | `scf.if` | With regions |
| match | `scf.if` chain or cf.switch | |

---

## Appendix C: File Mapping (Old → New)

| Current File | New Files |
|--------------|-----------|
| `emit.rs` (900 lines) | `context.rs`, `types.rs`, `builder.rs`, `emit.rs` |
| (new) | `dialects/arith.rs` |
| (new) | `dialects/func.rs` |
| (new) | `dialects/scf.rs` |
| (new) | `dialects/linalg.rs` |
| (new) | `dialects/goth.rs` |
| (new) | `passes/mod.rs` |
| (new) | `passes/bufferize.rs` |
| (new) | `passes/lower_goth.rs` |
| (new) | `passes/optimize.rs` |
