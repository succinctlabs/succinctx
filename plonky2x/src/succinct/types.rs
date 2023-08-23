use std::fmt;
use std::fmt::{Debug};
use crate::vars::{CircuitVariable};
use crate::builder::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::{CommonCircuitData, CircuitData};
use plonky2::util::serialization::{Buffer, IoResult};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;


mod cli {
    pub use crate::succinct::circuit::CircuitTrait;

    pub fn Run<C: CircuitTrait>(args: Vec<String>) {
        // TODO: parse args
        // TODO: instantiate a builder
        C::compile(builder);
        // TODO: check compilation is valid, i.e. the builder is properly constructed

        // if args.prove: prove(builder)

    }

    pub fn check_build(builder: CircuitBuilder<Goldilocks, 2>) {
        // TODO: check the build is valid, i.e. the builder is properly constructed
    }

    pub fn build_and_save(builder: CircuitBuilder<Goldilocks, 2>) {
        // TODO: build the circuit & save it to disk
    }

    pub fn load_and_prove(path: str, input_bytes: &[u8]) {
    }
}

mod circuit {
    pub trait CircuitTrait {
        fn compile(builder : &mut CircuitBuilder<Goldilocks, 2>) -> Self;
    }


}

mod user {
    use crate::succinct::circuit::CircuitTrait;

    pub struct MyCircuit {
    }

    impl CircuitTrait for MyCircuit {
        fn compile(builder : &mut CircuitBuilder<Goldilocks, 2>) {
            let a = builder.read_bytes(32);
            // let b = builder.read::<Uint256Variable>();
            // let c = builder.add::<Uint256Variable>(a, b);
            builder.write_bytes(&a);
        }
    }
}

// This is auto-generated from succinct.json
mod user_main {
    use crate::succinct::types::cli::Run;
    use crate::succinct::types::user::MyCircuit;

    fn main() {
        let args: Vec<String> = env::args().collect();
        Run::<MyCircuit>(args);
    }

}   