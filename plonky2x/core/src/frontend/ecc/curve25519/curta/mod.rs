use curta::chip::ec::edwards::ed25519::params::{Ed25519, Ed25519ScalarField};

pub mod accelerator;
pub mod air_parameters;
pub mod builder;
pub mod proof_hint;
pub mod request;
pub mod result_hint;
pub mod stark;

type Curve = Ed25519;
type ScalarField = Ed25519ScalarField;
