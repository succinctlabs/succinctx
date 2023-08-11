use crate::vars::BytesVariable;

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub BytesVariable<48>);

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub BytesVariable<20>);
