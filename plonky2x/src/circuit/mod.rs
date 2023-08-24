use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::builder::CircuitIO;
use crate::prelude::CircuitVariable;
use crate::vars::{ByteSerializable, EvmVariable, FieldSerializable};

/// A compiled circuit which can compute any function in the form `f(x)=y`.
pub struct Circuit<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub data: CircuitData<F, C, D>,
    pub io: CircuitIO<D>,
}

/// A circuit input. Write to the input using `write` and `evm_write`.
pub struct CircuitInput<F: RichField + Extendable<D>, const D: usize> {
    io: CircuitIO<D>,
    buffer: Vec<F>,
}

/// A circuit output. Read from the output using `read` `evm_read`.
pub struct CircuitOutput<F: RichField + Extendable<D>, const D: usize> {
    io: CircuitIO<D>,
    buffer: Vec<F>,
}

impl<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> Circuit<F, C, D> {
    /// Returns an input instance for the circuit.
    pub fn input(&self) -> CircuitInput<F, D> {
        CircuitInput {
            io: self.io.clone(),
            buffer: Vec::new(),
        }
    }

    /// Generates a proof for the circuit. The proof can be verified using `verify`.
    pub fn prove(
        &self,
        input: &CircuitInput<F, D>,
    ) -> (ProofWithPublicInputs<F, C, D>, CircuitOutput<F, D>) {
        // Get input variables from io.
        let input_variables = if self.io.evm.is_some() {
            self.io
                .evm
                .clone()
                .unwrap()
                .input_bytes
                .into_iter()
                .flat_map(|b| b.variables())
                .collect()
        } else if self.io.field.is_some() {
            self.io.field.clone().unwrap().input_variables
        } else {
            todo!()
        };
        assert_eq!(input_variables.len(), input.buffer.len());

        // Assign input variables.
        let mut pw = PartialWitness::new();
        for i in 0..input_variables.len() {
            input_variables[i].set(&mut pw, input.buffer[i].into());
        }

        // Generate the proof.
        let proof = self.data.prove(pw).unwrap();

        // Slice the public inputs to reflect the output portion of the circuit.
        let output = CircuitOutput {
            io: self.io.clone(),
            buffer: proof.public_inputs[input_variables.len()..].to_vec(),
        };

        (proof.clone(), output)
    }

    /// Verifies a proof for the circuit.
    pub fn verify(
        &self,
        proof: &ProofWithPublicInputs<F, C, D>,
        input: &CircuitInput<F, D>,
        output: &CircuitOutput<F, D>,
    ) {
        let mut public_inputs = Vec::new();
        public_inputs.extend(input.buffer.clone());
        public_inputs.extend(output.buffer.clone());
        assert_eq!(public_inputs.len(), proof.public_inputs.len());
        for i in 0..public_inputs.len() {
            assert_eq!(public_inputs[i], proof.public_inputs[i]);
        }
        self.data.verify(proof.clone()).unwrap();
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitInput<F, D> {
    /// Writes a value to the public circuit input using field-based serialization.
    pub fn write<V: CircuitVariable>(&mut self, value: V::ValueType<F>) {
        self.io.field.as_ref().expect("field io is not enabled");
        self.buffer.extend(value.elements());
    }

    /// Writes a value to the public circuit input using byte-based serialization (i.e., abi
    /// encoded types).
    pub fn evm_write<V: EvmVariable>(&mut self, value: <V as EvmVariable>::ValueType<F>) {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let bytes = value.bytes();
        let elements: Vec<F> = bytes
            .into_iter()
            .flat_map(|b| <u8 as FieldSerializable<F>>::elements(&b))
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
            .take(V::ValueType::<F>::nb_elements())
            .collect();
        V::ValueType::<F>::from_elements(elements.as_slice())
    }

    /// Reads a value from the public circuit output using byte-based serialization.
    pub fn evm_read<V: EvmVariable>(self) -> <V as EvmVariable>::ValueType<F> {
        self.io.evm.as_ref().expect("evm io is not enabled");
        let bytes: Vec<u8> = self
            .buffer
            .into_iter()
            .take(<V as EvmVariable>::ValueType::<F>::nb_bytes())
            .map(|f| <u8 as FieldSerializable<F>>::from_elements(&[f]))
            .collect();
        <V as EvmVariable>::ValueType::<F>::from_bytes(bytes.as_slice())
    }

    /// Reads a value from the circuit output. It also can access the value of any intermediate
    /// variable in the circuit.
    pub fn get<V: CircuitVariable>(&self, _: V) -> V::ValueType<F> {
        todo!()
    }
}
