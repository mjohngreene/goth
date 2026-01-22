# Web Browser Interface for Goth

This document analyzes options for creating a web-based interface for Goth, separate from the terminal TUI.

## Executive Summary

The Goth codebase is **exceptionally well-suited for WebAssembly (WASM) compilation**. The core interpreter crates have no platform-specific dependencies and can be compiled to WASM with minimal modifications. The main work required is abstracting the I/O layer.

## Architecture Analysis

### Crate Structure & WASM Compatibility

| Crate | Purpose | WASM Ready | Notes |
|-------|---------|------------|-------|
| `goth-ast` | AST definitions, types | ✅ Yes | Uses serde, bincode (WASM-compatible) |
| `goth-parse` | Lexer (logos) + Parser | ✅ Yes | Pure Rust, no platform deps |
| `goth-check` | Type inference/checking | ✅ Yes | Minimal dependencies |
| `goth-eval` | Interpreter runtime | ⚠️ Mostly | I/O primitives need abstraction |
| `goth-mir` | Middle IR lowering | ✅ Yes | Pure transformations |
| `goth-mlir` | MLIR emission | ✅ Yes | String generation |
| `goth-llvm` | LLVM IR generation | ✅ Yes | String generation |
| `goth-cli` | Terminal REPL | ❌ No | Uses `rustyline`, `dirs` |

### Current I/O Implementation

The I/O primitives in `goth-eval/src/prim.rs` are hardwired to native I/O:

```rust
// Print - uses std::io
PrimFn::Print => {
    print!("{}", s);
    println!();
    Ok(Value::Unit)
}

// ReadLine - uses stdin
PrimFn::ReadLine => {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(Value::string(&line))
}

// File I/O - uses std::fs
PrimFn::ReadFile => fs::read_to_string(&path)?
PrimFn::WriteFile => fs::write(&path, &contents)?
```

### Required Changes for Web

1. **I/O Abstraction Trait** - Create an `IoBackend` trait:
   ```rust
   pub trait IoBackend: Send + Sync {
       fn print(&self, text: &str);
       fn read_line(&self) -> Result<String, EvalError>;
       fn read_file(&self, path: &str) -> Result<String, EvalError>;
       fn write_file(&self, path: &str, content: &str) -> Result<(), EvalError>;
   }
   ```

2. **Web Backend Implementation**:
   ```rust
   struct WebIoBackend {
       output_callback: js_sys::Function,
       input_buffer: Rc<RefCell<Option<String>>>,
   }
   ```

3. **wasm-bindgen Exports**:
   ```rust
   #[wasm_bindgen]
   pub fn eval_expression(source: &str) -> Result<JsValue, JsValue> {
       let module = parse_module(source, "web")?;
       let mut evaluator = Evaluator::new_with_backend(web_backend);
       let result = evaluator.eval(&module)?;
       Ok(result.to_js_value())
   }
   ```

## Implementation Options

### Option 1: Minimal Web REPL (Recommended Start)

**Scope**: Parse expressions, evaluate, display results. No file I/O.

**Technologies**:
- `wasm-pack` for building WASM module
- `wasm-bindgen` for JS interop
- Simple HTML/CSS/JS frontend
- Monaco Editor or CodeMirror for syntax highlighting

**Effort**: 1-2 weeks

**Example Architecture**:
```
┌─────────────────────────────────────────┐
│         HTML/JavaScript Frontend         │
│  ┌─────────────────┐  ┌──────────────┐  │
│  │  Code Editor    │  │   Output     │  │
│  │  (Monaco/CM)    │  │   Panel      │  │
│  └────────┬────────┘  └──────▲───────┘  │
│           │                  │           │
│           ▼                  │           │
│  ┌─────────────────────────────────┐    │
│  │     WASM Module (goth-web)      │    │
│  │  ├─ parse_expr(source)          │    │
│  │  ├─ eval_expr(ast)              │    │
│  │  └─ check_type(ast)             │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

### Option 2: Terminal Emulator in Browser

**Scope**: Full REPL experience with xterm.js terminal emulation.

**Technologies**:
- [xterm.js](https://xtermjs.org/) for terminal emulation
- WASM module for evaluation
- ANSI escape code support (TUI would work!)

**Effort**: 2-3 weeks

**Advantages**:
- TUI/canvas graphics work unchanged
- Familiar terminal experience
- History, readline-like editing

**Example**:
```javascript
import { Terminal } from 'xterm';
import { WebLinksAddon } from 'xterm-addon-web-links';
import init, { create_repl } from './goth_web.js';

const term = new Terminal();
term.open(document.getElementById('terminal'));

await init();
const repl = create_repl();

term.onData(data => {
    const output = repl.input(data);
    term.write(output);
});
```

### Option 3: Rich Web IDE

**Scope**: Full-featured IDE with live type checking, error highlighting.

**Technologies**:
- Monaco Editor with custom language server
- React/Vue/Svelte frontend
- Web Workers for background type checking
- IndexedDB for file persistence

**Effort**: 4-8 weeks

**Features**:
- Live syntax highlighting
- Inline type annotations
- Error squiggles
- Auto-completion
- Multiple file support (virtual filesystem)

## TUI vs Web: Trade-offs

| Aspect | Terminal TUI | Web Interface |
|--------|--------------|---------------|
| **Setup** | `cargo install goth` | Visit URL |
| **Graphics** | ANSI escape codes | Canvas/SVG/WebGL |
| **Interactivity** | Keyboard-driven | Mouse + Keyboard |
| **File Access** | Full filesystem | Virtual/sandboxed |
| **Performance** | Native speed | ~0.8-0.95x native |
| **Offline** | Yes | With Service Worker |
| **Sharing** | Share files | Share URL + code |
| **Deployment** | Install required | Zero install |

## Recommended Approach

### Phase 1: Basic Web Playground (Week 1-2)
1. Create `goth-web` crate
2. Implement `WebIoBackend` with print → DOM
3. Compile to WASM with `wasm-pack`
4. Build simple HTML page with textarea + output div
5. Add Monaco Editor for syntax highlighting

### Phase 2: Terminal Emulation (Week 3-4)
1. Integrate xterm.js
2. Support ANSI escape codes (TUI works!)
3. Add readline-like input handling
4. Implement command history

### Phase 3: Rich Features (Week 5+)
1. Live type checking with web workers
2. Error recovery and suggestions
3. Documentation/examples panel
4. Shareable URLs (encode code in hash)

## Technical Implementation Guide

### Creating goth-web Crate

```toml
# crates/goth-web/Cargo.toml
[package]
name = "goth-web"
version = "0.1.0"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
goth-ast = { path = "../goth-ast" }
goth-parse = { path = "../goth-parse" }
goth-eval = { path = "../goth-eval" }
goth-check = { path = "../goth-check" }
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console"] }

[profile.release]
lto = true
opt-level = "s"  # Optimize for size
```

### Basic wasm-bindgen Interface

```rust
// crates/goth-web/src/lib.rs
use wasm_bindgen::prelude::*;
use goth_parse::parse_module;
use goth_eval::Evaluator;

#[wasm_bindgen]
pub struct GothInterpreter {
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl GothInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        GothInterpreter {
            evaluator: Evaluator::new(),
        }
    }

    #[wasm_bindgen]
    pub fn eval(&mut self, source: &str) -> Result<String, JsValue> {
        let module = parse_module(source, "web")
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        let result = self.evaluator.eval_module(&module)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok(format!("{}", result))
    }
}
```

### JavaScript Integration

```javascript
import init, { GothInterpreter } from './pkg/goth_web.js';

async function main() {
    await init();
    const goth = new GothInterpreter();

    const result = goth.eval('2 + 2 × 3');
    console.log(result);  // "8"

    const factorial = goth.eval(`
        ╭─ fact : I → I
        ╰─ if ₀ ≤ 1 then 1 else ₀ × fact (₀ - 1)

        fact 10
    `);
    console.log(factorial);  // "3628800"
}

main();
```

## Conclusion

A web interface for Goth is highly feasible due to the clean architecture and minimal platform dependencies. The recommended path is:

1. Start with a minimal web playground
2. Add xterm.js for full TUI support
3. Expand to a rich IDE as needed

The core work is creating an I/O abstraction layer (~200 lines) and the wasm-bindgen bindings (~100 lines). The frontend can range from a simple HTML page to a full React application depending on desired features.

## Resources

- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [WebAssembly with Rust (MDN)](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)
- [xterm.js Terminal](https://xtermjs.org/)
- [Monaco Editor](https://microsoft.github.io/monaco-editor/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
