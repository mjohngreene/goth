//! Effect system for Goth
//!
//! Effects track capabilities like IO, mutation, and randomness.
//! Pure (□) is the default; effects are explicitly annotated.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// An effect
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Effect {
    /// Pure (no effects) - □
    Pure,

    /// I/O effect (file, network, console) - ◇io
    Io,

    /// Local mutation - ◇mut
    Mut,

    /// Randomness (requires RNG) - ◇rand
    Rand,

    /// Possible non-termination - ◇div
    Div,

    /// May throw exception of type - ◇exn⟨T⟩
    Exn(Box<str>),

    /// Foreign function call with lifetime - ◇ffi⟨'a⟩
    Ffi(Box<str>),

    /// User-defined effect
    Custom(Box<str>),
}

/// A set of effects (effect row)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effects(pub BTreeSet<Effect>);

impl Effects {
    /// Pure effect set (no effects)
    pub fn pure() -> Self {
        Effects(BTreeSet::new())
    }

    /// Single effect
    pub fn single(e: Effect) -> Self {
        let mut set = BTreeSet::new();
        if e != Effect::Pure {
            set.insert(e);
        }
        Effects(set)
    }

    /// Union of effect sets
    pub fn union(&self, other: &Effects) -> Effects {
        Effects(self.0.union(&other.0).cloned().collect())
    }

    /// Check if this effect set is pure
    pub fn is_pure(&self) -> bool {
        self.0.is_empty()
    }

    /// Check if this effect set contains a specific effect
    pub fn contains(&self, e: &Effect) -> bool {
        self.0.contains(e)
    }

    /// Check if this effect set is a subset of another
    pub fn is_subset(&self, other: &Effects) -> bool {
        self.0.is_subset(&other.0)
    }

    /// Add an effect
    pub fn with(mut self, e: Effect) -> Self {
        if e != Effect::Pure {
            self.0.insert(e);
        }
        self
    }
}

impl Default for Effects {
    fn default() -> Self {
        Effects::pure()
    }
}

impl From<Effect> for Effects {
    fn from(e: Effect) -> Self {
        Effects::single(e)
    }
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Pure => write!(f, "□"),
            Effect::Io => write!(f, "◇io"),
            Effect::Mut => write!(f, "◇mut"),
            Effect::Rand => write!(f, "◇rand"),
            Effect::Div => write!(f, "◇div"),
            Effect::Exn(ty) => write!(f, "◇exn⟨{}⟩", ty),
            Effect::Ffi(lifetime) => write!(f, "◇ffi⟨'{}⟩", lifetime),
            Effect::Custom(name) => write!(f, "◇{}", name),
        }
    }
}

impl std::fmt::Display for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_pure() {
            write!(f, "□")
        } else {
            let parts: Vec<_> = self.0.iter().map(|e| format!("{}", e)).collect();
            write!(f, "{}", parts.join(" ∪ "))
        }
    }
}
