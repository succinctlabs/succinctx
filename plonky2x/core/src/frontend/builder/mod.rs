mod boolean;
pub mod io;
pub mod permutation;
mod proof;
pub mod watch;

use alloc::collections::BTreeMap;
use std::collections::HashMap;
use std::env;

use backtrace::Backtrace;
use curta::chip::ec::edwards::ed25519::params::Ed25519ScalarField;
use curta::machine::hash::sha::sha256::SHA256;
use curta::machine::hash::sha::sha512::SHA512;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use itertools::Itertools;
use plonky2::iop::generator::{SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder as CircuitAPI;
use plonky2::plonk::circuit_data::CircuitConfig;
use tokio::runtime::Runtime;

pub use self::io::CircuitIO;
use super::ecc::curve25519::curta::accelerator::EcOpAccelerator;
use super::hash::blake2::curta::Blake2bAccelerator;
use super::hash::sha::sha256::curta::SHA256Accelerator;
use super::hash::sha::sha512::curta::SHA512Accelerator;
use super::hint::HintGenerator;
use super::vars::EvmVariable;
use crate::backend::circuit::{CircuitBuild, DefaultParameters, MockCircuitBuild, PlonkParameters};
use crate::frontend::hint::asynchronous::generator::AsyncHintDataRef;
use crate::frontend::vars::{BoolVariable, CircuitVariable, Variable};
use crate::prelude::ArrayVariable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::eth::beaconchain::BeaconchainAPIClient;

/// The universal builder for building circuits using `plonky2x`.
pub struct CircuitBuilder<L: PlonkParameters<D>, const D: usize> {
    pub api: CircuitAPI<L::Field, D>,
    pub io: CircuitIO<D>,
    pub execution_client: Option<Provider<Http>>,
    pub chain_id: Option<u64>,
    pub beacon_client: Option<BeaconClient>,
    pub beaconchain_api_client: Option<BeaconchainAPIClient>,
    pub debug: bool,
    pub debug_variables: HashMap<usize, String>,
    pub(crate) hints: Vec<Box<dyn HintGenerator<L, D>>>,
    pub(crate) async_hints: Vec<AsyncHintDataRef<L, D>>,
    pub(crate) async_hints_indices: Vec<usize>,

    pub blake2b_accelerator: Option<Blake2bAccelerator<L, D>>,
    pub sha256_accelerator: Option<SHA256Accelerator>,
    pub sha512_accelerator: Option<SHA512Accelerator>,
    pub ec_ops_accelerator: Option<EcOpAccelerator<Ed25519ScalarField>>,
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
        let mut builder = Self {
            api,
            io: CircuitIO::new(),
            beacon_client: None,
            beaconchain_api_client: None,
            execution_client: None,
            chain_id: None,
            debug: false,
            debug_variables: HashMap::new(),
            hints: Vec::new(),
            async_hints: Vec::new(),
            async_hints_indices: Vec::new(),
            blake2b_accelerator: None,
            sha256_accelerator: None,
            sha512_accelerator: None,
            ec_ops_accelerator: None,
        };

        if let Ok(rpc_url) = env::var("CONSENSUS_RPC_1") {
            let client = BeaconClient::new(rpc_url);
            builder.set_beacon_client(client);
        }

        if let Ok(api_url) = env::var("BEACONCHAIN_API_URL_1") {
            if let Ok(api_key) = env::var("BEACONCHAIN_API_KEY_1") {
                let client = BeaconchainAPIClient::new(api_url, api_key);
                builder.set_beaconchain_api_client(client);
            }
        }

        builder
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

    pub fn set_beaconchain_api_client(&mut self, client: BeaconchainAPIClient) {
        self.beaconchain_api_client = Some(client);
    }

    /// Adds all the constraints nedded before building the circuit and registering hints.
    fn pre_build(&mut self) {
        let blake2b_accelerator = self.blake2b_accelerator.clone();
        if let Some(accelerator) = blake2b_accelerator {
            accelerator.build(self);
        }

        let sha256_accelerator = self.sha256_accelerator.clone();
        if let Some(accelerator) = sha256_accelerator {
            self.curta_constrain_sha::<SHA256, 64>(accelerator);
        }

        let sha512_accelerator = self.sha512_accelerator.clone();
        if let Some(accelerator) = sha512_accelerator {
            self.curta_constrain_sha::<SHA512, 80>(accelerator);
        }

        let ec_ops_accelerator = self.ec_ops_accelerator.clone();
        if let Some(accelerator) = ec_ops_accelerator {
            self.curta_constrain_ec_op::<Ed25519ScalarField>(accelerator);
        }

        for (index, gen_ref) in self
            .async_hints_indices
            .iter()
            .zip(self.async_hints.iter_mut())
        {
            let new_output_stream = self.hints[*index].output_stream_mut();
            let output_stream = gen_ref.0.output_stream_mut();
            *output_stream = new_output_stream.clone();
        }

        let hints = self.hints.drain(..).collect::<Vec<_>>();
        let generators = hints
            .into_iter()
            .map(|h| WitnessGeneratorRef(h))
            .collect::<Vec<_>>();
        self.api.add_generators(generators);

        match self.io {
            CircuitIO::Bytes(ref io) => {
                let input = io
                    .input
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                self.register_public_inputs(input.as_slice());
                self.register_public_inputs(output.as_slice());
            }
            CircuitIO::Elements(ref io) => {
                let input = io
                    .input
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                self.register_public_inputs(input.as_slice());
                self.register_public_inputs(output.as_slice());
            }
            CircuitIO::RecursiveProofs(ref io) => {
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                self.register_public_inputs(output.as_slice());
            }
            CircuitIO::None() => {}
        };
    }

    /// Constructs a map of async hints according to their generator indices.
    fn async_hint_map(
        generators: &[WitnessGeneratorRef<L::Field, D>],
        async_hints: Vec<AsyncHintDataRef<L, D>>,
    ) -> BTreeMap<usize, AsyncHintDataRef<L, D>> {
        let mut async_hint_indices = Vec::new();

        for (i, generator) in generators.iter().enumerate() {
            if generator.0.id().starts_with("--async") {
                async_hint_indices.push(i);
            }
        }

        assert_eq!(async_hint_indices.len(), async_hints.len());

        let mut async_hints_map = BTreeMap::new();
        for (key, gen) in async_hint_indices.iter().zip(async_hints) {
            async_hints_map.insert(*key, gen);
        }

        async_hints_map
    }

    /// Build the circuit.
    pub fn build(mut self) -> CircuitBuild<L, D> {
        self.pre_build();
        let data = self.api.build();
        let async_hints = Self::async_hint_map(&data.prover_only.generators, self.async_hints);
        CircuitBuild {
            data,
            io: self.io,
            async_hints,
        }
    }

    pub fn mock_build(mut self) -> MockCircuitBuild<L, D> {
        self.pre_build();
        let mock_data = self.api.mock_build();
        let async_hints = Self::async_hint_map(&mock_data.prover_only.generators, self.async_hints);

        MockCircuitBuild {
            data: mock_data,
            io: self.io,
            debug_variables: self.debug_variables,
            async_hints,
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

    /// Initializes a variable with no value in the circuit without any validity checks.
    pub fn init_unsafe<V: CircuitVariable>(&mut self) -> V {
        V::init_unsafe(self)
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

    /// Initializes a vector of variables constant values in the circuit without validity checks.
    pub fn constant_vec<V: CircuitVariable>(&mut self, value: &[V::ValueType<L::Field>]) -> Vec<V>
    where
        V::ValueType<L::Field>: Copy,
    {
        value.iter().map(|v| V::constant(self, *v)).collect()
    }

    /// Asserts that the given variable is valid.
    pub fn assert_is_valid<V: CircuitVariable>(&mut self, variable: V) {
        variable.assert_is_valid(self)
    }

    /// Registers the given targets as public inputs.
    pub(crate) fn register_public_inputs(&mut self, inputs: &[Variable]) {
        self.api
            .register_public_inputs(&inputs.iter().map(|v| v.0).collect_vec());
    }

    /// Inverse returns res = 1 / i1.
    pub fn inverse(&mut self, i1: Variable) -> Variable {
        self.api.inverse(i1.0).into()
    }

    // @audit
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

        self.api.is_equal(i1.0, zero).into()
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
            let target_eq: BoolVariable = self.api.is_equal(*t1, *t2).into();
            result = self.and(target_eq, result);
        }
        result
    }
    // @end-audit

    /// Connects two variables.
    pub fn connect<V: CircuitVariable>(&mut self, i1: V, i2: V) {
        let i1 = i1.targets();
        let i2 = i2.targets();
        for i in 0..i1.len() {
            self.api.connect(i1[i], i2[i]);
        }
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

    use log::debug;
    use plonky2::field::types::Field;

    use super::DefaultBuilder;
    use crate::prelude::*;
    use crate::utils;

    #[test]
    fn test_simple_circuit_with_field_io() {
        utils::setup_logger();
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
        debug!("{}", sum.0);
    }

    #[test]
    fn test_simple_circuit_with_evm_io() {
        utils::setup_logger();
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
        debug!("{}", xor);
    }
}
