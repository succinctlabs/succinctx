use core::fmt::Debug;

use super::hint::Hint;
use crate::backend::config::PlonkParameters;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::*;

pub trait SimpleHint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync
{
    type Input: CircuitVariable;
    type Output: CircuitVariable;

    fn hint(
        &self,
        input: &<Self::Input as CircuitVariable>::ValueType<L::Field>,
    ) -> <Self::Output as CircuitVariable>::ValueType<L::Field>;
}

#[derive(Debug, Clone)]
#[allow(clippy::type_complexity)]
pub struct SimpleHintFn<
    L: PlonkParameters<D>,
    I: CircuitVariable,
    O: CircuitVariable,
    const D: usize,
>(pub fn(&I::ValueType<L::Field>) -> O::ValueType<L::Field>);

#[derive(Debug, Clone)]
pub struct SimpleAsHint<H> {
    pub inner: H,
}

impl<L: PlonkParameters<D>, I: CircuitVariable, O: CircuitVariable, const D: usize> SimpleHint<L, D>
    for SimpleHintFn<L, I, O, D>
{
    type Input = I;
    type Output = O;

    fn hint(
        &self,
        input: &<Self::Input as CircuitVariable>::ValueType<L::Field>,
    ) -> <Self::Output as CircuitVariable>::ValueType<L::Field> {
        (self.0)(input)
    }
}

impl<L: PlonkParameters<D>, H: SimpleHint<L, D>, const D: usize> Hint<L, D> for SimpleAsHint<H> {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let input = input_stream.read_value::<H::Input>();
        let output = self.inner.hint(&input);
        output_stream.write_value::<H::Output>(output);
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn simple_hint<H: SimpleHint<L, D>>(
        &mut self,
        input: &H::Input,
        simple_hint: H,
    ) -> H::Output {
        let input_stream = VariableStream::from_variables(input.variables());

        let hint = SimpleAsHint { inner: simple_hint };

        let output_stream = self.hint(input_stream, hint);

        output_stream.read::<H::Output>(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plus_one(a: &u8) -> u8 {
        a + 1
    }

    #[test]
    fn test_simple_hint() {
        let mut builder = CircuitBuilderX::new();

        let a = builder.read::<ByteVariable>();

        let simple_hint = SimpleHintFn::<_, ByteVariable, ByteVariable, 2>(plus_one);

        let b: ByteVariable = builder.simple_hint(&a, simple_hint.clone());
        builder.write(b);

        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<ByteVariable>(5u8);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let byte_plus_one = output.read::<ByteVariable>();
        assert_eq!(byte_plus_one, 6u8);

        // Test the serialization
        let gate_serializer = GateRegistry::new();
        let mut generator_serializer = WitnessGeneratorRegistry::new();
        generator_serializer.register_simple_hint(simple_hint);
        circuit.test_serializers(&gate_serializer, &generator_serializer);
    }
}
