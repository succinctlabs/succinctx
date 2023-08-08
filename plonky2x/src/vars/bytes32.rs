use super::BoolVariable;

#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);

impl Bytes32Variable {
    pub fn to_bits(self, witness: &PartitionWitness<F>) -> [bool; 256] {
        todo!();
    }

    pub fn to_bytes_le(self, witness: &PartitionWitness<F>) {
        todo!();
    }
}

impl From<Target> for Variable {
    fn from(item: Target) -> Self {
        Self(item)
    }
}