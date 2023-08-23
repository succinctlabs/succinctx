mod boolean;

use ethers::providers::{Http, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::SimpleGenerator;
use plonky2::iop::target::BoolTarget;
use plonky2::plonk::circuit_builder::CircuitBuilder as _CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::GenericConfig;
use plonky2::iop::target::Target;

use crate::ethutils::beacon::BeaconClient;
use crate::vars::{BoolVariable, CircuitVariable, Variable, ByteVariable, Bytes32Variable};

// A struct related to EvmIO for the circuit builder
// This is only used in some cases
struct EvmIO {
    pub input_bytes: Vec<ByteVariable>,
    pub output_bytes: Vec<ByteVariable>,
    pub input_hash: Bytes32Variable,
    pub output_hash: Bytes32Variable,
    pub onchain_constraints_added: bool,
}
/// This is the API that we recommend developers use for writing circuits. It is a wrapper around
/// the basic plonky2 builder.
pub struct CircuitBuilder<F: RichField + Extendable<D>, const D: usize> {
    pub api: _CircuitBuilder<F, D>,
    pub execution_client: Option<Provider<Http>>,
    pub beacon_client: Option<BeaconClient>,
    pub evm_io: Option<EvmIO>,
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    /// Creates a new API for building circuits.
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let api = _CircuitBuilder::new(config);
        Self {
            api,
            beacon_client: None,
            execution_client: None,
            evm_io: None,
        }
    }

    pub fn set_execution_client(&mut self, client: Provider<Http>) {
        self.execution_client = Some(client);
    }

    pub fn set_beacon_client(&mut self, client: BeaconClient) {
        self.beacon_client = Some(client);
    }

    fn get_evm_io(&self, instantiate_if_missing: bool) -> &EvmIO {
        if self.evm_io.is_none() {
            if instantiate_if_missing {
                self.evm_io = Some(EvmIO {
                    input_bytes: Vec::new(),
                    output_bytes: Vec::new(),
                    input_hash: self.init::<Bytes32Variable>(),
                    output_hash: self.init::<Bytes32Variable>(),
                    onchain_constraints_added: false,
                });
            } else {
                panic!("EvmIO must be instantiated before use");
            }
        }
        self.evm_io.as_ref().unwrap().clone()
    } 

    // TODO: make this return an error 
    pub fn can_build_evm(&self) -> bool {
        if !self.evm_io.is_some() {
            return false;
        }
        let evm_io = self.get_evm_io(false);
        if evm_io.input_bytes.len() == 0 {
            return false;
        }
        if evm_io.output_bytes.len() == 0 {
            return false;
        }
        if !evm_io.onchain_constraints_added {
            return false;
        }
        return true
    }

    // Allocates bytes to `input_bytes` and returns newly allocated bytes
    pub fn read_bytes(&mut self, n: usize) -> Vec<ByteVariable> {
        let evm_io = self.get_evm_io(true);
        let mut bytes = Vec::new();
        for _ in 0..n {
            bytes.push(self.init::<ByteVariable>());
        }
        evm_io.input_bytes.append(&mut bytes);
        bytes
    }

    // Writes bytes to output bytes
    pub fn write_bytes(&mut self, bytes: &[ByteVariable]) {
        let evm_io = self.get_evm_io(true);
        for byte in bytes {
            evm_io.output_bytes.push(*byte);
        }
    }

    pub fn constraint_onchain(&mut self) {
        let evm_io = self.get_evm_io(false); // TODO: remove panic
        evm_io.onchain_constraints_added = true;
        // TODO constraint input_hash = hash(self.input_bytes)
        // TODO constraint output_hash = hash(self.output_bytes)
        // self.input_hash = Some(input_hash);
        // self.output_hash = Some(output_hash);
        self.api.register_public_inputs(&evm_io.input_hash.targets());
        self.api.register_public_inputs(&evm_io.output_hash.targets());
    }

    /// Build the circuit.
    pub fn build<C: GenericConfig<D, F = F>>(self) -> CircuitData<F, C, D> {
        self.api.build()
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

    pub fn assert_is_zero(&mut self, i1: Variable) {
        let zero = self.api.zero();
        self.assert_is_equal(i1, Variable(zero));
    }

    /// Fails if i1 != i2.
    pub fn assert_is_equal(&mut self, i1: Variable, i2: Variable) {
        self.api.connect(i1.0, i2.0);
    }

    pub fn register_public_inputs(&mut self, inputs: &[Target]) {
        self.api.register_public_inputs(inputs);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::builder::CircuitBuilder;
    use crate::vars::Variable;

    #[test]
    fn test_simple_circuit() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let zero = builder.zero::<Variable>();
        let one = builder.one::<Variable>();
        let sum = builder.add(zero, one);
        builder.assert_is_equal(sum, one);

        let pw = PartialWitness::new();
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Default for CircuitBuilder<F, D> {
    fn default() -> Self {
        Self::new()
    }
}
