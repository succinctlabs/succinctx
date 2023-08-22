use std::fmt;
use std::fmt::{Debug};
use crate::vars::{CircuitVariable};
use crate::builder::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::{CommonCircuitData, CircuitData};
use plonky2::util::serialization::{Buffer, IoResult};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

#[derive(Clone)]
struct Generator<F, const D: usize, I, O, ClosureType>
where
    F: RichField + Extendable<D>,
    I: CircuitVariable + Debug,
    O: CircuitVariable + Debug,
    ClosureType: Fn(I::ValueType<F>) -> O::ValueType<F> + Send + Sync + Clone,
{
    input: I,
    output: O,
    closure: ClosureType,
    _phantom: std::marker::PhantomData<F>,
}


impl<F, const D: usize, I, O, ClosureType> Generator<F, D, I, O, ClosureType>
where
    F: RichField + Extendable<D>,
    I: CircuitVariable + Debug,
    O: CircuitVariable + Debug,
    ClosureType: Fn(I::ValueType<F>) -> O::ValueType<F> + Send + Sync + Clone,
{
    fn new(builder: &mut CircuitBuilder<F, D>, input: I, closure: ClosureType) -> Self {
        let output = builder.init::<O>();
        Generator {
            input,
            output,
            closure,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, const D: usize, I, O, ClosureType> Debug for Generator<F, D, I, O, ClosureType>
where
    F: RichField + Extendable<D>,
    I: CircuitVariable + Debug,
    O: CircuitVariable + Debug,
    ClosureType: Fn(I::ValueType<F>) -> O::ValueType<F> + Send + Sync + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Generator")
            .field("input", &self.input)
            .field("output", &self.output)
            // .field("closure", &self.closure)  // We're intentionally skipping the closure.
            // The _phantom field does not contain any data, so it doesn't need to be included.
            .finish()
    }
}

impl<F, const D: usize, I, O, ClosureType> SimpleGenerator<F, D> for Generator<F, D, I, O, ClosureType>
where
    Self: 'static + Send + Sync + fmt::Debug + Clone,
    F: RichField + Extendable<D>,
    I: CircuitVariable + Debug,
    O: CircuitVariable + Debug,
    I::ValueType<F>: Debug,
    O::ValueType<F>: Debug,
    ClosureType: Fn(I::ValueType<F>) -> O::ValueType<F> + Send + Sync + Clone,
{
    fn id(&self) -> String {
        "hint".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        vec![self.input.targets()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, buffer: &mut GeneratedValues<F>) {
        println!("Running hint");
        let input_value = self.input.value(witness);
        println!("Hint: {:?}", input_value);
        let output_value = (self.closure)(input_value);
        println!("         -> {:?}", output_value);
        self.output.set(buffer, output_value);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

pub struct CircuitFunction {
    pub input_bytes: Vec<ByteVariable>,
    pub output_bytes: Vec<ByteVariable>,
    input_hash: Bytes32Variable,
    output_hash: Bytes32Variable,
    builder: CircuitBuilder<GoldilocksField, 2>,
    input_pointer: usize,
    output_pointer: usize,
}

// TODO this is the dream
// let mut builder = CircuitBuilder::<F, D>::new();
// let a = builder.read::<Uint256Variable>();
// let b = builder.read::<Uint256Variable>();
// let c = builder.add::<Uint256Variable>(a, b);
// builder.write::<Uint256Variable>(c);
// let circuit = builder.build::<C, F, D>();
// let function = CircuitFunction::new(circuit);
// function.run();


type F = GoldilocksField;
const D: usize = 2;

impl CircuitFunction {
    pub fn new(len_input: usize, len_output: usize) -> Self {
        let mut builder: CircuitBuilder<GoldilocksField, 2> = CircuitBuilder::<F, D>::new();
        let mut input_bytes = Vec::new();
        let mut output_bytes = Vec::new();
        for _ in 0..len_input {
            input_bytes.push(builder.init::<ByteVariable>());
        }
        for _ in 0..len_output {
            output_bytes.push(builder.init::<ByteVariable>());
        }
        // TODO constraint input_hash = hash(input_bytes)
        // TODO constraint output_hash = hash(output_bytes)
        let mut input_hash = builder.init::<Bytes32Variable>();
        let mut output_hash = builder.init::<Bytes32Variable>();
        CircuitFunction {
            input_bytes,
            output_bytes,
            input_hash,
            output_hash,
            builder,
            input_pointer: 0,
            output_pointer: 0,
        }
    }

    pub fn hint<I, O, ClosureType>(&mut self, input_variable: I, closure: ClosureType) -> O 
    where 
        I: CircuitVariable + 'static,
        O: CircuitVariable + 'static,
        I::ValueType<F>: Debug,
        O::ValueType<F>: Debug,
        ClosureType: Fn(I::ValueType<F>) -> O::ValueType<F> + Send + Sync + Clone + 'static,
    {
        let generator: Generator<F, D, I, O, ClosureType> = Generator::new(&mut self.builder, input_variable, closure);
        self.builder.add_simple_generator(&generator);
        return generator.output;
    }
}

mod test {
    use ethers::types::H256;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    use plonky2::iop::witness::PartialWitness;

    use crate::utils::serializer::{save_circuit, load_circuit};
    use crate::vars::BytesVariable;

    use super::*;
    #[test]
    fn example_hint() {
        let mut function, builder = CircuitFunction::new();
        builder.readUint256()
        
        println!("made new function");
        let closure = |x: H256| {
            let mut y = x.to_fixed_bytes();
            y[31] = 0;
            y[30] = 0;
            y[29] = 0;
            y[28] = 0;
            y[27] = 0;
            y[26] = 0;
            y[25] = 0;
            y[24] = 0;
            H256::from_slice(&y)
        };

        // TODO: this below UX needs to be fixed, it's 3 many layers of wrapper and pretty horrible
        // TODO: implement function.get_input_bytes() to get the clone of the input bytes
        let input_bytes_fixed: [ByteVariable; 32] = function.input_bytes.clone().try_into().unwrap();
        let input_bytes_as_bytes = BytesVariable(input_bytes_fixed);
        let input_bytes_as_bytes32: Bytes32Variable = Bytes32Variable(input_bytes_as_bytes);
        let output: Bytes32Variable = function.hint::<Bytes32Variable, Bytes32Variable, _>(input_bytes_as_bytes32, closure);
        for i in 1..24 {
            for j in 0..8 {
                // TODO we should really fix this hideous mess below
                function.builder.assert_is_equal(input_bytes_fixed[i].0[j].0, output.0.0[i].0[j].0);
            }
        }
        for i in 24..32 {
            for j in 0..8 {
                function.builder.assert_is_zero(output.0.0[i].0[j].0);
            }
        }
        // function.builder.write::<Bytes32Variable>(output);
        // let circuit = function.builder.build();
        // let witness = circuit.generate_witness();
        // let output = circuit.get_output(&witness);
        // assert_eq!(output, 0);
        println!("At the top of creating pw");
        let mut pw: PartialWitness<F> = PartialWitness::new();
        println!("Created setting input_bytes in pw");
        for i in 0..32 {
            function.input_bytes[i].set(&mut pw, i as u8);
        }
        println!("Created setting input_hash in pw");
        type C = PoseidonGoldilocksConfig;
        println!("Building circuit");
        let data = function.builder.build::<C>();
        println!("Built circuit, trying to print witness");
        println!("Input bytes in witness {:?}", input_bytes_as_bytes32.value(&pw));

        // save_circuit(&data, "test_circuit.bin".to_string());
        // let loaded_data: CircuitData<F, C, D> = load_circuit(&"test_circuit.bin".to_string());
        // TODO the below doesn't work since I believe the generators are only run during proving time
        // Because of this, there isn't a great way to debug your generator especially if there's a constraint error
        // println!("Address in witness {:?}", output.value(&pw));
        let proof = data.prove(pw).unwrap();
        // println!("Proof: {:?}", proof);
        // println!("Address in witness {:?}", output.value(&pw));



    }
}