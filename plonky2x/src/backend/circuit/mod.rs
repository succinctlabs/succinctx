pub mod io;
pub mod utils;

use core::marker::PhantomData;
use std::fs;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::{Buffer, IoResult, Read, Remaining, Write};

use self::io::{CircuitInput, CircuitOutput};
use self::utils::{CustomGateSerializer, CustomGeneratorSerializer};
use crate::frontend::builder::io::{EvmIO, FieldIO, RecursiveProofIO};
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
    pub fn input(&self) -> CircuitInput<F, C, D> {
        CircuitInput {
            io: self.io.clone(),
            buffer: Vec::new(),
            proofs: Vec::new(),
        }
    }

    /// Returns an input instance for the circuit.
    pub fn output(&self) -> CircuitOutput<F, C, D> {
        CircuitOutput {
            io: self.io.clone(),
            buffer: Vec::new(),
            proofs: Vec::new(),
        }
    }

    /// Generates a proof for the circuit. The proof can be verified using `verify`.
    pub fn prove(
        &self,
        input: &CircuitInput<F, C, D>,
    ) -> (ProofWithPublicInputs<F, C, D>, CircuitOutput<F, C, D>) {
        let mut pw = PartialWitness::new();

        // Set the input variables.
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
        for i in 0..input_variables.len() {
            input_variables[i].set(&mut pw, input.buffer[i]);
        }

        // Set the recursive proof inputs.
        if self.io.recursive_proof.is_some() {
            let proofs = self.io.recursive_proof.as_ref().unwrap().proofs.clone();
            for i in 0..proofs.len() {
                pw.set_proof_with_pis_target(&proofs[i], &input.proofs[i]);
            }
        }

        // Generate the proof.
        let proof = self.data.prove(pw).unwrap();

        // Slice the public inputs to reflect the output portion of the circuit.
        let output = CircuitOutput {
            io: self.io.clone(),
            buffer: proof.public_inputs[input_variables.len()..].to_vec(),
            proofs: Vec::new(),
        };

        (proof, output)
    }

    /// Verifies a proof for the circuit.
    pub fn verify(
        &self,
        proof: &ProofWithPublicInputs<F, C, D>,
        input: &CircuitInput<F, C, D>,
        output: &CircuitOutput<F, C, D>,
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

    fn serializers() -> (CustomGateSerializer, CustomGeneratorSerializer<C, D>) {
        let gate_serializer = CustomGateSerializer;
        let generator_serializer = CustomGeneratorSerializer::<C, D> {
            _phantom: PhantomData,
        };
        (gate_serializer, generator_serializer)
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

    pub fn serialize(&self) -> IoResult<Vec<u8>> {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = Self::serializers();

        // Setup buffer.
        let mut buffer = Vec::new();
        let circuit_bytes = self
            .data
            .to_bytes(&gate_serializer, &generator_serializer)?;
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
        }

        if self.io.recursive_proof.is_some() {
            let io = self.io.recursive_proof.as_ref().unwrap();
            buffer.write_usize(2)?;
            buffer.write_usize(io.proofs.len())?;
            for i in 0..io.proofs.len() {
                buffer.write_target_proof_with_public_inputs(&io.proofs[i])?;
            }
            for i in 0..io.proofs.len() {
                let bytes = &io.child_circuit_ids[i].as_bytes();
                buffer.write_usize(bytes.len())?;
                buffer.write_all(bytes)?;
            }
        }

        Ok(buffer)
    }

    pub fn deserialize(buffer: &[u8]) -> IoResult<Self> {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = Self::serializers();

        // Setup buffer.
        let mut buffer = Buffer::new(buffer);

        // Read circuit data from bytes.
        let circuit_bytes_len = buffer.read_usize()?;
        let mut circuit_bytes = vec![0u8; circuit_bytes_len];
        buffer.read_exact(circuit_bytes.as_mut_slice())?;
        let data = CircuitData::<F, C, D>::from_bytes(
            &circuit_bytes,
            &gate_serializer,
            &generator_serializer,
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

        if !buffer.is_empty() {
            buffer.read_usize()?;
            let input_proof_count = buffer.read_usize()?;
            let mut input_proofs = Vec::new();
            for _ in 0..input_proof_count {
                input_proofs.push(buffer.read_target_proof_with_public_inputs()?);
            }
            let mut child_circuit_ids = Vec::new();
            for _ in 0..input_proof_count {
                let length = buffer.read_usize()?;
                let mut circuit_id_bytes = vec![0u8; length];
                buffer.read_exact(&mut circuit_id_bytes)?;
                child_circuit_ids.push(String::from_utf8(circuit_id_bytes).unwrap());
            }
            circuit.io.recursive_proof = Some(RecursiveProofIO {
                proofs: input_proofs,
                child_circuit_ids,
            });
        }

        Ok(circuit)
    }

    pub fn save(&self, path: &String) {
        let bytes = self.serialize().unwrap();
        let dir = path.split('/').take(path.split('/').count() - 1).join("/");
        fs::create_dir_all(dir).unwrap_or(());
        fs::write(path, bytes).unwrap();
    }

    pub fn load(path: &str) -> IoResult<Self> {
        let bytes = fs::read(path).unwrap();
        Self::deserialize(bytes.as_slice())
    }

    pub fn save_to_build_dir(&self) {
        let path = format!("./build/{}.circuit", self.id());
        self.save(&path);
    }

    pub fn load_from_build_dir(circuit_id: String) -> IoResult<Self> {
        let path = format!("./build/{}.circuit", circuit_id);
        Self::load(&path)
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

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

        // Serialize.
        let bytes = circuit.serialize().unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();
        let old_output_variables = circuit.io.field.as_ref().unwrap().input_variables.clone();

        // Deserialize.
        let circuit = Circuit::<F, C, D>::deserialize(&bytes).unwrap();
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

        // Serialize.
        let bytes = circuit.serialize().unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_bytes = circuit.io.evm.as_ref().unwrap().input_bytes.clone();
        let old_output_bytes = circuit.io.evm.as_ref().unwrap().output_bytes.clone();

        // Deserialize.
        let circuit = Circuit::<F, C, D>::deserialize(&bytes).unwrap();
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
