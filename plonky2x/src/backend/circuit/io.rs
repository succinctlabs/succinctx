use curta::math::prelude::PrimeField64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable};

/// A circuit input. Write to the input using `write` and `evm_write`.
pub struct CircuitInput<F: RichField + Extendable<D>, const D: usize> {
    pub io: CircuitIO<D>,
    pub buffer: Vec<F>,
}

/// A circuit output. Read from the output using `read` `evm_read`.
pub struct CircuitOutput<F: RichField + Extendable<D>, const D: usize> {
    pub io: CircuitIO<D>,
    pub buffer: Vec<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitInput<F, D> {
    /// Writes a value to the public circuit input using field-based serialization.
    pub fn write<V: CircuitVariable>(&mut self, value: V::ValueType<F>) {
        self.io.field.as_ref().expect("field io is not enabled");
        self.buffer.extend(V::elements(value));
    }

    /// Writes a stream of field elements to the public circuit input.
    pub fn write_all(&mut self, value: &[F]) {
        self.io.field.as_ref().expect("field io is not enabled");
        self.buffer.extend(value);
    }

    /// Writes a value to the public circuit input using byte-based serialization (i.e., abi
    /// encoded types).
    pub fn evm_write<V: EvmVariable>(&mut self, value: V::ValueType<F>) {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let bytes = V::encode_value(value);
        let elements: Vec<F> = bytes
            .into_iter()
            .flat_map(|b| ByteVariable::elements(b))
            .collect();
        self.buffer.extend(elements);
    }

    /// Writes a stream of bytes to the public circuit input. Assumes that the bytes can be
    /// properly deserialized.
    pub fn evm_write_all(&mut self, bytes: &[u8]) {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let elements: Vec<F> = bytes
            .into_iter()
            .flat_map(|b| ByteVariable::elements(*b))
            .collect();
        self.buffer.extend(elements);
    }

    /// Sets a value to the circuit input. This method only works if the circuit is using
    /// field element-based IO.
    pub fn set<V: CircuitVariable>(&mut self, _: V, _: V::ValueType<F>) {
        todo!()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitOutput<F, D> {
    /// Reads a value from the public circuit output using field-based serialization.
    pub fn read<V: CircuitVariable>(self) -> V::ValueType<F> {
        self.io.field.as_ref().expect("field io is not enabled");
        let elements: Vec<F> = self
            .buffer
            .into_iter()
            .take(V::nb_elements::<F, D>())
            .collect();
        V::from_elements(elements.as_slice())
    }

    /// Reads the entire stream of field elements from the public circuit output.
    pub fn read_all(self) -> Vec<F> {
        self.io.field.as_ref().expect("field io is not enabled");
        self.buffer
    }

    /// Reads a value from the public circuit output using byte-based serialization.
    pub fn evm_read<V: EvmVariable>(self) -> V::ValueType<F> {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let nb_bytes = V::nb_bytes::<F, D>();
        let bits: Vec<F> = self.buffer.into_iter().take(nb_bytes * 8).collect();
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

    /// Reads the entire stream of bytes from the public circuit output.
    pub fn evm_read_all(self) -> Vec<u8> {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let bits: Vec<F> = self.buffer.into_iter().collect();
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

    /// Reads a value from the circuit output. It also can access the value of any intermediate
    /// variable in the circuit.
    pub fn get<V: CircuitVariable>(&self, _: V) -> V::ValueType<F> {
        todo!()
    }
}
