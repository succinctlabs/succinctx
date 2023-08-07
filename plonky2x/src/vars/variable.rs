use plonky2::iop::target::Target;

pub struct Variable {
    pub value: Target,
}

impl Variable {
    pub fn from_target(value: Target) -> Self {
        Self { value }
    }
}
