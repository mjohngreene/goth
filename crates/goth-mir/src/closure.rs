//! Closure conversion
//!
//! Transforms lambda expressions into:
//! 1. Top-level functions (lambda lifting)
//! 2. Closure creation with environment capture
//!
//! ## Example
//!
//! **Before:**
//! ```goth
//! let x = 10 in
//! let f = λ→ ₀ + x in  # Captures x
//! f 5
//! ```
//!
//! **After (MIR):**
//! ```text
//! fn lambda_0(env: {I64}, arg: I64) -> I64 {
//!   _0: I64 = TupleField(env, 0)    # Extract x
//!   _1: I64 = BinOp(Add, arg, _0)
//!   Return(_1)
//! }
//!
//! fn main() -> I64 {
//!   _0: I64 = Const(10)              # x
//!   _1: Closure = MakeClosure(lambda_0, [_0])  # Capture x
//!   _2: I64 = ClosureCall(_1, [Const(5)])
//!   Return(_2)
//! }
//! ```

use crate::mir::*;
use crate::error::{MirError, MirResult};
use goth_ast::expr::Expr;
use std::collections::HashSet;

/// Free variable analysis
pub fn free_variables(expr: &Expr) -> HashSet<u32> {
    let mut free = HashSet::new();
    free_vars_impl(expr, 0, &mut free);
    free
}

fn free_vars_impl(expr: &Expr, depth: u32, free: &mut HashSet<u32>) {
    match expr {
        Expr::Idx(idx) => {
            // If de Bruijn index points outside current depth, it's free
            if *idx >= depth {
                free.insert(*idx - depth);
            }
        }
        
        Expr::Lam(body) => {
            // Increase depth under lambda
            free_vars_impl(body, depth + 1, free);
        }
        
        Expr::App(func, arg) => {
            free_vars_impl(func, depth, free);
            free_vars_impl(arg, depth, free);
        }
        
        Expr::Let { value, body, .. } => {
            free_vars_impl(value, depth, free);
            free_vars_impl(body, depth + 1, free);
        }
        
        Expr::BinOp(_, left, right) => {
            free_vars_impl(left, depth, free);
            free_vars_impl(right, depth, free);
        }
        
        Expr::UnaryOp(_, operand) => {
            free_vars_impl(operand, depth, free);
        }
        
        Expr::If { cond, then_, else_ } => {
            free_vars_impl(cond, depth, free);
            free_vars_impl(then_, depth, free);
            free_vars_impl(else_, depth, free);
        }
        
        Expr::Tuple(exprs) | Expr::Array(exprs) => {
            for expr in exprs {
                free_vars_impl(expr, depth, free);
            }
        }
        
        // Literals, names, etc. have no free variables
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use goth_ast::literal::Literal;
    
    #[test]
    fn test_free_vars_closed() {
        // λ→ ₀ has no free variables
        let expr = Expr::Lam(Box::new(Expr::Idx(0)));
        let free = free_variables(&expr);
        assert!(free.is_empty());
    }
    
    #[test]
    fn test_free_vars_open() {
        // λ→ ₁ has one free variable (₁ refers outside the lambda)
        let expr = Expr::Lam(Box::new(Expr::Idx(1)));
        let free = free_variables(&expr);
        assert_eq!(free.len(), 1);
        assert!(free.contains(&0));
    }
    
    #[test]
    fn test_free_vars_nested() {
        // λ→ λ→ ₂ has one free variable
        let expr = Expr::Lam(Box::new(
            Expr::Lam(Box::new(Expr::Idx(2)))
        ));
        let free = free_variables(&expr);
        assert_eq!(free.len(), 1);
        assert!(free.contains(&0));
    }
}
