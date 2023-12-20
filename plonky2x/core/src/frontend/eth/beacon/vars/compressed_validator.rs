use plonky2::hash::hash_types::RichField;

use crate::frontend::vars::{EvmVariable, SSZVariable};
use crate::prelude::{
    bytes32, BoolVariable, Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters,
    U256Variable, Variable,
};

const ZERO_BYTE32: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, CircuitVariable)]
#[value_name(CompressedBeaconValidatorValue)]
pub struct CompressedBeaconValidatorVariable {
    pub is_zero_validator: BoolVariable,
    pub pubkey: Bytes32Variable,
    pub withdrawal_credentials: Bytes32Variable,
    pub h1: Bytes32Variable,
    pub h2: Bytes32Variable,
    pub exit_epoch: U256Variable,
    pub withdrawable_epoch: U256Variable,
}

impl SSZVariable for CompressedBeaconValidatorVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let tmp = self.pubkey;
        let mut a1 = tmp.0 .0.to_vec();
        a1.extend(self.withdrawal_credentials.0 .0.to_vec());

        let mut a4 = self.exit_epoch.encode(builder);
        a4.reverse();
        let mut tmp = self.withdrawable_epoch.encode(builder);
        tmp.reverse();
        a4.extend(&tmp);

        // h(pubkey, withdrawalCredentials) | h(effectiveBalance, slashed)
        let mut b1 = builder.curta_sha256(&a1).0 .0.to_vec();
        b1.extend(self.h1.0 .0.to_vec());

        // h(activationEligibilityEpoch, activationEpoch) | h(exitEpoch, withdrawableEpoch)
        let mut b2 = self.h2.0 .0.to_vec();
        b2.extend(builder.curta_sha256(&a4).0 .0.to_vec());

        let mut c1 = builder.curta_sha256(&b1).0 .0.to_vec();
        c1.extend(builder.curta_sha256(&b2).0 .0.to_vec());

        let leaf = builder.curta_sha256(&c1);
        let zero_leaf = builder.constant::<Bytes32Variable>(bytes32!(ZERO_BYTE32));

        builder.select(self.is_zero_validator, zero_leaf, leaf)
    }
}
