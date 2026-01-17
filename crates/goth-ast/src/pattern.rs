//! Pattern matching in Goth

use serde::{Deserialize, Serialize};
use crate::literal::Literal;
use crate::types::Type;

/// A pattern for matching
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard pattern: _
    Wildcard,

    /// Variable binding (introduces a name, but we track index for de Bruijn)
    /// In concrete syntax this might be written as a name,
    /// but internally it just adds to the environment
    Var(Option<Box<str>>),

    /// Literal pattern
    Lit(Literal),

    /// Array pattern: [p₀, p₁, ...]
    Array(Vec<Pattern>),

    /// Array with head/tail split: [h | t]
    ArraySplit {
        head: Vec<Pattern>,
        tail: Box<Pattern>,
    },

    /// Tuple pattern: ⟨p₀, p₁, ...⟩
    Tuple(Vec<Pattern>),

    /// Variant pattern: Constructor or Constructor payload
    Variant {
        constructor: Box<str>,
        payload: Option<Box<Pattern>>,
    },

    /// Typed pattern: p : T
    Typed(Box<Pattern>, Type),

    /// Or pattern: p₁ | p₂ (matches if either matches)
    Or(Box<Pattern>, Box<Pattern>),

    /// Guard pattern: p if condition
    Guard(Box<Pattern>, Box<crate::expr::Expr>),
}

impl Pattern {
    /// Create a wildcard pattern
    pub fn wildcard() -> Self {
        Pattern::Wildcard
    }

    /// Create a variable binding pattern with a name hint
    pub fn var(name: impl Into<Box<str>>) -> Self {
        Pattern::Var(Some(name.into()))
    }

    /// Create an anonymous variable binding pattern
    pub fn anon() -> Self {
        Pattern::Var(None)
    }

    /// Create a literal pattern
    pub fn lit(l: impl Into<Literal>) -> Self {
        Pattern::Lit(l.into())
    }

    /// Create a tuple pattern
    pub fn tuple(pats: Vec<Pattern>) -> Self {
        Pattern::Tuple(pats)
    }

    /// Create an array pattern
    pub fn array(pats: Vec<Pattern>) -> Self {
        Pattern::Array(pats)
    }

    /// Create a variant/constructor pattern
    pub fn variant(name: impl Into<Box<str>>, payload: Option<Pattern>) -> Self {
        Pattern::Variant {
            constructor: name.into(),
            payload: payload.map(Box::new),
        }
    }

    /// Add a type annotation
    pub fn typed(self, ty: Type) -> Self {
        Pattern::Typed(Box::new(self), ty)
    }

    /// Count the number of bindings this pattern introduces
    pub fn binding_count(&self) -> usize {
        match self {
            Pattern::Wildcard => 0,
            Pattern::Var(_) => 1,
            Pattern::Lit(_) => 0,
            Pattern::Array(pats) => pats.iter().map(|p| p.binding_count()).sum(),
            Pattern::ArraySplit { head, tail } => {
                head.iter().map(|p| p.binding_count()).sum::<usize>() + tail.binding_count()
            }
            Pattern::Tuple(pats) => pats.iter().map(|p| p.binding_count()).sum(),
            Pattern::Variant { payload, .. } => {
                payload.as_ref().map_or(0, |p| p.binding_count())
            }
            Pattern::Typed(p, _) => p.binding_count(),
            Pattern::Or(p1, p2) => {
                // Both branches must bind the same number
                p1.binding_count().max(p2.binding_count())
            }
            Pattern::Guard(p, _) => p.binding_count(),
        }
    }

    /// Check if this pattern is irrefutable (always matches)
    pub fn is_irrefutable(&self) -> bool {
        match self {
            Pattern::Wildcard => true,
            Pattern::Var(_) => true,
            Pattern::Lit(_) => false,
            Pattern::Array(_) => false, // Depends on length
            Pattern::ArraySplit { .. } => false,
            Pattern::Tuple(pats) => pats.iter().all(|p| p.is_irrefutable()),
            Pattern::Variant { .. } => false,
            Pattern::Typed(p, _) => p.is_irrefutable(),
            Pattern::Or(p1, p2) => p1.is_irrefutable() || p2.is_irrefutable(),
            Pattern::Guard(_, _) => false,
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Var(Some(name)) => write!(f, "{}", name),
            Pattern::Var(None) => write!(f, "_"),
            Pattern::Lit(lit) => write!(f, "{:?}", lit),
            Pattern::Array(pats) => {
                write!(f, "[")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
            Pattern::ArraySplit { head, tail } => {
                write!(f, "[")?;
                for (i, p) in head.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, " | {}]", tail)
            }
            Pattern::Tuple(pats) => {
                write!(f, "⟨")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, "⟩")
            }
            Pattern::Variant { constructor, payload } => {
                write!(f, "{}", constructor)?;
                if let Some(p) = payload {
                    write!(f, " {}", p)?;
                }
                Ok(())
            }
            Pattern::Typed(p, ty) => write!(f, "{} : {}", p, ty),
            Pattern::Or(p1, p2) => write!(f, "{} | {}", p1, p2),
            Pattern::Guard(p, _) => write!(f, "{} if ...", p),
        }
    }
}
