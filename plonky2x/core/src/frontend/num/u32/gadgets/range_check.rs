use alloc::vec;
use alloc::vec::Vec;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;

use super::arithmetic_u32::U32Target;
use crate::frontend::num::u32::gates::range_check_u32::U32RangeCheckGate;

pub fn range_check_u32_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    vals: Vec<U32Target>,
) {
    // Chunk the input u32's into 7-limb chunks, and add a range check gate for each chunk.
    vals.chunks(7).for_each(|chunk| {
        let num_input_limbs = chunk.len();
        let gate = U32RangeCheckGate::<F, D>::new(num_input_limbs);
        let row = builder.add_gate(gate, vec![]);

        for i in 0..num_input_limbs {
            builder.connect(
                Target::wire(row, gate.wire_ith_input_limb(i)),
                chunk[i].target,
            );
        }
    })
}
