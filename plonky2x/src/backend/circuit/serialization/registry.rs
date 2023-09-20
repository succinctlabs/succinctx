use core::any::Any;
use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

/// A trait for serializing and deserializing objects compatible with plonky2 traits.
pub trait Serializer<F: RichField + Extendable<D>, T, const D: usize>: 'static {
    fn read(&self, buf: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<T>;
    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &T,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()>;
}

/// A registry for storing serializers for objects.
pub(crate) struct SerializationRegistry<K: Hash, F: RichField + Extendable<D>, T, const D: usize> {
    pub registry: HashMap<K, Box<dyn Serializer<F, T, D>>>,
    pub index: HashMap<K, usize>,
    pub identifiers: Vec<K>,
    pub current_index: usize,
}

impl<K: Hash + Debug, F: RichField + Extendable<D>, T: Debug, const D: usize> Debug
    for SerializationRegistry<K, F, T, D>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SerializationRegistry")
            .field("ids of registered objects", &self.registry.keys())
            .field("index", &self.index)
            .field("identifiers", &self.identifiers)
            .field("current_index", &self.current_index)
            .finish()
    }
}

impl<F: RichField + Extendable<D>, K: PartialEq + Eq + Hash + Clone, T: Any, const D: usize>
    SerializationRegistry<K, F, T, D>
{
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            index: HashMap::new(),
            identifiers: Vec::new(),
            current_index: 0,
        }
    }

    /// Returns the serializer for the given object type.
    pub fn register<S: Serializer<F, T, D>>(&mut self, key: K, serializer: S) -> Result<()> {
        let exists = self.registry.insert(key.clone(), Box::new(serializer));

        if exists.is_some() {
            return Err(anyhow!("Object type already registered"));
        }

        self.identifiers.push(key.clone());
        self.index.insert(key, self.current_index);
        self.current_index += 1;

        Ok(())
    }
}
