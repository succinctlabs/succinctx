use std::marker::PhantomData;

use log::{log, Level};
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{IoResult, Read, Write};

use super::CircuitBuilder;
use crate::backend::circuit::PlonkParameters;
use crate::prelude::CircuitVariable;

#[derive(Debug, Clone)]
pub struct WatchGenerator<L: PlonkParameters<D>, const D: usize, V: CircuitVariable> {
    pub variables: Vec<V>,
    pub log: String,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn watch<V: CircuitVariable>(&mut self, variable: &V, log: &str) {
        let variable = variable.clone();
        let log = String::from(log);

        let generator: WatchGenerator<L, D, V> = WatchGenerator {
            variables: vec![variable],
            log,
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator);
    }

    pub fn watch_slice<V: CircuitVariable>(&mut self, variables: &[V], log: &str) {
        let variables = variables.to_vec();
        let log = String::from(log);

        let generator: WatchGenerator<L, D, V> = WatchGenerator {
            variables,
            log,
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator);
    }
}

impl<L: PlonkParameters<D>, const D: usize, V: CircuitVariable> WatchGenerator<L, D, V> {
    pub fn id() -> String {
        format!("WatchGenerator{}", std::any::type_name::<V>())
    }
}

impl<L: PlonkParameters<D>, V: CircuitVariable, const D: usize> SimpleGenerator<L::Field, D>
    for WatchGenerator<L, D, V>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.variables
            .iter()
            .flat_map(|x| x.targets())
            .collect::<Vec<Target>>()
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        let log_bytes = self.log.as_bytes();
        dst.write_usize(log_bytes.len())?;
        dst.write_all(log_bytes)?;
        dst.write_usize(self.variables.len())?;
        self.variables
            .iter()
            .map(|v| dst.write_target_vec(&v.targets()))
            .collect()
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut plonky2::util::serialization::Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self>
    where
        Self: Sized,
    {
        let log_size = src.read_usize()?;
        let mut log_bytes = vec![0u8; log_size];
        src.read_exact(&mut log_bytes)?;
        let log = String::from_utf8(log_bytes).unwrap();

        let variables_len = src.read_usize()?;
        let mut variables = Vec::new();

        for i in 0..variables_len {
            let targets = src.read_target_vec()?;
            variables.push(V::from_targets(&targets));
        }

        Ok(Self {
            variables,
            log,
            _phantom: PhantomData,
        })
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        _out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let values: Vec<V::ValueType<L::Field>> =
            self.variables.iter().map(|x| x.get(witness)).collect();
        let formatted_log = if values.len() == 1 {
            format!("[Watch] {}: {:?}", self.log, values[0])
        } else {
            format!("[Watch] {}: {:?}", self.log, values)
        };
        log!(Level::Info, "{}", formatted_log);
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::types::Field;

    use crate::frontend::builder::DefaultBuilder;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    #[test]
    fn test_watcher() {
        setup_logger();

        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.watch(&c, "c");
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
    }
}
