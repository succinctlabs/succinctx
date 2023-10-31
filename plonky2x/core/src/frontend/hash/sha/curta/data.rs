use serde::{Deserialize, Serialize};

use super::SHA;
use crate::prelude::{
    BoolVariable, CircuitVariable, PlonkParameters, ValueStream, Variable, VariableStream,
};

/// Circuit variables for the input data of a SHA computation.
pub struct SHAInputData<T> {
    /// The padded chunks of the input message.
    pub padded_chunks: Vec<T>,
    // A flag for each chunk indicating whether the hash state needs to be restarted after
    // processing the chunk.
    pub end_bits: Vec<BoolVariable>,
    /// A flag for each chunk indicating whether the digest should be read after processing the
    /// chunk.
    pub digest_bits: Vec<BoolVariable>,
    /// The index of the digests to be read, corresponding to their location in `padded chunks`.
    pub digest_indices: Vec<Variable>,
    /// The message digests.
    pub digests: Vec<[T; 8]>,
}

/// The values of the input data of a SHA computation.
///
/// This struct represents the values of the variables of `SHAInputData`.
pub struct SHAInputDataValues<
    L: PlonkParameters<D>,
    S: SHA<L, D, CYCLE_LEN>,
    const D: usize,
    const CYCLE_LEN: usize,
> {
    pub padded_chunks: Vec<S::Integer>,
    pub end_bits: Vec<bool>,
    pub digest_bits: Vec<bool>,
    pub digest_indices: Vec<L::Field>,
    pub digests: Vec<[S::Integer; 8]>,
}

/// The parameters required for reading the input data of a SHA computation from a stream.
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct SHAInputParameters {
    pub num_chunks: usize,
    pub num_digests: usize,
}

impl<T> SHAInputData<T> {
    /// Get parameters from the input data.
    pub fn parameters(&self) -> SHAInputParameters {
        SHAInputParameters {
            num_chunks: self.end_bits.len(),
            num_digests: self.digests.len(),
        }
    }
}

impl VariableStream {
    /// Read sha input data from the stream.
    pub fn write_sha_input<T: CircuitVariable>(&mut self, input: &SHAInputData<T>) {
        self.write_slice(&input.padded_chunks);
        self.write_slice(&input.end_bits);
        self.write_slice(&input.digest_bits);
        self.write_slice(&input.digest_indices);
        self.write_slice(&input.digests);
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    /// Read sha input data from the stream.
    pub fn read_sha_input_values<S: SHA<L, D, CYCLE_LEN>, const CYCLE_LEN: usize>(
        &mut self,
        parameters: SHAInputParameters,
    ) -> SHAInputDataValues<L, S, D, CYCLE_LEN> {
        let SHAInputParameters {
            num_chunks,
            num_digests,
        } = parameters;

        let padded_chunks = self.read_vec::<S::IntVariable>(num_chunks * 16);
        let end_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_bits = self.read_vec::<BoolVariable>(num_chunks);
        let digest_indices = self.read_vec::<Variable>(num_digests);
        let digests = self.read_vec::<[S::IntVariable; 8]>(num_digests);

        SHAInputDataValues {
            padded_chunks,
            end_bits,
            digest_bits,
            digest_indices,
            digests,
        }
    }
}
