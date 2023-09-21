#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_range_loop)]
#![allow(incomplete_features)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(core_intrinsics)]
#![feature(trait_upcasting)]

extern crate alloc;
extern crate clap;

pub mod backend;
pub mod frontend;
pub mod utils;

pub mod prelude {
    pub use plonky2::field::extension::Extendable;
    pub use plonky2::field::goldilocks_field::GoldilocksField;
    pub use plonky2::field::types::Field;
    pub use plonky2::hash::hash_types::RichField;
    pub use plonky2::iop::target::Target;
    pub use plonky2::iop::witness::{PartialWitness, Witness, WitnessWrite};
    pub use plonky2::plonk::config::PoseidonGoldilocksConfig;
    pub use plonky2x_derive::CircuitVariable;

    pub use crate::backend::circuit::config::{DefaultParameters, PlonkParameters};
    pub use crate::backend::circuit::{GateRegistry, HintRegistry};
    pub use crate::frontend::builder::{CircuitBuilder, DefaultBuilder};
    pub use crate::frontend::ops::*;
    pub use crate::frontend::vars::{
        ArrayVariable, BoolVariable, ByteVariable, Bytes32Variable, BytesVariable, CircuitVariable,
        Variable,
    };
    pub use crate::utils::{address, bytes, bytes32, hex};
}
