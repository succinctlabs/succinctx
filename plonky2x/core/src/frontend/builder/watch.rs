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
    pub log_level: Level,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn watch<V: CircuitVariable>(&mut self, variable: &V, log: &str) {
        let variable = variable.clone();
        let log = String::from(log);

        let generator: WatchGenerator<L, D, V> = WatchGenerator {
            variables: vec![variable],
            log,
            log_level: Level::Info,
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator);
    }

    pub fn watch_with_level<V: CircuitVariable>(
        &mut self,
        variable: &V,
        log: &str,
        log_level: Level,
    ) {
        let variable = variable.clone();
        let log = String::from(log);

        let generator: WatchGenerator<L, D, V> = WatchGenerator {
            variables: vec![variable],
            log,
            log_level,
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
            log_level: Level::Info,
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator);
    }

    pub fn watch_slice_with_level<V: CircuitVariable>(
        &mut self,
        variables: &[V],
        log: &str,
        log_level: Level,
    ) {
        let variables = variables.to_vec();
        let log = String::from(log);

        let generator: WatchGenerator<L, D, V> = WatchGenerator {
            variables,
            log,
            log_level,
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

        let log_level_num = match self.log_level {
            Level::Trace => 1,
            Level::Debug => 2,
            Level::Info => 3,
            Level::Warn => 4,
            Level::Error => 5,
        };
        dst.write_usize(log_level_num)?;

        dst.write_usize(self.variables.len())?;
        self.variables
            .iter()
            .try_for_each(|v| dst.write_target_vec(&v.targets()))
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

        let log_num = src.read_usize()?;
        let log_level = match log_num {
            1 => Level::Trace,
            2 => Level::Debug,
            3 => Level::Info,
            4 => Level::Warn,
            5 => Level::Error,
            _ => panic!("Invalid log level"),
        };

        let variables_len = src.read_usize()?;
        let mut variables = Vec::new();

        for i in 0..variables_len {
            let targets = src.read_target_vec()?;
            variables.push(V::from_targets(&targets));
        }

        Ok(Self {
            variables,
            log,
            log_level,
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
        log!(self.log_level, "{}", formatted_log);
    }
}

#[cfg(test)]
mod tests {
    use log::{debug, Level};

    use crate::prelude::*;
    use crate::utils;

    #[test]
    fn test_watcher() {
        utils::setup_logger();

        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.watch_with_level(&b, "b", Level::Debug);
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
        debug!("{}", sum.0);
    }
}
