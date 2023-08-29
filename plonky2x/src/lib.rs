#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_range_loop)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(core_intrinsics)]
#![feature(async_fn_in_trait)]

extern crate alloc;
extern crate clap;

pub mod backend;
pub mod frontend;
pub mod utils;

pub mod prelude {
    pub use plonky2::field::goldilocks_field::GoldilocksField;
    pub use plonky2::iop::witness::PartialWitness;
    pub use plonky2::plonk::config::PoseidonGoldilocksConfig;

    pub use crate::frontend::builder::{CircuitBuilder, CircuitBuilderX};
    pub use crate::frontend::hash::*;
    pub use crate::frontend::ops::*;
    pub use crate::frontend::vars::{
        BoolVariable, ByteVariable, BytesVariable, CircuitVariable, Variable,
    };
    pub use crate::utils::{address, bytes, bytes32, hex};
}
