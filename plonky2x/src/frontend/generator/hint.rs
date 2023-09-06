use core::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::backend::circuit::serialization::Serializer;
use crate::frontend::vars::{ElementBuffer, VariableBuffer};
use crate::prelude::{CircuitBuilder, CircuitVariable, Variable};

#[derive(Debug, Clone)]
pub struct Hint<F, const D: usize> {
    input_variables: Vec<Variable>,
    output_variables: Vec<Variable>,
    hint_fn: fn(&mut ElementBuffer<F, D>) -> Vec<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn hint(
        &mut self,
        input_buffer: &mut VariableBuffer,
        output_buffer: &[Variable],
        hint_fn: fn(&mut ElementBuffer<F, D>) -> Vec<F>,
    ) {
        let input_variables = input_buffer.read_all().to_vec();
        let output_variables = output_buffer.to_vec();

        let hint = Hint::<F, D> {
            input_variables,
            output_variables,
            hint_fn,
        };

        self.add_simple_generator(hint);
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D> for Hint<F, D> {
    fn id(&self) -> String {
        let hint_serializer = HintSerializer::<F, D>::new(self.hint_fn);
        hint_serializer.id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.input_variables.iter().map(|v| v.0).collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let input_values = self
            .input_variables
            .iter()
            .map(|v| v.get(witness))
            .collect::<Vec<_>>();
        let mut input_buffer = ElementBuffer::new(&input_values);
        let output_vec = (self.hint_fn)(&mut input_buffer);

        assert_eq!(output_vec.len(), self.output_variables.len());
        for (variable, value) in self.output_variables.iter().zip(output_vec.iter()) {
            variable.set(out_buffer, *value);
        }
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        let input_targets = self.input_variables.iter().map(|v| v.0).collect::<Vec<_>>();
        let output_targets = self
            .output_variables
            .iter()
            .map(|v| v.0)
            .collect::<Vec<_>>();
        dst.write_target_vec(&input_targets)?;
        dst.write_target_vec(&output_targets)
    }

    fn deserialize(_src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
    where
        Self: Sized,
    {
        unimplemented!("Hint functions are not deserializable through the plonky2 crate, only directly through the witness registry")
    }
}

#[derive(Debug)]
pub struct HintSerializer<F, const D: usize> {
    pub hint_fn: fn(&mut ElementBuffer<F, D>) -> Vec<F>,
}

impl<F, const D: usize> HintSerializer<F, D> {
    pub fn new(hint_fn: fn(&mut ElementBuffer<F, D>) -> Vec<F>) -> Self {
        Self { hint_fn }
    }

    pub fn id(&self) -> String {
        format!("--Hint, fn :{:?}", self.hint_fn).to_string()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Serializer<F, WitnessGeneratorRef<F, D>, D>
    for HintSerializer<F, D>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<WitnessGeneratorRef<F, D>> {
        let inputs = buf
            .read_target_vec()?
            .into_iter()
            .map(Variable)
            .collect::<Vec<_>>();
        let outputs = buf
            .read_target_vec()?
            .into_iter()
            .map(Variable)
            .collect::<Vec<_>>();

        let hint = Hint::<F, D> {
            input_variables: inputs,
            output_variables: outputs,
            hint_fn: self.hint_fn,
        };

        Ok(WitnessGeneratorRef::new(hint.adapter()))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &WitnessGeneratorRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::circuit::serialization::{GateRegistry, WitnessGeneratorRegistry};
    use crate::prelude::{ByteVariable, *};

    fn plus_one<F: RichField + Extendable<D>, const D: usize>(
        input_buffer: &mut ElementBuffer<F, D>,
    ) -> Vec<F> {
        let byte: u8 = input_buffer.read::<ByteVariable>();

        ByteVariable::elements(byte + 1)
    }

    #[test]
    fn test_hint_serialization() {
        let mut builder = CircuitBuilderX::new();

        let a = builder.read::<ByteVariable>();
        let b = builder.init::<ByteVariable>();

        let input_values = a.variables();
        let mut input_buffer = VariableBuffer::new(&input_values);

        builder.hint(&mut input_buffer, &b.variables(), plus_one);
        builder.write(b);

        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<ByteVariable>(5u8);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let byte_plus_one = output.read::<ByteVariable>();
        assert_eq!(byte_plus_one, 6u8);

        // Test the serialization
        let gate_serializer = GateRegistry::new();
        let mut generator_serializer = WitnessGeneratorRegistry::new::<PoseidonGoldilocksConfig>();
        generator_serializer.register_hint(plus_one);
        circuit.test_serializers(&gate_serializer, &generator_serializer);
    }
}
