//! An implementation of the keccak256 hash functions in a plonky2 circuit

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use self::keccak256::Keccak256Generator;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder, Variable};

pub mod keccak256;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn keccak256(&mut self, bytes: &[ByteVariable]) -> Bytes32Variable {
        let generator = Keccak256Generator {
            input: bytes.to_vec(),
            output: self.init(),
            length: None,
            _phantom: Default::default(),
        };
        self.add_simple_generator(&generator);
        generator.output
    }

    pub fn keccak256_variable(
        &mut self,
        bytes: &[ByteVariable],
        length: Variable,
    ) -> Bytes32Variable {
        let generator = Keccak256Generator {
            input: bytes.to_vec(),
            output: self.init(),
            length: Some(length),
            _phantom: Default::default(),
        };
        self.add_simple_generator(&generator);
        generator.output
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::prelude::CircuitBuilder;
    use crate::utils::bytes32;

    #[test]
    fn test_keccak256() {
        env_logger::init();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let word = builder.constant::<Bytes32Variable>(bytes32!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        ));
        let hash = builder.keccak256(&word.0 .0);
        builder.watch(&hash, "hi");

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);
    }
}
