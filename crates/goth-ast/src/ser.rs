//! Serialization for Goth AST
//!
//! Three formats:
//! - `.goth` - Unicode text (via pretty printer)
//! - `.gast` - JSON AST (via serde_json)  
//! - `.gbin` - Binary AST (via bincode)

use crate::decl::Module;
use crate::expr::Expr;
use thiserror::Error;

/// Serialization error
#[derive(Error, Debug)]
pub enum SerError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Binary error: {0}")]
    Binary(#[from] bincode::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SerError>;

// ============ JSON (.gast) ============

/// Serialize module to JSON string
pub fn to_json(module: &Module) -> Result<String> {
    Ok(serde_json::to_string_pretty(module)?)
}

/// Serialize module to JSON bytes
pub fn to_json_bytes(module: &Module) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec_pretty(module)?)
}

/// Serialize module to compact JSON (no whitespace)
pub fn to_json_compact(module: &Module) -> Result<String> {
    Ok(serde_json::to_string(module)?)
}

/// Deserialize module from JSON string
pub fn from_json(json: &str) -> Result<Module> {
    Ok(serde_json::from_str(json)?)
}

/// Deserialize module from JSON bytes
pub fn from_json_bytes(bytes: &[u8]) -> Result<Module> {
    Ok(serde_json::from_slice(bytes)?)
}

// ============ Binary (.gbin) ============

/// Serialize module to binary
pub fn to_binary(module: &Module) -> Result<Vec<u8>> {
    Ok(bincode::serialize(module)?)
}

/// Deserialize module from binary
pub fn from_binary(bytes: &[u8]) -> Result<Module> {
    Ok(bincode::deserialize(bytes)?)
}

// ============ Expression-level ============

/// Serialize expression to JSON
pub fn expr_to_json(expr: &Expr) -> Result<String> {
    Ok(serde_json::to_string_pretty(expr)?)
}

/// Deserialize expression from JSON
pub fn expr_from_json(json: &str) -> Result<Expr> {
    Ok(serde_json::from_str(json)?)
}

/// Serialize expression to binary
pub fn expr_to_binary(expr: &Expr) -> Result<Vec<u8>> {
    Ok(bincode::serialize(expr)?)
}

/// Deserialize expression from binary
pub fn expr_from_binary(bytes: &[u8]) -> Result<Expr> {
    Ok(bincode::deserialize(bytes)?)
}

// ============ File I/O ============

/// Write module to file (format inferred from extension)
pub fn write_file(module: &Module, path: &std::path::Path) -> Result<()> {
    use std::io::Write;
    
    let bytes = match path.extension().and_then(|e| e.to_str()) {
        Some("gast") => to_json_bytes(module)?,
        Some("gbin") => to_binary(module)?,
        Some("goth") => crate::pretty::print_module(module).into_bytes(),
        _ => to_json_bytes(module)?, // default to JSON
    };
    
    let mut file = std::fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Read module from file (format inferred from extension)
pub fn read_file(path: &std::path::Path) -> Result<Module> {
    let bytes = std::fs::read(path)?;
    
    match path.extension().and_then(|e| e.to_str()) {
        Some("gast") => from_json_bytes(&bytes),
        Some("gbin") => from_binary(&bytes),
        Some("goth") => {
            // TODO: implement parser
            Err(SerError::Io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Text parsing not yet implemented"
            )))
        }
        _ => from_json_bytes(&bytes), // try JSON by default
    }
}

// ============ Size Estimation ============

/// Estimate binary size of a module
pub fn estimate_binary_size(module: &Module) -> usize {
    // Quick estimate based on serialization
    bincode::serialized_size(module).unwrap_or(0) as usize
}

/// Estimate JSON size of a module (compact)
pub fn estimate_json_size(module: &Module) -> usize {
    serde_json::to_string(module).map(|s| s.len()).unwrap_or(0)
}
