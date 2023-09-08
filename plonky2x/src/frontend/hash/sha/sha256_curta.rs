use curta::chip::hash::sha::sha256::builder_gadget::{SHA256Builder, SHA256BuilderGadget};
use curta::chip::hash::sha::sha256::generator::SHA256HintGenerator;
use curta::math::field::Field;
use itertools::Itertools;
use plonky2::iop::target::Target;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use crate::backend::config::PlonkParameters;
use crate::frontend::hash::bit_operations::util::u64_to_bits;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder, CircuitVariable};

/// Pad the given input according to the SHA-256 spec.
pub fn pad_variable_to_target<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
) -> Vec<Target> {
    let mut bits = input
        .iter()
        .flat_map(|b| b.as_bool_targets().to_vec())
        .collect_vec();
    bits.push(builder.api._true());

    let l = bits.len() - 1;
    let mut k = 0;
    while (l + 1 + k + 64) % 512 != 0 {
        k += 1;
    }
    for _ in 0..k {
        bits.push(builder.api._false());
    }

    let be_bits = u64_to_bits(l as u64, &mut builder.api);
    for i in 0..be_bits.len() {
        bits.push(be_bits[i]);
    }

    let mut bytes = Vec::new();
    for i in 0..bits.len() / 8 {
        let mut byte = builder.api.zero();
        for j in 0..8 {
            let bit = bits[i * 8 + j].target;
            byte = builder
                .api
                .mul_const_add(L::Field::from_canonical_u8(1 << (7 - j)), bit, byte);
        }
        bytes.push(byte);
    }
    bytes
}

/// Convert an array of ByteVariable to a Vec<Target>.
pub fn bytes_to_target<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
) -> Vec<Target> {
    let mut bytes = Vec::new();
    for i in 0..input.len() {
        let mut byte = builder.api.zero();
        let targets = input[i].targets();
        for j in 0..8 {
            let bit = targets[j];
            byte = builder
                .api
                .mul_const_add(L::Field::from_canonical_u8(1 << (7 - j)), bit, byte);
        }
        bytes.push(byte);
    }
    bytes
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA256 hash on the given input. Assumes the message is already padded.
    pub fn sha256_curta_padded(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        assert!(input.len() % 64 == 0);

        let bytes = bytes_to_target(self, input);

        self.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        self.sha256_responses.push(digest);
        Bytes32Variable::from_targets(
            &digest
                .into_iter()
                .flat_map(|byte| {
                    let mut bits = self
                        .api
                        .low_bits(byte, 8, 8)
                        .into_iter()
                        .map(|b| b.target)
                        .collect_vec();
                    bits.reverse();
                    bits
                })
                .collect_vec(),
        )
    }

    /// Executes a SHA256 hash on the given input. (Assumes it's not padded)
    pub fn sha256_curta(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let bytes = pad_variable_to_target(self, input);

        self.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        self.sha256_responses.push(digest);
        Bytes32Variable::from_targets(
            &digest
                .into_iter()
                .flat_map(|byte| {
                    let mut bits = self
                        .api
                        .low_bits(byte, 8, 8)
                        .into_iter()
                        .map(|b| b.target)
                        .collect_vec();
                    bits.reverse();
                    bits
                })
                .collect_vec(),
        )
    }

    pub fn constraint_sha256_curta(&mut self)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut nb_chunks = 0;
        let mut rq_idx = 0;
        let mut array_end_idx = self.sha256_requests.len();

        let zero = self.constant::<ByteVariable>(0u8);
        let zero_chunk = [zero; 1];

        while rq_idx < array_end_idx {
            let nb_chunks_in_rq = self.sha256_requests[rq_idx].len() / 64;

            let temp_nb_chunks = nb_chunks + nb_chunks_in_rq;

            // If temp_nb_chunks / 1024 != nb_chunks / 1024 & temp_nb_chunks % 1024 != 0 we need to zero pad
            if (temp_nb_chunks / 1024 != nb_chunks / 1024) && temp_nb_chunks % 1024 != 0 {
                println!("zero padding in between");
                while nb_chunks % 1024 != 0 {
                    // We want to insert a 0 chunk in here!
                    let bytes = pad_variable_to_target(self, &zero_chunk);
                    self.sha256_requests.insert(rq_idx, bytes);

                    let digest = self.api.add_virtual_target_arr::<32>();
                    self.sha256_responses.insert(rq_idx, digest);

                    // Increment request index because we've inserted, and also increment the end index
                    rq_idx += 1;
                    array_end_idx += 1;

                    nb_chunks += 1;
                }
            }
            nb_chunks += nb_chunks_in_rq;
            rq_idx += 1;
        }

        println!("nb_chunks: {}", nb_chunks);
        println!("sha_256_requests len {}", self.sha256_requests.len());
        while nb_chunks % 1024 != 0 {
            self.sha256_curta(&zero_chunk);
            nb_chunks += 1;
        }
        println!("nb_chunks: {}", nb_chunks);
        println!("sha_256_requests len {}", self.sha256_requests.len());

        let gadgets: Vec<SHA256BuilderGadget<<L as PlonkParameters<D>>::Field, L::CubicParams, D>> =
            (0..nb_chunks / 1024)
                .map(|_| self.api.init_sha256())
                .collect_vec();

        let mut rq_idx = 0;
        for i in 0..gadgets.len() {
            println!("gadget num: {}", i);
            let mut gadget = gadgets[i].to_owned();

            let mut num_chunks_so_far = 0;

            while num_chunks_so_far < 1024 {
                // println!("num_chunks_so_far {}", num_chunks_so_far);

                gadget
                    .padded_messages
                    .extend_from_slice(&self.sha256_requests[rq_idx]);
                let hint = SHA256HintGenerator::new(
                    &self.sha256_requests[rq_idx],
                    self.sha256_responses[rq_idx],
                );
                self.add_simple_generator(hint);
                gadget
                    .digests
                    .extend_from_slice(&self.sha256_responses[rq_idx]);
                gadget
                    .chunk_sizes
                    .push(self.sha256_requests[rq_idx].len() / 64);

                num_chunks_so_far += self.sha256_requests[rq_idx].len() / 64;
                rq_idx += 1;
            }

            self.api.constrain_sha256_gadget::<L::Config>(gadget);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::backend::config::DefaultParameters;
    use crate::prelude::{ByteVariable, CircuitBuilder};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.constant::<ByteVariable>(0u8);
        let result = builder.sha256_curta(&[zero; 1]);
        builder.watch(&result, "result");
        builder.constraint_sha256_curta();

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        // circuit.test_default_serializers();
    }
}
