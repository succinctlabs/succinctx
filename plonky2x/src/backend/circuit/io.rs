use curta::math::prelude::PrimeField64;
use itertools::Itertools;

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable};

/// A circuit input. Write to the input using `write` and `evm_write`.
pub struct CircuitInput<L: PlonkParameters<D>, const D: usize> {
    pub io: CircuitIO<D>,
    pub buffer: Vec<L::Field>,
}

/// A circuit output. Read from the output using `read` `evm_read`.
pub struct CircuitOutput<L: PlonkParameters<D>, const D: usize> {
    pub io: CircuitIO<D>,
    pub buffer: Vec<L::Field>,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitInput<L, D> {
    /// Writes a value to the public circuit input using field-based serialization.
    pub fn write<V: CircuitVariable>(&mut self, value: V::ValueType<L::Field>) {
        match &self.io {
            CircuitIO::Field(_) => {
                self.buffer.extend(V::elements::<L, D>(value));
            }
            _ => panic!("field io is not enabled"),
        };
    }

    /// Writes a stream of field elements to the public circuit input.
    pub fn write_all(&mut self, value: &[L::Field]) {
        match &self.io {
            CircuitIO::Field(_) => {
                self.buffer.extend(value);
            }
            _ => panic!("field io is not enabled"),
        };
    }

    /// Writes a value to the public circuit input using byte-based serialization (i.e., abi
    /// encoded types).
    pub fn evm_write<V: EvmVariable>(&mut self, value: V::ValueType<L::Field>) {
        match &self.io {
            CircuitIO::Evm(_) => {
                let bytes = V::encode_value(value);
                let elements: Vec<L::Field> = bytes
                    .into_iter()
                    .flat_map(|b| ByteVariable::elements::<L, D>(b))
                    .collect();
                self.buffer.extend(elements);
            }
            _ => panic!("evm io is not enabled"),
        };
    }

    /// Writes a stream of bytes to the public circuit input. Assumes that the bytes can be
    /// properly deserialized.
    pub fn evm_write_all(&mut self, bytes: &[u8]) {
        match &self.io {
            CircuitIO::Evm(_) => {
                let elements: Vec<L::Field> = bytes
                    .iter()
                    .flat_map(|b| ByteVariable::elements::<L, D>(*b))
                    .collect();
                self.buffer.extend(elements);
            }
            _ => panic!("evm io is not enabled"),
        };
    }

    /// Sets a value to the circuit input. This method only works if the circuit is using
    /// field element-based IO.
    pub fn set<V: CircuitVariable>(&mut self, _: V, _: V::ValueType<L::Field>) {
        todo!()
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitOutput<L, D> {
    /// Reads a value from the public circuit output using field-based serialization.
    pub fn read<V: CircuitVariable>(&mut self) -> V::ValueType<L::Field> {
        match &self.io {
            CircuitIO::Field(_) => {
                let elements = self.buffer.drain(0..V::nb_elements()).collect_vec();
                V::from_elements::<L, D>(&elements)
            }
            _ => panic!("field io is not enabled"),
        }
    }

    /// Reads the entire stream of field elements from the public circuit output.
    pub fn read_all(&self) -> Vec<L::Field> {
        match &self.io {
            CircuitIO::Field(_) => self.buffer.clone(),
            _ => panic!("field io is not enabled"),
        }
    }

    /// Reads a value from the public circuit output using byte-based serialization.
    pub fn evm_read<V: EvmVariable>(&mut self) -> V::ValueType<L::Field> {
        match &self.io {
            CircuitIO::Evm(_) => {
                let nb_bytes = V::nb_bytes::<L, D>();
                let bits = self.buffer.drain(0..nb_bytes * 8).collect_vec();
                let mut bytes = Vec::new();
                for i in 0..bits.len() / 8 {
                    let mut byte = 0u8;
                    for j in 0..8 {
                        byte |= (bits[i * 8 + j].as_canonical_u64() << (7 - j)) as u8;
                    }
                    bytes.push(byte);
                }
                V::decode_value(bytes.as_slice())
            }
            _ => panic!("evm io is not enabled"),
        }
    }

    /// Reads the entire stream of bytes from the public circuit output.
    pub fn evm_read_all(&self) -> Vec<u8> {
        match &self.io {
            CircuitIO::Evm(_) => {
                let bits = self.buffer.iter().collect_vec();
                let mut bytes = Vec::new();
                for i in 0..bits.len() / 8 {
                    let mut byte = 0u8;
                    for j in 0..8 {
                        byte |= (bits[i * 8 + j].as_canonical_u64() << (7 - j)) as u8;
                    }
                    bytes.push(byte);
                }
                bytes
            }
            _ => panic!("evm io is not enabled"),
        }
    }

    /// Reads a value from the circuit output. It also can access the value of any intermediate
    /// variable in the circuit.
    pub fn get<V: CircuitVariable>(&self, _: V) -> V::ValueType<L::Field> {
        todo!()
    }
}
