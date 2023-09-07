//! An implementation of the keccak256 hash functions in a plonky2 circuit

use self::keccak256::Keccak256Generator;
use crate::backend::config::PlonkParameters;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder};

pub mod keccak256;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn keccak256(&mut self, bytes: &[ByteVariable]) -> Bytes32Variable {
        let generator: Keccak256Generator<L, D> = Keccak256Generator {
            input: bytes.to_vec(),
            output: self.init(),
            length: None,
            _phantom: Default::default(),
        };
        let output = generator.output;
        self.add_simple_generator(generator);
        output
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::backend::config::DefaultParameters;
    use crate::prelude::CircuitBuilder;
    use crate::utils::bytes32;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_keccak256() {
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        let word = builder.constant::<Bytes32Variable>(bytes32!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        ));
        let hash = builder.keccak256(&word.0 .0);
        builder.watch(&hash, "hi");

        let circuit = builder.build();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);
    }
}
