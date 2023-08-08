use plonky2::iop::target::Target;

#[derive(Debug, Clone, Copy)]
pub struct Variable(pub Target);

impl Variable {
    pub fn from_target(value: Target) -> Self {
        Self(value)
    }
}

impl From<Target> for Variable {
    fn from(item: Target) -> Self {
        Self(item)
    }
}
