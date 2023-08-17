// use core::marker::PhantomData;

// use plonky2::field::extension::Extendable;
// use plonky2::field::goldilocks_field::GoldilocksField;
// use plonky2::hash::hash_types::RichField;
// use plonky2::plonk::circuit_data::VerifierCircuitTarget;
// use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
// use plonky2::plonk::proof::ProofWithPublicInputs;
// use plonky2::util::serialization::{DefaultGateSerializer, DefaultGeneratorSerializer};

// use crate::builder::CircuitBuilder;
// use crate::vars::{CircuitVariable, VerifierCircuitVariable};

// pub struct MapGenerator<
//     F: RichField + Extendable<D>,
//     C,
//     I: CircuitVariable,
//     O: CircuitVariable,
//     const D: usize,
// > where
//     C: GenericConfig<D, F = F>,
//     <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
// {
//     pub input: I,
//     pub proof: ProofWithPublicInputs<F, C, D>,
//     pub output: O,
// }

// impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
//     pub fn map<I: CircuitVariable, O: CircuitVariable, C, M>(&mut self, inputs: Vec<I>, m: M)
//     where
//         C: GenericConfig<D, F = F> + 'static,
//         <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
//         M: Fn(I, &mut CircuitBuilder<GoldilocksField, 2>) -> O,
//     {
//         // Note that we enforce that the circuit is built with the PoseidonGoldilocksConfig.
//         let data = {
//             let mut builder = CircuitBuilder::<GoldilocksField, 2>::new();
//             let input = builder.init::<I>();
//             let output = m(input, &mut builder);
//             builder
//                 .api
//                 .register_public_inputs(output.targets().as_slice());
//             builder.build::<PoseidonGoldilocksConfig>()
//         };

//         // We load the compiled inner circuit's verifier data as a constant in the outer circuit.
//         let vd = self.constant::<VerifierCircuitVariable>(data.verifier_only);

//         // let gate_serializer = DefaultGateSerializer;
//         // let generator_serializer = DefaultGeneratorSerializer {
//         //     _phantom: PhantomData::<PoseidonGoldilocksConfig>,
//         // };
//         // let bytes = data
//         //     .to_bytes(&gate_serializer, &generator_serializer)
//         //     .unwrap();

//         let mut proofs = Vec::new();
//         for _ in 0..inputs.len() {
//             let proof = self.api.add_virtual_proof_with_pis(&data.common);
//             proofs.push(proof);
//         }

//         // generator to generate the proofs
//         // data = desrailzie(circuit_digest.bin)
//         // pf1 = data.prove(input)

//         // for i in 0..inputs.len() {
//         //     self.api.verify_proof::<C>(&proofs[i], &vd, &data.common);
//         // }

//         // let mut outputs = Vec::new();
//         // for i in 0..inputs.len() {
//         //     let output = O::from_targets(proofs[i].public_inputs.as_slice());
//         //     outputs.push(output)
//         // }
//     }
// }
