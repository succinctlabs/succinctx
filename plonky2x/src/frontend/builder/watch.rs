use log::{log, Level};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{IoResult, Read, Write};

use super::CircuitBuilder;
use crate::backend::config::PlonkParameters;
use crate::prelude::Variable;

#[derive(Debug, Clone)]
pub struct WatchGenerator<V: Variable> {
    pub variable: V,
    pub log: String,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn watch<V: Variable>(&mut self, variable: &V, log: &str) {
        let variable = variable.clone();
        let log = String::from(log);

        let generator = WatchGenerator { variable, log };
        self.add_simple_generator(generator);
    }
}

impl<V: Variable> WatchGenerator<V> {
    pub fn id() -> String {
        format!("WatchGenerator{}", std::any::type_name::<V>())
    }
}

impl<F: RichField + Extendable<D>, V: Variable, const D: usize> SimpleGenerator<F, D>
    for WatchGenerator<V>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.variable.targets()
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        let log_bytes = self.log.as_bytes();
        dst.write_usize(log_bytes.len())?;
        dst.write_all(log_bytes)?;
        let targets = self.variable.targets();
        dst.write_target_vec(&targets)
    }

    fn deserialize(
        src: &mut plonky2::util::serialization::Buffer,
        _common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<Self>
    where
        Self: Sized,
    {
        let log_size = src.read_usize()?;
        let mut log_bytes = vec![0u8; log_size];
        src.read_exact(&mut log_bytes)?;
        let log = String::from_utf8(log_bytes).unwrap();

        let targets = src.read_target_vec()?;
        let variable = V::from_targets(&targets);

        Ok(Self { variable, log })
    }

    fn run_once(&self, witness: &PartitionWitness<F>, _out_buffer: &mut GeneratedValues<F>) {
        let value = self.variable.get(witness);
        log!(Level::Info, "Variable {} was set to {:?}", self.log, value);
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::types::Field;

    use crate::frontend::builder::CircuitBuilderX;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    #[test]
    fn test_watcher() {
        setup_logger();

        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<FieldVariable>();
        let b = builder.read::<FieldVariable>();
        let c = builder.add(a, b);
        builder.watch(&c, "c");
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<FieldVariable>(GoldilocksField::TWO);
        input.write::<FieldVariable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<FieldVariable>();
        println!("{}", sum.0);
    }
}
