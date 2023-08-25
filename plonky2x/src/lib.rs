#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_range_loop)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(core_intrinsics)]
#![feature(async_fn_in_trait)]

extern crate alloc;

pub mod builder;
pub mod circuit;
pub mod ecc;
pub mod eth;
pub mod ethutils;
pub mod function;
pub mod hash;
// pub mod mapreduce;
pub mod num;
pub mod ops;

// pub mod prover;
pub mod uint;
pub mod utils;
pub mod vars;
pub mod wrapper;

pub mod prelude {
    pub use plonky2::field::goldilocks_field::GoldilocksField;
    pub use plonky2::iop::witness::PartialWitness;
    pub use plonky2::plonk::config::PoseidonGoldilocksConfig;

    pub use crate::builder::CircuitBuilder;
    pub use crate::ops::*;
    pub use crate::vars::{BoolVariable, ByteVariable, BytesVariable, CircuitVariable, Variable};
}
