use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, SSZVariable};
use crate::prelude::{ByteVariable, Variable};
use crate::utils::bytes32;

#[derive(Debug, Copy, Clone, CircuitVariable)]
#[value_name(BeaconHeaderValue)]
pub struct BeaconHeaderVariable {
    pub slot: U64Variable,
    pub proposer_index: U64Variable,
    pub parent_root: Bytes32Variable,
    pub state_root: Bytes32Variable,
    pub body_root: Bytes32Variable,
}

impl SSZVariable for BeaconHeaderVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let zero = builder.constant::<ByteVariable>(0);

        let slot_leaf = self.slot.hash_tree_root(builder);
        let proposer_index_leaf = self.proposer_index.hash_tree_root(builder);
        let parent_root_leaf = self.parent_root.hash_tree_root(builder);
        let state_root_leaf = self.state_root.hash_tree_root(builder);
        let body_root_leaf = self.body_root.hash_tree_root(builder);

        // ab, cd, ef, gh (f/g/h are zero)
        let zero_bytes = vec![zero; 32];

        let mut ab_input = Vec::new();
        ab_input.extend(slot_leaf.0 .0);
        ab_input.extend(proposer_index_leaf.0 .0);
        let ab = builder.curta_sha256(&ab_input);

        let mut cd_input = Vec::new();
        cd_input.extend(parent_root_leaf.0 .0);
        cd_input.extend(state_root_leaf.0 .0);
        let cd = builder.curta_sha256(&cd_input);

        let mut ef_input = Vec::new();
        ef_input.extend(body_root_leaf.0 .0);
        ef_input.extend(zero_bytes);
        let ef = builder.curta_sha256(&ef_input);

        // sha256(00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000)
        let gh = Bytes32Variable::constant(
            builder,
            bytes32!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b"),
        );

        let mut abcd_input = Vec::new();
        abcd_input.extend(ab.0 .0);
        abcd_input.extend(cd.0 .0);
        let abcd = builder.curta_sha256(&abcd_input);

        let mut efgh_input = Vec::new();
        efgh_input.extend(ef.0 .0);
        efgh_input.extend(gh.0 .0);
        let efgh = builder.curta_sha256(&efgh_input);

        let mut full_input = Vec::new();
        full_input.extend(abcd.0 .0);
        full_input.extend(efgh.0 .0);
        builder.curta_sha256(&full_input)
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use ethers::types::U64;

    use super::*;
    use crate::prelude::{CircuitBuilder, DefaultParameters, PlonkParameters};
    use crate::utils::eth::beacon::BeaconClient;

    type L = DefaultParameters;
    const D: usize = 2;

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_beacon_header_hash_tree_root() {
        dotenv::dotenv().ok();
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let mut builder = CircuitBuilder::<L, D>::new();
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());
        let header = client.get_header("7404237".to_string()).await.unwrap();
        let beacon_header = BeaconHeaderValue::<<L as PlonkParameters<D>>::Field> {
            slot: U64::from_dec_str(header.slot.as_str()).unwrap().as_u64(),
            proposer_index: U64::from_dec_str(header.proposer_index.as_str())
                .unwrap()
                .as_u64(),
            parent_root: bytes32!(header.parent_root),
            state_root: bytes32!(header.state_root),
            body_root: bytes32!(header.body_root),
        };
        let beacon_header_var = builder.constant::<BeaconHeaderVariable>(beacon_header);
        let hash_tree_root = beacon_header_var.hash_tree_root(&mut builder);
        builder.watch(&hash_tree_root, "hash_tree_root");
        let expected_root =
            bytes32!("0xd19b3ce6089e8451233ac1f75788ea7ad593a6da010b857fc888ce869699f451");
        let expected_root_var = builder.constant::<Bytes32Variable>(expected_root);
        builder.assert_is_equal(hash_tree_root, expected_root_var);
        let circuit = builder.mock_build();
        let input = circuit.input();
        circuit.mock_prove(&input);
    }
}
