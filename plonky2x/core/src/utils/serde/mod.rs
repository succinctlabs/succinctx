use itertools::Itertools;
use num::BigInt;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::{VerifierCircuitData, VerifierCircuitTarget};
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use serde::ser::SerializeSeq;
use serde::Deserialize;

pub trait BufferRead: Read {
    fn read_bytes(&mut self) -> IoResult<Vec<u8>> {
        let len = self.read_usize()?;
        let mut bytes = vec![0u8; len];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

impl<'a> BufferRead for Buffer<'a> {}

pub trait BufferWrite: Write {
    fn write_bytes(&mut self, bytes: &[u8]) -> IoResult<()> {
        self.write_usize(bytes.len())?;
        self.write_all(bytes)?;
        Ok(())
    }
}

impl BufferWrite for Vec<u8> {}

pub fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<BigInt>().map_err(serde::de::Error::custom)
}

pub fn serialize_hex<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex = hex::encode(bytes);
    serializer.serialize_str(format!("0x{}", hex).as_str())
}

pub fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    hex::decode(&s[2..]).map_err(serde::de::Error::custom)
}

pub fn serialize_elements<F: RichField, S>(elements: &[F], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let serialized_elements: Vec<String> = elements
        .iter()
        .map(|element| element.to_canonical_u64().to_string())
        .collect();
    let mut seq = serializer.serialize_seq(Some(serialized_elements.len()))?;
    for element in serialized_elements {
        seq.serialize_element(&element)?;
    }
    seq.end()
}

pub fn deserialize_elements<'de, F: RichField, D>(deserializer: D) -> Result<Vec<F>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(s.into_iter()
        .map(|q| F::from_canonical_u64(q.parse::<u64>().unwrap()))
        .collect_vec())
}

pub fn serialize_proof_with_pis<
    F: RichField + Extendable<E>,
    C: GenericConfig<E, F = F>,
    S,
    const E: usize,
>(
    proof_with_pis: &ProofWithPublicInputs<F, C, E>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = format!(
        "0x{}",
        hex::encode(bincode::serialize(proof_with_pis).unwrap())
    );
    serializer.serialize_str(&bytes)
}

pub fn deserialize_proof_with_pis<
    'de,
    F: RichField + Extendable<E>,
    C: GenericConfig<E, F = F>,
    D,
    const E: usize,
>(
    deserializer: D,
) -> Result<ProofWithPublicInputs<F, C, E>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(bincode::deserialize(&hex::decode(&s[2..]).unwrap()).unwrap())
}

pub fn serialize_proof_with_pis_vec<
    F: RichField + Extendable<E>,
    C: GenericConfig<E, F = F>,
    S,
    const E: usize,
>(
    proof_with_pis_vec: &[ProofWithPublicInputs<F, C, E>],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let serialized_proofs: Vec<String> = proof_with_pis_vec
        .iter()
        .map(|proof| format!("0x{}", hex::encode(bincode::serialize(proof).unwrap())))
        .collect();
    let mut seq = serializer.serialize_seq(Some(serialized_proofs.len()))?;
    for proof in serialized_proofs {
        seq.serialize_element(&proof)?;
    }
    seq.end()
}

pub fn deserialize_proof_with_pis_vec<
    'de,
    F: RichField + Extendable<E>,
    C: GenericConfig<E, F = F>,
    D,
    const E: usize,
>(
    deserializer: D,
) -> Result<Vec<ProofWithPublicInputs<F, C, E>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(s.into_iter()
        .map(|q| bincode::deserialize(&hex::decode(&q[2..]).unwrap()).unwrap())
        .collect_vec())
}

pub fn serialize_proof_with_pis_target<S, const D: usize>(
    proof_with_pis: &ProofWithPublicInputsTarget<D>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer
        .write_target_proof_with_public_inputs(proof_with_pis)
        .unwrap();
    let hex = hex::encode(buffer);
    serializer.serialize_str(&hex)
}

pub fn serialize_proof_with_pis_target_option<S, const D: usize>(
    proof_with_pis: &Option<ProofWithPublicInputsTarget<D>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match proof_with_pis {
        Some(proof_with_pis) => {
            let mut buffer: Vec<u8> = Vec::new();
            buffer
                .write_target_proof_with_public_inputs(proof_with_pis)
                .unwrap();
            let hex = hex::encode(buffer);
            serializer.serialize_some(&hex)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_proof_with_pis_target_option<'de, D, const E: usize>(
    deserialize: D,
) -> Result<Option<ProofWithPublicInputsTarget<E>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserialize)?;
    match s {
        Some(s) => {
            let bytes = hex::decode(s).unwrap();
            let mut buffer = Buffer::new(&bytes);
            buffer
                .read_target_proof_with_public_inputs()
                .map_err(serde::de::Error::custom)
                .map(|proof_with_pis| Some(proof_with_pis))
        }
        None => Ok(None),
    }
}

pub fn deserialize_proof_with_pis_target<'de, D, const E: usize>(
    deserialize: D,
) -> Result<ProofWithPublicInputsTarget<E>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserialize)?;
    let bytes = hex::decode(s).unwrap();
    let mut buffer = Buffer::new(&bytes);
    buffer
        .read_target_proof_with_public_inputs()
        .map_err(serde::de::Error::custom)
}

pub fn serialize_proof_with_pis_target_vec<S, const D: usize>(
    proof_with_pis_vec: &Vec<ProofWithPublicInputsTarget<D>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_usize(proof_with_pis_vec.len()).unwrap();
    for proof_with_pi in proof_with_pis_vec {
        buffer
            .write_target_proof_with_public_inputs(proof_with_pi)
            .unwrap();
    }
    let hex = hex::encode(buffer);
    serializer.serialize_str(&hex)
}

pub fn deserialize_proof_with_pis_target_vec<'de, D, const E: usize>(
    deserialize: D,
) -> Result<Vec<ProofWithPublicInputsTarget<E>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserialize)?;
    let bytes = hex::decode(s).unwrap();
    let mut buffer = Buffer::new(&bytes);
    let size = buffer.read_usize().unwrap();
    let mut proof_with_pis_vec = Vec::new();
    for _ in 0..size {
        proof_with_pis_vec.push(buffer.read_target_proof_with_public_inputs().unwrap());
    }
    Ok(proof_with_pis_vec)
}

// pub fn serialize_verifier_circuit<S>(
//     verifier_circuit: &VerifierCircuitData<F, C, D>,
//     serializer: S,
// ) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let mut buffer: Vec<u8> = Vec::new();
//     buffer
//         .write_verifier_circuit_data(verifier_circuit)
//         .unwrap();
//     let hex = hex::encode(buffer);
//     serializer.serialize_str(&hex)
// }

// pub fn deserialize_verifier_circuit<'de, D>(deserialize: D) -> Result<VerifierCircuitData, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: String = Deserialize::deserialize(deserialize)?;
//     let bytes = hex::decode(s).unwrap();
//     let mut buffer = Buffer::new(&bytes);
//     buffer
//         .read_verifier_circuit_data()
//         .map_err(serde::de::Error::custom)
// }

pub fn serialize_verifier_circuit_target<S>(
    verifier_circuit_target: &VerifierCircuitTarget,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer
        .write_target_verifier_circuit(verifier_circuit_target)
        .unwrap();
    let hex = hex::encode(buffer);
    serializer.serialize_str(&hex)
}

pub fn serialize_verifier_circuit_target_option<S>(
    verifier_circuit_target: &Option<VerifierCircuitTarget>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match verifier_circuit_target {
        Some(verifier_circuit_target) => {
            let mut buffer: Vec<u8> = Vec::new();
            buffer
                .write_target_verifier_circuit(verifier_circuit_target)
                .unwrap();
            let hex = hex::encode(buffer);
            serializer.serialize_some(&hex)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_verifier_circuit_target<'de, D>(
    deserialize: D,
) -> Result<VerifierCircuitTarget, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserialize)?;
    let bytes = hex::decode(s).unwrap();
    let mut buffer = Buffer::new(&bytes);
    buffer
        .read_target_verifier_circuit()
        .map_err(serde::de::Error::custom)
}

pub fn deserialize_verifier_circuit_target_option<'de, D>(
    deserialize: D,
) -> Result<Option<VerifierCircuitTarget>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserialize)?;
    match s {
        Some(s) => {
            let bytes = hex::decode(s).unwrap();
            let mut buffer = Buffer::new(&bytes);
            buffer
                .read_target_verifier_circuit()
                .map_err(serde::de::Error::custom)
                .map(|verifier_circuit_target| Some(verifier_circuit_target))
        }
        None => Ok(None),
    }
}

pub fn serialize_verifier_circuit_target_vec<S>(
    verifier_circuit_target_vec: &Vec<VerifierCircuitTarget>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer
        .write_usize(verifier_circuit_target_vec.len())
        .unwrap();
    for verifier_circuit_target in verifier_circuit_target_vec {
        buffer
            .write_target_verifier_circuit(verifier_circuit_target)
            .unwrap();
    }
    let hex = hex::encode(buffer);
    serializer.serialize_str(&hex)
}

pub fn deserialize_verifier_circuit_target_vec<'de, D>(
    deserialize: D,
) -> Result<Vec<VerifierCircuitTarget>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserialize)?;
    let bytes = hex::decode(s).unwrap();
    let mut buffer = Buffer::new(&bytes);
    let size = buffer.read_usize().unwrap();
    let mut verifier_circuit_target_vec = Vec::new();
    for _ in 0..size {
        verifier_circuit_target_vec.push(buffer.read_target_verifier_circuit().unwrap());
    }
    Ok(verifier_circuit_target_vec)
}
