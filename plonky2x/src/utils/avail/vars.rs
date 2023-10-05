use avail_subxt::primitives::Header;
use codec::{Decode, Encode};
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use sp_core::ed25519::{Public as EdPublic, Signature};
use sp_core::Bytes;

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

#[derive(Clone, Debug, Decode, Encode, Deserialize)]
pub struct Precommit {
    pub target_hash: H256,
    /// The target block's number
    pub target_number: u32,
}

#[derive(Clone, Debug, Decode, Deserialize)]
pub struct SignedPrecommit {
    pub precommit: Precommit,
    /// The signature on the message.
    pub signature: Signature,
    /// The Id of the signer.
    pub id: EdPublic,
}

#[derive(Clone, Debug, Decode, Deserialize)]
pub struct Commit {
    pub target_hash: H256,
    /// The target block's number.
    pub target_number: u32,
    /// Precommits for target block or any block after it that justify this commit.
    pub precommits: Vec<SignedPrecommit>,
}

#[derive(Clone, Debug, Decode)]
pub struct GrandpaJustification {
    pub round: u64,
    pub commit: Commit,
    pub votes_ancestries: Vec<Header>,
}

#[derive(Debug, Encode)]
pub enum SignerMessage {
    DummyMessage(u32),
    PrecommitMessage(Precommit),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedFinalityProof(pub Bytes);

#[derive(Debug, PartialEq, Encode, Decode, Clone, Deserialize)]
pub struct FinalityProof {
    /// The hash of block F for which justification is provided.
    pub block: H256,
    /// Justification of the block F.
    pub justification: Vec<u8>,
    /// The set of headers in the range (B; F] that we believe are unknown to the caller. Ordered.
    pub unknown_headers: Vec<Header>,
}

/// Header related constants and types

/// The batch size for each map job
pub const BATCH_SIZE: usize = 16;
pub const MAX_HEADER_CHUNK_SIZE: usize = 100;
pub const MAX_HEADER_SIZE: usize = MAX_HEADER_CHUNK_SIZE * 128;
pub const NUM_BLAKE2B_CHUNKS: usize = BATCH_SIZE * MAX_HEADER_CHUNK_SIZE;

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
