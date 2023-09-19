use core::marker::PhantomData;
use std::env;

use array_macro::array;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::vars::BeaconValidatorVariable;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 41;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum BeaconValidatorGeneratorInput {
    IndexConst(u64),
    IndexVariable(U64Variable),
    PubkeyVariable(BLSPubkeyVariable),
}

#[derive(Debug, Clone)]
pub struct BeaconValidatorGenerator<L: PlonkParameters<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    input: BeaconValidatorGeneratorInput,
    pub validator: BeaconValidatorVariable,
    pub validator_idx: U64Variable,
    pub proof: [Bytes32Variable; DEPTH],
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> BeaconValidatorGenerator<L, D> {
    pub fn new_with_index_const(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        validator_idx: u64,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconValidatorGeneratorInput::IndexConst(validator_idx),
            validator: builder.init::<BeaconValidatorVariable>(),
            validator_idx: builder.init::<U64Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: PhantomData,
        }
    }

    pub fn new_with_index_variable(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        validator_idx: U64Variable,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconValidatorGeneratorInput::IndexVariable(validator_idx),
            validator: builder.init::<BeaconValidatorVariable>(),
            validator_idx: builder.init::<U64Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: PhantomData,
        }
    }

    pub fn new_with_pubkey_variable(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        pubkey: BLSPubkeyVariable,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconValidatorGeneratorInput::PubkeyVariable(pubkey),
            validator: builder.init::<BeaconValidatorVariable>(),
            validator_idx: builder.init::<U64Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: PhantomData,
        }
    }

    pub fn id() -> String {
        "BeaconValidatorGenerator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for BeaconValidatorGenerator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        match self.input {
            BeaconValidatorGeneratorInput::IndexConst(_) => {}
            BeaconValidatorGeneratorInput::IndexVariable(idx) => {
                targets.extend(idx.targets());
            }
            BeaconValidatorGeneratorInput::PubkeyVariable(pubkey) => {
                targets.extend(pubkey.targets());
            }
        }
        targets
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let block_root = self.block_root.get(witness);
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt.block_on(async {
            match &self.input {
                BeaconValidatorGeneratorInput::IndexConst(idx) => self
                    .client
                    .get_validator(hex!(block_root), *idx)
                    .expect("failed to get validator"),
                BeaconValidatorGeneratorInput::IndexVariable(idx) => {
                    let idx = idx.get(witness).as_u64();
                    self.client
                        .get_validator(hex!(block_root), idx)
                        .expect("failed to get validator")
                }
                BeaconValidatorGeneratorInput::PubkeyVariable(pubkey) => {
                    let pubkey = hex!(pubkey.get(witness));
                    self.client
                        .get_validator_by_pubkey(hex!(block_root), pubkey)
                        .expect("failed to get validator")
                }
            }
        });
        self.validator.set(out_buffer, result.validator);
        self.validator_idx
            .set(out_buffer, result.validator_idx.into());
        for i in 0..DEPTH {
            self.proof[i].set(out_buffer, bytes32!(result.proof[i]));
        }
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        match self.input {
            BeaconValidatorGeneratorInput::IndexConst(idx) => {
                dst.write_usize(0)?;
                dst.write_usize(idx as usize)?;
            }
            BeaconValidatorGeneratorInput::IndexVariable(idx) => {
                dst.write_usize(1)?;
                dst.write_target_vec(&idx.targets())?;
            }
            BeaconValidatorGeneratorInput::PubkeyVariable(pubkey) => {
                dst.write_usize(2)?;
                dst.write_target_vec(&pubkey.targets())?;
            }
        }
        dst.write_target_vec(&self.validator.targets())?;
        dst.write_target_vec(&self.validator_idx.targets())?;
        for i in 0..DEPTH {
            dst.write_target_vec(&self.proof[i].targets())?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let input_type = src.read_usize()?;
        let input = if input_type == 0 {
            let idx = src.read_usize()?;
            BeaconValidatorGeneratorInput::IndexConst(idx as u64)
        } else if input_type == 1 {
            let idx = U64Variable::from_targets(&src.read_target_vec()?);
            BeaconValidatorGeneratorInput::IndexVariable(idx)
        } else if input_type == 2 {
            let pubkey = BLSPubkeyVariable::from_targets(&src.read_target_vec()?);
            BeaconValidatorGeneratorInput::PubkeyVariable(pubkey)
        } else {
            panic!("invalid input type")
        };
        let validator = BeaconValidatorVariable::from_targets(&src.read_target_vec()?);
        let validator_idx = U64Variable::from_targets(&src.read_target_vec()?);
        let proof = array![_ => Bytes32Variable::from_targets(&src.read_target_vec()?); DEPTH];
        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        Ok(Self {
            client,
            block_root,
            input,
            validator,
            validator_idx,
            proof,
            _phantom: PhantomData,
        })
    }
}
