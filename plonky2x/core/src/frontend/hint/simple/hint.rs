use core::fmt::Debug;

use serde::de::DeserializeOwned;

use super::generator::HintSimpleGenerator;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::{OutputVariableStream, ValueStream, VariableStream};
use crate::prelude::CircuitBuilder;

/// A hint for out of circuit computations.
///
/// A hint can be used to make a computation during witness generation that is not included in the
/// cicuit constraints. For example, this can be used for offloading difficult computations (i.e.
/// field inversion) outside the circuit and constraining the result to be correct.
///
/// ## Example
/// The following example shows how to use a hint that gets a field element and returns the inverse.
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use plonky2x::frontend::vars::ValueStream;
/// # use plonky2x::prelude::*;
/// # use plonky2x::frontend::hint::simple::hint::Hint;
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct InverseHint;
///
/// impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for InverseHint {
///    fn hint(
///        &self,
///        input_stream: &mut ValueStream<L, D>,
///        output_stream: &mut ValueStream<L, D>,
///     ) {
///         let input = input_stream.read_value::<Variable>();
///         let inverse = input.inverse();
///         output_stream.write_value::<Variable>(inverse);
///     }
/// }
/// ```
pub trait Hint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync + serde::Serialize + DeserializeOwned
{
    /// the hint function.
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>);

    /// a unique identifier for this hint.
    ///
    /// By default, this is the type name of the hint. This function should be overwriten in case
    /// type names vary between compilation units.
    fn id() -> String {
        std::any::type_name::<Self>().to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn hint<H: Hint<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        let output_stream = VariableStream::new();

        let generator = HintSimpleGenerator::new(input_stream, output_stream.clone(), hint);
        let hint_id = self.hints.len();
        self.hints.push(Box::new(generator));

        OutputVariableStream::new(hint_id)
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::prelude::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct AddSome {
        amount: u8,
    }

    impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for AddSome {
        fn hint(
            &self,
            input_stream: &mut ValueStream<L, D>,
            output_stream: &mut ValueStream<L, D>,
        ) {
            let a = input_stream.read_value::<ByteVariable>();

            output_stream.write_value::<ByteVariable>(a + self.amount)
        }
    }

    #[test]
    fn test_hint() {
        let mut builder = DefaultBuilder::new();

        let a = builder.read::<ByteVariable>();
        let c = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let hint = AddSome { amount: 2 };
        let output_stream = builder.hint(input_stream, hint);
        let b = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(b);

        let mut input_stream = VariableStream::new();
        input_stream.write(&c);
        let hint = AddSome { amount: 3 };
        let output_stream = builder.hint(input_stream, hint);
        let d = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(d);

        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<ByteVariable>(5u8);
        input.write::<ByteVariable>(1u8);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let byte_plus_two = output.read::<ByteVariable>();
        let c_plus_3 = output.read::<ByteVariable>();
        assert_eq!(byte_plus_two, 7u8);
        assert_eq!(c_plus_3, 4u8);
    }
}
