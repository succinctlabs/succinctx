use num::BigInt;
use serde::Deserialize;

pub fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<BigInt>().map_err(serde::de::Error::custom)
}
