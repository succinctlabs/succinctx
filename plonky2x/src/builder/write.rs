use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable, BytesVariable};

pub trait Write<F: RichField + Extendable<D>, const D: usize> {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[ByteVariable], builder: &mut CircuitBuilder<F, D>) -> usize;

    fn write_byte(&mut self, byte: ByteVariable, builder: &mut CircuitBuilder<F, D>) {
        let n = self.write(&[byte], builder);
        assert_eq!(n, 1);
    }

    fn write_bytes<const N: usize>(
        &mut self,
        bytes: BytesVariable<N>,
        builder: &mut CircuitBuilder<F, D>,
    ) {
        let n = self.write(&bytes.0, builder);
        assert_eq!(n, N);
    }

    fn write_bytes32(&mut self, bytes: Bytes32Variable, builder: &mut CircuitBuilder<F, D>) {
        self.write_bytes(bytes.0, builder);
    }
}

#[derive(Debug, Clone)]
pub struct OutputWriter<F: RichField + Extendable<D>, const D: usize> {
    output: Vec<ByteVariable>,
    _marker: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> OutputWriter<F, D> {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Write<F, D> for OutputWriter<F, D> {
    fn write(&mut self, buf: &[ByteVariable], _builder: &mut CircuitBuilder<F, D>) -> usize {
        let n = buf.len();
        self.output.extend_from_slice(buf);
        n
    }
}
