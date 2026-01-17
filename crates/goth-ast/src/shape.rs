//! Tensor shapes in Goth
//!
//! Shapes can be concrete (known dimensions) or symbolic (type variables).

use serde::{Deserialize, Serialize};

/// A single dimension in a shape
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dim {
    /// Concrete dimension (known at compile time)
    Const(u64),

    /// Symbolic dimension (type variable, resolved during type checking)
    Var(Box<str>),

    /// Binary operation on dimensions (for dependent shapes)
    BinOp(Box<Dim>, DimOp, Box<Dim>),
}

/// Operations on dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DimOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// A tensor shape (list of dimensions)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Shape(pub Vec<Dim>);

impl Shape {
    /// Scalar shape (rank 0)
    pub fn scalar() -> Self {
        Shape(vec![])
    }

    /// Vector shape [n]
    pub fn vector(n: Dim) -> Self {
        Shape(vec![n])
    }

    /// Matrix shape [m, n]
    pub fn matrix(m: Dim, n: Dim) -> Self {
        Shape(vec![m, n])
    }

    /// Create shape from concrete dimensions
    pub fn concrete(dims: &[u64]) -> Self {
        Shape(dims.iter().map(|&d| Dim::Const(d)).collect())
    }

    /// Create shape from symbolic variables
    pub fn symbolic(vars: &[&str]) -> Self {
        Shape(vars.iter().map(|&v| Dim::Var(v.into())).collect())
    }

    /// Rank (number of dimensions)
    pub fn rank(&self) -> usize {
        self.0.len()
    }

    /// Check if shape is fully concrete
    pub fn is_concrete(&self) -> bool {
        self.0.iter().all(|d| matches!(d, Dim::Const(_)))
    }

    /// Get total element count if shape is concrete
    pub fn elem_count(&self) -> Option<u64> {
        if self.is_concrete() {
            Some(self.0.iter().map(|d| match d {
                Dim::Const(n) => *n,
                _ => unreachable!(),
            }).product())
        } else {
            None
        }
    }
}

impl Dim {
    pub fn constant(n: u64) -> Self {
        Dim::Const(n)
    }

    pub fn var(name: impl Into<Box<str>>) -> Self {
        Dim::Var(name.into())
    }

    /// Check if this dimension is concrete
    pub fn is_concrete(&self) -> bool {
        matches!(self, Dim::Const(_))
    }
}

// Convenience: shape from slice of u64
impl From<&[u64]> for Shape {
    fn from(dims: &[u64]) -> Self {
        Shape::concrete(dims)
    }
}

// Convenience: shape from vec
impl From<Vec<Dim>> for Shape {
    fn from(dims: Vec<Dim>) -> Self {
        Shape(dims)
    }
}

impl std::fmt::Display for Dim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dim::Const(n) => write!(f, "{}", n),
            Dim::Var(name) => write!(f, "{}", name),
            Dim::BinOp(l, op, r) => {
                let op_str = match op {
                    DimOp::Add => "+",
                    DimOp::Sub => "-",
                    DimOp::Mul => "×",
                    DimOp::Div => "/",
                };
                write!(f, "({} {} {})", l, op_str, r)
            }
        }
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, dim) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", dim)?;
        }
        write!(f, "]")
    }
}
