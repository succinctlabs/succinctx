use serde::{Deserialize, Serialize};

use crate::prelude::{
    BoolVariable, PlonkParameters, U32Variable, U64Variable, ValueStream, Variable, VariableStream,
};

/// Circuit variables for the input data of a SHA computation.
pub struct BLAKE2BInputData {
    /// The padded chunks of the input message.
    pub padded_chunks: Vec<U64Variable>,
    /// The t values for each chunk.
    pub t_values: Vec<U32Variable>,
    // A flag for each chunk indicating whether the hash state needs to be restarted after
    // processing the chunk.
    pub end_bits: Vec<BoolVariable>,
    /// A flag for each chunk indicating whether the digest should be read after processing the
    /// chunk.
    pub digest_bits: Vec<BoolVariable>,
    /// The index of the digests to be read, corresponding to their location in `padded chunks`.
    pub digest_indices: Vec<Variable>,
    /// The message digests.
    pub digests: Vec<[U64Variable; 4]>,
}

/// The values of the input data of a BLAKE2B computation.
///
/// This struct represents the values of the variables of `BLAKE2BInputData`.
#[derive(Debug, Clone)]
pub struct BLAKE2BInputDataValues<L: PlonkParameters<D>, const D: usize> {
    pub padded_chunks: Vec<u64>,
    pub t_values: Vec<u32>,
    pub end_bits: Vec<bool>,
    pub digest_bits: Vec<bool>,
    pub digest_indices: Vec<L::Field>,
    pub digests: Vec<[u64; 4]>,
}

/// The parameters required for reading the input data of a BLAKE2B computation from a stream.
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct BLAKE2BInputParameters {
    pub num_chunks: usize,
    pub num_digests: usize,
}

impl BLAKE2BInputData {
    /// Get parameters from the input data.
    pub fn parameters(&self) -> BLAKE2BInputParameters {
        BLAKE2BInputParameters {
            num_chunks: self.end_bits.len(),
            num_digests: self.digests.len(),
        }
    }
}

impl VariableStream {
    /// Read blake2b input data from the stream.
    pub fn write_blake2b_input(&mut self, input: &BLAKE2BInputData) {
        self.write_slice(&input.padded_chunks);
        self.write_slice(&input.t_values);
        self.write_slice(&input.end_bits);
        self.write_slice(&input.digest_bits);
        self.write_slice(&input.digest_indices);
        self.write_slice(&input.digests);
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    /// Read sha input data from the stream.
    pub fn read_blake2b_input_values(
        &mut self,
        parameters: BLAKE2BInputParameters,
    ) -> BLAKE2BInputDataValues<L, D> {
        let BLAKE2BInputParameters {
            num_chunks,
            num_digests,
        } = parameters;

        let padded_chunks = self.read_vec::<U64Variable>(num_chunks * 16);
        let t_values = self.read_vec::<U32Variable>(num_chunks);
        let end_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_indices = self.read_vec::<Variable>(num_digests);
        let digests = self.read_vec::<[U64Variable; 4]>(num_digests);

        BLAKE2BInputDataValues {
            padded_chunks,
            end_bits,
            digest_bits,
            digest_indices,
            digests,
            t_values,
        }
    }
}
