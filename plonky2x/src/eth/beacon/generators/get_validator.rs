// use core::marker::PhantomData;

// use plonky2::field::extension::Extendable;
// use plonky2::field::types::Field;
// use plonky2::hash::hash_types::RichField;
// use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
// use plonky2::iop::target::Target;
// use plonky2::iop::witness::{PartitionWitness, Witness};
// use plonky2::plonk::circuit_data::CommonCircuitData;
// use plonky2::util::serialization::{Buffer, IoResult};

// use crate::eth::beacon::BeaconValidatorVariable;
// use crate::ethutils::beacon::BeaconClient;
// use crate::vars::BoolVariable;

// fn le_bits_to_bytes<const N: usize>(input: [bool; N]) -> [u8; N / 8] {
//     let mut output = [0; N / 8];
//     for i in 0..N {
//         for j in 0..8 {
//             if input[i * 8 + j] {
//                 output[i] |= 1 << j;
//             }
//         }
//     }
//     output
// }

// #[derive(Debug)]
// struct GetBeaconValidatorGenerator<F: RichField + Extendable<D>, const D: usize> {
//     beacon_id: String,
//     header_root: [BoolVariable; 256],
//     proof: Vec<[BoolVariable; 256]>,
//     validator: BeaconValidatorVariable,
//     client: BeaconClient,
//     _phantom: PhantomData<F>,
// }

// impl<F: RichField + Extendable<D>, const D: usize> GetBeaconValidatorGenerator<F, D> {
//     pub fn new(
//         beacon_id: String,
//         header_root: [BoolVariable; 256],
//         validator: BeaconValidatorVariable,
//         proof: Vec<[BoolVariable; 256]>,
//     ) -> GetBeaconValidatorGenerator<F, D> {
//         let client = BeaconClient::new("".to_string());
//         GetBeaconValidatorGenerator {
//             beacon_id,
//             header_root,
//             validator,
//             proof,
//             client,
//             _phantom: PhantomData,
//         }
//     }
// }

// trait PartitionWitnessX<F: Field>: Witness<F>  {
//     fn get_bits<const N: usize>(i1: [BoolVariable; N]) -> [bool; N] {
//         i1.map(|x| )
//     }
// }

// impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
//     for GetBeaconValidatorGenerator<F, D>
// {
//     fn id(&self) -> String {
//         "GetBeaconValidatorGenerator".to_string()
//     }

//     fn dependencies(&self) -> Vec<Target> {
//         vec![]
//     }

//     fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
//         let header_root_bits = self
//             .header_root
//             .map(|x| witness.get_target(x.0 .0) == F::ONE);
//         let header_root = hex::encode(le_bits_to_bytes::<256>(header_root_bits));
//         println!("{}", header_root);
//     }

//     #[allow(unused_variables)]
//     fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
//         todo!()
//     }

//     #[allow(unused_variables)]
//     fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
//         todo!()
//     }
// }
