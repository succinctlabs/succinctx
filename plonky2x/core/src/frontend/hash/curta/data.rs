use serde::{Deserialize, Serialize};

use super::Hash;
use crate::prelude::{
    BoolVariable, CircuitVariable, PlonkParameters, U32Variable, ValueStream, Variable,
    VariableStream,
};

/// Circuit variables for the input data of a hash computation.
pub struct HashInputData<T, const DIGEST_LEN: usize> {
    /// The padded chunks of the input message.
    pub padded_chunks: Vec<T>,
    /// The t values for each chunk.  This is used for the blake2b hash function.
    pub t_values: Option<Vec<U32Variable>>,
    // A flag for each chunk indicating whether the hash state needs to be restarted after
    // processing the chunk.
    pub end_bits: Vec<BoolVariable>,
    /// A flag for each chunk indicating whether the digest should be read after processing the
    /// chunk.
    pub digest_bits: Vec<BoolVariable>,
    /// The index of the digests to be read, corresponding to their location in `padded chunks`.
    pub digest_indices: Vec<Variable>,
    /// The message digests.
    pub digests: Vec<[T; DIGEST_LEN]>,
}

/// The values of the input data of a hash computation.
///
/// This struct represents the values of the variables of `HashInputData`.
pub struct HashInputDataValues<
    L: PlonkParameters<D>,
    S: Hash<L, D, CYCLE_LEN, HAS_T_VALUES, DIGEST_LEN>,
    const D: usize,
    const CYCLE_LEN: usize,
    const HAS_T_VALUES: bool,
    const DIGEST_LEN: usize,
> {
    pub padded_chunks: Vec<S::Integer>,
    pub t_values: Option<Vec<u32>>,
    pub end_bits: Vec<bool>,
    pub digest_bits: Vec<bool>,
    pub digest_indices: Vec<L::Field>,
    pub digests: Vec<[S::Integer; DIGEST_LEN]>,
}

/// The parameters required for reading the input data of a SHA computation from a stream.
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct HashInputParameters {
    pub num_chunks: usize,
    pub num_digests: usize,
}

impl<T, const DIGEST_LEN: usize> HashInputData<T, DIGEST_LEN> {
    /// Get parameters from the input data.
    pub fn parameters(&self) -> HashInputParameters {
        HashInputParameters {
            num_chunks: self.end_bits.len(),
            num_digests: self.digests.len(),
        }
    }
}

impl VariableStream {
    /// Read sha input data from the stream.
    pub fn write_hash_input<T: CircuitVariable, const DIGEST_LEN: usize>(
        &mut self,
        input: &HashInputData<T, DIGEST_LEN>,
    ) {
        self.write_slice(&input.padded_chunks);
        if input.t_values.is_some() {
            self.write_slice(&input.t_values.clone().unwrap());
        }
        self.write_slice(&input.end_bits);
        self.write_slice(&input.digest_bits);
        self.write_slice(&input.digest_indices);
        self.write_slice(&input.digests);
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    /// Read sha input data from the stream.
    pub fn read_hash_input_values<
        S: Hash<L, D, CYCLE_LEN, HAS_T_VALUES, DIGEST_LEN>,
        const CYCLE_LEN: usize,
        const HAS_T_VALUES: bool,
        const DIGEST_LEN: usize,
    >(
        &mut self,
        parameters: HashInputParameters,
    ) -> HashInputDataValues<L, S, D, CYCLE_LEN, HAS_T_VALUES, DIGEST_LEN> {
        let HashInputParameters {
            num_chunks,
            num_digests,
        } = parameters;

        let padded_chunks = self.read_vec::<S::IntVariable>(num_chunks * 16);
        let mut t_values = None;
        if HAS_T_VALUES {
            t_values = Some(self.read_vec::<U32Variable>(num_chunks));
        }
        let end_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_indices = self.read_vec::<Variable>(num_digests);
        let digests = self.read_vec::<[S::IntVariable; DIGEST_LEN]>(num_digests);

        HashInputDataValues {
            padded_chunks,
            t_values,
            end_bits,
            digest_bits,
            digest_indices,
            digests,
        }
    }
}
