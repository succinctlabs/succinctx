use core::marker::PhantomData;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use tokio::runtime::Runtime;

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
pub struct BeaconValidatorGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    input: BeaconValidatorGeneratorInput,
    pub validator: BeaconValidatorVariable,
    pub validator_idx: U64Variable,
    pub proof: [Bytes32Variable; DEPTH],
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconValidatorGenerator<F, D> {
    pub fn new_with_index_const(
        builder: &mut CircuitBuilder<F, D>,
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
        builder: &mut CircuitBuilder<F, D>,
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
        builder: &mut CircuitBuilder<F, D>,
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
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconValidatorGenerator<F, D>
{
    fn id(&self) -> String {
        "BeaconValidatorGenerator".to_string()
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

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = self.block_root.get(witness);
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt.block_on(async {
            match &self.input {
                BeaconValidatorGeneratorInput::IndexConst(idx) => self
                    .client
                    .get_validator(hex!(block_root), *idx)
                    .await
                    .expect("failed to get validator"),
                BeaconValidatorGeneratorInput::IndexVariable(idx) => {
                    let idx = idx.get(witness).as_u64();
                    self.client
                        .get_validator(hex!(block_root), idx)
                        .await
                        .expect("failed to get validator")
                }
                BeaconValidatorGeneratorInput::PubkeyVariable(pubkey) => {
                    let pubkey = hex!(pubkey.get(witness));
                    self.client
                        .get_validator_by_pubkey(hex!(block_root), pubkey)
                        .await
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
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}
