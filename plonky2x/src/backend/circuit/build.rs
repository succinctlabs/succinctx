use alloc::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use plonky2::field::types::PrimeField64;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, GenericHashOut};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::prover::prove_with_partition_witness;
use plonky2::util::serialization::{Buffer, GateSerializer, IoResult, Read, Write};
use plonky2::util::timing::TimingTree;
use tracing::debug;

use super::config::PlonkParameters;
use super::input::PublicInput;
use super::output::PublicOutput;
use super::serialization::hints::HintSerializer;
use super::serialization::{GateRegistry, HintRegistry};
use super::witness::{generate_witness, generate_witness_async};
use crate::frontend::builder::CircuitIO;
use crate::frontend::hint::asynchronous::generator::AsyncHintDataRef;
use crate::utils::hex;
use crate::utils::serde::{BufferRead, BufferWrite};

/// A compiled circuit.
///
/// It can compute a function in the form f(publicInputs, privateInputs) = publicOutputs.
#[derive(Debug)]
pub struct CircuitBuild<L: PlonkParameters<D>, const D: usize> {
    pub data: CircuitData<L::Field, L::Config, D>,
    pub io: CircuitIO<D>,
    pub async_hints: BTreeMap<usize, AsyncHintDataRef<L, D>>,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuild<L, D> {
    /// Returns a public inputs instance for the circuit.
    pub fn input(&self) -> PublicInput<L, D> {
        PublicInput::new(&self.io)
    }

    /// Generates a proof for the circuit. The proof can be verified using `verify`.
    pub fn prove(
        &self,
        input: &PublicInput<L, D>,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let start_time = Instant::now();
        let mut pw = PartialWitness::new();
        self.io.set_witness(&mut pw, input);
        debug!("Begin witness generation, yo!");
        let partition_witness = generate_witness(
            pw,
            &self.data.prover_only,
            &self.data.common,
            &self.async_hints,
        )
        .unwrap();
        let elapsed_time = start_time.elapsed();
        debug!("Witness generation took {:?}", elapsed_time);
        let proof_with_pis = prove_with_partition_witness::<L::Field, L::Config, D>(
            &self.data.prover_only,
            &self.data.common,
            partition_witness,
            &mut TimingTree::default(),
        )
        .unwrap();
        let output = PublicOutput::from_proof_with_pis(&self.io, &proof_with_pis);
        let elapsed_time = start_time.elapsed();
        debug!("proving took: {:?}", elapsed_time);
        (proof_with_pis, output)
    }

    /// Generates a proof for the circuit. The proof can be verified using `verify`.
    pub async fn prove_async(
        &self,
        input: &PublicInput<L, D>,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut pw = PartialWitness::new();
        let start_time = tokio::time::Instant::now();
        self.io.set_witness(&mut pw, input);
        let partition_witness = generate_witness_async(
            pw,
            &self.data.prover_only,
            &self.data.common,
            &self.async_hints,
        )
        .await
        .unwrap();
        let elapsed_time = start_time.elapsed();
        debug!("Witness generation took {:?}", elapsed_time);
        tokio::task::block_in_place(|| {
            let proof_with_pis = prove_with_partition_witness::<L::Field, L::Config, D>(
                &self.data.prover_only,
                &self.data.common,
                partition_witness,
                &mut TimingTree::default(),
            )
            .unwrap();
            let output = PublicOutput::from_proof_with_pis(&self.io, &proof_with_pis);
            let elapsed_time = start_time.elapsed();
            debug!("proving took: {:?}", elapsed_time);
            (proof_with_pis, output)
        })
    }

    /// Verifies a proof for the circuit.
    pub fn verify(
        &self,
        proof: &ProofWithPublicInputs<L::Field, L::Config, D>,
        input: &PublicInput<L, D>,
        output: &PublicOutput<L, D>,
    ) {
        let expected_input = PublicInput::<L, D>::from_proof_with_pis(&self.io, proof);
        let expected_output = PublicOutput::<L, D>::from_proof_with_pis(&self.io, proof);
        assert_eq!(input, &expected_input);
        assert_eq!(output, &expected_output);
        self.data.verify(proof.clone()).unwrap();
    }

    /// A unique identifier for the circuit.
    pub fn id(&self) -> String {
        let circuit_digest = hex!(self
            .data
            .verifier_only
            .circuit_digest
            .to_vec()
            .iter()
            .flat_map(|e| e.to_canonical_u64().to_be_bytes())
            .collect::<Vec<u8>>());
        circuit_digest[0..22].to_string()
    }

    /// Serializes the circuit to bytes.
    pub fn serialize(
        &self,
        gate_serializer: &impl GateSerializer<L::Field, D>,
        hint_serializer: &impl HintSerializer<L, D>,
    ) -> IoResult<Vec<u8>> {
        let mut buffer = Vec::new();

        let data = self.data.to_bytes(gate_serializer, hint_serializer)?;
        buffer.write_bytes(&data)?;

        let io = bincode::serialize(&self.io).unwrap();
        buffer.write_bytes(&io)?;

        // serialize the async generator map
        let map_size = self.async_hints.len();
        buffer.write_usize(map_size)?;
        for (key, hint_data) in self.async_hints.iter() {
            buffer.write_usize(*key)?;
            hint_serializer.write_async_hint(&mut buffer, hint_data, &self.data.common)?;
        }

        Ok(buffer)
    }

    /// Deserializes the circuit from bytes.
    pub fn deserialize(
        buffer: &[u8],
        gate_serializer: &impl GateSerializer<L::Field, D>,
        hint_serializer: &impl HintSerializer<L, D>,
    ) -> IoResult<Self> {
        let mut buffer = Buffer::new(buffer);

        let data = buffer.read_bytes()?;
        let data = CircuitData::<L::Field, L::Config, D>::from_bytes(
            &data,
            gate_serializer,
            hint_serializer,
        )?;

        let io = buffer.read_bytes()?;
        let io: CircuitIO<D> = bincode::deserialize(&io).unwrap();

        let mut async_hints = BTreeMap::new();
        let map_size = buffer.read_usize()?;
        for _ in 0..map_size {
            let key = buffer.read_usize()?;
            let hint_data = hint_serializer.read_async_hint(&mut buffer, &data.common)?;
            async_hints.insert(key, hint_data);
        }

        Ok(CircuitBuild {
            data,
            io,
            async_hints,
        })
    }

    /// Saves the circuit to a file.
    pub fn save(
        &self,
        path: &String,
        gate_serializer: &impl GateSerializer<L::Field, D>,
        hint_serializer: &impl HintSerializer<L, D>,
    ) {
        let path = Path::new(path);
        if let Some(parent_dir) = path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).unwrap();
            }
        }
        let bytes = self.serialize(gate_serializer, hint_serializer).unwrap();
        fs::write(path, bytes).unwrap();
    }

    /// Loads the circuit from a file.
    pub fn load(
        path: &str,
        gate_serializer: &impl GateSerializer<L::Field, D>,
        hint_serializer: &impl HintSerializer<L, D>,
    ) -> IoResult<Self> {
        let bytes = fs::read(path).unwrap();
        Self::deserialize(bytes.as_slice(), gate_serializer, hint_serializer)
    }

    /// Tests that the circuit can be serialized/deserialzie given the default serializers.
    pub fn test_default_serializers(&self)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let gate_serializer = GateRegistry::<L, D>::new();
        let hint_serializer = HintRegistry::<L, D>::new();
        self.test_serializers(&gate_serializer, &hint_serializer);
    }

    /// Tests that the circuit can be serialized/deserialzie with the given serializers.
    pub fn test_serializers(
        &self,
        gate_serializer: &GateRegistry<L, D>,
        hint_serializer: &HintRegistry<L, D>,
    ) {
        let serialized_bytes = self.serialize(gate_serializer, hint_serializer).unwrap();
        let deserialized_circuit = Self::deserialize(
            serialized_bytes.as_slice(),
            gate_serializer,
            hint_serializer,
        )
        .unwrap();
        assert_eq!(self.data, deserialized_circuit.data);
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::backend::circuit::serialization::{GateRegistry, HintRegistry};
    use crate::backend::circuit::CircuitBuild;
    use crate::frontend::builder::DefaultBuilder;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_serialize_with_field_io() {
        // Define your circuit.
        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Setup serializers
        let gate_serializer = GateRegistry::<L, D>::new();
        let hint_serializer = HintRegistry::<L, D>::new();

        // Serialize.
        let bytes = circuit
            .serialize(&gate_serializer, &hint_serializer)
            .unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_variables = circuit.io.input();
        let old_output_variables = circuit.io.output();

        // Deserialize.
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &hint_serializer).unwrap();
        let new_digest = circuit.data.verifier_only.circuit_digest;
        let new_input_variables = circuit.io.input();
        let new_output_variables = circuit.io.output();

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
        let mut builder = DefaultBuilder::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(1u8);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Setup serializers
        let gate_serializer = GateRegistry::<L, D>::new();
        let hint_serializer = HintRegistry::<L, D>::new();

        // Serialize.
        let bytes = circuit
            .serialize(&gate_serializer, &hint_serializer)
            .unwrap();
        let old_digest = circuit.data.verifier_only.circuit_digest;
        let old_input_bytes = circuit.io.input();
        let old_output_bytes = circuit.io.output();

        // Deserialize.
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &hint_serializer).unwrap();
        let new_digest = circuit.data.verifier_only.circuit_digest;
        let new_input_bytes = circuit.io.input();
        let new_output_bytes = circuit.io.output();

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
