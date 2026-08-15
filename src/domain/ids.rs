//! Typed identifiers.
//!
//! Every ID is a distinct type, so passing a `StudentId` where a `TeacherId`
//! is expected is a compile error rather than a runtime mystery.
//!
//! The counter-based IDs store a number, not a `String` — `TCH-001` is
//! rendered on demand. Hashing a `u32` beats hashing a heap string, and these
//! types are `HashMap` keys on nearly every lookup in the program.

use std::fmt;

/// Identifies a teacher. Unique across all schools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeacherId(u32);

impl TeacherId {
    pub const PREFIX: &'static str = "TCH";

    pub const fn from_number(n: u32) -> Self {
        Self(n)
    }

    pub const fn number(self) -> u32 {
        self.0
    }
}

impl fmt::Display for TeacherId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:03}", Self::PREFIX, self.0)
    }
}