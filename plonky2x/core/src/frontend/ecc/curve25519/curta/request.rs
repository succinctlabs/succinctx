use curta::chip::ec::EllipticCurve;
use serde::{Deserialize, Serialize};

use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::field::variable::FieldVariable;
use crate::prelude::U256Variable;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum EcOpRequestType {
    Add,
    ScalarMul,
    Decompress,
    IsValid,
}

/// A request for a EC OP computation.
#[derive(Debug, Clone)]
pub enum EcOpRequest<E: EllipticCurve> {
    /// Add
    Add(Box<AffinePointVariable<E>>, Box<AffinePointVariable<E>>),
    /// Scalar Mul
    ScalarMul(Box<U256Variable>, Box<AffinePointVariable<E>>),
    /// Decompress
    Decompress(Box<CompressedEdwardsYVariable>),
    /// IsValid
    IsValid(Box<AffinePointVariable<E>>),
}

#[derive(Debug, Clone)]
pub enum EcOpResponse<E: EllipticCurve> {
    Add(AffinePointVariable<E>),
    ScalarMul(AffinePointVariable<E>),
    Decompress(AffinePointVariable<E>, FieldVariable<E::BaseField>),
    IsValid,
}

impl<E: EllipticCurve> EcOpRequest<E> {
    /// Returns the type of the request.
    pub const fn req_type(&self) -> EcOpRequestType {
        match self {
            EcOpRequest::Add(_, _) => EcOpRequestType::Add,
            EcOpRequest::ScalarMul(_, _) => EcOpRequestType::ScalarMul,
            EcOpRequest::Decompress(_) => EcOpRequestType::Decompress,
            EcOpRequest::IsValid(_) => EcOpRequestType::IsValid,
        }
    }
}
