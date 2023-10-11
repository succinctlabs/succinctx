//! Overloadable operations in a circuit.
//!
//! This module contains traits for operations that can be overloaded as methods on
//! `CircuitBuilder`. These operations can be thought of as analogous to the overloadable
//! operators in the core Rust language, such as `Add` and `BitAnd`. For example, an implementation
//! of the `Add` trait for a type `Foo` allows the `builder.add(lhs, rhs)` method to be used on
//! `CircuitBuilder` instances, where `lhs` and `rhs` are instances of `Foo`.

pub mod bitwise;
pub mod index;
pub mod math;

pub use bitwise::*;
pub use index::*;
pub use math::*;
