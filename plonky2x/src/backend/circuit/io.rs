use std::fs::File;
use std::io::{Read, Write};

use curta::math::prelude::PrimeField64;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::Circuit;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable};

/// A circuit input. Write to the input using `write` and `evm_write`.
#[derive(Debug, Clone)]
pub struct CircuitInput<F, C, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    pub io: CircuitIO<D>,
    pub buffer: Vec<F>,
    pub proofs: Vec<ProofWithPublicInputs<F, C, D>>,
}

/// A circuit output. Read from the output using `read` `evm_read`.
#[derive(Debug, Clone)]
pub struct CircuitOutput<F, C, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    pub io: CircuitIO<D>,
    pub buffer: Vec<F>,
    pub proofs: Vec<ProofWithPublicInputs<F, C, D>>,
}

/// A trait that implements methods for loading and save proofs from disk.
pub trait ProofWithPublicInputsSerializable<F: RichField + Extendable<D>, const D: usize> {
    fn serialize_to_json(&self) -> String;
    fn deserialize_from_json(data: String) -> Self;
    fn save(&self, path: &str);
    fn load(&self, path: &str) -> Self;
}

impl<F, C, const D: usize> CircuitInput<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
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
            .iter()
            .flat_map(|b| ByteVariable::elements(*b))
            .collect();
        self.buffer.extend(elements);
    }

    /// Writes a proof to the public circuit input.
    pub fn proof_write(&mut self, proof: ProofWithPublicInputs<F, C, D>) {
        self.proofs.push(proof);
    }

    /// Writes a batch of proofs to the public circuit input.
    pub fn proof_write_all(&mut self, proofs: &[ProofWithPublicInputs<F, C, D>]) {
        self.proofs.extend_from_slice(proofs);
    }

    /// Sets a value to the circuit input. This method only works if the circuit is using
    /// field element-based IO.
    pub fn set<V: CircuitVariable>(&mut self, _: V, _: V::ValueType<F>) {
        todo!()
    }

    /// Serializes the input buffer to a vector of elements encoded as base 10 strings.
    pub fn serialize_to_json(&self) -> String {
        let buffer: Vec<String> = self
            .buffer
            .iter()
            .map(|x| x.as_canonical_u64().to_string())
            .collect();
        serde_json::to_string_pretty(&buffer).unwrap()
    }

    /// Deserializes the input buffer form a vector of elements encoded as base 10 strings.
    pub fn deserialize_from_json(&mut self, data: String) {
        let buffer: Vec<String> = serde_json::from_str(&data).unwrap();
        self.buffer = buffer
            .iter()
            .map(|x| F::from_canonical_u64(x.parse().unwrap()))
            .collect();
    }

    /// Saves the input buffer to a file.
    pub fn save(&self, path: &str) {
        let json = self.serialize_to_json();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    /// Loads the input buffer from a file.
    pub fn load(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        self.deserialize_from_json(data);
    }
}

impl<F, C, const D: usize> CircuitOutput<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    /// Reads a value from the public circuit output using field-based serialization.
    pub fn read<V: CircuitVariable>(&self) -> V::ValueType<F> {
        self.io.field.as_ref().expect("field io is not enabled");
        let elements = self
            .buffer
            .iter()
            .take(V::nb_elements::<F, D>())
            .collect_vec();
        V::from_elements(elements.into_iter().copied().collect_vec().as_slice())
    }

    /// Reads the entire stream of field elements from the public circuit output.
    pub fn read_all(&self) -> Vec<F> {
        self.io.field.as_ref().expect("field io is not enabled");
        self.buffer.clone()
    }

    /// Reads a value from the public circuit output using byte-based serialization.
    pub fn evm_read<V: EvmVariable>(&self) -> V::ValueType<F> {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let nb_bytes = V::nb_bytes::<F, D>();
        let bits = self.buffer.iter().take(nb_bytes * 8).collect_vec();
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
    pub fn evm_read_all(&self) -> Vec<u8> {
        self.io.evm.as_ref().expect("evm io is not enabled");
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

    /// Reads a proof from the public circuit output.
    pub fn proof_read(self) -> ProofWithPublicInputs<F, C, D> {
        self.proofs.into_iter().take(1).collect_vec()[0].to_owned()
    }

    /// Reads a batch of proofs from the public circuit output.
    pub fn proof_read_all(self) -> Vec<ProofWithPublicInputs<F, C, D>> {
        self.proofs
    }

    /// Reads a value from the circuit output. It also can access the value of any intermediate
    /// variable in the circuit.
    pub fn get<V: CircuitVariable>(&self, _: V) -> V::ValueType<F> {
        todo!()
    }

    /// Serializes the output buffer to json.
    pub fn serialize_to_json(&self) -> String {
        let buffer: Vec<String> = self
            .buffer
            .iter()
            .map(|x| x.as_canonical_u64().to_string())
            .collect();
        serde_json::to_string_pretty(&buffer).unwrap()
    }

    /// Deserializes the output buffer from json.
    pub fn deserialize_from_json(circuit: &Circuit<F, C, D>, data: String) -> Self
    where
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        println!("step1");
        let buffer: Vec<u64> = serde_json::from_str(&data).unwrap();
        println!("step2");
        let output = CircuitOutput {
            io: circuit.io.clone(),
            buffer: buffer.iter().map(|x| F::from_canonical_u64(*x)).collect(),
            proofs: Vec::new(),
        };
        output
    }

    /// Saves the output buffer to a file.
    pub fn save(&self, path: &str) {
        let json = self.serialize_to_json();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

impl<F: RichField + Extendable<D>, C, const D: usize> ProofWithPublicInputsSerializable<F, D>
    for ProofWithPublicInputs<F, C, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn serialize_to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn deserialize_from_json(data: String) -> Self {
        serde_json::from_str(&data).unwrap()
    }

    fn save(&self, path: &str) {
        let json = self.serialize_to_json();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn load(&self, path: &str) -> Self {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}
