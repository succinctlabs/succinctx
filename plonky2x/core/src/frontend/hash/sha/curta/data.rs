use serde::{Deserialize, Serialize};

use super::SHA;
use crate::prelude::{
    BoolVariable, CircuitVariable, PlonkParameters, ValueStream, Variable, VariableStream,
};

pub struct SHAInputData<T> {
    pub padded_chunks: Vec<T>,
    pub end_bits: Vec<BoolVariable>,
    pub digest_bits: Vec<BoolVariable>,
    pub digest_indices: Vec<Variable>,
    pub digests: Vec<[T; 8]>,
}

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

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct SHAInputParameters {
    pub num_chunks: usize,
    pub num_digests: usize,
}

impl<T> SHAInputData<T> {
    pub fn parameters(&self) -> SHAInputParameters {
        SHAInputParameters {
            num_chunks: self.end_bits.len(),
            num_digests: self.digests.len(),
        }
    }
}

impl VariableStream {
    pub fn write_sha_input<T: CircuitVariable>(&mut self, input: &SHAInputData<T>) {
        self.write_slice(&input.padded_chunks);
        self.write_slice(&input.end_bits);
        self.write_slice(&input.digest_bits);
        self.write_slice(&input.digest_indices);
        self.write_slice(&input.digests);
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    #[allow(clippy::type_complexity)]
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
