use curta::chip::hash::sha::sha256::builder_gadget::{SHA256Builder, SHA256BuilderGadget};
use curta::chip::hash::sha::sha256::generator::SHA256HintGenerator;
use curta::math::field::Field;
use itertools::Itertools;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use crate::backend::config::PlonkParameters;
use crate::frontend::hash::bit_operations::util::u64_to_bits;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder, Variable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn sha256_curta(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let mut bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().to_vec())
            .collect_vec();
        bits.push(self.api._true());

        let l = bits.len() - 1;
        let mut k = 0;
        while (l + 1 + k + 64) % 512 != 0 {
            k += 1;
        }
        for _ in 0..k {
            bits.push(self.api._false());
        }

        let be_bits = u64_to_bits(l as u64, &mut self.api);
        for i in 0..be_bits.len() {
            bits.push(be_bits[i]);
        }

        let mut bytes = Vec::new();
        for i in 0..bits.len() / 8 {
            let mut byte = self.api.zero();
            for j in 0..8 {
                let bit = bits[i * 8 + j].target;
                byte = self
                    .api
                    .mul_const_add(L::Field::from_canonical_u8(1 << (7 - j)), bit, byte);
            }
            bytes.push(byte);
        }

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
        let zero = self.constant::<ByteVariable>(0u8);
        let zero_chunk = [zero; 1];
        for i in 0..self.sha256_requests.len() {
            nb_chunks += self.sha256_requests[i].len() / 64;
        }
        while nb_chunks < 1024 {
            self.sha256_curta(&zero_chunk);
            nb_chunks += 1;
        }

        let mut gadget: SHA256BuilderGadget<L::Field, L::CubicParams, D> = self.api.init_sha256();

        for i in 0..self.sha256_requests.len() {
            gadget
                .padded_messages
                .extend_from_slice(&self.sha256_requests[i]);
            let hint = SHA256HintGenerator::new(&self.sha256_requests[i], self.sha256_responses[i]);
            self.add_simple_generator(hint);
            gadget.digests.extend_from_slice(&self.sha256_responses[i]);
            gadget.chunk_sizes.push(self.sha256_requests[i].len() / 64);
        }

        self.api.constrain_sha256_gadget::<L::Config>(gadget);
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
        circuit.test_default_serializers();
    }
}
