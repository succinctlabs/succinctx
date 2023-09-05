use std::fmt::Debug;

use ethers::types::{H160, U256, U64};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable};
use crate::prelude::{ByteVariable, Variable};

#[derive(Debug, Clone)]
pub struct BeaconWithdrawalValue {
    pub index: U64,
    pub validator_index: U64,
    pub address: H160,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub struct BeaconWithdrawalVariable {
    pub index: U64Variable,
    pub validator_index: U64Variable,
    pub address: AddressVariable,
    pub amount: U256Variable,
}

impl CircuitVariable for BeaconWithdrawalVariable {
    type ValueType<F: RichField> = BeaconWithdrawalValue;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            index: U64Variable::init(builder),
            validator_index: U64Variable::init(builder),
            address: AddressVariable::init(builder),
            amount: U256Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self {
            index: U64Variable::constant(builder, value.index),
            validator_index: U64Variable::constant(builder, value.validator_index),
            address: AddressVariable::constant(builder, value.address),
            amount: U256Variable::constant(builder, value.amount),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut variables = Vec::new();
        variables.extend(self.index.variables());
        variables.extend(self.validator_index.variables());
        variables.extend(self.address.variables());
        variables.extend(self.amount.variables());
        variables
    }

    fn from_variables(variables: &[Variable]) -> Self {
        let index = U64Variable::from_variables(&variables[0..2]);
        let validator_index = U64Variable::from_variables(&variables[2..4]);
        let address = AddressVariable::from_variables(&variables[4..164]);
        let amount = U256Variable::from_variables(&variables[164..172]);
        Self {
            index,
            validator_index,
            address,
            amount,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconWithdrawalValue {
            index: self.index.get(witness),
            validator_index: self.validator_index.get(witness),
            address: self.address.get(witness),
            amount: self.amount.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.index.set(witness, value.index);
        self.validator_index.set(witness, value.validator_index);
        self.address.set(witness, value.address);
        self.amount.set(witness, value.amount);
    }
}

impl SSZVariable for BeaconWithdrawalVariable {
    fn hash_tree_root<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
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
        let ab = builder.sha256(&ab_input);

        let mut cd_input = Vec::new();
        cd_input.extend(address_bytes);
        cd_input.extend(amount_bytes);
        let cd = builder.sha256(&cd_input);

        let mut input = Vec::new();
        input.extend(ab.0 .0);
        input.extend(cd.0 .0);
        builder.sha256(&input)
    }
}
