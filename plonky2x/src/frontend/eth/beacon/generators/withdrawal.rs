use core::marker::PhantomData;
use std::env;

use array_macro::array;
use ethers::types::{Address, U256};
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::vars::{
    BeaconWithdrawalValue, BeaconWithdrawalVariable, BeaconWithdrawalsVariable,
};
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 5;

#[derive(Debug, Clone)]
pub struct BeaconWithdrawalGenerator<L: PlonkParameters<D>, const D: usize> {
    client: BeaconClient,
    withdrawals: BeaconWithdrawalsVariable,
    idx: U64Variable,
    pub withdrawal_root: Bytes32Variable,
    pub withdrawal: BeaconWithdrawalVariable,
    pub proof: [Bytes32Variable; DEPTH],
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> BeaconWithdrawalGenerator<L, D> {
    pub fn new(
        builder: &mut CircuitBuilder<L, D>,
        client: BeaconClient,
        withdrawals: BeaconWithdrawalsVariable,
        idx: U64Variable,
    ) -> Self {
        Self {
            client,
            withdrawals,
            idx,
            withdrawal_root: builder.init::<Bytes32Variable>(),
            withdrawal: builder.init::<BeaconWithdrawalVariable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: Default::default(),
        }
    }

    pub fn id() -> String {
        "BeaconWithdrawalGenerator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for BeaconWithdrawalGenerator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.withdrawals.targets());
        targets.extend(self.idx.targets());
        targets
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let block_root = self.withdrawals.block_root.get(witness);
        let idx = self.idx.get(witness);

        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt.block_on(async {
            self.client
                .get_withdrawal(hex!(block_root.as_bytes()).to_string(), idx.as_u64())
                .await
                .expect("failed to get validators root")
        });

        println!("{}", result.withdrawal.amount);
        let withdrawal = BeaconWithdrawalValue {
            index: result.withdrawal.index.into(),
            validator_index: result.withdrawal.validator_index.into(),
            address: result.withdrawal.address.parse::<Address>().unwrap(),
            amount: U256::from_dec_str(result.withdrawal.amount.to_string().as_str()).unwrap(),
        };

        self.withdrawal.set(out_buffer, withdrawal);
        self.withdrawal_root
            .set(out_buffer, bytes32!(result.withdrawal_root));
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
        dst.write_target_vec(&self.withdrawals.targets())?;
        dst.write_target_vec(&self.idx.targets())?;
        dst.write_target_vec(&self.withdrawal_root.targets())?;
        dst.write_target_vec(&self.withdrawal.targets())?;
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
        let withdrawals = BeaconWithdrawalsVariable::from_targets(&src.read_target_vec()?);
        let idx = U64Variable::from_targets(&src.read_target_vec()?);
        let withdrawal_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let withdrawal = BeaconWithdrawalVariable::from_targets(&src.read_target_vec()?);
        let mut proof = Vec::new();
        for i in 0..DEPTH {
            proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        Ok(Self {
            client,
            withdrawals,
            idx,
            withdrawal_root,
            withdrawal,
            proof: proof.try_into().unwrap(),
            _phantom: Default::default(),
        })
    }
}
