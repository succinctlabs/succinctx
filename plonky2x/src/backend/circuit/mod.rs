pub mod io;
pub mod serialization;

use std::fs;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::{
    Buffer, GateSerializer, IoResult, Read, WitnessGeneratorSerializer, Write,
};

use self::io::{CircuitInput, CircuitOutput};
use crate::frontend::builder::io::{EvmIO, FieldIO};
use crate::frontend::builder::CircuitIO;
use crate::prelude::{ByteVariable, CircuitVariable, Variable};
use crate::utils::hex;

/// A compiled circuit which can compute any function in the form `f(x)=y`.
#[derive(Debug)]
pub struct Circuit<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub data: CircuitData<F, C, D>,
    pub io: CircuitIO<D>,
}

impl<F: RichField + Extendable<D>, C, const D: usize> Circuit<F, C, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
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
            vec![]
        };
        assert_eq!(input_variables.len(), input.buffer.len());

        // Assign input variables.
        let mut pw = PartialWitness::new();
        for i in 0..input_variables.len() {
            input_variables[i].set(&mut pw, input.buffer[i]);
        }

        // Generate the proof.
        let proof = self.data.prove(pw).unwrap();

        // Slice the public inputs to reflect the output portion of the circuit.
        let output = CircuitOutput {
            io: self.io.clone(),
            buffer: proof.public_inputs[input_variables.len()..].to_vec(),
        };

        (proof, output)
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

    pub fn id(&self) -> String {
        let circuit_digest = hex!(self
            .data
            .verifier_only
            .circuit_digest
            .elements
            .iter()
            .flat_map(|e| e.to_canonical_u64().to_be_bytes())
            .collect::<Vec<u8>>());
        circuit_digest[0..22].to_string()
    }

    pub fn serialize(
        &self,
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) -> IoResult<Vec<u8>> {
        // Setup buffer.
        let mut buffer = Vec::new();
        let circuit_bytes = self.data.to_bytes(gate_serializer, generator_serializer)?;
        buffer.write_usize(circuit_bytes.len())?;
        buffer.write_all(&circuit_bytes)?;

        if self.io.evm.is_some() {
            let io = self.io.evm.as_ref().unwrap();
            buffer.write_usize(0)?;
            buffer.write_target_vec(
                io.input_bytes
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect_vec()
                    .as_slice(),
            )?;

            buffer.write_target_vec(
                io.output_bytes
                    .iter()
                    .flat_map(|b| b.targets())
                    .collect_vec()
                    .as_slice(),
            )?;
        } else if self.io.field.is_some() {
            let io = self.io.field.as_ref().unwrap();
            buffer.write_usize(1)?;
            buffer.write_target_vec(
                io.input_variables
                    .iter()
                    .map(|v| v.0)
                    .collect_vec()
                    .as_slice(),
            )?;
            buffer.write_target_vec(
                io.output_variables
                    .iter()
                    .map(|v| v.0)
                    .collect_vec()
                    .as_slice(),
            )?;
        } else {
            buffer.write_usize(2)?;
        }

        Ok(buffer)
    }

    pub fn deserialize(
        buffer: &[u8],
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) -> IoResult<Self> {
        // Setup buffer.
        let mut buffer = Buffer::new(buffer);

        // Read circuit data from bytes.
        let circuit_bytes_len = buffer.read_usize()?;
        let mut circuit_bytes = vec![0u8; circuit_bytes_len];
        buffer.read_exact(circuit_bytes.as_mut_slice())?;
        let data = CircuitData::<F, C, D>::from_bytes(
            &circuit_bytes,
            gate_serializer,
            generator_serializer,
        )?;

        let mut circuit = Circuit {
            data,
            io: CircuitIO::new(),
        };

        let io_type = buffer.read_usize()?;
        if io_type == 0 {
            let input_targets = buffer.read_target_vec()?;
            let output_targets = buffer.read_target_vec()?;
            let input_bytes = (0..input_targets.len() / 8)
                .map(|i| ByteVariable::from_targets(&input_targets[i * 8..(i + 1) * 8]))
                .collect_vec();
            let output_bytes = (0..output_targets.len() / 8)
                .map(|i| ByteVariable::from_targets(&output_targets[i * 8..(i + 1) * 8]))
                .collect_vec();
            circuit.io.evm = Some(EvmIO {
                input_bytes,
                output_bytes,
            });
        } else if io_type == 1 {
            let input_targets = buffer.read_target_vec()?;
            let output_targets = buffer.read_target_vec()?;
            circuit.io.field = Some(FieldIO {
                input_variables: input_targets.into_iter().map(Variable).collect_vec(),
                output_variables: output_targets.into_iter().map(Variable).collect_vec(),
            });
        }

        Ok(circuit)
    }

    pub fn save(
        &self,
        path: &String,
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) {
        let bytes = self
            .serialize(gate_serializer, generator_serializer)
            .unwrap();
        fs::write(path, bytes).unwrap();
    }

    pub fn load(
        path: &str,
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) -> IoResult<Self> {
        let bytes = fs::read(path).unwrap();
        Self::deserialize(bytes.as_slice(), gate_serializer, generator_serializer)
    }

    pub fn save_to_build_dir(
        &self,
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) {
        let path = format!("./build/{}.circuit", self.id());
        self.save(&path, gate_serializer, generator_serializer);
    }

    pub fn load_from_build_dir(
        circuit_id: String,
        gate_serializer: &impl GateSerializer<F, D>,
        generator_serializer: &impl WitnessGeneratorSerializer<F, D>,
    ) -> IoResult<Self> {
        let path = format!("./build/{}.circuit", circuit_id);
        Self::load(&path, gate_serializer, generator_serializer)
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::backend::circuit::serialization::{GateRegistry, WitnessGeneratorRegistry};
    use crate::backend::circuit::Circuit;
    use crate::frontend::builder::CircuitBuilderX;
    use crate::prelude::*;

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_serialize_with_field_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build::<C>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Setup serializers
        let gate_serializer = GateRegistry::<F, D>::new();
        let generator_serializer = WitnessGeneratorRegistry::<F, D>::new::<C>();

        // Serialize.
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();
        let old_output_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();

        // Deserialize.
        let circuit =
            Circuit::<F, C, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let new_digest = circuit.data.verifier_only.circuit_digest;
        let new_input_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();
        let new_output_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();

        // Perform some sanity checks.
        assert_eq!(old_digest, new_digest);
        assert_eq!(old_input_variables.len(), new_input_variables.len());
        assert_eq!(old_output_variables.len(), new_output_variables.len());
        for i in 0..old_input_variables.len() {
            assert_eq!(old_input_variables[i].0, new_input_variables[i].0);
        }
        for i in 0..old_output_variables.len() {
            assert_eq!(old_output_variables[i].0, new_output_variables[i].0);
        }
    }

    #[test]
    fn test_serialize_with_evm_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Build your circuit.
        let circuit = builder.build::<C>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(1u8);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Setup serializers
        let gate_serializer = GateRegistry::<F, D>::new();
        let generator_serializer = WitnessGeneratorRegistry::<F, D>::new::<C>();

        // Serialize.
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_bytes = circuit.io.evm.as_ref().unwrap().input_bytes.clone();
        let old_output_bytes = circuit.io.evm.as_ref().unwrap().output_bytes.clone();

        // Deserialize.
        let circuit =
            Circuit::<F, C, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let new_digest = circuit.data.verifier_only.circuit_digest;
        let new_input_bytes = circuit.io.evm.as_ref().unwrap().input_bytes.clone();
        let new_output_bytes = circuit.io.evm.as_ref().unwrap().output_bytes.clone();

        // Perform some sanity checks.
        assert_eq!(old_digest, new_digest);
        assert_eq!(old_input_bytes.len(), new_input_bytes.len());
        assert_eq!(old_output_bytes.len(), new_output_bytes.len());
        for i in 0..old_input_bytes.len() {
            let old_targets = old_input_bytes[i].targets();
            let new_targets = new_input_bytes[i].targets();
            assert_eq!(old_targets.len(), new_targets.len());
            for j in 0..old_targets.len() {
                assert_eq!(old_targets[j], new_targets[j]);
            }
        }
        for i in 0..old_output_bytes.len() {
            let old_targets = old_output_bytes[i].targets();
            let new_targets = new_output_bytes[i].targets();
            assert_eq!(old_targets.len(), new_targets.len());
            for j in 0..old_targets.len() {
                assert_eq!(old_targets[j], new_targets[j]);
            }
        }
    }
}
