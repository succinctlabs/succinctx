use anyhow::Result;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{Target, BoolTarget};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};


struct Variable {
    value: Target
}

struct BoolVariable {
    value: Target
}

struct ByteVariable {
    value: Target
}

// impl Variable {
//     fn NewVariable() -> Variable {
//         return Variable {
//             value: Target::VirtualTarget { 0 }
//         }
//     }
// }

struct API {
    builder: CircuitBuilder<GoldilocksField, 2>,
}

impl API {
    fn NewAPI() -> API {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
    
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);
        return API {
            builder : builder,
        }
    }
}

struct MyCircuit {

}


struct SimpleSerializeAPI {
    api:  API
}


impl SimpleSerializeAPI {
    fn verify_proof<const DEPTH: usize, const GINDEX: usize>(
        self, 
        root: [ByteVariable;32],
        leaf: [ByteVariable;32],
        proof: [[ByteVariable;32]; DEPTH]
    ) -> Result<()> {
        // self.api.assertEqual(...)
        return Ok(());
    }
}

struct StorageProofAPI {
    api: API
    eth_rpc: ether
}

impl StorageProofAPI {
    fn verify_slot(
        self, 
        block_hash: [Variable;32],
        slot_key: [Variable;32], 
        slot_value: [Variable;32], 
        proof: [Variable;32]
    ) -> Result<()> {
        return Ok(());
    }

    fn get_verified_slot(
        self, 
        block_hash: [Variable;32],
        slot_key: [Variable;32],
    ) -> Result<>([Variable;32]) {
        // TODO get the witness
        self.verify_slot(block_hash, slot_key, slot_value, proof)
        return 
    }
}