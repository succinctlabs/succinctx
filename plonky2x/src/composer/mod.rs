use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::builder::CircuitBuilder;
use crate::vars::CircuitVariable;

pub struct CircuitComposer<F, C, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    _phantom1: PhantomData<F>,
    _phantom2: PhantomData<C>,
}

impl<F, C, const D: usize> CircuitComposer<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    pub fn new() -> Self {
        Self {
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }

    /// This function maps a vector of inputs (which may contain both compile time constants and values
    /// of variables) to a vector of built circuits and proofs when evaluated on those same inputs.
    pub fn map<I: CircuitVariable, S>(
        &self,
        inputs: Vec<I::ValueType>,
        spec: S,
    ) -> Vec<(CircuitData<F, C, D>, ProofWithPublicInputs<F, C, D>)>
    where
        S: Fn(&I, &mut CircuitBuilder<F, D>),
    {
        inputs
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                println!("mapping input {} to value {:?}", i, value);
                let mut builder = CircuitBuilder::new();
                let input = builder.init::<I>();
                spec(&input, &mut builder);
                let data = builder.build::<C>();

                let mut pw = PartialWitness::new();
                input.set(&mut pw, value);
                let proof = data.prove(pw).unwrap();
                (data, proof)
            })
            .collect()
    }

    /// This functions takes a vector of circuits and proofs and reduces them using a binary
    /// and linear tree reduction within a composition of circuits.
    pub fn reduce<I: CircuitVariable, S>(
        &self,
        proofs: Vec<(CircuitData<F, C, D>, ProofWithPublicInputs<F, C, D>)>,
        spec: S,
    ) -> (CircuitData<F, C, D>, ProofWithPublicInputs<F, C, D>)
    where
        S: Fn(&I, &I, &mut CircuitBuilder<F, D>),
    {
        let mut pfs = proofs;
        let rounds = (pfs.len() as f64).log2() as usize;
        for i in 0..rounds {
            println!("TREE LAYER {}", i);
            let mut tmp_pfs = Vec::new();
            for j in 0..pfs.len() / 2 {
                println!("combining proof {} and {}", j * 2, j * 2 + 1);
                let mut builder = CircuitBuilder::new();

                let p1 = builder.api.add_virtual_proof_with_pis(&pfs[j * 2].0.common);
                let p2 = builder
                    .api
                    .add_virtual_proof_with_pis(&pfs[j * 2 + 1].0.common);

                let vd1 = builder
                    .api
                    .add_virtual_verifier_data(pfs[j * 2].0.common.config.fri_config.cap_height);
                let vd2 = builder.api.add_virtual_verifier_data(
                    pfs[j * 2 + 1].0.common.config.fri_config.cap_height,
                );

                builder
                    .api
                    .verify_proof::<C>(&p1, &vd1, &pfs[j * 2].0.common);
                builder
                    .api
                    .verify_proof::<C>(&p2, &vd2, &pfs[j * 2 + 1].0.common);

                let i1 = I::from_targets(p1.public_inputs.as_slice());
                let i2 = I::from_targets(p2.public_inputs.as_slice());

                spec(&i1, &i2, &mut builder);
                let data = builder.build::<C>();

                let mut pw = PartialWitness::new();
                pw.set_proof_with_pis_target(&p1, &pfs[j * 2].1);
                pw.set_proof_with_pis_target(&p2, &pfs[j * 2 + 1].1);
                pw.set_verifier_data_target(&vd1, &pfs[j * 2].0.verifier_only);
                pw.set_verifier_data_target(&vd2, &pfs[j * 2 + 1].0.verifier_only);

                let proof = data.prove(pw).unwrap();
                println!("finished. acc={:?}", proof.public_inputs[0]);
                tmp_pfs.push((data, proof));
            }
            pfs = tmp_pfs;
        }
        pfs.remove(0)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::composer::CircuitComposer;
    use crate::vars::{CircuitVariable, Variable};

    #[test]
    fn test_simple_circuit() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let composer = CircuitComposer::<F, C, D>::new();
        let inputs: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let l1 = composer.map(inputs, |i1: &Variable, builder| {
            builder.api.register_public_inputs(i1.targets().as_slice());
        });

        println!("finished mapping");

        let l2 = composer.reduce(l1, |i1: &Variable, i2: &Variable, builder| {
            let sum = builder.add(*i1, *i2);
            builder.api.register_public_inputs(sum.targets().as_slice());
        });

        println!("finished reducing");

        println!("{:#?}", l2.1.public_inputs[0]);
    }
}
