mod boolean;
pub mod io;
mod proof;
pub mod watch;

use std::collections::HashMap;

use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::SimpleGenerator;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder as _CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::GenericConfig;
use tokio::runtime::Runtime;

pub use self::io::CircuitIO;
use crate::backend::circuit::Circuit;
use crate::frontend::vars::{BoolVariable, CircuitVariable, Variable};
use crate::utils::eth::beacon::BeaconClient;

/// The universal api for building circuits using `plonky2x`.
pub struct CircuitBuilder<F: RichField + Extendable<D>, const D: usize> {
    pub api: _CircuitBuilder<F, D>,
    pub io: CircuitIO<D>,
    pub constants: HashMap<Variable, F>,
    pub execution_client: Option<Provider<Http>>,
    pub chain_id: Option<u64>,
    pub beacon_client: Option<BeaconClient>,
    pub debug_variables: HashMap<usize, String>,
}

/// The default suggested circuit builder using the Goldilocks field and the fast recursion config.
pub struct CircuitBuilderX {}

impl CircuitBuilderX {
    /// Creates a new builder.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> CircuitBuilder<GoldilocksField, 2> {
        CircuitBuilder::<GoldilocksField, 2>::new()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    /// Creates a new builder.
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let api = _CircuitBuilder::new(config);
        Self {
            api,
            io: CircuitIO::new(),
            constants: HashMap::new(),
            beacon_client: None,
            execution_client: None,
            debug_variables: HashMap::new(),
            chain_id: None,
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

    pub fn debug(&mut self, index: usize) {
        println!("Debugging variable {}", index);
        self.debug_variables.insert(index, "".to_string());
    }

    /// Build the circuit.
    pub fn build<C: GenericConfig<D, F = F>>(mut self) -> Circuit<F, C, D> {
        if self.io.evm.is_some() {
            let io = self.io.evm.as_ref().unwrap();
            let inputs: Vec<Target> = io.input_bytes.iter().flat_map(|b| b.targets()).collect();
            let outputs: Vec<Target> = io.output_bytes.iter().flat_map(|b| b.targets()).collect();
            self.register_public_inputs(inputs.as_slice());
            self.register_public_inputs(outputs.as_slice());
        } else if self.io.field.is_some() {
            let io = self.io.field.as_ref().unwrap();
            let inputs: Vec<Target> = io.input_variables.iter().map(|v| v.0).collect();
            let outputs: Vec<Target> = io.output_variables.iter().map(|v| v.0).collect();
            self.register_public_inputs(inputs.as_slice());
            self.register_public_inputs(outputs.as_slice());
        }

        let data = self.api.build();
        Circuit { data, io: self.io }
    }

    /// Add simple generator.
    pub fn add_simple_generator<G: SimpleGenerator<F, D> + Clone>(&mut self, generator: &G) {
        self.api.add_simple_generator(generator.clone())
    }

    /// Initializes a variable with no value in the circuit.
    pub fn init<V: CircuitVariable>(&mut self) -> V {
        V::init(self)
    }

    /// Initializes a variable with a constant value in the circuit.
    pub fn constant<V: CircuitVariable>(&mut self, value: V::ValueType<F>) -> V {
        V::constant(self, value)
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

    /// Select if b is true, yields i1 else yields i2.
    pub fn select(&mut self, selector: BoolVariable, i1: Variable, i2: Variable) -> Variable {
        self.api
            .select(BoolTarget::new_unsafe(selector.0 .0), i1.0, i2.0)
            .into()
    }

    /// Returns 1 if i1 is zero, 0 otherwise as a boolean.
    pub fn is_zero(&mut self, i1: Variable) -> BoolVariable {
        let zero = self.api.zero();
        self.api.is_equal(i1.0, zero).target.into()
    }

    /// TODO: should we change to `assert_eq`?
    /// Fails if i1 != i2.
    pub fn assert_is_equal<V: CircuitVariable>(&mut self, i1: V, i2: V) {
        assert_eq!(i1.targets().len(), i2.targets().len());
        for (t1, t2) in i1.targets().iter().zip(i2.targets().iter()) {
            self.api.connect(*t1, *t2);
        }
    }

    /// TODO: should we change to `eq`?
    /// Returns 1 if i1 == i2 and 0 otherwise as a BoolVariable
    pub fn is_equal<V: CircuitVariable>(&mut self, i1: V, i2: V) -> BoolVariable {
        assert_eq!(i1.targets().len(), i2.targets().len());
        let mut result = self.constant::<BoolVariable>(true);
        for (t1, t2) in i1.targets().iter().zip(i2.targets().iter()) {
            let target_eq = BoolVariable(Variable(self.api.is_equal(*t1, *t2).target));
            result = self.and(target_eq, result);
        }
        result
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Default for CircuitBuilder<F, D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use super::CircuitBuilderX;
    use crate::prelude::*;

    #[test]
    fn test_simple_circuit_with_field_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
    }

    #[test]
    fn test_simple_circuit_with_evm_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(7u8);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let xor = output.evm_read::<ByteVariable>();
        println!("{}", xor);
    }
}
