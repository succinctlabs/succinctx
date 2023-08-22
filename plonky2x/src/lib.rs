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
pub mod composer;
pub mod eth;
pub mod ethutils;
pub mod mapreduce;
pub mod prover;
pub mod utils;
pub mod vars;
