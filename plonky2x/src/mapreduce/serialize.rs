use core::marker::PhantomData;
use std::fs::{self, File};

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, DefaultGateSerializer, Read, Write};

use super::utils::CustomGeneratorSerializer;
use crate::utils::hex;
use crate::vars::CircuitVariable;

pub trait CircuitDataSerializable<F: RichField + Extendable<D>, C, const D: usize>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String;

    fn serializers() -> (DefaultGateSerializer, CustomGeneratorSerializer<C, D>) {
        let gate_serializer = DefaultGateSerializer;
        let generator_serializer = CustomGeneratorSerializer::<C, D> {
            _phantom: PhantomData,
        };
        (gate_serializer, generator_serializer)
    }

    fn save_with_input_targets(&self, targets: &[Target], path: String);

    fn load_with_input_targets(path: String) -> (CircuitData<F, C, D>, Vec<Target>);

    fn save_with_proof_targets(
        &self,
        child_circuit: &CircuitData<F, C, D>,
        proofs: &[ProofWithPublicInputsTarget<D>],
        path: String,
    );

    fn load_with_proof_targets(
        path: String,
    ) -> (
        CircuitData<F, C, D>,
        CircuitData<F, C, D>,
        Vec<ProofWithPublicInputsTarget<D>>,
    );

    fn save<V: CircuitVariable>(&self, variable: V, path: String) {
        self.save_with_input_targets(variable.targets().as_slice(), path)
    }

    fn load<V: CircuitVariable>(path: String) -> (CircuitData<F, C, D>, V) {
        let (circuit, targets) =
            <Self as CircuitDataSerializable<F, C, D>>::load_with_input_targets(path);
        (circuit, V::from_targets(targets.as_slice()))
    }
}

impl<F: RichField + Extendable<D>, C, const D: usize> CircuitDataSerializable<F, C, D>
    for CircuitData<F, C, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String {
        let circuit_digest = hex!(self
            .verifier_only
            .circuit_digest
            .elements
            .iter()
            .map(|e| e.to_canonical_u64().to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>());
        circuit_digest[0..22].to_string()
    }

    fn save_with_input_targets(&self, targets: &[Target], path: String) {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = CircuitData::<F, C, D>::serializers();

        // Setup buffer.
        let mut buffer = Vec::new();

        // Serialize circuit data & input targets to buffer.
        let circuit_bytes = self
            .to_bytes(&gate_serializer, &generator_serializer)
            .unwrap();
        buffer.write_usize(circuit_bytes.len()).unwrap();
        buffer.write_all(&circuit_bytes).unwrap();
        buffer.write_target_vec(targets).unwrap();

        // Write buffer to path.
        let mut file = File::create(path).unwrap();
        std::io::Write::write_all(&mut file, &buffer).unwrap();
    }

    fn load_with_input_targets(path: String) -> (CircuitData<F, C, D>, Vec<Target>) {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = CircuitData::<F, C, D>::serializers();

        // Setup buffer.
        let bytes = fs::read(path.clone()).unwrap();
        let mut buffer = Buffer::new(&bytes);

        // Read circuit data from bytes.
        let circuit_bytes_len = buffer.read_usize().unwrap();
        let mut circuit_bytes = vec![0u8; circuit_bytes_len];
        buffer.read_exact(circuit_bytes.as_mut_slice()).unwrap();
        let circuit = CircuitData::<F, C, D>::from_bytes(
            &circuit_bytes,
            &gate_serializer,
            &generator_serializer,
        )
        .unwrap();

        // Deserialize input targets from bytes.
        let targets = buffer.read_target_vec().unwrap();

        (circuit, targets)
    }

    fn save_with_proof_targets(
        &self,
        child_circuit: &CircuitData<F, C, D>,
        proofs: &[ProofWithPublicInputsTarget<D>],
        path: String,
    ) {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = CircuitData::<F, C, D>::serializers();

        // Setup buffer.
        let mut buffer = Vec::new();

        // Serialize circuit data & proofs to buffer.
        let circuit_bytes = self
            .to_bytes(&gate_serializer, &generator_serializer)
            .unwrap();
        buffer.write_usize(circuit_bytes.len()).unwrap();
        buffer.write_all(&circuit_bytes).unwrap();

        let child_circuit_bytes = child_circuit
            .to_bytes(&gate_serializer, &generator_serializer)
            .unwrap();
        buffer.write_usize(child_circuit_bytes.len()).unwrap();
        buffer.write_all(&child_circuit_bytes).unwrap();

        buffer.write_usize(proofs.len()).unwrap();
        for i in 0..proofs.len() {
            buffer
                .write_target_proof_with_public_inputs(&proofs[i])
                .unwrap()
        }

        // Write bytes to path.
        let mut file = File::create(path).unwrap();
        std::io::Write::write_all(&mut file, &buffer).unwrap();
    }

    fn load_with_proof_targets(
        path: String,
    ) -> (
        CircuitData<F, C, D>,
        CircuitData<F, C, D>,
        Vec<ProofWithPublicInputsTarget<D>>,
    ) {
        // Setup serializers.
        let (gate_serializer, generator_serializer) = CircuitData::<F, C, D>::serializers();

        // Setup buffer.
        let bytes = fs::read(path).unwrap();
        let mut buffer = Buffer::new(&bytes);

        // Read circuit data from bytes.
        let circuit_bytes_len = buffer.read_usize().unwrap();
        let mut circuit_bytes = vec![0u8; circuit_bytes_len];
        buffer.read_exact(circuit_bytes.as_mut_slice()).unwrap();
        let circuit = CircuitData::<F, C, D>::from_bytes(
            &circuit_bytes,
            &gate_serializer,
            &generator_serializer,
        )
        .unwrap();

        let child_circuit_bytes_len = buffer.read_usize().unwrap();
        let mut child_circuit_bytes = vec![0u8; child_circuit_bytes_len];
        buffer
            .read_exact(child_circuit_bytes.as_mut_slice())
            .unwrap();
        let child_circuit = CircuitData::<F, C, D>::from_bytes(
            &child_circuit_bytes,
            &gate_serializer,
            &generator_serializer,
        )
        .unwrap();

        // Deserialize proof targets from bytes.
        let proofs_len = buffer.read_usize().unwrap();
        let mut proofs = Vec::new();
        for _ in 0..proofs_len {
            proofs.push(buffer.read_target_proof_with_public_inputs().unwrap());
        }

        (circuit, child_circuit, proofs)
    }
}
