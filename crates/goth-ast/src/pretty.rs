//! Pretty printing for Goth AST
//!
//! Renders AST to Unicode text format (.goth)

use crate::decl::{FnDecl, Module, Decl, TypeDecl, ClassDecl, ImplDecl, LetDecl};
use crate::expr::Expr;
use crate::types::Type;

/// Pretty print configuration
#[derive(Debug, Clone)]
pub struct PrettyConfig {
    /// Use Unicode operators (true) or ASCII fallbacks (false)
    pub unicode: bool,
    /// Indentation string
    pub indent: String,
    /// Maximum line width before wrapping
    pub max_width: usize,
}

impl Default for PrettyConfig {
    fn default() -> Self {
        PrettyConfig {
            unicode: true,
            indent: "  ".to_string(),
            max_width: 100,
        }
    }
}

impl PrettyConfig {
    pub fn ascii() -> Self {
        PrettyConfig {
            unicode: false,
            ..Default::default()
        }
    }

    pub fn compact() -> Self {
        PrettyConfig {
            max_width: usize::MAX,
            ..Default::default()
        }
    }
}

/// Pretty printer
pub struct Pretty {
    config: PrettyConfig,
    output: String,
    current_indent: usize,
}

impl Pretty {
    pub fn new(config: PrettyConfig) -> Self {
        Pretty {
            config,
            output: String::new(),
            current_indent: 0,
        }
    }

    pub fn default() -> Self {
        Pretty::new(PrettyConfig::default())
    }

    /// Pretty print a module
    pub fn print_module(&mut self, module: &Module) -> &str {
        if let Some(name) = &module.name {
            self.write("module ");
            self.write(name);
            self.newline();
            self.newline();
        }

        for (i, decl) in module.decls.iter().enumerate() {
            if i > 0 { self.newline(); }
            self.print_decl(decl);
        }

        &self.output
    }

    /// Pretty print a declaration
    pub fn print_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Fn(f) => self.print_fn(f),
            Decl::Type(t) => self.print_type_decl(t),
            Decl::Class(c) => self.print_class(c),
            Decl::Impl(i) => self.print_impl(i),
            Decl::Let(l) => self.print_let_decl(l),
            Decl::Op(_) => todo!("op decl pretty printing"),
        }
    }

    /// Pretty print a function declaration
    pub fn print_fn(&mut self, f: &FnDecl) {
        // Header line: ╭─ name : sig
        self.write(if self.config.unicode { "╭─ " } else { "/- " });
        self.write(&f.name);
        self.write(" : ");
        self.print_type(&f.signature);
        self.newline();

        // Constraints
        for c in &f.constraints {
            self.write(if self.config.unicode { "│  where " } else { "|  where " });
            self.write(&format!("{:?}", c)); // TODO: proper constraint printing
            self.newline();
        }

        // Preconditions
        for pre in &f.preconditions {
            self.write(if self.config.unicode { "│  ⊢ " } else { "|  |- " });
            self.print_expr(pre);
            self.newline();
        }

        // Postconditions
        for post in &f.postconditions {
            self.write(if self.config.unicode { "│  ⊨ " } else { "|  |= " });
            self.print_expr(post);
            self.newline();
        }

        // Body line: ╰─ expr
        self.write(if self.config.unicode { "╰─ " } else { "\\- " });
        self.print_expr(&f.body);
        self.newline();
    }

    fn print_type_decl(&mut self, t: &TypeDecl) {
        self.write(&t.name);
        self.write(if self.config.unicode { " ≡ " } else { " == " });
        self.print_type(&t.definition);
        self.newline();
    }

    fn print_class(&mut self, c: &ClassDecl) {
        self.write("class ");
        self.write(&c.name);
        self.write(" ");
        self.write(&c.param.name);
        if !c.superclasses.is_empty() {
            self.write(" extends ");
            for (i, sc) in c.superclasses.iter().enumerate() {
                if i > 0 { self.write(", "); }
                self.write(sc);
            }
        }
        self.write(" where");
        self.newline();
        self.indent();
        for m in &c.methods {
            self.write_indent();
            self.write(&m.name);
            self.write(" : ");
            self.print_type(&m.signature);
            self.newline();
        }
        self.dedent();
    }

    fn print_impl(&mut self, i: &ImplDecl) {
        self.write("impl ");
        self.write(&i.class_name);
        self.write(" ");
        self.print_type(&i.target);
        self.write(" where");
        self.newline();
        self.indent();
        for m in &i.methods {
            self.write_indent();
            self.write(&m.name);
            self.write(if self.config.unicode { " ← " } else { " <- " });
            self.print_expr(&m.body);
            self.newline();
        }
        self.dedent();
    }

    fn print_let_decl(&mut self, l: &LetDecl) {
        self.write("let ");
        self.write(&l.name);
        if let Some(ty) = &l.type_ {
            self.write(" : ");
            self.print_type(ty);
        }
        self.write(if self.config.unicode { " ← " } else { " <- " });
        self.print_expr(&l.value);
        self.newline();
    }

    /// Pretty print a type
    pub fn print_type(&mut self, ty: &Type) {
        self.write(&format!("{}", ty));
    }

    /// Pretty print an expression
    pub fn print_expr(&mut self, expr: &Expr) {
        self.write(&format!("{}", expr));
    }

    // ============ Helpers ============

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn newline(&mut self) {
        self.output.push('\n');
    }

    fn indent(&mut self) {
        self.current_indent += 1;
    }

    fn dedent(&mut self) {
        self.current_indent = self.current_indent.saturating_sub(1);
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent {
            self.output.push_str(&self.config.indent);
        }
    }

    /// Get the output string
    pub fn finish(self) -> String {
        self.output
    }
}

// ============ Convenience Functions ============

/// Pretty print a module with default config
pub fn print_module(module: &Module) -> String {
    let mut p = Pretty::default();
    p.print_module(module);
    p.finish()
}

/// Pretty print a declaration with default config
pub fn print_decl(decl: &Decl) -> String {
    let mut p = Pretty::default();
    p.print_decl(decl);
    p.finish()
}

/// Pretty print a function with default config
pub fn print_fn(f: &FnDecl) -> String {
    let mut p = Pretty::default();
    p.print_fn(f);
    p.finish()
}

/// Pretty print an expression with default config
pub fn print_expr(expr: &Expr) -> String {
    format!("{}", expr)
}

/// Pretty print a type with default config
pub fn print_type(ty: &Type) -> String {
    format!("{}", ty)
}
