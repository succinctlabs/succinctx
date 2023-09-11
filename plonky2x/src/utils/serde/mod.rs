use itertools::Itertools;
use num::BigInt;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
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
    serializer.serialize_str(&hex)
}

pub fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    hex::decode(s).map_err(serde::de::Error::custom)
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
        .map(|proof| hex::encode(bincode::serialize(proof).unwrap()))
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
        .map(|q| bincode::deserialize(&hex::decode(q).unwrap()).unwrap())
        .collect_vec())
}
