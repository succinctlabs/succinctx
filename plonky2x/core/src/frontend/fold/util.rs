use plonky2::gates::noop::NoopGate;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use crate::prelude::{CircuitBuilder, PlonkParameters};

pub fn common_data_for_recursion<L: PlonkParameters<D>, const D: usize>(
) -> CommonCircuitData<L::Field, D>
where
    <<L as crate::prelude::PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<L::Field>,
{
    let builder = CircuitBuilder::<L, D>::new();
    let data = builder.build();
    let mut builder = CircuitBuilder::<L, D>::new();
    let proof = builder.add_virtual_proof_with_pis(&data.data.common);
    let verifier_data = builder
        .api
        .add_virtual_verifier_data(data.data.common.config.fri_config.cap_height);
    builder.verify_proof::<L>(&proof, &verifier_data, &data.data.common);
    let data = builder.build();

    let mut builder = CircuitBuilder::<L, D>::new();
    let proof = builder.add_virtual_proof_with_pis(&data.data.common);
    let verifier_data = builder
        .api
        .add_virtual_verifier_data(data.data.common.config.fri_config.cap_height);
    builder.verify_proof::<L>(&proof, &verifier_data, &data.data.common);
    while builder.api.num_gates() < 1 << 12 {
        builder.api.add_gate(NoopGate, vec![]);
    }
    builder.build().data.common
}
