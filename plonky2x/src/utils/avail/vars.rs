use crate::frontend::ecc::ed25519::curve::curve_types::AffinePoint;
use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
use crate::frontend::ecc::ed25519::gadgets::curve::AffinePointTarget;
use crate::frontend::ecc::ed25519::gadgets::eddsa::EDDSASignatureTarget;
use crate::frontend::uint::uint32::U32Variable;
use crate::prelude::{
    ArrayVariable, ByteVariable, Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters,
    RichField, Variable, Witness, WitnessWrite,
};

/// Justification related constants and types
pub const MAX_AUTHORITY_SET_SIZE: usize = 100;
pub const ENCODED_PRECOMMIT_LENGTH: usize = 53;

pub type Curve = Ed25519;
pub type EDDSAPublicKeyVariable = AffinePointTarget<Curve>;

pub type SignatureValueType<F> = <EDDSASignatureTarget<Curve> as CircuitVariable>::ValueType<F>;

pub struct SimpleJustificationData {
    pub authority_set_id: u64,
    pub signed_message: Vec<u8>,
    pub validator_signed: Vec<bool>,
    pub pubkeys: Vec<AffinePoint<Curve>>,
    pub signatures: Vec<[u8; 64]>,
}

/// Header related constants and types

/// The batch size for each map job
pub const BATCH_SIZE: usize = 12;
pub const MAX_HEADER_CHUNK_SIZE: usize = 100;
pub const MAX_HEADER_SIZE: usize = MAX_HEADER_CHUNK_SIZE * 128;

#[derive(Clone, Debug, CircuitVariable)]
#[value_name(EncodedHeader)]
pub struct EncodedHeaderVariable<const S: usize> {
    pub header_bytes: ArrayVariable<ByteVariable, S>,
    pub header_size: Variable,
}
#[derive(Clone, Debug, CircuitVariable)]
#[value_name(HeaderValueType)]
pub struct HeaderVariable {
    pub block_number: U32Variable,
    pub parent_hash: Bytes32Variable,
    pub state_root: Bytes32Variable,
    pub data_root: Bytes32Variable,
}
