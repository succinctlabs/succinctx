use core::borrow::Borrow;
use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable, BytesVariable};

pub trait Read<F: RichField + Extendable<D>, const D: usize> {
    /// Reads values into the provided buffer, returning the number of values read.
    fn read(&mut self, buf: &mut [ByteVariable], builder: &mut CircuitBuilder<F, D>) -> usize;

    fn read_byte(&mut self, builder: &mut CircuitBuilder<F, D>) -> ByteVariable {
        let mut buf = [builder.init()];
        let n = self.read(&mut buf, builder);
        assert_eq!(n, 1);
        buf[0]
    }

    fn read_bytes<const N: usize>(
        &mut self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> BytesVariable<N> {
        let mut buf = [builder.init(); N];
        let n = self.read(&mut buf, builder);
        assert_eq!(n, N);
        BytesVariable(buf)
    }

    fn read_bytes32(&mut self, builder: &mut CircuitBuilder<F, D>) -> Bytes32Variable {
        Bytes32Variable(self.read_bytes(builder))
    }
}

#[derive(Debug, Clone)]
pub struct InputReader<F: RichField + Extendable<D>, const D: usize> {
    input: Vec<ByteVariable>,
    index: usize,
    _marker: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> InputReader<F, D> {
    pub fn new<I: Borrow<ByteVariable>>(bytes: impl IntoIterator<Item = I>) -> Self {
        let bytes = bytes.into_iter().map(|b| *b.borrow()).collect::<Vec<_>>();
        Self {
            input: bytes,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Read<F, D> for InputReader<F, D> {
    fn read(&mut self, buf: &mut [ByteVariable], _builder: &mut CircuitBuilder<F, D>) -> usize {
        let n = buf.len();
        let end = self.index + n;
        buf.copy_from_slice(&self.input[self.index..end]);
        self.index = end;
        n
    }
}

// Blanket implementations
impl<F: RichField + Extendable<D>, const D: usize, T: Read<F, D>> Read<F, D> for &mut T {
    fn read(&mut self, buf: &mut [ByteVariable], builder: &mut CircuitBuilder<F, D>) -> usize {
        (**self).read(buf, builder)
    }
}
