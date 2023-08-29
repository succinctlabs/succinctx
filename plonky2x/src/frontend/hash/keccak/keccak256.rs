use core::marker::PhantomData;

use ethers::types::H256;
use ethers::utils::keccak256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::frontend::vars::{ByteVariable, Bytes32Variable, CircuitVariable, Variable};

#[derive(Debug, Clone)]
pub struct Keccack256Generator<F: RichField + Extendable<D>, const D: usize> {
    pub input: Vec<ByteVariable>,
    pub output: Bytes32Variable,
    pub length: Option<Variable>,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for Keccack256Generator<F, D>
{
    fn id(&self) -> String {
        "Keccack256Generator".to_string()
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

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        println!("Running keccak256 generator");
        let mut length = self.input.len();
        if let Some(length_variable) = self.length {
            length = length_variable.get(witness).to_canonical_u64() as usize;
        }
        let input: Vec<u8> = self.input.iter().map(|x| x.get(witness)).collect();
        let result = keccak256(&input[..length]);
        self.output.set(out_buffer, H256::from_slice(&result[..]));
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        // Write each input as a target
        let input_bytes = self
            .input
            .iter()
            .flat_map(|x| x.targets())
            .collect::<Vec<Target>>();
        dst.write_target_vec(&input_bytes)?;

        dst.write_target_vec(&self.output.targets())?;

        if self.length.is_some() {
            dst.write_target_vec(&self.length.unwrap().targets())
        } else {
            Ok(())
        }
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
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
        let output = Bytes32Variable::from_targets(&input_targets);

        let length = src
            .read_target_vec()
            .map(|targets| Some(Variable::from_targets(&targets)))
            .unwrap_or(None);

        Ok(Self {
            input,
            output,
            length,
            _phantom: PhantomData::<F>,
        })
    }
}
