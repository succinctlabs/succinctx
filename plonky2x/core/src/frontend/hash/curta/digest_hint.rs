use core::marker::PhantomData;

use curta::math::prelude::PrimeField64;
use serde::{Deserialize, Serialize};

use super::Hash;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// Provides the SHA of a message usign the algorithm specified by `S`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HashDigestHint<
    S,
    const CYCLE_LEN: usize,
    const USE_T_VALUES: bool,
    const DIGEST_LEN: usize,
> {
    _marker: PhantomData<S>,
}

impl<
        L: PlonkParameters<D>,
        H: Hash<L, D, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>,
        const D: usize,
        const CYCLE_LEN: usize,
        const USE_T_VALUES: bool,
        const DIGEST_LEN: usize,
    > Hint<L, D> for HashDigestHint<H, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let length = input_stream.read_value::<Variable>().as_canonical_u64() as usize;
        // Read the padded chunks from the input stream.
        let message = input_stream.read_vec::<ByteVariable>(length);

        let digest = H::hash(message);
        // Write the digest to the output stream.
        output_stream.write_value::<[H::IntVariable; DIGEST_LEN]>(digest)
    }
}

impl<S, const CYCLE_LEN: usize, const USE_T_VALUES: bool, const DIGEST_LEN: usize>
    HashDigestHint<S, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S, const CYCLE_LEN: usize, const USE_T_VALUES: bool, const DIGEST_LEN: usize> Default
    for HashDigestHint<S, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>
{
    fn default() -> Self {
        Self::new()
    }
}
