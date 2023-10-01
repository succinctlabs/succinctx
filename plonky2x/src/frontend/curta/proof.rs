use core::iter::once;

use curta::plonky2::stark::config::{CurtaConfig, StarkyConfig};
use curta::plonky2::stark::proof::{
    StarkOpeningSet, StarkOpeningSetTarget, StarkProof, StarkProofTarget,
};
use curta::plonky2::stark::Starky;
use curta::plonky2::Plonky2Air;

use crate::frontend::recursion::extension::ExtensionVariable;
use crate::frontend::recursion::fri::proof::FriProofVariable;
use crate::frontend::recursion::hash::MerkleCapVariable;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarkProofVariable<const D: usize> {
    pub trace_caps: Vec<MerkleCapVariable>,
    pub quotient_polys_cap: MerkleCapVariable,
    pub global_values: Vec<Variable>,
    pub openings: StarkOpeningSetVariable<D>,
    pub opening_proof: FriProofVariable<D>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarkOpeningSetVariable<const D: usize> {
    pub local_values: Vec<ExtensionVariable<D>>,
    pub next_values: Vec<ExtensionVariable<D>>,
    pub quotient_polys: Vec<ExtensionVariable<D>>,
}

impl VariableStream {
    pub fn read_stark_proof<F, A, C, const D: usize>(
        &mut self,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkProofVariable<D>
    where
        F: RichField + Extendable<D>,
        A: Plonky2Air<F, D>,
        C: CurtaConfig<D, F = F, FE = F::Extension>,
    {
        let fri_params = config.fri_params();
        let cap_height = fri_params.config.cap_height;

        let num_leaves_per_oracle = stark
            .air()
            .round_data()
            .into_iter()
            .map(|x| x.num_columns)
            .chain(once(
                stark.air().quotient_degree_factor() * config.num_challenges,
            ))
            .collect::<Vec<_>>();

        let num_rounds = stark.air().num_rounds();
        let num_global_values = stark.air().num_global_values();
        let global_values = self.read_exact(num_global_values).to_vec();
        let trace_caps = (0..num_rounds)
            .map(|_| self.read_merkle_cap(cap_height))
            .collect::<Vec<_>>();
        StarkProofVariable {
            trace_caps,
            quotient_polys_cap: self.read_merkle_cap(cap_height),
            openings: self.read_stark_opening_set(stark, config),
            global_values,
            opening_proof: self.read_fri_proof(&num_leaves_per_oracle, &fri_params),
        }
    }

    pub fn read_stark_opening_set<
        F: RichField + Extendable<D>,
        A: Plonky2Air<F, D>,
        C: CurtaConfig<D, F = F>,
        const D: usize,
    >(
        &mut self,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkOpeningSetVariable<D> {
        let num_challenges = config.num_challenges;
        StarkOpeningSetVariable {
            local_values: self.read_vec::<ExtensionVariable<D>>(stark.air().num_columns()),
            next_values: self.read_vec::<ExtensionVariable<D>>(stark.air().num_columns()),
            quotient_polys: self.read_vec::<ExtensionVariable<D>>(
                stark.air().quotient_degree_factor() * num_challenges,
            ),
        }
    }

    pub fn write_stark_proof<const D: usize>(&mut self, proof: &StarkProofVariable<D>) {
        let StarkProofVariable {
            trace_caps,
            quotient_polys_cap,
            global_values,
            openings,
            opening_proof,
        } = proof;

        trace_caps.iter().for_each(|cap| {
            self.write_merkle_cap(cap);
        });
        self.write_merkle_cap(quotient_polys_cap);
        self.write_slice(global_values);
        self.write_stark_opening_set(openings);
        self.write_fri_proof(opening_proof);
    }

    pub fn write_stark_opening_set<const D: usize>(&mut self, proof: &StarkOpeningSetVariable<D>) {
        let StarkOpeningSetVariable {
            local_values,
            next_values,
            quotient_polys,
        } = proof;

        self.write_slice(local_values);
        self.write_slice(next_values);
        self.write_slice(quotient_polys);
    }
}

impl<L: PlonkParameters<D>, const D: usize> OutputVariableStream<L, D> {
    pub fn read_stark_proof<A, C>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkProofVariable<D>
    where
        A: Plonky2Air<L::Field, D>,
        C: CurtaConfig<D, F = L::Field, FE = <L::Field as Extendable<D>>::Extension>,
    {
        let fri_params = config.fri_params();
        let cap_height = fri_params.config.cap_height;

        let num_leaves_per_oracle = stark
            .air()
            .round_data()
            .into_iter()
            .map(|x| x.num_columns)
            .chain(once(
                stark.air().quotient_degree_factor() * config.num_challenges,
            ))
            .collect::<Vec<_>>();

        let num_rounds = stark.air().num_rounds();
        let num_global_values = stark.air().num_global_values();
        let global_values = self.read_exact(builder, num_global_values).to_vec();
        let trace_caps = (0..num_rounds)
            .map(|_| self.read_merkle_cap(builder, cap_height))
            .collect::<Vec<_>>();
        StarkProofVariable {
            trace_caps,
            quotient_polys_cap: self.read_merkle_cap(builder, cap_height),
            openings: self.read_stark_opening_set(builder, stark, config),
            global_values,
            opening_proof: self.read_fri_proof(builder, &num_leaves_per_oracle, &fri_params),
        }
    }

    pub fn read_stark_opening_set<A: Plonky2Air<L::Field, D>, C: CurtaConfig<D, F = L::Field>>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkOpeningSetVariable<D> {
        let num_challenges = config.num_challenges;
        StarkOpeningSetVariable {
            local_values: self.read_vec::<ExtensionVariable<D>>(builder, stark.air().num_columns()),
            next_values: self.read_vec::<ExtensionVariable<D>>(builder, stark.air().num_columns()),
            quotient_polys: self.read_vec::<ExtensionVariable<D>>(
                builder,
                stark.air().quotient_degree_factor() * num_challenges,
            ),
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    pub fn read_stark_proof<A, C>(
        &mut self,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkProof<L::Field, C, D>
    where
        A: Plonky2Air<L::Field, D>,
        C: CurtaConfig<D, F = L::Field, FE = <L::Field as Extendable<D>>::Extension>,
    {
        let fri_params = config.fri_params();
        let cap_height = fri_params.config.cap_height;

        let num_leaves_per_oracle = stark
            .air()
            .round_data()
            .into_iter()
            .map(|x| x.num_columns)
            .chain(once(
                stark.air().quotient_degree_factor() * config.num_challenges,
            ))
            .collect::<Vec<_>>();

        let num_rounds = stark.air().num_rounds();
        let num_global_values = stark.air().num_global_values();
        let global_values = self.read_exact(num_global_values).to_vec();
        let trace_caps = (0..num_rounds)
            .map(|_| self.read_merkle_cap(cap_height))
            .collect::<Vec<_>>();
        StarkProof {
            trace_caps,
            quotient_polys_cap: self.read_merkle_cap(cap_height),
            openings: self.read_stark_opening_set(stark, config),
            global_values,
            opening_proof: self.read_fri_proof(&num_leaves_per_oracle, &fri_params),
        }
    }

    pub fn read_stark_opening_set<A, C>(
        &mut self,
        stark: &Starky<A>,
        config: &StarkyConfig<C, D>,
    ) -> StarkOpeningSet<L::Field, D>
    where
        A: Plonky2Air<L::Field, D>,
        C: CurtaConfig<D, F = L::Field>,
    {
        let num_challenges = config.num_challenges;
        StarkOpeningSet {
            local_values: self.read_extension_vec(stark.air().num_columns()),
            next_values: self.read_extension_vec(stark.air().num_columns()),
            quotient_polys: self
                .read_extension_vec(stark.air().quotient_degree_factor() * num_challenges),
        }
    }

    pub fn write_stark_proof<C: CurtaConfig<D, F = L::Field>>(
        &mut self,
        proof: StarkProof<L::Field, C, D>,
    ) {
        let StarkProof {
            trace_caps,
            quotient_polys_cap,
            global_values,
            openings,
            opening_proof,
        } = proof;

        trace_caps.into_iter().for_each(|cap| {
            self.write_merkle_cap(cap);
        });
        self.write_merkle_cap(quotient_polys_cap);
        self.write_slice(&global_values);
        self.write_stark_opening_set(openings);
        self.write_fri_proof(opening_proof);
    }

    pub fn write_stark_opening_set(&mut self, openning_set: StarkOpeningSet<L::Field, D>) {
        let StarkOpeningSet {
            local_values,
            next_values,
            quotient_polys,
        } = openning_set;

        self.write_extension_vec(local_values);
        self.write_extension_vec(next_values);
        self.write_extension_vec(quotient_polys);
    }
}

impl<const D: usize> From<StarkOpeningSetVariable<D>> for StarkOpeningSetTarget<D> {
    fn from(value: StarkOpeningSetVariable<D>) -> Self {
        let local_values = value.local_values.into_iter().map(|v| v.into()).collect();
        let next_values = value.next_values.into_iter().map(|v| v.into()).collect();
        let quotient_polys = value.quotient_polys.into_iter().map(|v| v.into()).collect();
        Self {
            local_values,
            next_values,
            quotient_polys,
        }
    }
}

impl<const D: usize> From<StarkOpeningSetTarget<D>> for StarkOpeningSetVariable<D> {
    fn from(value: StarkOpeningSetTarget<D>) -> Self {
        let local_values = value.local_values.into_iter().map(|v| v.into()).collect();
        let next_values = value.next_values.into_iter().map(|v| v.into()).collect();
        let quotient_polys = value.quotient_polys.into_iter().map(|v| v.into()).collect();
        Self {
            local_values,
            next_values,
            quotient_polys,
        }
    }
}

impl<const D: usize> From<StarkProofVariable<D>> for StarkProofTarget<D> {
    fn from(value: StarkProofVariable<D>) -> Self {
        let trace_caps = value.trace_caps.into_iter().map(|v| v.into()).collect();
        let quotient_polys_cap = value.quotient_polys_cap.into();
        let global_values = value.global_values.into_iter().map(|v| v.0).collect();
        let openings = value.openings.into();
        let opening_proof = value.opening_proof.into();
        Self {
            trace_caps,
            quotient_polys_cap,
            global_values,
            openings,
            opening_proof,
        }
    }
}

impl<const D: usize> From<StarkProofTarget<D>> for StarkProofVariable<D> {
    fn from(value: StarkProofTarget<D>) -> Self {
        let trace_caps = value.trace_caps.into_iter().map(|v| v.into()).collect();
        let quotient_polys_cap = value.quotient_polys_cap.into();
        let global_values = value.global_values.into_iter().map(|v| v.into()).collect();
        let openings = value.openings.into();
        let opening_proof = value.opening_proof.into();
        Self {
            trace_caps,
            quotient_polys_cap,
            global_values,
            openings,
            opening_proof,
        }
    }
}

#[cfg(test)]
mod tests {
    use curta::chip::builder::AirBuilder;
    use curta::chip::instruction::empty::EmptyInstruction;
    use curta::chip::register::element::ElementRegister;
    use curta::chip::register::{Register, RegisterSerializable};
    use curta::chip::trace::generator::ArithmeticGenerator;
    use curta::chip::AirParameters;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use curta::plonky2::stark::config::{
        CurtaPoseidonGoldilocksConfig, PoseidonGoldilocksStarkConfig,
    };
    use curta::plonky2::stark::gadget::StarkGadget;
    use curta::plonky2::stark::prover::StarkyProver;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FibonacciParameters;

    impl AirParameters for FibonacciParameters {
        type Field = GoldilocksField;
        type CubicParams = GoldilocksCubicParameters;
        type Instruction = EmptyInstruction<GoldilocksField>;
        const NUM_ARITHMETIC_COLUMNS: usize = 0;
        const NUM_FREE_COLUMNS: usize = 2;
    }

    fn fibonacci<F: Field>(n: usize, x0: F, x1: F) -> F {
        (0..n).fold((x0, x1), |x, _| (x.1, x.0 + x.1)).1
    }

    #[test]
    fn test_conversion() {
        type L = FibonacciParameters;
        type SC = PoseidonGoldilocksStarkConfig;

        let mut air_builder = AirBuilder::<L>::new();
        let x_0 = air_builder.alloc::<ElementRegister>();
        let x_1 = air_builder.alloc::<ElementRegister>();

        // x0' <- x1
        air_builder.set_to_expression_transition(&x_0.next(), x_1.expr());
        // x1' <- x0 + x1
        air_builder.set_to_expression_transition(&x_1.next(), x_0.expr() + x_1.expr());

        let num_rows = 1 << 10;

        let (air, _) = air_builder.build();

        let stark = Starky::new(air);
        let config = SC::standard_fast_config(num_rows);

        let mut builder = DefaultBuilder::new();

        let proof = builder.api.add_virtual_stark_proof(&stark, &config);

        let proof_variable = StarkProofVariable::from(proof.clone());
        let proof_back = StarkProofTarget::from(proof_variable.clone());

        assert_eq!(proof, proof_back);
    }

    #[test]
    fn test_variable_stream() {
        type L = FibonacciParameters;
        type SC = PoseidonGoldilocksStarkConfig;

        let mut air_builder = AirBuilder::<L>::new();
        let x_0 = air_builder.alloc::<ElementRegister>();
        let x_1 = air_builder.alloc::<ElementRegister>();

        // x0' <- x1
        air_builder.set_to_expression_transition(&x_0.next(), x_1.expr());
        // x1' <- x0 + x1
        air_builder.set_to_expression_transition(&x_1.next(), x_0.expr() + x_1.expr());

        let num_rows = 1 << 10;

        let (air, _) = air_builder.build();

        let stark = Starky::new(air);
        let config = SC::standard_fast_config(num_rows);

        let mut builder = DefaultBuilder::new();

        let proof_target = builder.api.add_virtual_stark_proof(&stark, &config);
        let proof_variable = StarkProofVariable::from(proof_target);

        let mut stream = VariableStream::new();
        stream.write_stark_proof(&proof_variable);

        let proof_back = stream.read_stark_proof(&stark, &config);

        assert_eq!(proof_variable, proof_back);
    }

    #[test]
    fn test_value_stream() {
        type F = GoldilocksField;
        type L = FibonacciParameters;
        type SC = PoseidonGoldilocksStarkConfig;
        type C = CurtaPoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut air_builder = AirBuilder::<L>::new();
        let x_0 = air_builder.alloc::<ElementRegister>();
        let x_1 = air_builder.alloc::<ElementRegister>();

        // x0' <- x1
        air_builder.set_to_expression_transition(&x_0.next(), x_1.expr());
        // x1' <- x0 + x1
        air_builder.set_to_expression_transition(&x_1.next(), x_0.expr() + x_1.expr());

        let num_rows = 1 << 5;
        let public_inputs = [F::ZERO, F::ONE, fibonacci(num_rows - 1, F::ZERO, F::ONE)];

        let (air, air_data) = air_builder.build();

        let stark = Starky::new(air);
        let config = SC::standard_fast_config(num_rows);

        let generator = ArithmeticGenerator::<L>::new(air_data, num_rows);

        let writer = generator.new_writer();

        writer.write(&x_0, &F::ZERO, 0);
        writer.write(&x_1, &F::ONE, 0);

        for i in 0..num_rows {
            writer.write_row_instructions(&generator.air_data, i);
        }

        let proof =
            StarkyProver::<F, C, D>::prove(&config, &stark, &generator, &public_inputs).unwrap();

        let mut stream = ValueStream::<DefaultParameters, 2>::new();
        stream.write_stark_proof(proof.clone());

        let proof_back: StarkProof<GoldilocksField, CurtaPoseidonGoldilocksConfig, 2> =
            stream.read_stark_proof(&stark, &config);

        assert_eq!(proof, proof_back);
    }
}
