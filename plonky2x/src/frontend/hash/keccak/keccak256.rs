use core::marker::PhantomData;

use ethers::types::H256;
use ethers::utils::keccak256;
use plonky2::field::types::PrimeField64;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::backend::config::PlonkParameters;
use crate::frontend::vars::{ByteVariable, Bytes32Variable, FieldVariable, Variable};

#[derive(Debug, Clone)]
pub struct Keccak256Generator<L: PlonkParameters<D>, const D: usize> {
    pub input: Vec<ByteVariable>,
    pub output: Bytes32Variable,
    pub length: Option<FieldVariable>,
    pub _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> Keccak256Generator<L, D> {
    pub fn id() -> String {
        "Keccak256Generator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for Keccak256Generator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(
            self.input
                .iter()
                .flat_map(|x| x.targets())
                .collect::<Vec<Target>>(),
        );
        if let Some(length) = self.length {
            targets.extend(length.targets());
        }
        targets
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let mut length = self.input.len();
        if let Some(length_variable) = self.length {
            length = length_variable.get(witness).to_canonical_u64() as usize;
        }
        let input: Vec<u8> = self.input.iter().map(|x| x.get(witness)).collect();
        let result = keccak256(&input[..length]);
        self.output.set(out_buffer, H256::from_slice(&result[..]));
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        // Write each input as a target
        let input_bytes = self
            .input
            .iter()
            .flat_map(|x| x.targets())
            .collect::<Vec<Target>>();
        dst.write_target_vec(&input_bytes)?;

        dst.write_target_vec(&self.output.targets())?;

        dst.write_bool(self.length.is_some())?;
        if self.length.is_some() {
            dst.write_target_vec(&self.length.unwrap().targets())
        } else {
            Ok(())
        }
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        let input_targets = src.read_target_vec()?;
        // Convert input_targets progressively into ByteVariables by chunks of 8
        let mut input = Vec::new();
        for i in 0..input_targets.len() / 8 {
            let mut byte_targets = Vec::new();
            for j in 0..8 {
                byte_targets.push(input_targets[i * 8 + j]);
            }
            let byte_variable = ByteVariable::from_targets(&byte_targets);
            input.push(byte_variable);
        }

        let output_targets = src.read_target_vec()?;
        let output = Bytes32Variable::from_targets(&output_targets);

        let length_exists = src.read_bool()?;
        let length = if length_exists {
            Some(FieldVariable::from_targets(&src.read_target_vec()?))
        } else {
            None
        };

        Ok(Self {
            input,
            output,
            length,
            _phantom: PhantomData::<L>,
        })
    }
}
