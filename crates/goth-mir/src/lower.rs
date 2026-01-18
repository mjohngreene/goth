//! AST → MIR lowering
//!
//! This module transforms typed AST expressions into MIR.
//! Key transformations:
//! - De Bruijn indices → explicit locals
//! - Nested let bindings → sequential statements
//! - Lambda expressions → closure creation (handled by closure.rs)

use crate::mir::*;
use crate::error::{MirError, MirResult};
use goth_ast::expr::Expr;
use goth_ast::literal::Literal;
use goth_ast::decl::{Module, Decl};
use goth_ast::types::Type;

/// Lowering context
pub struct LoweringContext {
    /// Stack of locals: index 0 = most recent binding (de Bruijn 0)
    locals: Vec<LocalId>,
    /// Counter for fresh locals
    next_local: u32,
    /// Counter for fresh functions (lifted lambdas)
    next_fn: u32,
    /// Accumulated statements
    stmts: Vec<Stmt>,
    /// Generated functions (from lambda lifting)
    functions: Vec<Function>,
    /// Global function names and their types
    globals: std::collections::HashMap<String, Type>,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            locals: Vec::new(),
            next_local: 0,
            next_fn: 0,
            stmts: Vec::new(),
            functions: Vec::new(),
            globals: std::collections::HashMap::new(),
        }
    }
    
    /// Generate a fresh local variable
    fn fresh_local(&mut self) -> LocalId {
        let id = LocalId::new(self.next_local);
        self.next_local += 1;
        id
    }
    
    /// Generate a fresh function name
    fn fresh_fn_name(&mut self) -> String {
        let name = format!("lambda_{}", self.next_fn);
        self.next_fn += 1;
        name
    }
    
    /// Push a local onto the stack (for de Bruijn index resolution)
    fn push_local(&mut self, local: LocalId) {
        self.locals.push(local);
    }
    
    /// Pop a local from the stack
    fn pop_local(&mut self) {
        self.locals.pop();
    }
    
    /// Look up a de Bruijn index
    fn lookup_index(&self, idx: u32) -> MirResult<LocalId> {
        let idx = idx as usize;
        if idx < self.locals.len() {
            // de Bruijn index 0 = most recent = end of stack
            Ok(self.locals[self.locals.len() - 1 - idx])
        } else {
            Err(MirError::UnboundVariable(idx as u32))
        }
    }
    
    /// Emit a statement
    fn emit(&mut self, dest: LocalId, ty: Type, rhs: Rhs) {
        self.stmts.push(Stmt { dest, ty, rhs });
    }
    
    /// Take all accumulated statements and reset
    fn take_stmts(&mut self) -> Vec<Stmt> {
        std::mem::take(&mut self.stmts)
    }
}

/// Lower an expression to MIR, returning the operand that holds the result
pub fn lower_expr_to_operand(ctx: &mut LoweringContext, expr: &Expr) -> MirResult<(Operand, Type)> {
    match expr {
        // ============ Literals ============
        
        Expr::Lit(lit) => {
            let (constant, ty) = lower_literal(lit);
            Ok((Operand::Const(constant), ty))
        }
        
        // ============ Variables ============
        
        Expr::Idx(idx) => {
            // De Bruijn index - look up in context
            let local = ctx.lookup_index(*idx)?;
            // TODO: Get type from somewhere - need type information!
            // For now, we'll need to thread types through
            Err(MirError::Internal("Need type information for variables".into()))
        }
        
        Expr::Name(name) => {
            // Global name - will be a function call or constant
            if let Some(ty) = ctx.globals.get(name.as_ref()) {
                // For now, just error - need to handle this properly
                Err(MirError::Internal(format!("Global name {} not yet supported", name)))
            } else {
                Err(MirError::UndefinedName(name.to_string()))
            }
        }
        
        // ============ Binary Operations ============
        
        Expr::BinOp(op, left, right) => {
            let (left_op, left_ty) = lower_expr_to_operand(ctx, left)?;
            let (right_op, right_ty) = lower_expr_to_operand(ctx, right)?;
            
            // Result type depends on operation
            // TODO: Proper type inference
            let result_ty = left_ty.clone();  // Simplified for now
            
            let dest = ctx.fresh_local();
            ctx.emit(dest, result_ty.clone(), Rhs::BinOp(op.clone(), left_op, right_op));
            
            Ok((Operand::Local(dest), result_ty))
        }
        
        // ============ Unary Operations ============
        
        Expr::UnaryOp(op, operand) => {
            let (op_val, op_ty) = lower_expr_to_operand(ctx, operand)?;
            
            // Result type depends on operation
            let result_ty = match op {
                goth_ast::op::UnaryOp::Floor | goth_ast::op::UnaryOp::Ceil => {
                    Type::Prim(goth_ast::types::PrimType::F64)
                }
                goth_ast::op::UnaryOp::Sqrt => Type::Prim(goth_ast::types::PrimType::F64),
                goth_ast::op::UnaryOp::Not => Type::Prim(goth_ast::types::PrimType::Bool),
                goth_ast::op::UnaryOp::Neg => op_ty.clone(),
                _ => op_ty.clone(),
            };
            
            let dest = ctx.fresh_local();
            ctx.emit(dest, result_ty.clone(), Rhs::UnaryOp(*op, op_val));
            
            Ok((Operand::Local(dest), result_ty))
        }
        
        // ============ Let Bindings ============
        
        Expr::Let { pattern, value, body } => {
            // Lower the value
            let (val_op, val_ty) = lower_expr_to_operand(ctx, value)?;
            
            // For now, only handle simple variable patterns
            // TODO: Pattern compilation for complex patterns
            let local = ctx.fresh_local();
            ctx.emit(local, val_ty, Rhs::Use(val_op));
            
            // Push onto stack for de Bruijn resolution
            ctx.push_local(local);
            
            // Lower the body
            let result = lower_expr_to_operand(ctx, body)?;
            
            // Pop the local
            ctx.pop_local();
            
            Ok(result)
        }
        
        // ============ Tuples ============
        
        Expr::Tuple(exprs) => {
            let mut ops = Vec::new();
            let mut field_tys = Vec::new();
            
            for expr in exprs {
                let (op, ty) = lower_expr_to_operand(ctx, expr)?;
                ops.push(op);
                field_tys.push(ty);
            }
            
            let tuple_ty = Type::tuple(field_tys);
            let dest = ctx.fresh_local();
            ctx.emit(dest, tuple_ty.clone(), Rhs::Tuple(ops));
            
            Ok((Operand::Local(dest), tuple_ty))
        }
        
        // ============ Arrays ============
        
        Expr::Array(exprs) => {
            let mut ops = Vec::new();
            let mut elem_ty = None;
            
            for expr in exprs {
                let (op, ty) = lower_expr_to_operand(ctx, expr)?;
                ops.push(op);
                
                if elem_ty.is_none() {
                    elem_ty = Some(ty);
                }
            }
            
            let elem_ty = elem_ty.unwrap_or(Type::Prim(goth_ast::types::PrimType::I64));
            let array_ty = Type::vector(
                goth_ast::shape::Dim::constant(exprs.len() as u64),
                elem_ty
            );
            
            let dest = ctx.fresh_local();
            ctx.emit(dest, array_ty.clone(), Rhs::Array(ops));
            
            Ok((Operand::Local(dest), array_ty))
        }
        
        // ============ TODO: More expressions ============
        
        _ => Err(MirError::CannotLower(format!("Expression type not yet implemented: {:?}", expr))),
    }
}

/// Lower a literal to a constant and its type
fn lower_literal(lit: &Literal) -> (Constant, Type) {
    match lit {
        Literal::Int(n) => {
            (Constant::Int(*n as i64), Type::Prim(goth_ast::types::PrimType::I64))
        }
        Literal::Float(x) => {
            (Constant::Float(*x), Type::Prim(goth_ast::types::PrimType::F64))
        }
        Literal::True => {
            (Constant::Bool(true), Type::Prim(goth_ast::types::PrimType::Bool))
        }
        Literal::False => {
            (Constant::Bool(false), Type::Prim(goth_ast::types::PrimType::Bool))
        }
        Literal::Unit => {
            (Constant::Unit, Type::Tuple(vec![]))
        }
        Literal::Char(_) => {
            // TODO: Handle char literals
            (Constant::Int(0), Type::Prim(goth_ast::types::PrimType::Char))
        }
        _ => {
            // TODO: Handle other literal types
            (Constant::Unit, Type::Tuple(vec![]))
        }
    }
}

/// Lower a top-level expression to a Program
pub fn lower_expr(expr: &Expr) -> MirResult<Program> {
    let mut ctx = LoweringContext::new();
    
    let (result_op, result_ty) = lower_expr_to_operand(&mut ctx, expr)?;
    
    // Create main function
    let stmts = ctx.take_stmts();
    let body = Block {
        stmts,
        term: Terminator::Return(result_op),
    };
    
    let main_fn = Function {
        name: "main".to_string(),
        params: vec![],
        ret_ty: result_ty,
        body,
        is_closure: false,
    };
    
    Ok(Program {
        functions: vec![main_fn],
        entry: "main".to_string(),
    })
}

/// Lower a module to a Program
pub fn lower_module(module: &Module) -> MirResult<Program> {
    let mut ctx = LoweringContext::new();
    
    // Process declarations
    for decl in &module.decls {
        match decl {
            Decl::Fn(fn_decl) => {
                // Register global function
                ctx.globals.insert(fn_decl.name.to_string(), fn_decl.signature.clone());
                
                // TODO: Lower function body
            }
            Decl::Let(let_decl) => {
                // Register global let binding
                // TODO: Infer or use annotated type
            }
            _ => {}
        }
    }
    
    // For now, just create an empty main
    let main_fn = Function {
        name: "main".to_string(),
        params: vec![],
        ret_ty: Type::Tuple(vec![]),
        body: Block::with_return(Operand::Const(Constant::Unit)),
        is_closure: false,
    };
    
    Ok(Program {
        functions: vec![main_fn],
        entry: "main".to_string(),
    })
}
