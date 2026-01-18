//! MIR lowering errors

use thiserror::Error;

pub type MirResult<T> = Result<T, MirError>;

#[derive(Error, Debug, Clone)]
pub enum MirError {
    #[error("Unbound variable: ₍{0}₎")]
    UnboundVariable(u32),
    
    #[error("Undefined name: {0}")]
    UndefinedName(String),
    
    #[error("Cannot lower expression: {0}")]
    CannotLower(String),
    
    #[error("Closure conversion failed: {0}")]
    ClosureError(String),
    
    #[error("Pattern compilation failed: {0}")]
    PatternError(String),
    
    #[error("Type error during lowering: {0}")]
    TypeError(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
