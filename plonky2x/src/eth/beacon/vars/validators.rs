use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{Bytes32Variable, CircuitVariable};

/// The value type for `BeaconValidatorsVariable`. Note that this struct does not have a natural
/// representation of the beacon validators. Instead it stores commitments to the underlying
/// data and some auxiliary information, such as the block root. This unnatural respresentation
/// is why we don't store the struct in the `ethutils` package.
#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorsValue {
    pub block_root: H256,
    pub validators_root: H256,
}

/// The container which holds all beacon validators at specific block root as variable in the
/// circuit. Note that under the hood, we only store the commitment to the validators. To access
/// the underlying data, we witness merkle proofs.
#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorsVariable {
    pub block_root: Bytes32Variable,
    pub validators_root: Bytes32Variable,
}

impl CircuitVariable for BeaconValidatorsVariable {
    type ValueType = BeaconValidatorsValue;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            validators_root: Bytes32Variable::init(builder),
            block_root: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        Self {
            block_root: Bytes32Variable::constant(builder, value.block_root),
            validators_root: Bytes32Variable::constant(builder, value.validators_root),
        }
    }

    fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        targets.extend(self.validators_root.targets());
        targets
    }

    #[allow(unused_variables)]
    fn from_targets(targets: &[Target]) -> Self {
        let mut ptr = 0;
        let block_root = Bytes32Variable::from_targets(&targets[ptr..ptr + 256]);
        ptr += 256;
        let validators_root = Bytes32Variable::from_targets(&targets[ptr..ptr + 256]);
        Self {
            block_root,
            validators_root,
        }
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        BeaconValidatorsValue {
            block_root: self.block_root.value(witness),
            validators_root: self.validators_root.value(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        self.validators_root.set(witness, value.validators_root);
        self.block_root.set(witness, value.block_root);
    }
}
