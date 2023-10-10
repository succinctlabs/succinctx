mod boolean;
pub mod io;
mod proof;
pub mod watch;

use std::collections::HashMap;

use backtrace::Backtrace;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use plonky2::iop::generator::SimpleGenerator;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder as CircuitAPI;
use plonky2::plonk::circuit_data::CircuitConfig;
use tokio::runtime::Runtime;

pub use self::io::CircuitIO;
use super::generator::HintRef;
use super::vars::EvmVariable;
use crate::backend::circuit::{CircuitBuild, DefaultParameters, MockCircuitBuild, PlonkParameters};
use crate::frontend::vars::{BoolVariable, CircuitVariable, Variable};
use crate::prelude::ArrayVariable;
use crate::utils::eth::beacon::BeaconClient;

/// The universal builder for building circuits using `plonky2x`.
pub struct CircuitBuilder<L: PlonkParameters<D>, const D: usize> {
    pub api: CircuitAPI<L::Field, D>,
    pub io: CircuitIO<D>,
    pub execution_client: Option<Provider<Http>>,
    pub chain_id: Option<u64>,
    pub beacon_client: Option<BeaconClient>,
    pub debug: bool,
    pub debug_variables: HashMap<usize, String>,
    pub(crate) hints: Vec<Box<dyn HintRef<L, D>>>,
    pub sha256_requests: Vec<Vec<Target>>,
    pub sha256_responses: Vec<[Target; 32]>,
}

/// The universal api for building circuits using `plonky2x` with default parameters.
pub struct DefaultBuilder {}

impl DefaultBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> CircuitBuilder<DefaultParameters, 2> {
        CircuitBuilder::<DefaultParameters, 2>::new()
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Creates a new builder.
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let api = CircuitAPI::new(config);
        Self {
            api,
            io: CircuitIO::new(),
            beacon_client: None,
            execution_client: None,
            chain_id: None,
            debug: false,
            debug_variables: HashMap::new(),
            hints: Vec::new(),
            sha256_requests: Vec::new(),
            sha256_responses: Vec::new(),
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn debug_target(&mut self, target: Target) {
        if !self.debug {
            return;
        }
        match target {
            Target::VirtualTarget { index } => {
                let bt = Backtrace::new();
                self.debug_variables.insert(index, format!("{:#?}", bt));
            }
            _ => panic!("Expected a virtual target"),
        }
    }

    pub fn set_execution_client(&mut self, client: Provider<Http>) {
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result: U256 = rt.block_on(async {
            client
                .get_chainid()
                .await
                .expect("Failed to get chain id from provided RPC")
        });
        let result_cast = result.as_u64();
        self.execution_client = Some(client);
        self.chain_id = Some(result_cast);
    }

    pub fn get_chain_id(&self) -> u64 {
        self.chain_id.unwrap()
    }

    pub fn set_beacon_client(&mut self, client: BeaconClient) {
        self.beacon_client = Some(client);
    }

    /// Build the circuit.
    pub fn build(mut self) -> CircuitBuild<L, D> {
        if !self.sha256_requests.is_empty() {
            self.curta_constrain_sha256();
        }

        let hints = self.hints.drain(..).collect::<Vec<_>>();
        for hint in hints {
            hint.register(&mut self);
        }
        match self.io {
            CircuitIO::Bytes(ref io) => {
                let input = io
                    .input
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect::<Vec<_>>();
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect::<Vec<_>>();
                self.register_public_inputs(input.as_slice());
                self.register_public_inputs(output.as_slice());
            }
            CircuitIO::Elements(ref io) => {
                let input = io
                    .input
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect::<Vec<_>>();
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect::<Vec<_>>();
                self.register_public_inputs(input.as_slice());
                self.register_public_inputs(output.as_slice());
            }
            CircuitIO::None() => {}
            _ => panic!("unsupported io type"),
        };

        let data = self.api.build();
        CircuitBuild { data, io: self.io }
    }

    pub fn mock_build(self) -> MockCircuitBuild<L, D> {
        let mock_circuit = self.api.mock_build();
        MockCircuitBuild {
            data: mock_circuit,
            io: self.io,
            debug_variables: self.debug_variables,
        }
    }

    /// Add simple generator.
    pub fn add_simple_generator<G: SimpleGenerator<L::Field, D> + Clone>(&mut self, generator: G) {
        self.api.add_simple_generator(generator)
    }

    /// Initializes a variable with no value in the circuit.
    pub fn init<V: CircuitVariable>(&mut self) -> V {
        V::init(self)
    }

    /// Initializes an array of variables with no value in the circuit.
    pub fn init_array<V: CircuitVariable, const N: usize>(&mut self) -> ArrayVariable<V, N> {
        ArrayVariable::init(self)
    }

    /// Initializes an array of variables with no value in the circuit without any validity checks.
    pub fn init_array_unsafe<V: CircuitVariable, const N: usize>(&mut self) -> ArrayVariable<V, N> {
        ArrayVariable::init_unsafe(self)
    }

    /// Initializes a variable with a constant value in the circuit.
    pub fn constant<V: CircuitVariable>(&mut self, value: V::ValueType<L::Field>) -> V {
        V::constant(self, value)
    }

    /// Initializes an array of variables with a constant value in the circuit.
    pub fn constant_array<V: CircuitVariable, const N: usize>(
        &mut self,
        value: &[V::ValueType<L::Field>],
    ) -> ArrayVariable<V, N> {
        assert_eq!(value.len(), N);
        ArrayVariable::constant(self, value.to_vec())
    }

    /// Asserts that the given variable is valid.
    pub fn assert_is_valid<V: CircuitVariable>(&mut self, variable: V) {
        variable.assert_is_valid(self)
    }

    /// Registers the given targets as public inputs.
    pub fn register_public_inputs(&mut self, inputs: &[Target]) {
        self.api.register_public_inputs(inputs);
    }

    /// Add returns res = i1 + i2 + ...
    pub fn add_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.add(acc, values[i].0);
        }
        acc.into()
    }

    /// Sub returns res = i1 - i2 - ...
    pub fn sub_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.sub(acc, values[i].0);
        }
        acc.into()
    }

    /// Mul returns res = i1 * i2 * ...
    pub fn mul_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.mul(acc, values[i].0);
        }
        acc.into()
    }

    /// Inverse returns res = 1 / i1.
    pub fn inverse(&mut self, i1: Variable) -> Variable {
        self.api.inverse(i1.0).into()
    }

    /// If selector is true, yields i1 else yields i2.
    pub fn select<V: CircuitVariable>(&mut self, selector: BoolVariable, i1: V, i2: V) -> V {
        assert_eq!(i1.targets().len(), i2.targets().len());
        let mut targets = Vec::new();
        for (t1, t2) in i1.targets().iter().zip(i2.targets().iter()) {
            targets.push(
                self.api
                    .select(BoolTarget::new_unsafe(selector.targets()[0]), *t1, *t2),
            );
        }
        V::from_targets(&targets)
    }

    /// Returns 1 if i1 is zero, 0 otherwise as a boolean.
    pub fn is_zero(&mut self, i1: Variable) -> BoolVariable {
        let zero = self.api.zero();
        self.api.is_equal(i1.0, zero).target.into()
    }

    /// Fails if i1 != i2.
    pub fn assert_is_equal<V: CircuitVariable>(&mut self, i1: V, i2: V) {
        for (t1, t2) in i1.targets().iter().zip(i2.targets().iter()) {
            self.api.connect(*t1, *t2);
        }
    }

    /// Returns 1 if i1 == i2 and 0 otherwise as a BoolVariable.
    pub fn is_equal<V: CircuitVariable>(&mut self, i1: V, i2: V) -> BoolVariable {
        let mut result = self._true();
        for (t1, t2) in i1.targets().iter().zip(i2.targets().iter()) {
            let target_eq = BoolVariable(Variable(self.api.is_equal(*t1, *t2).target));
            result = self.and(target_eq, result);
        }
        result
    }

    pub fn to_le_bits<V: EvmVariable>(&mut self, variable: V) -> Vec<BoolVariable> {
        variable.to_le_bits(self)
    }

    pub fn to_be_bits<V: EvmVariable>(&mut self, variable: V) -> Vec<BoolVariable> {
        variable.to_be_bits(self)
    }
}

impl<L: PlonkParameters<D>, const D: usize> Default for CircuitBuilder<L, D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use super::DefaultBuilder;
    use crate::prelude::*;

    #[test]
    fn test_simple_circuit_with_field_io() {
        // Define your circuit.
        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
    }

    #[test]
    fn test_simple_circuit_with_evm_io() {
        // Define your circuit.
        let mut builder = DefaultBuilder::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(7u8);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let xor = output.evm_read::<ByteVariable>();
        println!("{}", xor);
    }
}
