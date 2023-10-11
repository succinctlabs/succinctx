use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable};
use crate::prelude::{ByteVariable, Variable};

#[derive(Debug, Copy, Clone, CircuitVariable)]
#[value_name(BeaconWithdrawalValue)]
pub struct BeaconWithdrawalVariable {
    pub index: U64Variable,
    pub validator_index: U64Variable,
    pub address: AddressVariable,
    pub amount: U256Variable,
}

impl SSZVariable for BeaconWithdrawalVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let zero = builder.constant::<ByteVariable>(0);

        let mut index_bytes = self.index.encode(builder);
        index_bytes.reverse();
        index_bytes.extend([zero; 24]);

        let mut validator_index_bytes = self.validator_index.encode(builder);
        validator_index_bytes.reverse();
        validator_index_bytes.extend([zero; 24]);

        let mut address_bytes = self.address.encode(builder);
        address_bytes.extend([zero; 12]);

        let mut amount_bytes = self.amount.encode(builder);
        amount_bytes.reverse();

        let mut ab_input = Vec::new();
        ab_input.extend(index_bytes);
        ab_input.extend(validator_index_bytes);
        let ab = builder.curta_sha256(&ab_input);

        let mut cd_input = Vec::new();
        cd_input.extend(address_bytes);
        cd_input.extend(amount_bytes);
        let cd = builder.curta_sha256(&cd_input);

        let mut input = Vec::new();
        input.extend(ab.0 .0);
        input.extend(cd.0 .0);
        builder.curta_sha256(&input)
    }
}
