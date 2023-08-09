use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};

use super::{BoolVariable, BytesVariable, ByteVariable};
use super::bytes::WitnessMethods;
use crate::utils::{le_bits_to_bytes, bytes_to_bits};
// use crate::impl_variable_methods;
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);
