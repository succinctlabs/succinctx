use log::{log, Level};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{IoResult, Read, Write};

use super::CircuitBuilder;
use crate::prelude::CircuitVariable;

#[derive(Debug, Clone)]
pub struct WatchGenerator<V: CircuitVariable> {
    pub variables: Vec<V>,
    pub log: String,
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn watch<V: CircuitVariable>(&mut self, variable: &V, log: &str) {
        let variable = variable.clone();
        let log = String::from(log);

        let generator = WatchGenerator {
            variables: vec![variable],
            log,
        };
        self.add_simple_generator(&generator);
    }

    pub fn watch_array<V: CircuitVariable>(&mut self, variables: &[V], log: &str) {
        let variables = variables.to_vec();
        let log = String::from(log);

        let generator = WatchGenerator { variables, log };
        self.add_simple_generator(&generator);
    }
}

impl<F: RichField + Extendable<D>, V: CircuitVariable, const D: usize> SimpleGenerator<F, D>
    for WatchGenerator<V>
{
    fn id(&self) -> String {
        "WatchGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.variables
            .iter()
            .flat_map(|x| x.targets())
            .collect::<Vec<Target>>()
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!();
        // let log_bytes = self.log.as_bytes();
        // dst.write_usize(log_bytes.len())?;
        // dst.write_all(log_bytes)?;
        // dst.write_usize(self.variables.len());
        // for i in 0..self.variables.len() {
        //     let targets = self.variables.targets();
        //     dst.write_target_vec(&targets);
        // }
        // dst.write_usize(0) // This is just to get the return type right
    }

    fn deserialize(
        src: &mut plonky2::util::serialization::Buffer,
        _common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<Self>
    where
        Self: Sized,
    {
        todo!();
        // let log_size = src.read_usize()?;
        // let mut log_bytes = vec![0u8; log_size];
        // src.read_exact(&mut log_bytes)?;
        // let log = String::from_utf8(log_bytes).unwrap();

        // let targets = src.read_target_vec()?;
        // let variable = V::from_targets(&targets);

        // Ok(Self { variable, log })
    }

    fn run_once(&self, witness: &PartitionWitness<F>, _out_buffer: &mut GeneratedValues<F>) {
        let values: Vec<V::ValueType<F>> = self.variables.iter().map(|x| x.get(witness)).collect();
        if values.len() == 1 {
            println!("[Watch] {}: {:?}", self.log, values[0]);
        } else {
            println!("[Watch] {}: {:?}", self.log, values);
        }
        // log!(Level::Info, "Variable {} was set to {:?}", self.log, value);
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
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.watch(&c, "c");
        builder.write(c);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
    }
}
