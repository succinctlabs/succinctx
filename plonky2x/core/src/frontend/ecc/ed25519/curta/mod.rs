use curta::chip::ec::edwards::ed25519::params::Ed25519;

pub mod accelerator;
pub mod builder;
pub mod request;
pub mod result_hint;

type Curve = Ed25519;
