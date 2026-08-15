//! An interactive, multi-tenant school management system.
//!
//! Everything of substance lives here rather than in `main.rs`, so that the
//! tests — and anything else that ever wants to drive this — can import it.

pub mod domain;