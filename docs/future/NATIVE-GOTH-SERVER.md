# Native Goth Web Server: Future Design

> **Status:** Future consideration - not ready for implementation yet
> **Prerequisite:** Concurrency model design
> **Created:** 2026-01-22

## Overview

This document outlines how Goth could support writing web servers natively in the language itself, rather than using Rust wrappers that evaluate Goth code.

## Current State

- `goth-server` is a Rust/Axum server that exposes Goth evaluation via HTTP
- Goth has basic IO primitives (`readFile`, `writeFile`, `print`, `readLine`)
- No networking primitives exist
- No concurrency model exists

## Required Primitives

### Low-Level (Socket API)

```goth
listen   : Port → ◇net Socket
accept   : Socket → ◇net Connection
recv     : Connection → ◇net Bytes
send     : Connection → Bytes → ◇net ()
close    : Connection → ◇net ()
```

### High-Level (HTTP API)

```goth
httpServe   : Port → (Request → Response) → ◇net ()
httpRequest : Method → Url → Headers → Body → ◇net Response

# Request/Response types
type Request  = ⟨method: Method, path: String, headers: Headers, body: Bytes⟩
type Response = ⟨status: I64, headers: Headers, body: Bytes⟩
type Method   = ⟨GET | POST | PUT | DELETE | PATCH⟩
```

## Hypothetical Server Syntax

```goth
# Simple echo server
╭─ echoServer : Port → ◇net ()
╰─ httpServe ₀ (λ req → respond 200 (body req))

# Router with pattern matching
╭─ router : Request → Response
╰─ match (method ₀, path ₀) {
     (GET, "/")        → html "Hello, Goth!"
     (GET, "/health")  → json ⟨status: "ok"⟩
     (POST, "/echo")   → json (body ₀)
     _                 → notFound
   }

╭─ main : () → ◇net ()
╰─ httpServe 8080 router
```

## Concurrency Model Options

A web server must handle multiple concurrent connections. Possible models:

### 1. Single-Threaded Event Loop
- **Approach:** Like early Node.js - one thread, non-blocking IO
- **Pros:** Simple, no shared state, fits functional paradigm
- **Cons:** Limited scalability, CPU-bound work blocks all requests
- **Goth fit:** Good starting point

### 2. Async/Await
- **Approach:** Explicit async functions, await for IO
- **Pros:** Familiar to most developers, efficient
- **Cons:** "Colored" functions, complex semantics
- **Goth fit:** Possible but adds complexity to type system

### 3. Actor Model (Erlang-style)
- **Approach:** Isolated processes communicating via messages
- **Pros:** Fault-tolerant, naturally functional, no shared state
- **Cons:** Different paradigm, message passing overhead
- **Goth fit:** Philosophically aligned with functional approach

### 4. Green Threads
- **Approach:** Lightweight threads managed by runtime
- **Pros:** Simple mental model, looks synchronous
- **Cons:** Hidden complexity, preemption questions
- **Goth fit:** Unclear

**Recommendation:** Start with single-threaded event loop, design for future actor model.

## Implementation Strategies

### Strategy A: FFI to Rust Runtime
- Expose Tokio/Axum primitives to Goth via FFI
- Goth becomes scripting layer for request handling
- Quick to implement but limits Goth's role

### Strategy B: Native Interpreter Primitives
- Add socket operations directly to `goth-eval`
- More authentic but significant work
- Would need async runtime in interpreter

### Strategy C: Compile to Async Rust
- Generate Rust async code from Goth source
- Most powerful, best performance
- Requires async semantics in Goth type system

## Effect System Integration

Network operations should be tracked by the effect system:

```goth
# Pure function - no effects
╭─ parseRequest : String → Request
╰─ ...

# Effectful - requires ◇net
╭─ handleRequest : Request → ◇net Response
╰─ ...

# Effect polymorphic handler
╭─ withLogging : ∀ε. (Request → ε Response) → Request → ε⊕◇io Response
╰─ ...
```

## Design Principles

1. **Pure handlers where possible** - Request → Response should be pure
2. **Effects explicit** - All IO tracked in types
3. **Pattern matching for routing** - Leverage Goth's strength
4. **Compositional** - Middleware as function composition
5. **Tensor-native** - Could process batch requests as tensors?

## Open Questions

1. How does the concurrency model interact with the effect system?
2. Should we support WebSockets? Server-sent events?
3. How do we handle streaming request/response bodies?
4. What's the error handling model? (Effect-based? Result types?)
5. Could tensor operations enable novel batch processing of requests?

## Prerequisites Before Implementation

- [ ] Concurrency model design and decision
- [ ] Effect system fully implemented and enforced
- [ ] Standard library with string/bytes manipulation
- [ ] LLVM backend working for native compilation
- [ ] FFI system for calling into Rust async runtime

## References

- Haskell Servant: Type-level routing DSL
- Elixir Phoenix: Actor-based web framework
- Rust Axum: Tower-based middleware composition
- OCaml Dream: Simple functional web framework
