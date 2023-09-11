use itertools::Itertools;
use num::BigInt;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::Deserialize;

pub fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<BigInt>().map_err(serde::de::Error::custom)
}

pub fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    hex::decode(s).map_err(serde::de::Error::custom)
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
