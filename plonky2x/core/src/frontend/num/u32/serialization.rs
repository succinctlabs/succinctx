use alloc::vec::Vec;

use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;

pub trait WriteU32 {
    fn write_target_u32(&mut self, x: U32Target) -> IoResult<()>;
}

impl WriteU32 for Vec<u8> {
    #[inline]
    fn write_target_u32(&mut self, x: U32Target) -> IoResult<()> {
        self.write_target(x.target)
    }
}

pub trait ReadU32 {
    fn read_target_u32(&mut self) -> IoResult<U32Target>;
}

impl ReadU32 for Buffer<'_> {
    #[inline]
    fn read_target_u32(&mut self) -> IoResult<U32Target> {
        // We assume that serialized value is within U32Target's range.
        Ok(U32Target::from_target_unsafe(self.read_target()?))
    }
}
