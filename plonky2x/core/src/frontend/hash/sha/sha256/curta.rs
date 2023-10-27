use core::marker::PhantomData;

use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::AirParameters;
use curta::machine::hash::sha::sha256::SHA256;
use digest::typenum::U3;
use serde::{Deserialize, Serialize};

use crate::frontend::hash::sha::curta::accelerator::SHAAccelerator;
use crate::frontend::hash::sha::curta::data::SHAInputData;
use crate::frontend::hash::sha::curta::SHA;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

pub type SHA256Accelerator = SHAAccelerator<U32Variable>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SHA256AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for SHA256AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 605;
    const EXTENDED_COLUMNS: usize = 351;
}

impl<L: PlonkParameters<D>, const D: usize> SHA<L, D, 64> for SHA256 {
    type IntVariable = U32Variable;

    type AirParameters = SHA256AirParameters<L, D>;
    type AirInstruction = UintInstruction;

    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_message_sha256(input);

        padded_bytes
            .chunks_exact(4)
            .map(|bytes| U32Variable::decode(builder, bytes))
            .collect()
    }

    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        length: Variable,
    ) -> Vec<Self::IntVariable> {
        todo!()
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as curta::chip::register::Register>::Value<Variable>,
    ) -> Self::IntVariable {
        let mut acc = builder.zero::<Variable>();
        for (i, byte) in value.into_iter().enumerate() {
            let two_i = builder.constant::<Variable>(L::Field::from_canonical_u32(1 << (8 * i)));
            let two_i_byte = builder.mul(two_i, byte);
            acc = builder.add(acc, two_i_byte);
        }
        U32Variable::from_variables_unsafe(&[acc])
    }
}
