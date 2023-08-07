#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_range_loop)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

extern crate alloc;

pub mod hash;
pub mod ecc;
pub mod num;
pub mod builder;